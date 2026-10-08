//! gray-preset — named working-preference bundles.
//!
//! Port of pi's `preset` extension. Pi presets were JSON config
//! (model/tools/thinking/instructions) applied through host APIs; gray
//! sidecars can't retune the model, so a preset here is a markdown file of
//! working preferences at `~/.gray/presets/<name>.md`. `/preset use <name>`
//! returns `{prompt}` so the harness applies it as a real instruction turn.

use std::io::{BufRead, Write};
use std::path::PathBuf;

use serde_json::{Value, json};

fn manifest() -> Value {
    json!({
        "name": "preset",
        "version": env!("CARGO_PKG_VERSION"),
        "protocol": "1.1",
        "tools": [],
        "commands": ["/preset"],
    })
}

fn presets_dir() -> PathBuf {
    let home = std::env::var_os("GRAY_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".gray")))
        .unwrap_or_else(|| PathBuf::from("."));
    home.join("presets")
}

/// Normalize a preset name to a bare file stem inside `presets/` — rejects
/// separators, dots-only names and anything that could escape the dir.
fn normalize(name: &str) -> Option<String> {
    let stem = name.trim().strip_suffix(".md").unwrap_or(name.trim());
    if stem.is_empty()
        || stem.starts_with('.')
        || stem.contains(['/', '\\'])
        || !stem.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return None;
    }
    Some(stem.to_string())
}

fn list_names() -> Vec<String> {
    let Ok(rd) = std::fs::read_dir(presets_dir()) else {
        return Vec::new();
    };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("md") {
                p.file_stem()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
}

/// `/preset …` → the command/run result: `{text}` to say, `{prompt}` to run
/// as a user instruction turn. `argv` excludes the command name.
fn run_command(argv: &[&str]) -> Value {
    match argv {
        [] | ["list"] => {
            let names = list_names();
            if names.is_empty() {
                json!({"text": format!(
                    "No presets yet. `/preset new <name>` creates {}; fill it in, then `/preset use <name>`.",
                    presets_dir().join("<name>.md").display()
                )})
            } else {
                let mut out = format!("{} preset(s) in {}:\n", names.len(), presets_dir().display());
                for n in &names {
                    out.push_str(&format!("  {n}\n"));
                }
                out.push_str("use one with `/preset use <name>`");
                json!({"text": out})
            }
        }
        ["use", rest @ ..] => {
            let Some(name) = rest.first().and_then(|n| normalize(n)) else {
                return json!({"text": "usage: /preset use <name>"});
            };
            let path = presets_dir().join(format!("{name}.md"));
            match std::fs::read_to_string(&path) {
                Ok(contents) if !contents.trim().is_empty() => json!({
                    "prompt": format!(
                        "Apply these working preferences for the rest of the session:\n{}",
                        contents.trim()
                    )
                }),
                Ok(_) => json!({"text": format!(
                    "Preset \"{name}\" is empty — fill in {} first.",
                    path.display()
                )}),
                Err(_) => {
                    let available = list_names();
                    json!({"text": format!(
                        "Unknown preset \"{name}\". Available: {}",
                        if available.is_empty() { "(none)".into() } else { available.join(", ") }
                    )})
                }
            }
        }
        ["new", rest @ ..] => {
            let Some(name) = rest.first().and_then(|n| normalize(n)) else {
                return json!({"text": "usage: /preset new <name> (letters, digits, -, _, .)"});
            };
            let dir = presets_dir();
            let path = dir.join(format!("{name}.md"));
            let existed = path.exists();
            if let Err(e) = std::fs::create_dir_all(&dir).and_then(|_| {
                if existed { Ok(()) } else { std::fs::write(&path, b"") }
            }) {
                return json!({"text": format!("couldn't create preset: {e}")});
            }
            json!({"text": format!(
                "Preset \"{name}\" {} at {} — write your working preferences there, then `/preset use {name}`.",
                if existed { "already exists" } else { "created" },
                path.display()
            )})
        }
        _ => json!({"text": "usage: /preset [list] | /preset use <name> | /preset new <name>"}),
    }
}

/// One request → `Some(reply)`, or `None` for notifications. The bool asks
/// the loop to exit after writing the reply.
fn handle(req: &Value) -> (Option<Value>, bool) {
    let id = req.get("id").cloned();
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let Some(id) = id else {
        return (None, method == "plugin/shutdown");
    };
    let result = match method {
        "plugin/manifest" => manifest(),
        "command/run" => {
            let argv: Vec<&str> = params
                .get("argv")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            run_command(&argv)
        }
        "plugin/shutdown" => return (Some(json!({ "id": id, "result": {} })), true),
        _ => {
            let error = json!({ "code": -32601, "message": "method not found" });
            return (Some(json!({ "id": id, "error": error })), false);
        }
    };
    (Some(json!({ "id": id, "result": result })), false)
}

fn main() -> std::io::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("manifest") {
        println!("{}", manifest());
        return Ok(());
    }
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        let Ok(req) = serde_json::from_str::<Value>(&line) else { continue };
        let (reply, exit) = handle(&req);
        if let Some(reply) = reply {
            writeln!(stdout, "{reply}")?;
            stdout.flush()?;
        }
        if exit {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(method: &str, params: Value) -> Value {
        handle(&json!({ "id": 1, "method": method, "params": params })).0.unwrap()
    }

    /// Serialized: GRAY_HOME is process-global and tests share the process.
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    static CTR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    fn with_home(body: impl FnOnce(&std::path::Path)) {
        let _g = LOCK.lock().unwrap();
        let n = CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("gray-preset-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("GRAY_HOME", &dir) };
        body(&dir);
        unsafe { std::env::remove_var("GRAY_HOME") };
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn preset(argv: &[&str]) -> Value {
        call("command/run", json!({"name": "/preset", "argv": argv, "session": {"id": "s", "cwd": "/tmp"}}))
    }

    #[test]
    fn manifest_has_preset_command() {
        let m = call("plugin/manifest", Value::Null)["result"].clone();
        assert_eq!(m["name"], "preset");
        assert_eq!(m["commands"], json!(["/preset"]));
        assert_eq!(m["tools"], json!([]));
    }

    #[test]
    fn new_list_use_flow() {
        with_home(|home| {
            // Empty list hints at /preset new.
            let r = preset(&[]);
            assert!(r["result"]["text"].as_str().unwrap().contains("No presets"));

            // new creates the file and points the user at it.
            let r = preset(&["new", "plan"]);
            let text = r["result"]["text"].as_str().unwrap();
            assert!(text.contains("created") && text.contains("plan.md"));
            let file = home.join("presets/plan.md");
            assert!(file.exists());

            // new on an existing file doesn't clobber.
            std::fs::write(&file, "Read-only mode. Do not edit files.").unwrap();
            let r = preset(&["new", "plan"]);
            assert!(r["result"]["text"].as_str().unwrap().contains("already exists"));
            assert_eq!(std::fs::read_to_string(&file).unwrap(), "Read-only mode. Do not edit files.");

            // list shows it.
            let r = preset(&["list"]);
            assert!(r["result"]["text"].as_str().unwrap().contains("plan"));

            // use returns {prompt} carrying the file's contents.
            let r = preset(&["use", "plan"]);
            assert_eq!(
                r["result"]["prompt"],
                "Apply these working preferences for the rest of the session:\nRead-only mode. Do not edit files."
            );

            // Unknown preset says what's available.
            let r = preset(&["use", "nope"]);
            assert!(r["result"]["text"].as_str().unwrap().contains("Unknown preset \"nope\""));
            assert!(r["result"]["text"].as_str().unwrap().contains("plan"));
        });
    }

    #[test]
    fn empty_preset_does_not_prompt() {
        with_home(|_| {
            preset(&["new", "blank"]);
            let r = preset(&["use", "blank"]);
            assert!(r["result"]["prompt"].is_null());
            assert!(r["result"]["text"].as_str().unwrap().contains("empty"));
        });
    }

    #[test]
    fn names_cant_escape_dir() {
        assert_eq!(normalize("plan"), Some("plan".into()));
        assert_eq!(normalize("plan.md"), Some("plan".into()));
        assert_eq!(normalize("../evil"), None);
        assert_eq!(normalize("a/b"), None);
        assert_eq!(normalize(".."), None);
        assert_eq!(normalize(""), None);
        with_home(|_| {
            let r = preset(&["new", "../evil"]);
            assert!(r["result"]["text"].as_str().unwrap().contains("usage:"));
        });
    }

    #[test]
    fn shutdown_replies_then_exits() {
        let (reply, exit) = handle(&json!({ "id": 2, "method": "plugin/shutdown" }));
        assert!(reply.is_some() && exit);
    }
}

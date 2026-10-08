# gray-preset

Named working-preference bundles — `/preset` command. Port of pi's `preset`
extension.

## Commands

- `/preset` or `/preset list` — list presets
- `/preset use <name>` — returns `{prompt}`, so the harness applies the
  preset's contents as a real instruction turn for the rest of the session
- `/preset new <name>` — creates an empty preset file and tells you the path
  to fill in (no editors opened)

## Presets

Markdown files at `~/.gray/presets/<name>.md` (honors `$GRAY_HOME`). Write
whatever working preferences you want the agent to follow — pi shipped pairs
like `plan` (read-only, explore, output a structured plan) and `implement`
(tight surgical edits, run tests).

Pi presets were JSON config (model/thinking/tools/instructions) applied
through host APIs. Gray sidecars can't retune the model or tool set from the
wire, so this port applies the *instructions* half via `command/run`'s
`{prompt}` result — equivalent to pasting the file into the chat yourself.

## Wire methods

`plugin/manifest`, `command/run`, `plugin/shutdown`. Protocol 1.1. No tools,
no capabilities, no host→sidecar requests.

## Install

```sh
gray plugin install preset
```

## Develop

```sh
cargo test
gray account check      # entry point + manifest handshake
gray account publish    # check → build → release → publish to the gray registry
```

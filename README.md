<div align="center">
  <img alt="gray-preset" src="assets/icon.svg" width="120" height="120" />
  <h1>gray-preset</h1>
  <p><strong>Named bundles of working preferences, applied to the current session.</strong></p>
  <p>
    <a href="https://gray.alignment.id">Website</a> ·
    <a href="https://gray.alignment.id/plugins/gray-preset">Store</a> ·
    <a href="https://github.com/vstaln/gray-preset">Source</a> ·
    <a href="https://github.com/vstaln/gray">gray</a>
  </p>
  <p>
    <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-1c1c20?style=flat-square&labelColor=0a0a0b" /></a>
    <a href="https://www.rust-lang.org"><img alt="Built with Rust" src="https://img.shields.io/badge/built%20with-rust-1c1c20?style=flat-square&labelColor=0a0a0b&logo=rust&logoColor=d4a373" /></a>
    <a href="https://gray.alignment.id/plugins/gray-preset"><img alt="gray plugin" src="https://img.shields.io/badge/gray-plugin-1c1c20?style=flat-square&labelColor=0a0a0b&color=7aa2f7" /></a>
  </p>
</div>

<br/>

```bash
gray plugin install gray-preset
```

Keep named bundles of working preferences and apply one to the current
session with `/preset`.

## Commands

- `/preset` — with `host.ask` granted, opens a "Use which preset?" picker
  (each preset plus "none"). Without an ask channel it falls back to the
  list.
- `/preset list` — list available presets.
- `/preset use <name>` — returns `{prompt}`, so gray applies the preset as
  a real instruction turn for the rest of the session.
- `/preset new <name>` — creates an empty preset file and prints the path
  to fill in.

## Presets

Presets are Markdown files at `~/.gray/presets/<name>.md` (honors
`$GRAY_HOME`). Write whatever standing preferences you want the agent to
follow: exploration style, edit discipline, review checklists, communication
preferences, or project conventions.

## Wire methods

`plugin/manifest`, `command/run`, `plugin/shutdown`, plus sidecar→host
`host/ask` for the bare `/preset` picker. Protocol 1.1. Capability:
`host.ask` (the command degrades to the text list without it).

## Install

```sh
gray plugin install preset
gray plugin capabilities preset --all   # grants host.ask → bare /preset picker
```

## Develop

```sh
cargo test
gray account check      # entry point + manifest handshake
gray account publish    # check → build → release → publish to the gray registry
```

## Tags

`gray` `plugin` `preset` `rust`

---
Part of the [gray](https://github.com/vstaln/gray) plugin ecosystem —
the open-source AI agent harness. <https://gray.alignment.id>

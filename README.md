<p align="center">
  <img src="assets/gray-logo.svg" alt="gray" width="96">
</p>
<h1 align="center">gray-preset</h1>
<p align="center">Named session presets and working-preference bundles.</p>
<p align="center">
  <a href="https://github.com/vstaln/gray-preset/blob/main/LICENSE"><img alt="MIT License" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img alt="gray plugin" src="https://img.shields.io/badge/gray-plugin-7aa2f7.svg">
  <img alt="rust" src="https://img.shields.io/badge/built%20with-rust-orange.svg">
</p>

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

---
Part of the [gray](https://github.com/vstaln/gray) plugin ecosystem —
the open-source AI agent harness. <https://gray.alignment.id>

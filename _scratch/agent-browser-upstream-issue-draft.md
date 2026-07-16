# Clarify that `mobile` is an MCP profile, not a bundled skill

## Problem

In agent-browser 0.30.0, the README lists `mobile` under MCP tool profiles near
the bundled-skills documentation. The skill registry does not contain a
`mobile` skill, so an agent can reasonably try to load one and hit an error.

## Reproduction

```console
$ agent-browser --version
agent-browser 0.30.0

$ agent-browser skills list
agentcore
core
dogfood
electron
slack
vercel-sandbox

$ agent-browser skills get mobile
✗ Skill not found: mobile
```

Mobile emulation itself is available through the core CLI:

```bash
agent-browser set viewport 390 844 3
agent-browser set device "iPhone 12"
```

## Expected

Please make the README and core skill explicit that `mobile` is an MCP tools
profile, not a name accepted by `agent-browser skills get`. Alternatively,
ship a bundled `mobile` skill and include it in `skills list`.

The smallest documentation fix would add the supported viewport/device
commands to the Skills section or core skill and state that no separate mobile
skill is required.

## Environment

- agent-browser 0.30.0
- macOS arm64

> Draft only. Do not post without Trey Goff's approval.

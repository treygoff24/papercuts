# Align agent-browser advertised skills with the installed CLI

> Draft only. Do not post without Trey approval. Papercut: `pc_a96389c3cc68`; Family: `agent-browser-contract-drift`; Proposed upstream owner: agent-browser bundled skills/docs

## Problem

```text
agent-browser core advertises a mobile specialized skill, but 'agent-browser skills get mobile' returns Skill not found; document the actual viewport workflow or ship the skill.
```

The installed agent-browser README advertises a bundled mobile skill that the actual 0.30.0 skill registry does not ship. The documented mobile workflow is therefore unreachable by name.

## Requested change

Upstream agent-browser should either ship/version the mobile skill or remove it from README; core should explicitly show `agent-browser set viewport` and `set device` as the supported workflow.

## Evidence

1. Live `agent-browser skills list` exposes agentcore, core, dogfood, electron, slack, and vercel-sandbox only.
2. Live `agent-browser skills get mobile` exits 1 with `Skill not found: mobile`.
3. Installed README still lists `mobile: Viewport/device/geolocation/media, touch, swipe...`.

## Constraints

Documentation must stay generated from the same registry as `skills list` to prevent repeat drift.

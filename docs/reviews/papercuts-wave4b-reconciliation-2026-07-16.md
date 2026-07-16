# Papercuts Wave 4b reconciliation

## Status

Completed. The live doctor exits 0 with two acknowledged harness-specific
divergences and no unacknowledged findings. No changes were made to
`~/.claude-shared/skills.globals`.

## Named items

- **browser-use:** Replaced both divergent copies with the same deactivated
  redirect stub. The stub declares
  `requires: {executables: [agent-browser]}` and directs agents to
  `agent-browser skills get core`. The skill remains library-only, not global
  or project-active.
- **Impeccable:** Replaced the stale 3.6.0 pooled copy, including its
  `.claude/skills` references, with the authoritative 3.9.1 copy recorded in
  `~/.agents/.skill-lock.json`. Global membership was preserved unchanged.
- **exa-agent skill:** Did not install the staged Wave 5 skill. All installed
  `exa-agent-cli` surfaces resolve to
  `~/.agents/skill-library/exa-agent-cli` and share SKILL.md SHA-256
  `25ff5f45945e07feb5ac1f537f33e016e35abec12a819a4ac4d38409a18d2cff`.
  The staged `remediation-wave5` skill has SHA-256
  `20bb8e23ebafc79b6c66ac80b820da8f3664223d4db77bd8b62d6865a5ced2bf`.
  The live binary reports `exa-agent 0.2.0`, not the coordinator context's
  0.3.0. Installed copies do not diverge from each other, so no allowlist entry
  was added. Reconcile only after Trey authorizes the Wave 5 merge and binary
  installation.
- **agent-browser mobile skill/README drift:** Drafted, but did not post,
  `_scratch/agent-browser-upstream-issue-draft.md`. Live 0.30.0 evidence shows
  `mobile` is documented as an MCP profile while `skills get mobile` exits 1;
  the draft asks upstream to clarify the distinction or ship the skill.

## Doctor before and after

| State | Skills | Findings | Duplicate divergences | Scan errors | Unacknowledged |
| --- | ---: | ---: | ---: | ---: | ---: |
| Before | 331 | 32 | 20 | 12 | 32 |
| After | 331 | 2 | 2 | 0 | 0 |

Final command:

```bash
env -u BASH_ENV /Users/treygoff/.local/bin/claude-skill doctor --json
```

Exit: 0.

## Allowlist

- `duplicate-divergence:91251059e4f67486` — The shared copy is Codex-native;
  the Claude copy keeps the Claude-specific user-invoked thermo-nuclear review
  workflow.
- `duplicate-divergence:e9dfd55d3f2d4445` — Amp and Claude copies
  intentionally retain their runner-specific Desloppify overlays.

Allowlist file:
`~/.claude-shared/claude-skill-doctor.allowlist`.

## Reconciled copies

The pooled copy was replaced from the authoritative `~/.agents/skills` copy
for these 17 skills:

- `ui-ux-pro-max`
- `parallel-web-search`
- `ai-seo`
- `setup-matt-pocock-skills`
- `agents`
- `hyperframes`
- `music`
- `tdd`
- `grilling`
- `parallel-cli-setup`
- `speech-engine`
- `writing-great-skills`
- `ad-creative`
- `supabase-postgres-best-practices`
- `hyperframes-cli`
- `prototype`
- `impeccable`

## Trashed originals and backups

Every original below was copied before mutation to
`~/.claude/skill-library/.wave4b-backup-2026-07-16/<original-path>`.
`MANIFEST.tsv` records all 31 original-to-backup mappings.

### Replaced stale directories

| Trashed original | Backup |
| --- | --- |
| `~/.agents/skill-library/ui-ux-pro-max` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/ui-ux-pro-max` |
| `~/.agents/skill-library/parallel-web-search` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/parallel-web-search` |
| `~/.agents/skill-library/ai-seo` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/ai-seo` |
| `~/.agents/skill-library/setup-matt-pocock-skills` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/setup-matt-pocock-skills` |
| `~/.agents/skill-library/agents` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/agents` |
| `~/.agents/skill-library/hyperframes` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/hyperframes` |
| `~/.agents/skill-library/music` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/music` |
| `~/.agents/skill-library/tdd` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/tdd` |
| `~/.agents/skill-library/grilling` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/grilling` |
| `~/.agents/skill-library/parallel-cli-setup` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/parallel-cli-setup` |
| `~/.agents/skill-library/speech-engine` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/speech-engine` |
| `~/.agents/skill-library/writing-great-skills` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/writing-great-skills` |
| `~/.agents/skill-library/ad-creative` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/ad-creative` |
| `~/.agents/skill-library/supabase-postgres-best-practices` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/supabase-postgres-best-practices` |
| `~/.agents/skill-library/hyperframes-cli` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/hyperframes-cli` |
| `~/.agents/skill-library/prototype` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/prototype` |
| `~/.agents/skill-library/impeccable` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/impeccable` |
| `~/.agents/skill-library/browser-use` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/browser-use` |
| `~/.agents/skills/browser-use` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skills/browser-use` |

### Removed broken symlinks

| Trashed original | Backup |
| --- | --- |
| `~/.agents/skill-library/website-to-hyperframes` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/website-to-hyperframes` |
| `~/.agents/skill-library/contribute-catalog` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/contribute-catalog` |
| `~/.agents/skill-library/lottie` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/lottie` |
| `~/.agents/skill-library/animejs` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/animejs` |
| `~/.claude/skill-library/parallel-subagent-discipline` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.claude/skill-library/parallel-subagent-discipline` |
| `~/.agents/skill-library/waapi` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/waapi` |
| `~/.agents/skill-library/zoom-out` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/zoom-out` |
| `~/.agents/skill-library/gsap` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/gsap` |
| `~/.agents/skill-library/diagnose` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/diagnose` |
| `~/.agents/skill-library/three` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/three` |
| `~/.agents/skill-library/tailwind` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/tailwind` |
| `~/.agents/skill-library/css-animations` | `~/.claude/skill-library/.wave4b-backup-2026-07-16/Users/treygoff/.agents/skill-library/css-animations` |

## Verification

- Doctor: exit 0; 331 skills; 2 acknowledged findings; 0 unacknowledged.
- `claude-skill search test`: exit 0.
- `claude-skill info impeccable`: exit 0; pooled 3.9.1 copy visible.
- `claude-skill info browser-use`: exit 0; redirect stub and `requires`
  visible; library-only and inactive.
- Backup manifest: 31/31 backup paths present.
- `skills.globals`: SHA-256 unchanged at
  `93eda978f58dffe8d6fc6dc073e8ca489c75619ad51af25d43b4831264890e1a`.

## Papercuts filed

- `pc_38e2b7e9a76d` — macOS `stat` flags failed under the GNU-preferring agent
  shell.
- `pc_d57b22b4cb84` — Wave 4b handoff said exa-agent 0.3.0; live binary is
  0.2.0.
- `pc_c8f20c914ca7` — required `_scratch` artifact needed an undocumented
  force-add because the directory is gitignored.
- `pc_05eec6b832a4` — a `fork_turns=all` review-lane spawn failed because the
  collaboration thread was not found; retrying with a self-contained prompt
  worked.

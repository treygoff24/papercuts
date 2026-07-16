# Wave 1.6 acceptance evidence

Date: 2026-07-15. Scope: live-machine acceptance only. No credential value was printed, no destructive command was executed, no global instruction was edited, and `.papercuts.jsonl` was not changed.

## Execution contract

Each agent-shell command below used exactly `SHELL=/opt/homebrew/bin/bash`, `BASH_ENV=~/.config/agent-shell/agent.sh`, and `/opt/homebrew/bin/bash -c`. The interactive-zsh control is called out separately.

| Label | Result | Exit |
| --- | --- | ---: |
| `shell_identity` | GNU Bash 5.3.15; `sed` and `awk` resolve to Homebrew GNU shims. | 0 |
| `command_paths` | `papercuts=/Users/treygoff/.cargo/bin/papercuts`; `delegate=/Users/treygoff/.local/bin/delegate`; `exa-agent=/Users/treygoff/.cargo/bin/exa-agent`. | 0 |
| `shell_expressions` | Mutable `status=1` and `path=x`: true; `${!a[@]}`: `0 1`; `${v,,}`: `abc`; unmatched glob stayed literal; GNU awk capture: `Y`; GNU `csplit '{*}'`: three parts; GNU `sed -i`: `beta`. Scratch directories were removed with `trash`. | 0 |
| `papercuts_doctor` | Healthy. | 0 |
| `delegate_models` | JSON valid and command successful; full model output was not copied. | 0 |
| `morning_json` | JSON valid and ran in documented read-only mode. | 0 |
| `claude_skill_search` | Successful; 35 output lines and 5,515 bytes. Results were not copied. | 0 |

The documented entrypoint is `/Users/treygoff/bin/morning`, a symlink to `/Users/treygoff/Code/morning/morning.py`. Its own help documents `morning --json` as implying `--no-kill` unless `--kill` is supplied; `toolshed.md` also documents `morning --json` for agents. The exact command already exists, so no duplicate `~/.local/bin/morning` wrapper was added.

## Hook contracts

Live hook sources inspected: `~/.claude-shared/hooks/rm-guard.mjs` and `~/.claude-shared/hooks/guard-subagent-model.mjs` (the latter is a live symlink to `~/Code/fable-meter/hooks/guard-subagent-model.mjs`). Histories and backups were excluded.

| Label | Result | Exit |
| --- | --- | ---: |
| `hook_syntax` | `node --check` passed for both hook files. | 0 |
| `rm_guard_synthetic` | A synthetic PreToolUse Bash payload was denied. The payload was fed to the hook only; no deletion command executed. | 0 |
| `subagent_guard_default_synthetic` | An Agent payload with no model was allowed with model `sonnet`. | 0 |
| `subagent_guard_fable_synthetic` | An untagged Fable Agent payload was denied. | 0 |
| `subagent_guard_registration` | `~/.claude/settings.json`, `~/.claude-work/settings.json`, and `~/.claude-personal/settings.json` each register exactly one `PreToolUse` `Agent` command: `node /Users/treygoff/.claude-shared/hooks/guard-subagent-model.mjs`. | 0 |
| `subagent_guard_registered_command` | The registered command passed the direct synthetic default-allow/rewrite-to-`sonnet` and untagged-Fable-deny contracts. | 0 |

The three profile settings preserve their existing `rm-guard` Bash registration and now each contain the separate `Agent` registration above. `~/.claude-shared/settings.json` remains unchanged and registers neither hook.

## Credential-presence checks

Each profile run first removed the tested names, then loaded the accepted agent shell. The checked names were `SUPABASE_ACCESS_TOKEN`, `OPENROUTER_API_KEY`, `ELEVENLABS_API_KEY`, and `CONTEXT7_API_KEY`. All four were present under the default, `CLAUDE_CONFIG_DIR=/Users/treygoff/.claude-work`, and `CLAUDE_CONFIG_DIR=/Users/treygoff/.claude-personal` states; every run exited 0. No values were printed. The current shell source names `SUPABASE_ACCESS_TOKEN` rather than a generic `SUPABASE` variable.

## Shell separation and Delegate status

The interactive zsh control resolved `sed=/usr/bin/sed` and `awk=/usr/bin/awk`; GNU sed's `--version` flag was rejected. The accepted agent shell resolved Homebrew GNU `sed` and `awk`. This preserves BSD-first interactive zsh and GNU-first agent Bash.

| Harness | State | Run handle | Reason |
| --- | --- | --- | --- |
| Claude work profile | Accepted | `claude-3` / `del_20260715T183236Z_38beca` | Safe run exited 0 with `resultQuality=ok`; it resolved Homebrew Bash 5.3, GNU tools, and the required CLI paths. See `docs/plans/papercuts-wave3-staged-instruction-edits.md`. |
| Codex/Luna | Deferred | `codex-17` / `del_20260715T182351Z_2e70af` | Safe run exited 0 but used zsh and BSD tools; no supported shell knob exists. |
| Cursor/Grok | Deferred | `cursor-5` / `del_20260715T182351Z_aee6d1` | Safe run exited 0 but used zsh and BSD tools. |

## Verification boundary

Wave 1.6 registration and the direct registered-command contract pass. A true native Claude `Agent` end-to-end invocation was intentionally not run because this was a Delegate-only run; no Agent/subagent tool was invoked. That constraint is a verification boundary, not evidence of native end-to-end hook execution.

# Wave 1 staged instruction edits

- Agent shells use Bash 5 with the GNU userland supplied by the agent-only shell layer; write shell snippets with Bash idioms rather than zsh-specific syntax.
- Interactive zsh remains on the shared POSIX environment layer and does not receive the GNU-first PATH.
- Pre-flip BSD-idiom sweep dispositions:
  - `~/.agents/skill-library/cmux-diagnostics/scripts/cmux-diagnostics:58` — fixed: use `/usr/bin/stat` for BSD formatting before its GNU fallback.
  - `~/.agents/skill-library/web-artifacts-builder/scripts/init-artifact.sh:27-32` — fixed: `sed_inplace()` passes GNU `sed -i "$@"` or BSD `/usr/bin/sed -i '' "$@"` as separate arguments; the prior scalar command claim was incorrect. Both branches were exercised on scratch files.
  - `~/.agents/skill-library/embedded-captions/scripts/render-and-composite.sh:268` — fixed: the live `stat -f%z` hit now calls `/usr/bin/stat -f%z` for explicit macOS BSD semantics; scratch-file smoke passed.
  - `~/.local/bin/cmux-dev:6` — fixed: use `/usr/bin/stat` for BSD ownership lookup before its GNU fallback.
  - `~/.local/bin/claude-skill:99` — fixed: use `/usr/bin/stat` for the macOS symlink fallback.
  - `~/.agents/skill-library/internal-report-writing/SKILL.md:84` and `~/.agents/skill-library/agent-ergonomics-and-intuitiveness-maximization-for-cli-tools/references/methodology/CONTINUOUS-IMPROVEMENT.md:48-49` — documentation examples only; no executable change.
  - `~/.agents/skill-library/.pool-backups/**` and `~/.local/bin/claude-skill.bak-20260702T180438:96` — preserved backup copies; no change.
  - `~/.claude-shared/hooks` — no BSD-idiom hits.
- Context7: `~/.ai-profiles/context7.env.zsh` is the currently sourced file. It contains only a POSIX-compatible `export` assignment; `sh -n` and a names-only source smoke passed, so no `.sh` twin was created.
- Shell control-flow inventory: the retained original zshenv controls were cargo env, Supabase, Context7, agent-env secrets, OpenRouter, and ElevenLabs; `env.sh` retains those controls and adds the planned kimi/local/cargo PATH guarantees. The original zsh-only function sources were not imported into the POSIX layer.
- `~/.config/agent-env/load-secrets.sh` now uses function positional parameters only, matches the original `agent-env:<NAME>` keychain service, leaves no `name`/`current`/`value` scratch globals, and unsets its helper after calls. Name/boolean-only checks passed for personal and work Claude profiles.
- `env.sh` and `agent.sh` passed empty/unset-PATH source smokes; repeated `agent.sh` sourcing left each present gnubin directory at most once and in listed order. No global instruction files were changed.

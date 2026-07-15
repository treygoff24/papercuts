# Wave 1 staged instruction edits

- Agent shells use Bash 5 with the GNU userland supplied by the agent-only shell layer; write shell snippets with Bash idioms rather than zsh-specific syntax.
- Interactive zsh remains on the shared POSIX environment layer and does not receive the GNU-first PATH.
- Pre-flip BSD-idiom sweep dispositions:
  - `~/.agents/skill-library/cmux-diagnostics/scripts/cmux-diagnostics:58` — fixed: use `/usr/bin/stat` for BSD formatting before its GNU fallback.
  - `~/.agents/skill-library/web-artifacts-builder/scripts/init-artifact.sh:27-30` — fixed: select `sed -i` syntax with a `sed --version` capability check, not `OSTYPE`.
  - `~/.local/bin/cmux-dev:6` — fixed: use `/usr/bin/stat` for BSD ownership lookup before its GNU fallback.
  - `~/.local/bin/claude-skill:99` — fixed: use `/usr/bin/stat` for the macOS symlink fallback.
  - `~/.agents/skill-library/internal-report-writing/SKILL.md:84` and `~/.agents/skill-library/agent-ergonomics-and-intuitiveness-maximization-for-cli-tools/references/methodology/CONTINUOUS-IMPROVEMENT.md:48-49` — documentation examples only; no executable change.
  - `~/.agents/skill-library/.pool-backups/**` and `~/.local/bin/claude-skill.bak-20260702T180438:96` — preserved backup copies; no change.
  - `~/.claude-shared/hooks` — no BSD-idiom hits.

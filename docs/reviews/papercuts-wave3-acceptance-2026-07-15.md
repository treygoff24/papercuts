# Wave 3 acceptance evidence

## Applied instruction surfaces

- `~/.claude-shared/rules/shell-footguns.md` is the single 15-physical-line Bash-oriented recipe surface (`wc -l`); `~/.claude-work/CLAUDE.md` and `~/.claude/CLAUDE.md` remain symlinks to `~/.claude-shared/CLAUDE.md`.
- `~/.claude-shared/CLAUDE.md` and `~/.codex/AGENTS.md` carry the staged shell, evidence, doctor, and future Delegate wording once; Codex links to the shell rule rather than copying it.

## Local mapping

- `agent-memory/docs/reviews/memora-arc/trey-decision-packet-2026-07-12.md`: exact `memoryd doctor --reindex` repair command and scope.
- `papercuts/AGENTS.md`: callable `re.sub` replacement and one-filter Cargo guidance.
- `Dropbox/Prospera/Policy/pact-act/AGENTS.md`: authored-file review globs and non-Git preflight.
- `gavel/CLAUDE.md`: persistent-session server lifecycle.
- `prospera-radar-build/docs/agent-guidance.md`: staged Wave 7 `test:web:file` guidance and existing typecheck command.

## Open boundaries

- `pc_b8fe2e571b1f` stays open: the live OPM search did not return a verified latest complete part set, so no recipe was authored.
- `pc_b37f54ccfbe6` stays open for Wave 7 because the root `test:web:file` script is not yet implemented.
- `pc_6ffe1c95444b` is resolved through the available append-only log; live process cuts `pc_da645c71b260` and `pc_edd06dd0a7a5` were also atomically resolved after their corrective small-hunk and GNU-sed checks. The other Wave 3 IDs remain open pending recovery of their absent source events: `pc_02430da9ef6d`, `pc_086ff9f44d41`, `pc_0aa764d04f6d`, `pc_0aef5be73d6b`, `pc_2413fc2383b5`, `pc_2a32afa6a5d9`, `pc_3681878d4d1b`, `pc_37830dd5b21e`, `pc_48bcd0653758`, `pc_4ba151a66c8d`, `pc_51e571c07493`, `pc_63ff0a8d6ed3`, `pc_657859fb968d`, `pc_69f47212dc0a`, `pc_6c5b407e3864`, `pc_7a6283b8f24b`, `pc_88e09fdfbb7f`, `pc_8ae5f391206a`, `pc_8d0d40377a6b`, `pc_98be51fc86c0`, `pc_a21c970bd217`, `pc_a385533b3e95`, `pc_abce1276d1ce`, `pc_b61350696e1c`, `pc_c027e2058acb`, `pc_d09a98689667`, `pc_db0db641f6cf`, `pc_e1e215f2bcc1`, `pc_ee1f80f998cb`, `pc_f160678a51cc`, and `pc_f9dba97b97ea`.

## Checks

- `find . -name '*.json' -print0 | xargs -0 -n1 jq empty`, shell-rule concept checks, `zsh -df`, and the GNU `gsed -i` scratch smoke passed. The deferred Codex shell kept BSD `sed`, so the explicit GNU binary is the applicable acceptance check.
- `scripts/check-manifest.sh --diagnostic-only --log .papercuts.jsonl` exited 0 and reported 3/132 frozen-event coverage; a state `PASS` is blocked by missing source logs, not by a manifest inconsistency.
- `cargo build --release`, `cargo test --all-features` (59 tests), `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `git diff --check` passed. No store/concurrency code changed, so the five-run test rule did not apply.

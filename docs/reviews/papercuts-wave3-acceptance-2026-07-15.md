# Wave 3 acceptance evidence

## Applied instruction surfaces

- `~/.claude-shared/rules/shell-footguns.md` is the single Bash-oriented recipe surface (18 physical lines): it covers record-safe reads, orientation/file-matrix loops, awk labels, parent-only AGENTS discovery, and quoted sed ranges.
- `~/.claude-work/CLAUDE.md` and `~/.claude/CLAUDE.md` remain symlinks to `~/.claude-shared/CLAUDE.md`; the shared source carries test discovery, callable `re.sub`, Python set union, Cargo filter, archive-validation, doctor, and Delegate wording once.
- `~/.codex/AGENTS.md` links to the shared shell rules and now exposes archive validation. No global block was duplicated.
- `Dropbox/Prospera/Policy/pact-act/AGENTS.md` now defaults reviews to authored files and excludes `90-archive/**`, `.text-cache/**`, `**/transcripts/**`, `**/*.transcript.md`, and `**/*.raw-transcript.md`; a task must explicitly name transcript material to opt in.

## Complete bounded source-log union

The checker reads the following sorted inventory. Canonical home and top-level repo logs are included first; the three retained Delegate worktree logs are included only because they contain the otherwise absent frozen IDs `pc_8c2350511589`, `pc_df6af25a100a`, `pc_944d374ac9c4`, and `pc_f8eb38d950f5`.

| Source log | Frozen IDs present |
| --- | ---: |
| `$HOME/.papercuts/log.jsonl` | 58 |
| `/Users/treygoff/Code/agent-memory/.papercuts.jsonl` | 3 |
| `/Users/treygoff/Code/claude-space/.papercuts.jsonl` | 11 |
| `/Users/treygoff/Code/delegate-agent/.papercuts.jsonl` | 2 |
| `/Users/treygoff/Code/delegate-worktrees/69a3a224e753/codex-20260711T044830Z_1e145a/.papercuts.jsonl` | 3 |
| `/Users/treygoff/Code/delegate-worktrees/69a3a224e753/codex-20260711T094110Z_69b9fc/.papercuts.jsonl` | 2 |
| `/Users/treygoff/Code/delegate-worktrees/69a3a224e753/codex-20260711T155101Z_6f69ac/.papercuts.jsonl` | 2 |
| `/Users/treygoff/Code/exa-agent-cli/.papercuts.jsonl` | 6 |
| `/Users/treygoff/Code/gavel/.papercuts.jsonl` | 17 |
| `/Users/treygoff/Code/hearth/.papercuts.jsonl` | 9 |
| `/Users/treygoff/Code/papercuts/.papercuts.jsonl` | 3 |
| `/Users/treygoff/Code/prospera-radar-build/.papercuts.jsonl` | 13 |
| `/Users/treygoff/Code/warroom/.papercuts.jsonl` | 6 |

The per-log count is 135 because `pc_a782707f3a97` is present in four recorded sources; the checker folds those duplicate occurrences and proves a 132/132 union. No log was rewritten and no cut event was synthesized.

## Acceptance mapping and resolutions

Each batch below was resolved atomically in its named source log with `papercuts --file <source> resolve <IDs...>`. The Wave 1 shell rows are due because the recorded Claude work-profile Bash/GNU acceptance is established in `docs/plans/papercuts-wave3-staged-instruction-edits.md`; Codex remains explicitly deferred.

| Source / evidence | Frozen IDs resolved |
| --- | --- |
| Claude Bash/GNU acceptance; `$HOME/.papercuts/log.jsonl` | `pc_a5bac2dcb6b8`, `pc_17287b48a152`, `pc_db9e8d6227fa`, `pc_246f9cd9b37b`, `pc_6157c26ecfce` |
| Claude Bash/GNU acceptance; `gavel/.papercuts.jsonl` | `pc_9517658ecfbe`, `pc_ada11ec02666`, `pc_88a67e8cea07` |
| Claude Bash/GNU acceptance; retained Delegate worktree log | `pc_944d374ac9c4` |
| Claude Bash/GNU acceptance; `warroom/.papercuts.jsonl` | `pc_8c05423be42a` |
| Claude Bash/GNU acceptance; `hearth/.papercuts.jsonl` | `pc_bd55a6a719a4` |
| Shared shell rules: loops, URLs, `jq`, awk labels, parent-only AGENTS lookup, named checks, frontmatter, `zsh -df`, and GNU sed; `$HOME/.papercuts/log.jsonl` | `pc_48bcd0653758`, `pc_c027e2058acb`, `pc_657859fb968d`, `pc_b61350696e1c`, `pc_abce1276d1ce`, `pc_2a32afa6a5d9`, `pc_086ff9f44d41`, `pc_2413fc2383b5`, `pc_98be51fc86c0`, `pc_ee1f80f998cb`, `pc_d09a98689667`, `pc_88e09fdfbb7f`, `pc_4ba151a66c8d`, `pc_7a6283b8f24b`, `pc_a21c970bd217`, `pc_37830dd5b21e`, `pc_e1e215f2bcc1` |
| `agent-memory/docs/reviews/memora-arc/trey-decision-packet-2026-07-12.md`: exact `memoryd doctor --reindex` repair command | `pc_3681878d4d1b` |
| `~/.claude-shared/CLAUDE.md`: `seen.update(tokens)` / `seen |= tokens` | `pc_a385533b3e95` |
| `~/.claude-shared/rules/shell-footguns.md`: parent-only `AGENTS.md` discovery while working in `warroom` (no local warroom guidance) | `pc_6c5b407e3864` |
| `~/.claude-shared/CLAUDE.md` test discovery and `shell-footguns.md` safe orientation loop/small-hunk guidance; `hearth/.papercuts.jsonl` | `pc_63ff0a8d6ed3`, `pc_69f47212dc0a`, `pc_02430da9ef6d`, `pc_51e571c07493` |
| `gavel/CLAUDE.md` persistent-session lifecycle plus shared parent-walk, `trash`, and small-hunk guidance | `pc_0aef5be73d6b`, `pc_db0db641f6cf`, `pc_0aa764d04f6d`, `pc_f9dba97b97ea` |
| `~/.claude-shared/CLAUDE.md` and `~/.codex/AGENTS.md` archive guidance; `claude-space/.papercuts.jsonl` | `pc_f160678a51cc` |
| `/Users/treygoff/Code/papercuts/AGENTS.md`: exact one-filter Cargo command | `pc_8ae5f391206a` |
| `papercuts/AGENTS.md`: callable `re.sub`/`re.subn` replacement only | `pc_6ffe1c95444b` (already resolved in its source log) |
| `Dropbox/Prospera/Policy/pact-act/AGENTS.md`: authored review globs with transcript opt-in | `pc_8d0d40377a6b` |

`pc_da645c71b260` is post-snapshot and ignored by the checker. It maps only to the small-hunk `apply_patch` evidence, not callable `re.sub`. The append-only resolve history was not and cannot be rewritten.

## Immutable-note errata

- `pc_8d0d40377a6b` was resolved by the PACT `AGENTS.md` authored-file and transcript exclusion, not by shared shell rules.
- `pc_6ffe1c95444b` proves callable `re.sub` only; `pc_da645c71b260` proves small unique `apply_patch` hunks only. Their shared historical note is overbroad.
- `pc_edd06dd0a7a5` is post-snapshot and maps only to the explicit installed GNU-sed smoke under the documented deferred Codex shell.

Append-only notes are not rewritten. This acceptance artifact is the correction.

## Local guidance map

- Test discovery: `~/.claude-shared/CLAUDE.md`.
- Archive layout: `~/.claude-shared/CLAUDE.md` and `~/.codex/AGENTS.md`.
- Python set union: `~/.claude-shared/CLAUDE.md` and `/Users/treygoff/Code/papercuts/AGENTS.md`.
- awk labels and safe Bash loops: `~/.claude-shared/rules/shell-footguns.md`.
- Cargo filter and callable `re.sub`: `/Users/treygoff/Code/papercuts/AGENTS.md`.
- PACT/Packard authored-review expansion: `/Users/treygoff/Library/CloudStorage/Dropbox/Prospera/Policy/pact-act/AGENTS.md`.

## Open boundaries

- `pc_b8fe2e571b1f` stays open. At `2026-07-15T22:02:18Z`, the bounded live command `exa-agent search "OPM Federal Workforce Data latest employment files complete part set" --include-domain opm.gov --num-results 10 --json` returned 10 OPM results identifying May 2026 data and the historical API index, but did not verify every required part/version for the latest publication. No recipe was authored and no completeness attestation was passed.
- `pc_b37f54ccfbe6` stays open for Wave 7 because the root `test:web:file` script is not implemented.
- Codex shell rows remain open under the manifest's explicit `--defer-harness codex` disposition; no unproven shell acceptance was claimed.

## Checks

- The PACT authored-file expansion returned 1,023 files, zero transcript hits, zero Packard transcript hits, and retained `01-current/american-economic-statecraft/strategy/{master-strategy-v3,movement-architecture-v2,strategy-memo-v4}.md`.
- `scripts/check-manifest.sh --after-wave 3` over the full inventory passes with 132/132 coverage.
- `cargo build --release`, `cargo test --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `git diff --check` are recorded after the final edits. No store/concurrency code changed, so the five-run test rule does not apply.

## Closeout status

- `papercuts`: Wave 3 commit `98efa19` included intentional `.papercuts.jsonl` journal entries, `AGENTS.md`, the manifest, this acceptance artifact, and `model-performance-journal.md`. The current follow-up was uncommitted at authoring time; no push was made.
- Source-log repos: `agent-memory`, all three retained Delegate worktrees, and `hearth` show their append-only `.papercuts.jsonl` as untracked; `claude-space` and `exa-agent-cli` show it modified; `gavel` shows it untracked. Existing dirty paths in those worktrees, including `agent-memory` source/docs, `claude-space` artifacts, `gavel/CLAUDE.md`, and Delegate `docs/perf/`/`scripts/perf/`, were preserved.
- `delegate-agent`, `warroom`, and `prospera-radar-build` were inspected without any unrelated cleanup; their pre-existing states remain as found. The global and Dropbox targets are non-Git directories. No commit or push was made.

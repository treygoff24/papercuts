# Papercuts remediation plan — 2026-07-15

Implementation plan for the 132-cut diagnostic report (`docs/papercuts-diagnostic-report-2026-07-15.md`). The report is the per-cut spec: every cut ID below refers to its detailed finding there. This plan reshapes the report's 87 per-family corrections into eight dependency-ordered waves, replacing ~50 scattered instruction edits with environment- and product-level fixes.

**Decisions already made by Trey (2026-07-15, this session):**
1. Agent shells flip to Homebrew bash 5 + GNU userland ("never fight the weights").
2. Delegate work-mode default isolation changes to worktree.
3. Plan gate = Sol xhigh + Grok safe reviews (in lieu of a Fable gate).

**Standing constraints (from the report, unchanged):** external-symlink containment, credential redaction, `__Host` cookie invariant, and destructive-command guards are never weakened to make a cut disappear. Commits are ungated and per-wave; **no pushes, PRs, tags, or upstream issue posts without Trey's explicit go-ahead, each time.**

---

## Wave 1 — Agent shell environment flip (kills ~16 cuts at the root)

Covers families: `zsh-reserved-status-variable` (5), `zsh-path-special-variable` (2), `zsh-unmatched-glob` (2), `bash-array-indirection-under-zsh` (2), `macos_portable_shell_tools` (3), `unquoted-url-glob-expansion` (1), `invalid-orientation-loop` (1), and defuses `zsh_startup_xtrace_secret_exposure` (1).

**Root cause being fixed:** agent harnesses run `/bin/zsh 5.9` + BSD userland; the models emit bash-on-GNU/Linux. Conform the environment to the weights.

### 1.1 Shared agent-shell profile (the PATH problem)

`~/.bash_profile` is currently 46 bytes (Garry only). Bash agents would lose `~/.local/bin` (papercuts, delegate, claude-skill), `~/.cargo/bin` (exa-agent), and Homebrew. Fix by extraction, not duplication:

- Create `~/.config/agent-shell/env.sh` (chmod 600), the single source of shared env:
  - PATH, in order: GNU gnubin dirs (`coreutils`, `gnu-sed`, `gawk`, `findutils`, `grep` — each guarded with a `[ -d ... ]` test), `/opt/homebrew/bin`, `~/.local/bin`, `~/.cargo/bin`, `~/.garry/bin`, then system PATH.
  - `. "$HOME/.cargo/env"` and `export SUPABASE_ACCESS_TOKEN=...` migrate here from `~/.zshenv` (values move verbatim; file stays 600 like `.zshenv`).
- `~/.zshenv` shrinks to `source ~/.config/agent-shell/env.sh` plus anything zsh-specific.
- `~/.bash_profile` and `~/.bashrc` gain the same source line (keep existing fzf/Garry lines).
- POSIX-sh syntax only in `env.sh` — it must parse under both shells.

### 1.2 GNU userland

- `brew install gawk findutils grep` (coreutils and gnu-sed already installed; verify all five gnubin dirs after install).
- Acceptance: in a fresh agent shell, `sed --version` and `awk --version` report GNU; `csplit` accepts `{*}`.

### 1.3 Claude Code shell flip

- No dedicated shell setting exists (verified via docs 2026-07-15); the Bash tool follows `SHELL`. Add to `~/.claude/settings.json` `env` block: `"SHELL": "/opt/homebrew/bin/bash"`.
- **Empirical acceptance (must pass before proceeding):** fresh Claude Code session runs `ps -o comm= -p $$; echo $BASH_VERSION` → expect bash 5.3.x. If the env block does not influence the Bash tool's shell choice, fallback: launch wrapper (`SHELL=/opt/homebrew/bin/bash claude ...`) via the shell alias/launcher Trey uses, and file an upstream feature-request papercut.
- Never `chsh` — Trey's interactive zsh is untouched throughout this wave.

### 1.4 Codex shell flip (verify-first)

- The knob is unverified. Investigate: `codex exec` docs / `config.toml` reference for shell selection; test whether Codex respects `SHELL` from its parent env.
- If a knob exists: set it. If not: file a papercut (`--tag codex`), leave Codex lanes on zsh, and rely on the Wave 3 footguns doc for that surface. Do not fake it with fragile wrappers.

### 1.5 Delegate lanes

- Delegate spawns child harness CLIs; verify children inherit the launcher's `SHELL` (test one `delegate codex safe` + one `delegate cursor safe` probe printing shell identity).

### 1.6 Regression smoke (the whole point)

Run under a fresh agent bash shell:
- `status=1`, `path=x` assign cleanly; `${!a[@]}` and `${v,,}` work; `echo no-match-*.xyz` passes through literally; `awk 'match($0,/x(y)/,m)'` parses.
- `papercuts doctor`, `delegate --json models` (offline), `morning --json`, `claude-skill search test` all run (PATH intact).
- The machine-wide `rm` block hook and `guard-subagent-model.mjs` still fire (they inspect command strings; verify, don't assume).

### 1.7 Documentation

- Update `~/.claude-work/CLAUDE.md` and `~/.codex/AGENTS.md`: one line stating agent shells are bash 5 + GNU userland, and that shell-guidance snippets are bash-idiom.

**Gate:** all 1.6 checks green. Commit config changes where versioned; record the machine-config changes in this plan's completion notes (dotfiles are not in a repo).

---

## Wave 2 — Papercuts tooling upgrades (this repo)

The report's own diagnostic pass hit the gap 15+ times ("no argv/exit/stderr retained", "complaint-only evidence") and all 7 `needs-repro` dispositions exist because filings dropped evidence. Also: this plan needs to resolve ~110 cuts, and `resolve` takes one ID at a time.

### 2.1 Evidence fields on `add`

- New optional flags: `--cmd TEXT` (the failing command line), `--exit N`, `--stderr-file PATH` (read at filing time, truncated to a bounded size — 4 KiB — stored inline), and freeform `--evidence TEXT`.
- Stored as an optional `evidence` object on the cut record: `{cmd?, exit?, stderr?, note?}`. Additive optional field → **contract stays 1**; `schema` output documents it. Text cap (10 000 bytes) applies to `text` only; evidence has its own bounds.
- ID hash inputs unchanged (evidence is not identity).

### 2.2 Bulk resolve

- `resolve` accepts multiple positional IDs: `papercuts resolve pc_a pc_b pc_c --note "..."` → one resolve record appended per ID, one envelope listing all. All-or-nothing validation up front (any unknown/ambiguous ID fails the whole call before appending).

### 2.3 Resolution-vs-add guard

- `add` warns (exit 0, `meta.warnings`) when text matches `^\s*RESOLUTION|^\s*RESOLVED` — "this looks like a resolution; did you mean `papercuts resolve <id>`?" Non-blocking: append-only ethos, never lose a filing. (From pc_183edfed93b6.)

### 2.4 Read-side duplicate collapse

- `list` deduplicates identical cut IDs within a log (worktree-copy merges produced 9 duplicate events across 141). Last resolution event wins. Verify current `store.rs` behavior first; if it already collapses, document and skip.

### 2.5 Filing guidance

- Update the papercuts section of global CLAUDE.md/AGENTS.md: for tool failures, attach `--cmd`/`--exit`/`--stderr-file`. One sentence, not a treatise.

**Gate:** `cargo test` + `cargo clippy` in this repo; live smoke of each new flag against a scratch `--file`.

---

## Wave 3 — Shell footguns doc + instruction-only consolidation

Depends on Wave 1 (idioms must be bash, not zsh — the report's proposed `*.md(N)` style guidance is now wrong).

### 3.1 One doc, not fifty edits

Create `~/.claude-shared/rules/shell-footguns.md` (also referenced from `~/.codex/AGENTS.md`), hard cap ~40 lines, covering only the survivors that the bash flip does not kill:

- Host-language interpolation: never embed shell `${...}` inside JS template literals / `functions.exec` wrappers; pass plain strings or script files (`nested-host-shell-interpolation`, `functions-exec-malformed-wrapper` — 6 cuts).
- Records and word-splitting: tab-delimited + `IFS=$'\t' read -r`, or parallel arrays (`shell-array-record-splitting`).
- Compound checks print a named failure label before exiting (`shell-compound-check-diagnostics`).
- Prefer `jq` over regexing path-bearing JSON; `rg --no-filename` when extracting (`rg-filename-prefix-path-extraction`).
- Quote every URL (`unquoted-url-glob-expansion` — belt-and-suspenders even under bash).
- Non-git workspace verification preflight: `git rev-parse` first; hash/copy targeted files otherwise (`non-git-workspace-verification` — 3 cuts).
- Upward-only AGENTS.md lookup; never `find ..`/`rg ..` (`bounded-agent-guidance-discovery` — 2 cuts).
- apply_patch: small hunks, shortest unique context, reread after dependency-mutating commands (`apply-patch-context-fragility` — 3 cuts).
- Tested YAML-frontmatter extraction one-liner (`shell-oneoff-parser-error`).
- `zsh -df` for secret-safe shell smokes where zsh is still the target (`zsh_startup_xtrace_secret_exposure`).
- One safe temp-dir cleanup snippet using `trash` (`agent-safety-guard-friction`).

Explicitly **not** implemented: the report's ~25 zsh/BSD-specific instruction proposals (killed by Wave 1) and per-cut one-off snippets (`shell-sed-range-quoting-error`, `python-set-union-oneoff`, `awk-label-file-argument-mixup`, etc.) — those resolve with a note pointing at the doc's principles, no new text.

### 3.2 Deliberately skipped: preflight lint hook

The report floats a shellcheck-style PreToolUse lint. With bash as the agent shell the deterministic-offender list shrinks to near zero. Skip it; revisit only if post-flip papercuts show a new repeating class. (YAGNI, and hooks add per-command latency.)

### 3.3 Remaining instruction-only items outside shell

- `memoryd-reindex-runbook-shorthand`: fix the decision-packet line to the exact `memoryd doctor --reindex` command (one-line edit in agent-memory docs).
- `python-regex-replacement-escaping`, `repo-aware-test-discovery`, `opm-current-file-discovery`, `dropbox-workspace-review-guidance`, `cargo_single_test_filter`, `archive_layout_assumption`, `agent-exec-lifecycle`: fold each into the most local existing doc (workspace runbook / research skill), one or two lines each — not global context.

**Gate:** doc under 45 lines; every covered cut ID listed in a resolution-mapping table appended to this plan.

---

## Wave 4 — Skill library doctor

Covers `browser-use-skill-missing-executable` (3), `skill-path-resolution` (1), `exa-cli-contract-and-doc-drift` (2), `agent-browser-contract-drift` (part), plus the report's repeated "stale duplicate skill copies" risk notes.

### 4.1 `requires:` frontmatter convention

- Skill frontmatter gains optional `requires: {executables: [name...], versionCmd?, minVersion?}`. Documented in the claude-skill README; existing skills without it are unaffected.

### 4.2 `claude-skill doctor`

New subcommand sweeping all active skill surfaces (`~/.claude/skills`, `~/.agents/skills`, `~/.claude-shared/skills.globals` targets):
- `requires.executables` → `command -v` each; missing → finding.
- Duplicate copies of the same skill name across surfaces → content-hash compare; divergent → finding.
- `versionCmd`/`minVersion` declared → run and compare → finding on skew.
- Exit 0 clean / 1 findings; `--json` for agents. Read-only.

### 4.3 Fix current findings

- browser-use: add `requires: {executables: [browser-use]}`; deactivate the skill (or install the CLI — Trey's call at implementation time; default: deactivate, note agent-browser as the working alternative).
- Reconcile the two divergent browser-use copies to one canonical contract.
- Impeccable: delete/sync the stale skill-library copy still referencing `.claude/skills` (pc_f8d120f7e054).
- exa-agent skill: version-pin check wired to Wave 5's regenerated docs.
- agent-browser mobile-skill/README drift: draft upstream issue text (**posting gated on Trey**).

**Gate:** `claude-skill doctor` runs clean after fixes (or with only acknowledged findings).

---

## Wave 5 — exa-agent contract hardening (~/Code/exa-agent-cli)

Covers `exa-contents-text-cap-contract` (4), `exa-positional-urls-schema-drift` (2), `exa-contents-failure-semantics` (2 of 3), `exa_repo_no_live_guard` (1), `exa-json-envelope-path-docs` (1), `exa-source-domain-availability` docs (1), `rust_msrv_gate` (1). Root cause: three hand-maintained representations of one contract (clap defs, generated schema, skill doc).

1. **Single-source generation:** schema and `robot-docs` derive from the clap layer. Positional args represented as `inputKind: "argument"` (keep the legacy `flag` field one release for compat, per report risk note). Numeric ranges (`--text` 1..10000) emitted into schema constraints *and* `--help` text.
2. **Outcome field, not `ok` flip:** total-URL-failure keeps `ok:true`/exit 10 but gains a required top-level `outcome: "no_content" | "partial" | "full"` field. Additive; no client breakage. Empty upstream error objects labeled `upstream_reason_unavailable` with a suggested retry/direct-fetch next action.
3. **`EXA_AGENT_NO_NETWORK=1`** guard at the centralized transport boundary, failing before credential resolution; repo test/docs probes set it.
4. **Skill regeneration:** the skill's command-reference section generated from `robot-docs`; add the live-verified jq path for search results, the quoted-URL example, `--include-domain` as the domain-restriction recipe, and the 10 000 cap in the normal recipe. One live search to verify the JSON path (budgeted, single call).
5. **MSRV job:** CI adds a Rust 1.85 all-features clippy/build job.
6. Parity tests: schema ↔ clap ↔ help assertions so drift fails CI.

**Gate:** `cargo test` + new parity tests + MSRV job green locally; regenerated skill synced to all copies (Wave 4 doctor verifies).

---

## Wave 6 — Delegate hardening (~/Code/delegate-agent)

1. **Work-mode worktree default** (pc_42a766f4dfcc; Trey-approved): `_map_auto_isolation` returns `ISOLATION_WORKTREE` for work mode on known engines (persistent lifecycle already exists). Explicit `--isolation none` remains the real-tree opt-out. Completion output must print the worktree path + merge/handoff instructions. **Same commit:** update delegate docs *and* the delegate section of Trey's global CLAUDE.md ("`work` modes edit an isolated persistent worktree by default; `--isolation none` for the real tree").
2. **Source-root guard** (same cut): deny `trash`/`mv`/`rm` targeting the source root or execution root at the launcher/hook boundary — normal file moves unaffected.
3. **Empty-success result quality** (pc_c7f83ba034d1): when the invocation contract expects a report (safe/call review prompts), blank final text → `resultQuality: "empty"` + one retry with an explicit final-answer request; preserve raw stdout/stderr.
4. **`run-output --tail` implies `--stdout`** (pc_e31354465446); never implicitly select stderr.
5. **Bare-handle resolution UX** (pc_d1a5192425bc): surface workspace, run ID, alias, age; warn on stale (>24 h) matches with `--cwd` guidance.
6. **Safe-workspace skill materialization** (pc_f8eb38d950f5, pc_df6af25a100a): replace external-symlink placeholders with validated read-only *copies* for paths under `.claude/skills`/`.codex/skills` only, bounded size, no nested-link following. Containment stays.
7. **Devin safe preflight** (pc_d741782a7167): fail fast with "filesystem survey unsupported in Devin safe; use another safe harness."
8. **Verify-and-close**: run the existing codex classifier/report regression suite to confirm the 4 `already-fixed` classification cuts, then resolve them.

**Gate:** delegate's own test suite + one live `safe` smoke in a scratch repo confirming worktree default, guard, and result-quality paths.

---

## Wave 7 — Per-repo point fixes (report is the spec; dispositions `fix`)

Grouped by repo; each item cites its cut. Small, independent; good delegate/subagent fan-out candidates after Waves 1–2 land.

- **prospera-radar-build:** E2E-only distDir (pc_f821dfb7ca32); clone-safe E2E worktree helper, no node_modules symlinks (pc_fdd9d446d4c6); document `RADAR_PLAYWRIGHT_PORT` (pc_71fc5d5bea37); one typed read-only ops lookup script covering item/watch-area/entity + publication timing, root package script, key via stdin/secret bootstrap (pc_ae44fb08f5ce, pc_dc81b1ac1f3f, pc_b66b74817bba, pc_aff4d7f9b134); "never source .env.local" + launcher note (pc_8312d8ea11fd); `test:web:file` root script (pc_32ee1733d053).
- **gavel:** `turbopack.root` (pc_26ad9661d970); `impeccable detect --quiet src` with nonzero-on-findings verified (pc_325e89b9af88); local HTTPS VQA launcher, `__Host` invariant untouched (pc_278287fce683).
- **contacts-cli:** lowercase/trim email targets at the touch boundary + regression (pc_dd0267276789); refuse warmth downgrades with structured error, `--force-downgrade --why` escape hatch (pc_ceff5f2fafc5).
- **agent-memory:** dream subcommands' `--runtime` defaults to `<repo>/.memoryd` via the shared resolver, all dream variants + repo-only CLI test (pc_a782707f3a97).
- **x-watch:** `doctor --online` opt-in probe, 401 → `credential_invalid` with portal remediation (pc_b66efae3997d, pc_3d8f55856fe6). **Trey task (~2 min): regenerate the app keys in the X developer portal** — code fix is useless until then.
- **transcribe-url:** regression asserting nonzero exit when no transcript is produced; header hint for event-page → podcast/YouTube mirror resolution (pc_97ff0f165238).
- **ai-profiles:** `ai-profile credential-names` — name/presence/source/equality booleans only, backups/runtime excluded by construction, values never read into output (pc_91911d7ae332, pc_4642e6d76ee3, pc_b95946c00f3d).
- **FEC key** (pc_828f1dfa2edc): **Trey task: create a personal DATA.gov key**; then store in the protected profile, add to the managed credential union, research helper prefers it over DEMO_KEY with explicit 429 handling.
- **Packard:** lane prompts emit absolute corpus paths or a declared-root variable (pc_2f283876d668).
- **Doctor convention (cross-cutting, one paragraph in global guidance):** every local CLI doctor gets an opt-in `--online` probe distinguishing credential-present from credential-valid (x-watch, exa-agent, receipts pattern).

**Gate:** each repo's canonical gate (npm test / cargo test / vitest / acceptance script) run at the coordinator level, not trusted from subagents.

---

## Wave 8 — Verification sweep + log hygiene

1. **`already-fixed` (15 cuts):** run each cited regression/live check from the report's evidence section; resolve with a note naming the check. Any that fail verification get re-dispositioned, not resolved.
2. **No-bug metas:** resolve pc_506af2585d23 + pc_183edfed93b6 as no-bug/meta.
3. **`needs-repro` (7 cuts):** stay open. Add the report's capture recipe to each area's runbook where one exists; the Wave 2 evidence flags are the systemic fix.
4. **`external-upstream` (14 cuts):** draft upstream issue text for hyperframes, agent-browser, gog calendar, Vercel CLI, Codex collaboration/exec-capture. **All posting gated on Trey, per item.** Locally: make the mandated-collaboration research/review skills conditional on a live coordinator thread (pc_08099f2644cd's local half).
5. **Bulk-resolve** everything implemented in Waves 1–7 using the new multi-ID resolve, family by family, note citing wave + commit.
6. **End state:** `papercuts list --status open` drops from 129 to ≈25 (7 needs-repro + 14 external-upstream + residue), each survivor with a reason.

---

## Sequencing, ownership, and risk

- **Order is load-bearing:** 1 → 2 → 3 (doc idioms + resolve tooling depend on 1–2). 4, 5, 6 are independent of each other after 1–2. 7 fans out last; 8 closes.
- **Rollback:** Wave 1 is one env-block line + dotfile edits — revert `SHELL` and the flip is off; `env.sh` stays harmless. Wave 6's default change reverts to `_map_auto_isolation`'s old two lines.
- **Biggest risks:** (a) Claude Code ignoring the `SHELL` env override → 1.3's fallback path; (b) some script on this machine silently depending on BSD sed/awk in agent context → gnubin is agent-PATH only via `env.sh`, and Wave 1.6 smokes the known hooks; (c) delegate worktree default surprising muscle memory → completion-output UX + same-commit doc updates are the mitigation.
- **Trey's two human tasks:** X portal key regeneration; DATA.gov key creation. Everything else is agent-executable.

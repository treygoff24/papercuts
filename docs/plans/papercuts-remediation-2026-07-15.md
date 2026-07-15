# Papercuts remediation plan — 2026-07-15 (r2)

Implementation plan for the 132-cut diagnostic report (`docs/papercuts-diagnostic-report-2026-07-15.md`). The report is the per-cut spec: every cut ID below refers to its detailed finding there. This plan reshapes the report's 87 per-family corrections into eight dependency-ordered waves, replacing ~50 scattered instruction edits with environment- and product-level fixes.

**Decisions already made by Trey (2026-07-15, this session):**
1. Agent shells flip to Homebrew bash 5 + GNU userland ("never fight the weights").
2. Delegate work-mode default isolation changes to worktree.
3. Plan gate = Sol xhigh + Grok + native plan-reviewer (ran 2026-07-15; findings folded into this r2).

**DECISION pending Trey ratification:** Wave 6.1 dirty-tree behavior — recommendation is auto-`--include-dirty` with a loud note (see 6.1).

**Standing constraints (from the report, unchanged):** external-symlink containment, credential redaction, `__Host` cookie requirements, and destructive-command guards are never weakened to make a cut disappear. Commits are ungated and per-wave; **no pushes, PRs, tags, or upstream issue posts without Trey's explicit go-ahead, each time.**

**Instruction-surface ownership rule:** all edits to `~/.claude-work/CLAUDE.md`, `~/.claude/CLAUDE.md`, and `~/.codex/AGENTS.md` across every wave are batched and applied once, by Wave 3, from a staged list. No other wave touches those files directly (prevents collisions under fan-out).

---

## Wave 0 — Disposition manifest (before any implementation)

Produce `docs/plans/papercuts-remediation-manifest.md`: a table of all 132 cut IDs → wave → expected end state (`resolve-on-<wave>`, `resolve-on-verification`, `stays-open-needs-repro`, `stays-open-external`, `stays-open-trey-task`, `already-resolved`). Include a count assertion per wave. A tiny check script (`scripts/check-manifest.sh`) asserts: no open `fix`/`instruction-only` cut is unmapped, and after each wave the actual `papercuts list` counts match the manifest's expectations. This kills approximation drift ("~16", "≈25") — every claim becomes an exact ID list. (Review findings: Sol R14, Grok gate-falsifiability, plan-reviewer count checks.)

---

## Wave 1 — Agent shell environment flip

Targets (exact IDs in the manifest): `zsh-reserved-status-variable` (pc_9517658ecfbe, pc_944d374ac9c4, pc_ff863521e129, pc_211736ebcc47, pc_ada11ec02666), `zsh-path-special-variable` (pc_8c05423be42a, pc_a5bac2dcb6b8), `zsh-unmatched-glob` (pc_17287b48a152, pc_bd55a6a719a4), `bash-array-indirection-under-zsh` (pc_db9e8d6227fa, pc_246f9cd9b37b), `macos_portable_shell_tools` (pc_6157c26ecfce, pc_88a67e8cea07, pc_7892e011944e), `unquoted-url-glob-expansion` (pc_88e09fdfbb7f — partial: bash leaves unmatched patterns literal; quoting guidance stays in Wave 3), `invalid-orientation-loop` (pc_02430da9ef6d — partial: glob-abort eliminated, loop-construction guidance stays in Wave 3). `zsh_startup_xtrace_secret_exposure` (pc_48bcd0653758) is **not** claimed here — its `zsh -df` documentation fix lives in Wave 3.

**Cuts resolve per harness, not per family:** a cut filed from a harness still on zsh stays open until that harness is verified on bash (Sol R3, Grok end-state finding).

### 1.1 Env loading — BASH_ENV is the mechanism, not .bash_profile

Non-interactive `bash -c` (the Bash-tool shape) reads **neither** `.bash_profile` nor `.bashrc` — only `$BASH_ENV`. And the current zsh setup works only because zsh sources `.zshenv` on every invocation. So:

- `~/.config/agent-shell/env.sh` (chmod 600, POSIX sh only) becomes the canonical shared environment: a faithful port of **all** of `~/.zshenv`'s current content — cargo env, `SUPABASE_ACCESS_TOKEN`, the Context7 loader, the agent-env keychain loader, the OpenRouter key loader, the **profile-aware ElevenLabs routing keyed off `CLAUDE_CONFIG_DIR` including its fail-closed unset branches (preserve verbatim semantics)**, the kimi-code PATH stanza — plus explicit PATH guarantees for `~/.local/bin` and `~/.cargo/bin`. **No gnubin here.** Audit `~/.config/agent-env/load-secrets.zsh` for POSIX compatibility before sourcing it from bash; if it is zsh-only, provide a `.sh` twin.
- `~/.zshenv` shrinks to one line sourcing `env.sh` — identical behavior for every existing zsh consumer, interactive included.
- `~/.config/agent-shell/agent.sh` = `. env.sh` **then** prepends the GNU gnubin dirs (coreutils, gnu-sed, gawk, findutils, grep — each `[ -d ]`-guarded). This file is loaded **only** via harness env injection, so **GNU-first PATH is agent-scoped; Trey's interactive zsh never sees it** (Grok B1, plan-reviewer R2).
- Harness injection (Claude Code `~/.claude/settings.json` env block): `"SHELL": "/opt/homebrew/bin/bash"`, `"BASH_ENV": "/Users/treygoff/.config/agent-shell/agent.sh"` (Sol B1, Grok B2, plan-reviewer B1).

### 1.2 GNU userland + BSD-idiom sweep

- `brew install gawk findutils grep` (coreutils, gnu-sed already present); verify all five gnubin dirs.
- **Pre-flip sweep** (Sol R1): `rg` the skill libraries, `~/.claude-shared/hooks`, and `~/.local/bin` scripts for BSD-only idioms — `stat -f`, `sed -i ''`, `date -v`, OSTYPE-gated tool selection. Known hits: `embedded-captions/scripts/render-and-composite.sh` (`stat -f%z`), `web-artifacts-builder/scripts/init-artifact.sh` (Darwin-gated `sed -i ''`). Fix each to capability-detect (`stat --version` probe) or invoke the BSD binary by absolute path. The sweep's hit list and dispositions go in the wave commit message.

### 1.3 Claude Code flip + acceptance

- Apply the 1.1 env block. **Empirical acceptance before anything else proceeds:** fresh Claude Code session runs `ps -o comm= -p $$; echo $BASH_VERSION; command -v sed awk papercuts delegate exa-agent` → bash 5.3.x, gnubin sed/awk, all CLIs resolving. This also covers the login-shell `path_helper` reordering concern — the assertion is on final resolution, not on file mechanics (plan-reviewer R1).
- Fallback if the env block doesn't reach the Bash tool: launcher wrapper (`SHELL=... BASH_ENV=... claude`), plus an upstream feature-request papercut.
- Never `chsh`.

### 1.4 Codex flip (verify-first)

- Investigate the Codex shell-selection knob (config.toml reference; test whether `codex exec` respects parent `SHELL`/`BASH_ENV`). If a knob exists, set it and run the same acceptance probe. If not: file a papercut (`--tag codex`), **Codex-filed shell cuts stay open**, Wave 3 doc covers that surface.

### 1.5 Delegate lanes

- Verify children inherit `SHELL`/`BASH_ENV` (one probe per lane family printing shell identity).

### 1.6 Regression smoke

Under a fresh agent bash shell: `status=1`, `path=x`, `${!a[@]}`, `${v,,}`, unmatched-glob literal passthrough, `awk 'match($0,/x(y)/,m)'`, `csplit` with `{*}`; `papercuts doctor`, `delegate --json models`, `morning --json`, `claude-skill search test`; `sed -i` GNU form on a scratch file; the `rm`-block hook and `guard-subagent-model.mjs` still fire. Presence-only credential checks per profile (names + booleans, never values): SUPABASE, OPENROUTER, ELEVENLABS under both `CLAUDE_CONFIG_DIR` states, context7.

### 1.7 Staged instruction edits

Stage (do not apply — Wave 3 owns the files): agent shells are bash 5 + GNU userland; shell snippets are bash-idiom.

**Gate:** every 1.6 check green; 1.3 acceptance matrix recorded per harness. Dotfile diffs reviewed line-by-line against the 1.1 inventory before replacing `.zshenv`.

---

## Wave 2 — Papercuts tooling upgrades (this repo)

The diagnostic hit the evidence gap 15+ times; all 7 `needs-repro` cuts exist because filings dropped evidence. Resolution volume (~110 IDs) needs multi-ID resolve.

**Repo gate (per AGENTS.md, applies to every item):** `cargo build --release`, `cargo test --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`; **test suite 5× for anything touching `store.rs`** (Sol R9). The normative contract is `docs/plans/2026-07-09-papercuts-design.md` — changes below amend it explicitly (new Amendments entries), never silently.

### 2.1 Evidence fields on `add`

- Optional flags: `--cmd TEXT`, `--exit N`, `--stderr-file PATH` (read at filing time, truncated to 4 KiB, stored inline), `--evidence TEXT`. Stored as optional `evidence: {cmd?, exit?, stderr?, note?}` with `#[serde(skip_serializing_if = "Option::is_none")]` so existing records/emissions are byte-identical (plan-reviewer N4).
- **Identity:** evidence is *not* folded into the ID (dedup-by-text is the feature). Consequence, documented in the design amendment: a duplicate-ID `add` keeps the first record — first-wins, per the shipped fold — so a second filing's differing evidence is not stored; `add` returns the existing ID with a `duplicate_cut` warning naming that (Sol B2, adopted as document-plus-warn rather than a contract-2 identity change: making evidence identity-bearing would turn every re-filing with a different stderr into a new cut and destroy dedup).
- **Redaction boundary (Sol B5):** all evidence inputs pass a deterministic redaction pass at write time — patterns for `key/token/secret/password/authorization/bearer` assignments and long high-entropy strings → `<redacted>`; tests cover common token shapes. Schema documents evidence as best-effort-sanitized; guidance says never pipe raw env dumps.

### 2.2 Multi-ID resolve, back-compat shape

- `resolve` accepts 1..N positional IDs. **Output shape is unchanged for one ID** (`{changed, record}`); two or more IDs return `{changed, records: [...]}`. All-or-nothing validation before any append. Schema + design amendment document both shapes (Sol B3, Grok, plan-reviewer R3 — adopted as an extension of `resolve` rather than a separate `resolve-many` command; one command, two documented arities).

### 2.3 Resolution-vs-add guard

- `add` warns (exit 0, `meta.warnings`) when text matches `^\s*RESOLUTION|^\s*RESOLVED` — suggest `papercuts resolve <id>`. Non-blocking. (pc_183edfed93b6 — the cut itself resolves as meta/no-bug, not as a defect fixed by this guard.)

### 2.4 Duplicate handling: verified, no change

- `store.rs` already collapses duplicate IDs **first-wins** (fold keeps first cut, first resolve — `store.rs:243-261`, `fold_matrix` test). Document in the design doc's fold notes; **do not change fold semantics** (Sol B4, Grok B5, plan-reviewer N1 — r1's "last wins" was wrong).

### 2.5 Staged filing guidance

Stage for Wave 3: one sentence — attach `--cmd/--exit/--stderr-file` when filing tool failures.

---

## Wave 3 — Shell footguns doc + all instruction-surface edits (single owner)

Depends on Waves 1–2. This wave applies **every** staged global-instruction edit (from 1.7, 2.5, 6.x, 7.x) in one batch.

### 3.1 One doc, not fifty edits

`~/.claude-shared/rules/shell-footguns.md` (referenced from `~/.codex/AGENTS.md`), hard cap ~45 lines, bash-idiom, covering the cross-shell survivors:

- Host-language interpolation: never embed shell `${...}` in JS template literals / `functions.exec` wrappers (pc_657859fb968d, pc_37830dd5b21e, pc_e1e215f2bcc1, pc_d09a98689667, pc_98be51fc86c0, pc_086ff9f44d41).
- Records: tab-delimited + `IFS=$'\t' read -r` or parallel arrays (pc_63ff0a8d6ed3).
- Compound checks print a named failure label (pc_c027e2058acb).
- `jq` over regexing path-bearing JSON; `rg --no-filename` (pc_2413fc2383b5); portable `jq -c '.[]'` splitter recipe (belt-and-suspenders for pc_6157c26ecfce per the report's own proposed correction).
- Quote every URL (pc_88e09fdfbb7f).
- Non-git verification preflight (pc_b61350696e1c, pc_2a32afa6a5d9, pc_4ba151a66c8d).
- Upward-only AGENTS.md lookup (pc_6c5b407e3864, pc_0aef5be73d6b).
- apply_patch: small hunks, shortest unique context, reread after dependency mutations (pc_51e571c07493, pc_f9dba97b97ea, pc_a21c970bd217).
- Tested frontmatter-extraction one-liner (pc_abce1276d1ce).
- `zsh -df` for secret-safe zsh smokes (pc_48bcd0653758).
- Safe temp-dir cleanup via `trash` (pc_db0db641f6cf).
- Transition note: GNU sed is first on agent PATH — `sed -i` / `sed -i.bak`, never BSD `sed -i ''`.

### 3.2 Deliberately skipped: preflight lint hook

Post-flip the deterministic-offender list shrinks to near zero. Revisit only if post-flip papercuts show a new repeating class.

### 3.3 Local (non-global) instruction items

- `memoryd-reindex-runbook-shorthand` (pc_3681878d4d1b): exact `memoryd doctor --reindex` command in the decision packet.
- pc_6ffe1c95444b (re.sub callable), pc_69f47212dc0a (test discovery), pc_8d0d40377a6b (Packard review globs), pc_8ae5f391206a (cargo single filter), pc_f160678a51cc (archive layout), pc_0aa764d04f6d (exec lifecycle): one–two lines each in the most local existing doc.
- pc_b8fe2e571b1f (OPM recipe — **tentative**): write the recipe only with a live-verified complete-part-set check at authoring time; if it can't be verified cheaply, leave the cut open (report's tentative-diagnosis boundary; Sol R12 partially adopted).
- pc_b37f54ccfbe6 (radar typecheck guidance): one line in radar's agent-guidance next to the Wave 7 `test:web:file` change (Sol R13, Grok — was unassigned in r1).
- Doctor convention (one paragraph, global): every local CLI doctor gets an opt-in `--online` probe distinguishing credential-present from credential-valid (moved here from Wave 7; Grok N7).

**Gate:** doc ≤45 lines; `scripts/check-manifest.sh` passes with Wave 3 resolutions applied.

---

## Wave 4a — Skill doctor framework  → (Wave 5) →  Wave 4b — reconciliation

r1 had 4↔5 circular; order is now explicit (Sol R8, Grok, plan-reviewer R5).

### 4a.1 `requires:` frontmatter convention

`requires: {executables: [name...], version?: {executable, minVersion}}`. **No `versionCmd` free-form command execution** — the doctor invokes only the declared executable with a fixed `--version` argument, no shell evaluation, sanitized env, timeout (Sol B7: arbitrary frontmatter commands would make a read-only doctor an untrusted-exec surface).

### 4a.2 `claude-skill doctor`

- **Sweep roots derived from claude-skill's own topology** — `~/.claude/skill-library`, `~/.agents/skill-library`, `~/.claude/skills`, `~/.agents/skills`, `~/.claude-shared/skills.globals` targets — deduplicated by resolved realpath (Sol B6, Grok, plan-reviewer R6: the divergent copies live in the skill-library dirs r1 omitted).
- Checks: `requires` executables present; duplicate copies content-hash compared; declared version vs `--version` output. Exit 0/1, `--json`. Findings allowlist file (ID-based) so "acknowledged findings" is falsifiable (Sol R11).

### 4b (after Wave 5) Fix findings + reconcile

- browser-use: add `requires`; default action deactivate, **leaving a stub skill that redirects to agent-browser** (Grok N4); reconcile the two divergent copies to one contract.
- Impeccable: delete/sync the stale copy referencing `.claude/skills` (pc_f8d120f7e054).
- exa-agent skill: verify the Wave 5 regenerated copy is the one installed everywhere.
- agent-browser mobile-skill/README drift: draft upstream issue text (**posting gated on Trey**).

**Gate:** doctor clean or allowlisted, run after Wave 5 lands.

---

## Wave 5 — exa-agent contract hardening (~/Code/exa-agent-cli)

Covers `exa-contents-text-cap-contract` (pc_a1553455a3d4, pc_0ab19b19876d, pc_2df63b1c0880, pc_dc0fd914fe93), `exa-positional-urls-schema-drift` (pc_cc2d338911db, pc_b057ceb4523e), `exa-contents-failure-semantics` (pc_bce78a0aff06; pc_f2720a4950c7 local half), `exa_repo_no_live_guard` (pc_cb37997204ff), `exa-json-envelope-path-docs` (pc_3615c044abbd), `exa-source-domain-availability` docs (pc_d5448baaf2f5 local half), `rust_msrv_gate` (pc_26b94226e075).

1. **Registry as the single source, not clap** (Sol R6 — r1 had this backwards): request-body paths and API metadata already live in the build-time registry (`build.rs`, `src/registry/`). Extend registry field metadata with `inputKind` (`flag` | `argument`), name, arity, and numeric ranges; schema and help render from it; **parity tests assert registry ↔ clap agreement** so drift fails CI. Keep the legacy `flag` key one release for schema consumers.
2. **`--text` cap:** 1..10000 emitted into schema constraints and help text; local rejection before network with `--text full`/`10000` suggestions.
3. **`outcome` field:** required on contents/fetch envelopes (`no_content` | `partial` | `full`); `ok`/exit-10/warning behavior byte-preserved; the written contract (`docs/v2/contracts.md`) updated in the same change, plus **compatibility fixtures capturing the current exact envelope** so strict consumers are tested, not assumed (Sol R7, Grok). Empty upstream error objects labeled `upstream_reason_unavailable` with a suggested retry/direct-fetch next action.
4. **`EXA_AGENT_NO_NETWORK=1`:** guard placed **before every credential-resolution path** (credentials currently resolve before transport — Sol B10), plus a defense-in-depth assertion at the transport send boundary. Repo test/docs probes set it.
5. **Skill regeneration** from robot-docs: live-verified jq path for search results (one budgeted call), quoted-URL example, `--include-domain` recipe, the 10000 cap in the normal recipe.
6. **MSRV:** named local command (`cargo +1.85 clippy --all-features --all-targets`) and the matching CI job — the local run is the gate; CI green is confirmed post-push whenever Trey authorizes one (Sol R11).

**Gate:** `cargo test` + parity tests + compat fixtures + local MSRV command green; regenerated skill staged for Wave 4b.

---

## Wave 6 — Delegate hardening (~/Code/delegate-agent)

1. **Work-mode worktree default** (pc_42a766f4dfcc; Trey-approved) — **workspace-aware**, not engine-blanket (Sol B8, Grok B3, plan-reviewer B2/R4):
   - Auto-isolation maps work → worktree **only when the workspace is a Git repo with a valid HEAD**. Ordinary directories and unborn-HEAD repos keep in-place execution with a one-line warning (the git signal lives in the callers that already know `workspace_kind`, not inside `_map_auto_isolation`'s signature — implementation picks the seam, behavior is what's specified here).
   - **Dirty trees (DECISION for Trey):** recommended default = auto-apply `--include-dirty` with a loud note (preserves "work on my current state" while gaining isolation). Alternative: hard error citing the three outs (the current `dirty_source_workspace` message already names them). Cases tested: clean, staged, unstaged, untracked, submodule-dirty, unborn, non-git.
   - Completion output prints worktree path + merge/handoff instructions. **Retention:** add a worktree inventory/cleanup command with age/size reporting; never auto-delete unmerged work (Sol R5, Grok).
   - Same change updates delegate docs and the **canonical + installed** delegate-agent skill copies ("`work` edits an isolated persistent worktree by default for clean git repos; `--isolation none` for the real tree"); the global CLAUDE.md edit is staged for Wave 3's batch.
2. **Source-root guard, enforceable seams only** (Sol B9, Grok B4 — r1's "launcher boundary" can't see child shell commands): (a) Delegate preflight refuses operations it itself performs against source/execution roots; (b) Delegate exports canonical roots (e.g. `DELEGATE_SOURCE_ROOT`) into child env, and the machine hook family (`rm-guard.mjs` pattern) extends to `trash`/`mv`/`rm` targeting those roots — wired per harness that supports command hooks; (c) harnesses without hook support are a **documented residual risk**, not claimed coverage. Tests: absolute, relative, symlinked, quoted, compound commands.
3. **Empty-success retry policy** (pc_c7f83ba034d1): `resultQuality=empty` already ships (`harness_events.py:103,130`); the missing piece is policy — for safe/call report-contract runs only, retry once with an explicit final-answer request; if still empty, keep `succeeded` + `resultQuality=empty` honest and preserve raw stdout/stderr (Grok N6).
4. **`run-output --tail` implies `--stdout`** (pc_e31354465446); never implicitly stderr.
5. **Bare-handle resolution UX** (pc_d1a5192425bc): surface workspace, run ID, alias, age; stale warning (>24h) with `--cwd` guidance.
6. **Safe-workspace skill materialization** (pc_f8eb38d950f5, pc_df6af25a100a): read-only copies for external-symlink skill entrypoints under `.claude/skills`/`.codex/skills` only — **bounded**: per-file size cap, refuse nested symlinks inside copied trees, denylist `.env`/key/credential filenames; escape-attempt tests (Grok R9).
7. **Devin safe preflight** (pc_d741782a7167): fail fast with the unsupported-survey message.
8. **Verify-and-close the 3 `already-fixed` classification cuts** (pc_344b79d2e28e, pc_9d8218775b5b, pc_b967e7071e47 — r1 said four; the index has three, Sol N1) by running the classifier/report regression suite.

**Gate:** delegate test suite + a live **work-mode** smoke in a scratch repo (r1 wrongly said safe — Sol R10): assert source checkout untouched, edit present in the persistent worktree, handoff output correct; plus a dirty-tree and a non-git work-mode probe matching the specified behaviors.

---

## Wave 7 — Per-repo point fixes (report is the spec; dispositions `fix`)

Grouped by repo; independent; fan-out candidates after Waves 1–2. Each repo's gate is its canonical command set, run at the coordinator: papercuts = AGENTS.md four commands; radar = `npm run typecheck` + targeted tests + playwright smoke where touched; gavel = `npm run gate`; contacts-cli / agent-memory / exa-agent-cli = `cargo test` (+ clippy where configured); x-watch = `npm test`; delegate = pytest (Sol R11 — gates named, not vibed).

- **prospera-radar-build:** E2E-only distDir (pc_f821dfb7ca32); clone-safe E2E worktree helper, deps installed inside the tree (pc_fdd9d446d4c6); document `RADAR_PLAYWRIGHT_PORT` (pc_71fc5d5bea37); one typed read-only ops lookup covering item/watch-area/entity + publication timing, root package script, key via stdin/secret bootstrap (pc_ae44fb08f5ce, pc_dc81b1ac1f3f, pc_b66b74817bba, pc_aff4d7f9b134); "never source .env.local" launcher note (pc_8312d8ea11fd); `test:web:file` root script (pc_32ee1733d053) + the pc_b37f54ccfbe6 guidance line (staged via Wave 3.3).
- **gavel:** `turbopack.root` (pc_26ad9661d970); `impeccable detect --quiet src` with nonzero-on-findings verified (pc_325e89b9af88); local HTTPS VQA launcher, `__Host` invariant untouched (pc_278287fce683).
- **contacts-cli:** lowercase/trim email targets at the touch boundary + regression (pc_dd0267276789); refuse warmth downgrades, `--force-downgrade` with required `--why` (pc_ceff5f2fafc5).
- **agent-memory:** dream `--runtime` defaults to `<repo>/.memoryd` via the shared resolver, all dream variants + repo-only CLI test (pc_a782707f3a97).
- **x-watch:** `doctor --online` opt-in probe, 401 → `credential_invalid` + portal remediation (pc_b66efae3997d, pc_3d8f55856fe6). **Trey task (~2 min): regenerate app keys in the X developer portal** — code fix is inert until then.
- **transcribe-url:** regression asserting nonzero exit on no-transcript; event-page → podcast/YouTube mirror hint in header docs (pc_97ff0f165238).
- **ai-profiles:** `ai-profile credential-names` — names/presence/source/equality booleans only; backups/runtime excluded by construction; values never read into output (pc_91911d7ae332, pc_4642e6d76ee3, pc_b95946c00f3d).
- **FEC key** (pc_828f1dfa2edc): **Trey task: create a personal DATA.gov key**; then wire into the protected profile + managed union; helper prefers it over DEMO_KEY with explicit 429 handling.
- **Packard:** lane prompts emit absolute corpus paths or a declared-root variable (pc_2f283876d668).
- **Collaboration-mandating skills** (local half of `collaboration-thread-absence`): make background/parallel passes conditional on a live coordinator thread; degrade to direct execution with an explicit note (pc_08099f2644cd et al. locally actionable part).

---

## Wave 8 — Verification sweep + log hygiene

1. **Remaining `already-fixed` cuts** (15 minus the three Wave 6 closed — Sol N2): run each cited regression/live check from the report's evidence; resolve with a note naming the check. Failures get re-dispositioned, never resolved.
2. **No-bug/meta resolutions:** pc_506af2585d23 (already-fixed, resolve-as-no-bug per report §95) and pc_183edfed93b6 (meta filing) — notes matching the report's language.
3. **`needs-repro` (7):** stay open; the Wave 2 evidence flags are the systemic fix.
4. **`external-upstream` (14):** draft upstream issue text (hyperframes, agent-browser, gog calendar, Vercel CLI, Codex collaboration/exec-capture); **all posting gated on Trey, per item.**
5. **Bulk-resolve** implemented cuts wave-by-wave via the new multi-ID resolve, per the manifest; `scripts/check-manifest.sh` must pass after each batch.
6. **End state is a formula, not a number:** open = 7 needs-repro + 14 external-upstream + shell cuts on any harness not yet verified on bash + cuts blocked on the two Trey tasks. With Codex verified and both Trey tasks done, the manifest predicts the exact count; assert it.

---

## Sequencing, ownership, risk

- **Order:** 0 → 1 → 2 → 3 → 4a → 5 → 4b → 6 → 7 → 8. (6 can start after 2; 7 after 1–2; the 4a→5→4b chain replaces r1's false "4/5/6 independent".)
- **File ownership:** global instruction surfaces = Wave 3 only. Delegate skill copies = Wave 6. Exa skill copies = Wave 5 (verified by 4b). Manifest = Wave 0, updated by every wave's resolution batch.
- **Rollback:** Wave 1 = remove two env-block keys; `env.sh` stays harmless (zsh still sources it, same behavior as today). Wave 6.1 = restore `_map_auto_isolation`'s safe-only worktree mapping.
- **Biggest residual risks:** (a) Claude Code env block not reaching the Bash tool → 1.3 fallback; (b) a BSD-idiom script the 1.2 sweep misses → loud failure post-flip, fix-forward; (c) harnesses without command hooks can't enforce the source-root guard → documented residual, delegate preflight still covers delegate's own operations.
- **Trey's human tasks:** ratify the Wave 6.1 dirty-tree default; X portal key regeneration; DATA.gov key creation.

---

## Plan revision history

- **r2 (2026-07-15):** Three-lane review (Sol xhigh via delegate codex safe, Grok via delegate grok safe, native plan-reviewer). Convergent blockers adopted: BASH_ENV as the env mechanism (non-interactive bash reads no profile files); gnubin scoped to agents via `agent.sh`, `.zshenv` behavior preserved for interactive zsh; full `.zshenv` inventory (Context7/agent-env/OpenRouter/ElevenLabs-profile-routing/kimi — r1 had only cargo+Supabase); worktree default made workspace-aware (git+valid-HEAD only) with dirty-tree DECISION; first-resolve-wins retained (r1's "last wins" was wrong); multi-ID resolve keeps single-ID shape; Wave 4↔5 cycle broken into 4a→5→4b; doctor sweeps skill-library roots; Wave 0 manifest replaces approximate counts. Sol uniques adopted: evidence redaction pass; duplicate-add evidence warning; no arbitrary `versionCmd` execution; exa no-network guard pre-credential-resolution; registry (not clap) as exa schema source; outcome-field compat fixtures; papercuts AGENTS.md gate incl. 5× store.rs tests; work-mode (not safe) delegate smoke; per-harness cut resolution. Rejected: contract-2 evidence identity (kills dedup — documented first-wins + warning instead); separate `resolve-many` command (extended `resolve` instead); Grok's `sed -i ''` as blocker (adopted as sweep + transition note). Counts corrected: 3 classification cuts not 4; per-harness end-state formula.
- **r1 (2026-07-15):** initial draft, commit d63d7a8.

# Model performance journal — papercuts

## 2026-07-09 - grok-4.5-fast-xhigh via cursor - adversarial design review (plan r1)

Command and run: `delegate --group papercuts-plan-review cursor safe --prompt-file _scratch/plan-review-prompt.md`; alias `cursor-1`; mode/isolation: safe/isolated copy.

Task and expectation: adversarial review of the papercuts design doc against the rust-agent-cli skill, 7 named hunt areas; expected a ranked findings list.

Outcome and verification: excellent. 2 true blockers (ID-determinism/doctor contradiction; torn-last-line unspecified) + ~14 majors/minors, near-zero noise. Notably resourceful: safe isolation blocked the repo's skill symlink, and the lane hunted down the canonical skill path on its own instead of reviewing blind. Coordinator triage accepted ~80% of findings; rejections were genuine judgment calls, not errors.

Performance observations: ~7 min wall. Findings arrived well-tabled with severities (its known flat-intensity weakness was mostly absent). One background search hung at the end without affecting output.

Routing assessment: confirms Cursor/Grok-4.5 as the fleet's first-choice design reviewer; use again unhesitatingly. Confidence: high.

## 2026-07-09 - gpt-5.6 codex (sol, xhigh) via codex - adversarial design review (plan r2)

Command and run: `delegate --group papercuts-plan-review codex safe --model sol --reasoning-effort xhigh --prompt-file _scratch/plan-review-prompt-r2.md`; alias `codex-3` (runs codex-1/codex-2 failed pre-model: work-account quota exhaustion, then an expired restored personal token — harness failures, no model signal).

Task and expectation: fresh-eyes review of the once-amended r2 doc, hunt areas aimed at the amendments themselves.

Outcome and verification: outstanding — 1 blocker + 12 majors + 1 minor, and triage accepted every single one (unprecedented zero-reject round). The blocker (content-addressed ID ≠ retry-idempotency; ts changes on retry) was a real reasoning flaw both the coordinator and the first reviewer missed. Caught doc-internal inconsistencies (doctor --fix ghosts in two sections, sort-order contradiction between synopsis and normative fold) and deep contract holes (write() vs write_all partial-write poisoning, lock-hang with no timeout policy, missing-file semantics). Checked local rustc File::lock docs before opining on locking.

Performance observations: ~12 min wall at xhigh. Findings precise, each with concrete failure scenario + 1-2 line fix; report format followed exactly. Zero fabrication; zero scope creep.

Routing assessment: Sol xhigh is a frontier-grade design reviewer — for contract-dense specs it outperformed the same doc's first-round review on depth (different axis: precision holes vs design contradictions; the two rounds were complementary, validating the two-family sequence). Use Sol xhigh for judgment-dense spec gates; keep Cursor for artifact-probing reviews. Confidence: high.

## 2026-07-09 - gpt-5.6 codex (sol, high) via codex - wave 1: full CLI implementation

Command and run: `delegate --group papercuts-wave1 codex work --model sol --reasoning-effort high --prompt-file _scratch/implement-wave1-prompt.md`; alias `codex-4`; mode work, in-place tree.

Task and expectation: author the entire v0.1 CLI (~2,400 lines incl. tests) from the r3 design doc in one clustered lane.

Outcome and verification: exceptional fidelity. Coordinator re-ran the full gate independently (build/clippy/fmt + 5x sweep) — green, 22 tests. Coordinator riskiest-file read (store.rs, add.rs, resolve.rs, doctor spot-checks) found zero defects: critical sections correct, length-prefixed hash byte-exact to spec, tear-heal + rollback present, fold matrix genuinely adversarial. Cross-family review (cursor-2) found 1 real blocker (pre-lock exists() TOCTOU + NotFound→74) and 4 majors — author-blindness held true at the contract margins, not the core.

Performance observations: ~19 min wall for a whole product. Zero scope creep, zero deviations, honest report.

Routing assessment: Sol high remains the fleet author. The review lane still earns its keep — never skip it. Confidence: high.

## 2026-07-09 - grok-4.5-fast-xhigh via cursor - wave-1 adversarial code review

Command and run: `delegate --group papercuts-wave1-review cursor safe --prompt-file _scratch/review-wave1-prompt.md`; alias `cursor-2`; safe/isolated.

Task and expectation: attack the uncommitted wave-1 diff against the r3 contract, 7 named hunt areas.

Outcome and verification: 1 blocker + 4 majors + 3 minors, all verified real by coordinator read; zero false positives. The blocker (TOCTOU exists()-then-open, NotFound mapped to the wrong exit) had survived BOTH the author's tests and the coordinator's manual trace of the same files — textbook decorrelation value. Also produced a non-findings checklist confirming verified-OK areas, which cut triage time.

Performance observations: ~11 min. Findings came with file:line, concrete scenarios, and a hunt-checklist table. Static-only (no test execution) — noted honestly.

Routing assessment: Cursor/Grok-4.5 confirmed as the fleet ATTACKER on code as well as designs. Confidence: high.

## 2026-07-09 - swe-1.7 via devin - wave-1 fix rounds 1+2

Command and run: `delegate --group papercuts-wave1-fix devin work --prompt-file _scratch/fix-wave1-prompt.md` (devin-1) and `..._scratch/fix2-wave1-prompt.md` (devin-2); work mode, in-place.

Task and expectation: execute findings-shaped fix lists (7 findings round 1, 4 round 2), each with required pinning tests, no scope creep.

Outcome and verification: flawless both rounds. Every fix verified on disk by coordinator grep/read; coordinator gate green 5x after each. Round 1 included one deviation (ambiguous_id exit 2→65) that was argued correctly and honestly flagged. Round 2 exceeded spec in the right way: made error.rs a const contract table and derived the schema command's dictionaries from it, structurally killing the drift class rather than patching the instance — then manually verified the nested-git test-fragility fix by running the suite with TMPDIR inside a fresh git repo, unprompted thoroughness.

Performance observations: ~9 and ~7 min. Zero unrequested edits; reports exactly in the requested format.

Routing assessment: Devin remains the SURGEON — findings-shaped fix work routes here by default. Confidence: high.

## 2026-07-09 - live acceptance (coordinator-driven)

Release binary driven through the full agent lifecycle in a fresh playground repo: schema self-orientation, harness detection, stdin add, fixed-clock duplicate handling, 12-way concurrent adds (15/15 clean lines), md digest with strikethrough, resolve by prefix + idempotent re-resolve, torn-line self-heal, doctor exit dictionary, explicit-missing=66 vs virtual-empty, home fallback, byte-identical determinism. Zero unexplained failures; the only anomalies were the coordinator's own jq paths against the flattened ListItem shape.

## 2026-07-15 - plan review round (remediation plan r1 → r2)

Two safe lanes on one shared brief (`papercuts-plan-r1` group), plus native opus plan-reviewer, reviewing docs/plans/papercuts-remediation-2026-07-15.md against the 132-cut diagnostic and live code seams.

**Sol xhigh** (`delegate codex safe --model sol --reasoning-effort xhigh`, run del_20260715T154256Z_b6a1a6, ~17 min, resultQuality ok): 10 blockers / 14 risks / 2 nits, deepest of the three. Uniques nobody else found: evidence fields as a credential-persistence hazard in an append-only log; `versionCmd` frontmatter as an untrusted-exec surface; exa no-network guard ordered after credential resolution; exa schema actually sourced from the build-time registry not clap; papercuts AGENTS.md 5× store.rs test mandate; work-mode (not safe) smoke needed for a work-mode default change. One overreach: proposed contract-2 identity including evidence — would destroy dedup-by-text; rejected. Severity anchor held. Routing: keep Sol xhigh as the premier plan-gate lane. Confidence: high.

**Grok 4.5** (`delegate grok safe`, run del_20260715T154258Z_2375e6, ~4.5 min, resultQuality ok — no empty-envelope recurrence): 5 blockers / 14 risks / 7 nits, fast and dense. Convergent on all the real killers (BASH_ENV, interactive-zsh contamination, dirty/non-git worktree, first-resolve-wins, 4↔5 cycle, skill-library sweep roots). Distinct value: instruction-surface ownership collisions, retention policy, materialization bounds, per-harness end-state formula. Coordinator initially mis-rejected its `.zshenv` finding after a flawed regex re-check — Grok had read the file correctly; lesson recorded: verify with full-file reads, not anchored regexes, before overruling a lane. Routing: grok lane fully credible as the second plan-gate voice when cursor is quota-limited. Confidence: high.

**Native plan-reviewer (opus)**: 2 blockers / 6 risks / 4 nits; smallest set but earliest to the BASH_ENV blocker with the settings.json evidence, and the only lane to flag `skip_serializing_if`. Went idle once before delivering; one SendMessage ping recovered it (known behavior).

Convergence pattern matched the standing-pair thesis: ~40% overlap on blockers, uniques decorrelated. All three lanes' convergent findings survived verification; two unique findings were rejected with reasons (Sol contract-2, and nothing from Grok after the .zshenv reversal).

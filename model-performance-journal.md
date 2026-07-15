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

## 2026-07-15 - gpt-5.6-luna via codex - remediation Wave 0 manifest

Command and run: `delegate --group papercuts-remediation-wave0 codex work --model luna --reasoning-effort high --isolation none --prompt-file /tmp/papercuts-wave0.md`; alias/variant/effort: `luna`, high; mode/isolation: work/in-place; run handle: `codex-6`.

Task and expectation: Create the exact 132-cut disposition manifest and its checker from the remediation plan, diagnostic report, and current log without touching the log or other files.

Outcome and verification: The lane stopped without edits after finding one newer live cut, `pc_7e7d9f9a0385`, absent from the diagnostic snapshot. Coordinator inspection confirmed no requested artifacts were created. The stop was conservative but over-applied the reconciliation rule: Wave 0 is explicitly a manifest of the 132-cut diagnostic snapshot, so later live cuts can be called out separately rather than blocking the snapshot manifest.

Performance observations: 3m26s. It reconciled diagnostic disposition totals and current log scopes accurately and avoided inventing a mapping, but it did not distinguish snapshot scope from post-diagnostic log growth and therefore delivered no implementation.

Routing assessment: Retry Luna high with an explicit snapshot-boundary rule; use again for exact inventory work when source-of-truth scope is stated unambiguously. A comparison against Terra on reconciliation judgment would reduce uncertainty. Confidence: medium.

## 2026-07-15 - gpt-5.6-luna via codex - remediation Wave 0 retry

Command and run: `delegate --group papercuts-remediation-wave0 codex work --model luna --reasoning-effort high --isolation none --prompt-file /tmp/papercuts-wave0-retry.md`; alias/variant/effort: `luna`, high; mode/isolation: work/in-place; run handle: `codex-7`.

Task and expectation: Implement the frozen 132-cut manifest and checker after clarifying that post-snapshot live cuts are reported but do not block snapshot accounting.

Outcome and verification: Created the two owned artifacts. Independent coordinator runs of `bash scripts/check-manifest.sh` and `--after-wave 0` passed with 132 unique IDs. Counts reconcile to 43 fix, 50 instruction-only, 15 already-fixed, 14 external, 7 needs-repro, and 3 already resolved. No out-of-scope edits were made.

Performance observations: 10m48s. Exact reconciliation was thorough and the checker covers duplicate/unknown IDs, frozen-report coverage, wave/end-state enums, and incomplete live-log handling. The 173-line checker is longer than ideal but handles the plan's several falsifiability requirements without a new dependency.

Routing assessment: Use Luna high again for bounded inventory-heavy implementation after explicit scope boundaries; follow with adversarial review for count semantics. Confidence: high.

## 2026-07-15 - gpt-5.6-sol via codex - Wave 0 adversarial review

Command and run: `delegate --group papercuts-wave0-review codex safe --model sol --reasoning-effort xhigh --prompt-file /tmp/papercuts-wave0-review.md`; alias/variant/effort: `sol`, xhigh; mode/isolation: safe/temporary worktree; run handle: `codex-8`.

Task and expectation: Attack the manifest and checker for semantic assignment errors and false-pass wave states.

Outcome and verification: Found four blockers, four majors, and three minors. Coordinator verified the central defects: 4b/5 ranks were reversed; incomplete logs could print PASS; one global verification boolean could not model per-harness and per-cut acceptance; and human-task state omitted the DATA.gov cut. It also caught wrong final waves for two partial shell fixes and the Wave 8 meta resolution. No edits.

Performance observations: 10m29s. High-value semantic review with concrete counterexamples and exact lines. It ran read-only parsing probes and clearly separated verified-clean inventory facts. One portability suggestion required coordinator adaptation because repo policy forbids `rm` cleanup.

Routing assessment: Keep Sol xhigh as the severity anchor for plan-derived verifiers; the review prevented a self-consistent but non-falsifying gate from becoming foundational. Confidence: high.

## 2026-07-15 - grok-4.5-fast-xhigh via cursor - Wave 0 adversarial review

Command and run: `delegate --group papercuts-wave0-review cursor safe --prompt-file /tmp/papercuts-wave0-review.md`; alias/variant/effort: `grok-4.5-fast-xhigh`; mode/isolation: safe/temporary worktree; run handle: `cursor-4`.

Task and expectation: Independently attack snapshot coverage, wave semantics, CLI parsing, and false-pass cases without shell execution.

Outcome and verification: Converged on incomplete/wrong-log false PASS and global verification semantics, and uniquely emphasized no-op Wave 1/2 gates, unreachable final-open formula, and observable rather than asserted open/resolved totals. Static-only as requested. The suggested `rm -rf` fallback was rejected because machine policy requires `trash`; the underlying missing-dependency concern remains valid.

Performance observations: 2m29s, roughly four times faster than Sol. Findings were dense, scenario-based, and substantially overlapping on blockers while adding useful operational cases. It respected the no-shell constraint.

Routing assessment: Use again as the fast attacker beside Sol; coordinator must continue re-ranking severity and filtering fixes against local policy. Confidence: high.

## 2026-07-15 - gpt-5.6-terra via codex - Wave 0 review fixes

Command and run: `delegate --group papercuts-wave0-fix codex work --model terra --reasoning-effort high --isolation none --prompt-file /tmp/papercuts-wave0-fix.md`; alias/variant/effort: `terra`, high; mode/isolation: work/in-place; run handle: `codex-9`.

Task and expectation: Repair the adjudicated Sol/Grok findings in the manifest and checker, including exact sequencing, fail-closed full-log gates, row-scoped acceptance, and human-task transitions.

Outcome and verification: Reworked only the two owned files. The checker now requires complete 132-ID coverage for PASS, labels partial runs DIAGNOSTIC, accepts deterministic multi-log union, uses named per-row conditions, models three task-blocked cuts across two tasks, and reports the final 21-open state. Coordinator reran syntax and ShellCheck successfully and confirmed the critical manifest rows and usage contract.

Performance observations: 15m50s. Terra handled an entangled verifier rewrite without scope creep and supplied a targeted synthetic matrix covering the prior false-pass classes. The resulting 356-line shell script is substantial, but most size is explicit fail-closed validation rather than abstraction.

Routing assessment: Use Terra high again for findings-shaped corrections with coupled data/schema semantics; fresh adversarial review remains required because the checker itself is now more complex. Confidence: high.

## 2026-07-15 - gpt-5.6-sol via codex - Wave 0 post-fix review

Command and run: `delegate --group papercuts-wave0-rereview codex safe --model sol --reasoning-effort high --prompt-file /tmp/papercuts-wave0-rereview.md`; alias/variant/effort: `sol`, high; mode/isolation: safe/temporary worktree; run handle: `codex-10`.

Task and expectation: Fresh review of Terra's repaired manifest/checker, focused on remaining false-pass and reachability defects.

Outcome and verification: Found four blockers and one major. The strongest findings were ignored parser warnings/torn-log metadata, stale-open plus newer-resolved multi-log conflicts, nine incorrect shell-harness assignments derived from filing provenance, and a Wave 1 no-op without explicit acceptance/deferment. It also clarified that the exact 21-open formula requires the conditional OPM verification.

Performance observations: 8m1s. This review probed source fold/list behavior and filing provenance rather than only rereading the manifest, catching defects the prior synthetic suite did not cover. It was the more conservative severity lane and supplied exact remappings.

Routing assessment: Sol high is sufficient for focused post-fix verifier review; retain it as the final blocker authority when Grok says shippable. Confidence: high.

## 2026-07-15 - grok-4.5 via grok - Wave 0 post-fix review

Command and run: `delegate --group papercuts-wave0-rereview grok safe --reasoning-effort high --prompt-file /tmp/papercuts-wave0-rereview.md`; alias/variant/effort: `grok-4.5`, high; mode/isolation: safe/temporary worktree; run handle: `grok-2`.

Task and expectation: Independently re-review the repaired gate for false PASS/FAIL behavior and confirm prior issue classes.

Outcome and verification: Declared no blockers but found one real major: legitimate early/parallel resolutions fail earlier wave gates because not-yet-due resolvable rows are forced open. It confirmed the prior ranking, completeness, row-condition, task, and cleanup fixes. Its harness provenance conclusion conflicted with Sol's direct filing-event inspection and needs coordinator verification.

Performance observations: 4m51s. Fast, structured, and useful on state-machine monotonicity, but materially undercalled severity by missing malformed-log warnings, multi-log stale resolution, and the mandatory Wave 1 acceptance hole.

Routing assessment: Use as a complementary state-machine attacker, never as sole ship authority for verifier/security semantics. Confidence: high.

## 2026-07-15 - gpt-5.6-terra via codex - Wave 0 review fixes round 2

Command and run: `delegate --group papercuts-wave0-fix2 codex work --model terra --reasoning-effort high --isolation none --prompt-file /tmp/papercuts-wave0-fix2.md`; alias/variant/effort: `terra`, high; mode/isolation: work/in-place; run handle: `codex-11`.

Task and expectation: Fix the second review's malformed-log, monotonic union, harness provenance, no-op Wave 1, early-resolution, and final-formula defects without touching other files.

Outcome and verification: Updated only manifest and checker. State gates now reject unsafe envelopes, merge stale-open plus resolved monotonically, use corrected filing provenance, require explicit Wave 1 harness outcomes with Claude acceptance mandatory, tolerate valid early resolutions, and distinguish bounded PASS from exact 21-open completion. Coordinator reran syntax, ShellCheck, diagnostic-only mode, and diff checks successfully.

Performance observations: 8m33s. Terra again stayed inside ownership and added targeted synthetic cases for every adjudicated failure class. The checker grew to about 440 lines, reflecting explicit input/state validation; a future rewrite should require evidence that this shell implementation is actually the maintenance bottleneck.

Routing assessment: Continue using Terra high for narrow verifier corrections; the strong synthetic matrix materially lowered coordinator burden. Confidence: high.

## 2026-07-15 - gpt-5.6-sol via codex - Wave 0 final ship gate

Command and run: `delegate --group papercuts-wave0-final-review codex safe --model sol --reasoning-effort high --prompt-file /tmp/papercuts-wave0-final-review.md`; alias/variant/effort: `sol`, high; mode/isolation: safe/temporary worktree; run handle: `codex-12`.

Task and expectation: Final blocker/major-only verification of the committed Wave 0 artifacts after the second repair round.

Outcome and verification: CLEAN. It verified fail-closed unsafe envelopes, monotonic multi-log resolution, corrected 13-Claude/3-Codex provenance, mandatory Wave 1 outcomes, early-resolution tolerance, and exact 21-open completion accounting. No edits.

Performance observations: 5m13s. Faster than the earlier broad reviews because the prompt was tightly bounded; it still performed syntax, ShellCheck, count reconciliation, and filing-agent checks in the isolated copy.

Routing assessment: Sol high is an efficient closing judge once the finding set is bounded; use xhigh for broad discovery and high for focused closure. Confidence: high.

## 2026-07-15 - grok-4.5 via grok - Wave 0 final ship gate

Command and run: `delegate --group papercuts-wave0-final-review grok safe --reasoning-effort high --prompt-file /tmp/papercuts-wave0-final-review.md`; alias/variant/effort: `grok-4.5`, high; mode/isolation: safe/temporary worktree; run handle: `grok-3`.

Task and expectation: Independent final blocker/major-only review of the six adjudicated Wave 0 defect classes.

Outcome and verification: CLEAN. It traced each defect through manifest/checker and list/store envelope behavior, including defer-vs-accept semantics and Wave 3 partial shell rows. No edits.

Performance observations: 3m54s. Concise and source-grounded; no-shell limitation was stated. The result converged with Sol after the prior severity divergence.

Routing assessment: Keep direct Grok as the fast independent closing voice beside Sol; convergence after fixes is useful evidence, not a substitute for coordinator runtime checks. Confidence: high.

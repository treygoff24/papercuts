# Papercuts Wave 5 acceptance

**Decision:** Accepted on 2026-07-16. Wave 5 (exa-agent contract hardening) is blocker/major clean.

## Artifact

- Repo: `/Users/treygoff/Code/exa-agent-cli`, branch `remediation-wave5`, tip `498f44346a8eb0a244b3cfeade9eb846bf6530f3` (15 commits over `main` @ `36f1080`). Unpushed, unmerged — merge and push gated on Trey.
- Staged skill: `work/generated/exa-agent-cli/SKILL.md` (Wave 4b input).

## Rounds

1. Implement: Sol high, work mode (`codex-64`, ~20 min). Transplanted the audited Wave 5 slice from an earlier unmerged feature branch onto the 0.3.0 baseline; caught and reverted an inherited exit-10/`all_urls_failed` contract break during its own review pass.
2. Adversarial review: Grok 4.5 safe (`grok-1`, ~3.5 min). 0 blockers / 6 majors / 5 minors. Key majors coordinator-verified before acting: fixture circularity (confirmed — fixtures absent on `main`, test injected `outcome` before comparing), dry-run missing required `outcome`, parity not covering ranges/enums, dual `--text` cap sources.
3. Fix round: Sol high, work mode (`codex-65`, ~24 min). All 9 assigned findings fixed; its internal two-lane review surfaced 2 more (enum parity vacuity via registry-derived clap metadata; argument-order loss of search diagnostics), both fixed. Rejected one internal-review finding correctly (`vendor-spec --check` guard — explicitly out of scope).

## Coordinator decisions

- Dry-run previews do NOT carry `outcome`; `docs/v2/contracts.md` scopes it to live contents/fetch result envelopes, pinned by an explicit test.
- Evidence-fixture honesty: legacy envelopes must come from a pre-change binary; provenance (main SHA + capture commands) recorded in `tests/fixtures/contents/README.md`. Verified present.

## Evidence

- Coordinator re-ran `cargo test --all-features` at `498f443`: all suites green.
- Sol's final gate verbatim in `delegate run-output codex-65 --completion-report`: build/test/clippy/fmt/MSRV (`cargo +1.85 clippy --all-features --all-targets`) plus both generated-artifact checks green.
- Single budgeted live call verified `.data.results[]` jq path (provenance in exa-agent CHANGELOG).

## Residuals

- CI unexecuted until a push is authorized (new `msrv` job included on the branch).
- Documented, untouched: `auth status` refuses under `EXA_AGENT_NO_NETWORK`; `requested_count` not used for exit classification.
- Intentional break, CHANGELOG-documented: `--text 0|true|false` forms removed.

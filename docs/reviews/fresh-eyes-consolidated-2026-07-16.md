# Fresh-eyes review — consolidated findings and triage (2026-07-16)

Six independent Sol-xhigh review lanes (Codex work mode), one per repo, over today's full workstream diffs. Per-repo detail lives in each repo's `fresh-eyes-review-2026-07-16.md`. All lanes ran the repo gate green before probing; every finding below survived the lane's own verification (VERIFIED = reproduced/traced, PLAUSIBLE = static read).

**Totals: 0 blockers · 12 majors · 15 minors.** Coordinator triage column is a recommendation for Trey, not a decision.

## Majors

| # | Repo | Finding (file:line) | Verdict | Triage rec |
|---|---|---|---|---|
| 1 | papercuts | Token-only URL userinfo never redacted (`src/commands/add.rs:284`) — `https://ghp_…@github.com` stored verbatim | VERIFIED | **Fix before push.** Redaction is the advertised boundary; append-only log makes leaks permanent. |
| 2 | papercuts | Lowercase compound keys (`clientsecret=`, `dbpassword=`…) bypass redaction (`add.rs:322`) | VERIFIED | **Fix before push.** Same class as #1; one fix round covers both. |
| 3 | papercuts | check-manifest hard-codes two ledger-lost IDs as `open`, but manifest says `resolve-on-6` — every Wave 6+ gate will stay red (`check-manifest.sh:375`) | VERIFIED | **Fix now** (coordinator-authored bug, mine). Lost-ID entries need wave-aware expected status or an attested-resolve mechanism for orphan resolves. |
| 4 | papercuts | Condition-backed manifest rows pass without attestation when flag omitted (`check-manifest.sh:413`) | VERIFIED | **Fix now.** Silent `expected="either"` defeats the falsifiability the gate exists for. |
| 5 | exa | `schema refresh --check` never fetches — always reports `current`, even offline against a dead endpoint (`src/lib.rs:6494`) | VERIFIED | **Fix before merge.** Advertised live comparison is a no-op; worse than absent. |
| 6 | exa | Missing-reason recovery command turns `--ids` requests into positional URL requests (`lib.rs:6216`) | VERIFIED | **Fix before merge.** Machine-followable recovery that changes request semantics is an agent trap. |
| 7 | radar | Item lookup can't return `source_published_at` after promotion deletes the pending row; test fixture fabricated a state prod can't have (`scripts/ops/radar-lookup.ts:155`) | VERIFIED | **Fix before push.** The timestamp was the point of the cut (pc_b66b74817bba); needs a real source (persist on item or read item payload) + honest fixture. |
| 8 | contacts | Downgrade guard compares against stale snapshot, ignores newer warmth in the authoritative override file (`src/store.rs:335`) | VERIFIED | **Fix before push.** Real supported failure path, silently loses data the guard exists to protect. |
| 9 | contacts | Guard fails OPEN on unreadable/corrupt snapshot — `.ok()` → `cold` → downgrade allowed (`store.rs:341`) | VERIFIED | **Fix before push.** Destructive-path safety must fail closed. |
| 10 | x-watch | Online doctor echoes upstream error text unredacted; a header-reflecting proxy leaks the bearer token to stdout (`src/http.ts:45`) | VERIFIED | **Fix before push.** Route through existing `src/redact.ts`. |
| 11 | ai-profiles | Static parser reports source text, not zsh state: `KEY=$BASE` with differing BASEs → `equal:true` (wrong security verdict); `)` in a comment silently drops keys (`credential_names.py:13`) | VERIFIED | **Fix before push.** Fail closed on syntax the parser can't evaluate (plan already prescribes scoped `zsh -f` path). |
| 12 | ai-profiles | O_NOFOLLOW guards only the final path component; intermediate symlink escapes the profile root (`credential_names.py:28`) | VERIFIED | **Fix before push.** Confinement promise in README. |

## Minors

| Repo | Finding | Triage rec |
|---|---|---|
| papercuts | Duplicate-add warning string changed (`duplicate papercut;…` → `duplicate_cut:…`) despite byte-identical promise | Fix with majors (restore string or amend design doc explicitly) |
| exa | No-network error suggests `--dry-run` at call sites where dry-run can't bypass | Fix in exa round |
| exa | New parse-error path drops `--correlation-id=` / env-var forms | Fix in exa round |
| exa | `commands.md:369` still documents exit 10 for mixed outcomes | Fix in exa round (docs) |
| exa | `architecture.md:260` still documents removed `--text false` | Fix in exa round (docs) |
| radar | `agent-sweep` lookup presented as read-only but makes billed live Exa call | Fix in radar round (label + `--live` ack flag or doc) |
| radar | STATE.md stale the moment range pushes | Defer — STATE.md updates at next session per its own convention |
| contacts | `--why '   '` (whitespace) satisfies forced-downgrade reason | Fix with majors (trim check) |
| x-watch | Insecure-permission credential file reported as missing | Fix in x-watch round |
| x-watch | `--env-file` swallows `--online` as its path | Fix in x-watch round |
| x-watch | `x-watch.error.v1` contract doc omits `CREDENTIAL_INVALID` | Fix in x-watch round (docs) |
| x-watch | Test fixtures leak temp credential-shaped dirs | Fix in x-watch round (cleanup hook) |
| ai-profiles | Regular-file check after blocking open — FIFO hangs command | Fix in ai-profiles round (O_NONBLOCK probe or stat-first) |
| ai-profiles | `--fixture-root` accepted but ignored by credential-names | Fix in ai-profiles round |
| ai-profiles | stderr leak assertion omits four fixture secrets | Fix in ai-profiles round (test) |

## Cross-cutting observations

1. **Credential redaction is the systemic weak spot** — 5 of 12 majors (papercuts ×2, x-watch, ai-profiles ×2) are variations of "the redaction/confinement boundary has a bypass." Suggests a shared hardening checklist for any lane touching credential paths: URL userinfo forms, lowercase/concatenated keys, upstream-echo paths, symlinked parents, eval-vs-source-text.
2. **Test theater appeared twice** even after today's fixture-honesty fixes (radar fixture fabricating impossible DB state; ai-profiles stderr assertion missing half the fixtures). The "fixtures must be derivable from production-reachable state" rule is worth adding to lane briefs.
3. Every lane confirmed the gates themselves green — all 12 majors passed CI-grade checks. Adversarial probing remains the only thing that catches this class.

## Suggested execution if Trey approves fixes

One findings-shaped fix lane per repo (6 lanes, parallelizable; contacts/ai-profiles/x-watch are small), each pinned to this doc's items with regression tests required, then single re-review pass per repo on the fix diff only. Estimated: one session.

---

## Outcomes (appended post-fix-round, same day)

**All 12 majors and 15 minors: FIXED** (except two coordinator-deferred minors: radar STATE.md refresh — updates at next session per its own convention; exa schema-refresh timeout pin — rides the next exa round). Every fix carries a pinning test demonstrated red on pre-fix code. Six scoped Grok re-reviews confirmed all findings CLOSED, zero reopened, and surfaced five NEW findings:

- x-watch N1 (major): shared error envelope still reflected upstream text — **fixed** (`4940834`), and the fix exposed + repaired a latent offset bug inside `redact.ts` itself.
- radar N2 (operational): non-castable timestamps hard-failed the promote RPC — **fixed** via append-only safe-cast migration (`8388873`).
- ai-profiles NEW-2: `--fixture-root` bypassed confinement — **fixed** (`b1fa411`); NEW-1 (zsh -f executes key files) **accepted as documented trade-off** — the files are already sourced by every interactive zsh; README security-model note added.
- papercuts: over-broad userinfo redaction **accepted** (defense-in-depth); evidence-surface test-coverage nit accepted.
- contacts NEW-1/nit: accepted at nit level.

Final state per repo: all gates green, all work committed locally, nothing pushed.

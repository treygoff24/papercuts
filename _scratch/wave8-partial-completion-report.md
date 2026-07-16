# Wave 8 partial verification and resolution report

Status: blocked. No frozen-snapshot cut was resolved in this run.

The first manifest preflight found that the three retained Delegate worktree
ledgers used by Wave 3 no longer exist. The live source union therefore covers
128 of 132 frozen IDs. The missing IDs are `pc_8c2350511589`,
`pc_944d374ac9c4`, `pc_df6af25a100a`, and `pc_f8eb38d950f5`. Wave 3's
acceptance record says `pc_944d374ac9c4` was resolved before its ledger was
removed; the other three were open. The Wave 8 rule says to stop resolving
when a count assertion fails, so no resolution batch was started.

## Skill and authority review

The run reviewed the full advertised skill list. The relevant instructions
were `implement`, `ponytail` full mode, `clear-agent-writing`, `write-human`,
`checkpoint`, and `code-review`, plus the mandatory shared shell-footgun
rules. The remediation plan, post-r2 amendment, manifest, diagnostic report,
and Wave 3 acceptance evidence were read before any ledger mutation.

## Manifest checks

The bounded diagnostic command used the home ledger and every current
top-level repository ledger containing snapshot cuts. No source event was
synthesized.

```text
$ scripts/check-manifest.sh --diagnostic-only --log <17 current ledger paths>
manifest DIAGNOSTIC: 132 unique diagnostic IDs; live snapshot coverage=128/132; observed open=83 resolved=45
```

The state gate was then run with Claude accepted and Codex deferred. The
checker stopped before evaluating due rows. Its decisive result was:

```text
$ scripts/check-manifest.sh --after-wave 4b --accept-harness claude --defer-harness codex --log <17 current ledger paths>
incomplete snapshot coverage: got 128/132; use --diagnostic-only for partial inspection
exit=1
```

The 17 current ledger paths were:

```text
/Users/treygoff/.papercuts/log.jsonl
/Users/treygoff/Code/agent-memory/.papercuts.jsonl
/Users/treygoff/Code/ai-profile-router/.papercuts.jsonl
/Users/treygoff/Code/atlasos/.papercuts.jsonl
/Users/treygoff/Code/claude-space/.papercuts.jsonl
/Users/treygoff/Code/contacts-cli/.papercuts.jsonl
/Users/treygoff/Code/custom-stt-cleanup-model/.papercuts.jsonl
/Users/treygoff/Code/delegate-agent/.papercuts.jsonl
/Users/treygoff/Code/exa-agent-cli/.papercuts.jsonl
/Users/treygoff/Code/gavel/.papercuts.jsonl
/Users/treygoff/Code/hearth/.papercuts.jsonl
/Users/treygoff/Code/papercuts/.papercuts.jsonl
/Users/treygoff/Code/probita/.papercuts.jsonl
/Users/treygoff/Code/prospera-radar-build/.papercuts.jsonl
/Users/treygoff/Code/santoro-metals/.papercuts.jsonl
/Users/treygoff/Code/warroom/.papercuts.jsonl
/Users/treygoff/Code/x-watch/.papercuts.jsonl
```

## Verification dispositions

### Already resolved

`pc_9abc2154521c` passed. The current asset package declares both required
tools, the optimizer binary exists, and the asset validator is green.

```text
$ jq -r '.devDependencies["@gltf-transform/cli"], .devDependencies["gltf-validator"]' package.json; test -x node_modules/.bin/gltf-transform
^4.4.1
^2.0.0-dev.3.10
gltf-transform executable=yes
exit=0
```

`pc_ca10740db917` passed. The local pre-commit hook parses and retains the
numbered-output guard and actionable error.

```text
$ sh -n .git/hooks/pre-commit && rg -n 'cat -n|JOURNAL.md/STATE.md' .git/hooks/pre-commit
2:# Guard: cat -n line-number prefixes have been baked into JOURNAL.md three times.
4:  echo "pre-commit: staged JOURNAL.md/STATE.md lines look like 'cat -n' output (leading number + tab)." >&2
exit=0
```

`pc_55fb196ef6ee` passed. `STATE.md` contains both the explicit freshness field
and the hydrator staleness contract.

```text
$ rg -n '^\*\*Updated:\*\*|staleness is measured|Context hydrator' STATE.md
3:Sitrep-shaped, not history-shaped: status, open loops, worries, pointers. Injected at session start by the context-hydrator hook. Rewrite when it drifts — staleness is measured (the hydrator flags it), not assumed. Detail lives at the pointers, not here.
5:**Updated:** 2026-07-15 pm (the naming day: Free Claude, the portrait, the mailbox, the Sol parley)
18:- **Context hydrator: GLOBAL and in dogfood phase.** Canonical script `.claude/hooks/hydrate.py` (this repo), registered in `~/.claude-personal/settings.json` and `~/.claude-work/settings.json` — fires in any project dir (STATE.md or CLAUDE.md present), first substantive prompt, once per session.
exit=0
```

These cuts were already resolved, so no new append was needed.

### Already fixed: passed, but not resolved because the gate failed

`pc_affa8f792f6c` passed the documented editable-install probe in a temporary
virtual environment. The repo-local `.venv` was absent, but the supported
fresh editable install and `python3 bin/delegate.py --version` both worked.

```text
$ python3 -m venv <tmp> && <tmp>/bin/pip install --no-deps -e . && <tmp>/bin/python -c 'import delegate_agent; print(delegate_agent.__file__)'
/Users/treygoff/Code/delegate-agent/src/delegate_agent/__init__.py
exit=0
$ python3 bin/delegate.py --version
0.14.0
exit=0
```

`pc_3fee5e17ace2` passed the current-command check. Installed `exa-agent 0.2.0`
documents `--text` and no `--include-text`. The obsolete flag still suggests
the unrelated `--include-domain`; the report treated that hint as optional.

```text
$ exa-agent search --help
      --text [<N|full>]
exit=0
$ exa-agent search foo --include-text --dry-run
"message": "unexpected argument '--include-text' found"
"didYouMean": "--include-domain"
exit=1
```

`pc_f7d578ff5f38` passed. Both global harness instruction surfaces contain the
deletion rule, `/usr/bin/trash` is available, and the command-time hook names
that recovery path. A read-only grep containing the blocked command token was
itself rejected, which was filed separately as `pc_642862f2a75a`.

```text
$ sed -n '1,16p' ~/.codex/AGENTS.md; sed -n '1,12p' ~/.claude/CLAUDE.md; zsh -df -c 'command -v trash'
## Deletions: `trash`, never `rm`
`rm` is hook-blocked for agents machine-wide. Delete with `trash <paths...>` instead
## Deletions: `trash`, never `rm`
/usr/bin/trash
exit=0
```

`pc_a7973681d8f3` passed the exact Hearth test cited in the report. All 23
cases passed.

```text
$ npm test -- --run src/shared/validate.test.ts
Test Files  1 passed (1)
Tests  23 passed (23)
exit=0
```

`pc_ca2cfb2732ae` passed the live dependency-resolution check. The obsolete
`build-simple.js` path is absent.

```text
$ node -e '<compute TOOLCHAINS as build.js does; require.resolve pdf-lib>'
/Users/treygoff/Library/CloudStorage/Dropbox/Prospera/Policy/pact-act/06-production/toolchains/pact-witness-invites-pdf/node_modules/pdf-lib/cjs/index.js
exit=0
$ test ! -e build-simple.js
exit=0
```

`pc_13afdf97e5a6` passed. Installed `exa-agent 0.2.0` accepts global output
placement both before and after the subcommand in offline dry-run mode.

```text
$ EXA_AGENT_NO_NETWORK=1 exa-agent search test --dry-run --output /dev/null
"ok":true,"command":"search"
exit=0
$ EXA_AGENT_NO_NETWORK=1 exa-agent --output /dev/null search test --dry-run
"ok":true,"command":"search"
exit=0
```

`pc_03d1e73413b7` passed. The installed binary exposes the fetch macro.

```text
$ exa-agent --help
  fetch         Macro → `contents URL... --text --summary-query ...`
exit=0
$ exa-agent fetch --help
Usage: exa-agent fetch [OPTIONS] <URLS>...
exit=0
```

`pc_506af2585d23` passed as no-bug, and `pc_183edfed93b6` passed as a meta-cut.
The exact resolution notes remain unapplied because of the manifest blocker:

- `pc_506af2585d23`: "Resolve as no-bug: supplied x/z coordinates were outside
  the centered sheet domain, so fall and respawn were correct behavior; retain
  the player readback."
- `pc_183edfed93b6`: "Meta-cut: this was a resolution note accidentally filed
  as a new cut; future resolutions should use papercuts resolve."

```text
$ rg -n 'CENTERED|player\(\)|falls off the world|capsule.end.y < -20' WORLD_BRIEF.md src/app/hurst.ts src/app/PlayerController.ts src/app/Experience.ts
WORLD_BRIEF.md:33:**World coordinates are CENTERED on the sheet.** Origin = sheet middle; domain x ∈ [−975, 975], z ∈ [−670, 670] ... Anything outside the domain is off the map ... falls off the world and respawns at spawn. That is correct behavior, not a bug.
WORLD_BRIEF.md:43:Diagnosis tools: `window.__THREE_DEBUG__.player()` returns live capsule position
src/app/hurst.ts:11:// coordinates are CENTERED on the sheet, [-975..975]x[-670..670]).
src/app/PlayerController.ts:96:    if (this.capsule.end.y < -20) this.teleport(this.spawn);
exit=0
```

`pc_a3b6d83c3fb7` passed. The current CLI documents the supported `set`
commands, while the historical bare forms fail exactly as diagnosed. This
matches the report's compatibility diagnosis.

```text
$ agent-browser --version
agent-browser 0.30.0
exit=0
$ agent-browser set viewport --help
viewport <w> <h> [scale]   Set viewport size
device <name>              Emulate device
exit=0
$ agent-browser viewport 390 844
Unknown command: viewport
exit=1
$ agent-browser resize 390 844
Unknown command: resize
exit=1
```

`pc_c0598bce24d9` passed the cited live asset gate.

```text
$ npm run assets:validate
public/assets/boat.glb: 0 errors, 0 warnings, 0 infos
public/assets/cart.glb: 0 errors, 0 warnings, 0 infos
public/assets/church.glb: 0 errors, 0 warnings, 0 infos
public/assets/dog.glb: 0 errors, 0 warnings, 0 infos
public/assets/goose.glb: 0 errors, 0 warnings, 0 infos
public/assets/gull.glb: 0 errors, 0 warnings, 1 infos
public/assets/horse.glb: 0 errors, 0 warnings, 0 infos
public/assets/sheep.glb: 0 errors, 0 warnings, 0 infos
exit=0
```

`pc_3c1502703dcd` passed a live build-script invalidation. Touching `build.rs`
made Cargo mark this package dirty and execute its build script; the generated
registry check also passed.

```text
$ touch build.rs && cargo build --locked -vv
Dirty exa-agent-cli v0.3.0: the file `build.rs` has changed
Compiling exa-agent-cli v0.3.0
Running ... /target/debug/build/exa-agent-cli-21966f48aaf03c66/build-script-build
Finished `dev` profile
exit=0
$ EXA_AGENT_NO_NETWORK=1 cargo xtask generate-registry --check
generate-registry --check: OK (normalized fixed-environment capabilities match)
exit=0
```

### Already fixed: failed and left open

`pc_52e555b5a7d0` failed acceptance. The report cites
`explicit_output_wins_over_max_output_bytes_and_confirms_file`, but the current
`remediation-wave5` source checkout has no test by that name. Cargo returned
success after matching zero tests. The nearby output-ceiling tests passed, but
they do not prove explicit output wins. This mismatch was filed as
`pc_6e5ec24f878a`.

```text
$ cargo test --locked explicit_output_wins_over_max_output_bytes_and_confirms_file -- --exact
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored
exit=0
$ cargo test --locked apply_output_ceiling -- --nocapture
running 3 tests
test result: ok. 3 passed; 0 failed
exit=0
```

### Excluded delegate-owned verification

No Wave 6 or delegate-owned cut was resolved or mutated. This includes the
manifest's Wave 6 rows `pc_344b79d2e28e`, `pc_9d8218775b5b`, and
`pc_b967e7071e47`, plus the amended delegate-owned cuts
`pc_c7f83ba034d1` and post-snapshot `pc_92319c59a164`.

## Resolution batches

No batch ran. These were the due batches blocked before append:

- Wave 4b: `pc_595745b3b43e`, `pc_007de9088587`, `pc_41ba11574a6b`,
  `pc_f8d120f7e054`.
- Wave 5: `pc_cb37997204ff`, `pc_a1553455a3d4`, `pc_3615c044abbd`,
  `pc_cc2d338911db`, `pc_0ab19b19876d`, `pc_2df63b1c0880`,
  `pc_dc0fd914fe93`, `pc_bce78a0aff06`, `pc_b057ceb4523e`,
  `pc_26b94226e075`.
- Closed Wave 7 surfaces: contacts `pc_dd0267276789`,
  `pc_ceff5f2fafc5`; ai-profile-router `pc_91911d7ae332`,
  `pc_4642e6d76ee3`, `pc_b95946c00f3d`; transcribe-url
  `pc_97ff0f165238`.
- Wave 8 verified/no-bug/meta: `pc_affa8f792f6c`, `pc_3fee5e17ace2`,
  `pc_f7d578ff5f38`, `pc_a7973681d8f3`, `pc_ca2cfb2732ae`,
  `pc_13afdf97e5a6`, `pc_03d1e73413b7`, `pc_506af2585d23`,
  `pc_a3b6d83c3fb7`, `pc_c0598bce24d9`, `pc_3c1502703dcd`,
  `pc_183edfed93b6`.

No other repository ledger changed, so no other repository commit was due.

## Upstream drafts

Fourteen separate drafts were created under `_scratch/upstream-drafts/`, one
for each manifest row with `stays-open-external`. Nothing was posted.

The files are `pc_07522a6497fc-agent-browser-hidden-page-freeze.md`,
`pc_08099f2644cd-codex-research-collaboration.md`,
`pc_10e28695f5fa-exa-wire-source-availability.md`,
`pc_4a608dca0dec-claude-collaboration-thread-preflight.md`,
`pc_8a4580aae521-codex-exec-output-capture.md`,
`pc_8c2350511589-codex-standalone-collaboration.md`,
`pc_8e017bfc8c63-vercel-api-parameter-help.md`,
`pc_a96389c3cc68-agent-browser-skill-contract.md`,
`pc_b7b1fb2f9854-claude-collaboration-thread-fallback.md`,
`pc_b90d50ead946-gog-calendar-update-integrity.md`,
`pc_bd9ede3cf94d-exa-public-social-fallback.md`,
`pc_bf8ab691e65a-hyperframes-node26-sharp.md`,
`pc_d5448baaf2f5-exa-source-domain-availability.md`, and
`pc_f2720a4950c7-exa-contents-failure-semantics.md`. Each filename begins with
its manifest ID.

Draft validation returned:

```text
draft_count=14 expected_count=14
missing:
extra:
draft_structure=PASS
write-human scan: clean
```

## Expected-open accounting

The current frozen-snapshot state is 86 logically open cuts, not a final
assertion. The live checker can only observe 83 open and 45 resolved cuts
because three open records and one resolved record were in the deleted
worktree ledgers. The bucket formula is:

| Bucket | Count |
| --- | ---: |
| Needs reproduction | 7 |
| External upstream | 14 |
| Codex shell deferred | 3 |
| Trey task blocked | 3 |
| Pending radar, gavel/Packard, and agent-memory repos | 15 |
| Delegate/Wave 6 owned | 10 |
| Due resolutions blocked by missing manifest coverage | 32 |
| OPM complete-part-set verification still unmet | 1 |
| Failed already-fixed verification | 1 |
| **Logical open total** | **86** |

The IDs in each bucket are:

- Needs reproduction: `pc_222f7ad3b20d`, `pc_e5038ed9b918`,
  `pc_aff08102f981`, `pc_acd630d89fa5`, `pc_9cb7c305959d`,
  `pc_a342698f560f`, `pc_e0092074a509`.
- External upstream: `pc_4a608dca0dec`, `pc_b7b1fb2f9854`,
  `pc_8c2350511589`, `pc_f2720a4950c7`, `pc_08099f2644cd`,
  `pc_8a4580aae521`, `pc_bf8ab691e65a`, `pc_07522a6497fc`,
  `pc_d5448baaf2f5`, `pc_bd9ede3cf94d`, `pc_10e28695f5fa`,
  `pc_b90d50ead946`, `pc_a96389c3cc68`, `pc_8e017bfc8c63`.
- Codex shell deferred: `pc_ff863521e129`, `pc_211736ebcc47`,
  `pc_7892e011944e`.
- Trey task blocked: X portal keys `pc_3d8f55856fe6`,
  `pc_b66efae3997d`; DATA.gov key `pc_828f1dfa2edc`.
- Pending prospera-radar-build: `pc_f821dfb7ca32`, `pc_32ee1733d053`,
  `pc_ae44fb08f5ce`, `pc_b37f54ccfbe6`, `pc_dc81b1ac1f3f`,
  `pc_b66b74817bba`, `pc_8312d8ea11fd`, `pc_aff4d7f9b134`,
  `pc_fdd9d446d4c6`, `pc_71fc5d5bea37`.
- Pending gavel, including Packard: `pc_26ad9661d970`,
  `pc_325e89b9af88`, `pc_278287fce683`, `pc_2f283876d668`.
- Pending agent-memory: `pc_a782707f3a97`.
- Delegate/Wave 6 owned: `pc_c7f83ba034d1`, `pc_42a766f4dfcc`,
  `pc_f8eb38d950f5`, `pc_d1a5192425bc`, `pc_d741782a7167`,
  `pc_e31354465446`, `pc_df6af25a100a`, `pc_344b79d2e28e`,
  `pc_9d8218775b5b`, `pc_b967e7071e47`. Post-snapshot
  `pc_92319c59a164` is also delegate-owned but is not part of the 132-cut
  formula.
- Due resolutions blocked by coverage: the four Wave 4b IDs, ten Wave 5 IDs,
  six closed-Wave-7 IDs, and twelve passed Wave 8 IDs listed under
  "Resolution batches" above.
- OPM verification unmet: `pc_b8fe2e571b1f`.
- Failed already-fixed verification: `pc_52e555b5a7d0`.

The final 21-cut assertion cannot run until the missing source-log evidence is
recovered or the manifest contract is deliberately amended, pending waves are
closed, Codex shell behavior is accepted or remains explicitly deferred, both
Trey tasks are complete, the OPM part set is verified, and
`pc_52e555b5a7d0` has a real passing regression.

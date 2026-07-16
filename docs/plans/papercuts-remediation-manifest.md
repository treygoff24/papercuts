# Papercuts remediation manifest — Wave 0

This is the exact cut-level manifest for the frozen 132-cut snapshot in
`docs/papercuts-diagnostic-report-2026-07-15.md`. The report index is the
source of truth for IDs and dispositions; this table assigns each ID exactly
once to the remediation plan's wave and expected end state.

Execution order: `0`, `1`, `2`, `3`, `4a`, `5`, `4b`, `6`, `7`, `8`.
Allowed assignment waves: `1`, `2`, `3`, `4a`, `5`, `4b`, `6`, `7`, `8`.

Allowed end states: `resolve-on-1`, `resolve-on-2`, `resolve-on-3`,
`resolve-on-4a`, `resolve-on-5`, `resolve-on-4b`, `resolve-on-6`,
`resolve-on-7`, `resolve-on-8`, `resolve-on-verification`,
`stays-open-needs-repro`, `stays-open-external`, `stays-open-trey-task`,
`already-resolved`.

`resolve-on-verification` is intentionally not implied by a wave-only check:
it remains open until its named acceptance or regression condition is attested.
The condition table below is row-scoped; there is no global verification flag.
Wave `4a` has no cut-level rows and is a no-cut-delta framework gate before
Wave `5`; its doctor/framework evidence is checked outside cut status.

## Gate usage

`scripts/check-manifest.sh --diagnostic-only --log PATH` inspects a partial
log and prints `DIAGNOSTIC`, never `PASS`. It reports list warnings,
truncation, malformed/torn-record indications, and count/total inconsistencies
without treating the log as gate evidence. A state gate rejects any of those
conditions and requires the complete frozen universe: pass every source with repeated
`--log PATH` arguments (the checker deterministically unions matching IDs), or
pass an explicit aggregate log. For example:

```sh
scripts/check-manifest.sh --after-wave 4a --log "$HOME/.papercuts/log.jsonl" --log .papercuts.jsonl
scripts/check-manifest.sh --after-wave 5 --log "$HOME/.papercuts/log.jsonl" --log .papercuts.jsonl
scripts/check-manifest.sh --after-wave 4b --log "$HOME/.papercuts/log.jsonl" --log .papercuts.jsonl
```

`4a` has no cut-status delta; its command proves only that the same resolved
state remains valid while the separate doctor/framework gate is recorded.
At Wave 1 and later, every harness represented by a shell cut needs exactly
one explicit outcome: `--accept-harness NAME` or `--defer-harness NAME`.
Claude must be accepted; Codex may be deferred, in which case its shell cuts
remain open. Duplicate, conflicting, and unknown outcomes are rejected. The
corrected filing provenance has no delegate shell cuts, so delegate has no
cut-state outcome; its separate lane probe remains a non-cut gate.

Use `--accept-harness claude|codex`, `--defer-harness codex`, `--verify-id ID`,
`--verify-opm-complete-part-set`, `--task-x-key`, and `--task-data-gov-key`
only for the conditions actually completed. Repeating an attestation or naming
an unknown condition is an error. After a condition-backed row has already
been resolved, later gates may omit its old attestation without treating that
monotonic state as a failure.

The final exact formula is: **open = 21** only when every shell harness has
been accepted, all 15 already-fixed verification IDs have passed, the OPM
complete-part-set check has passed, both human tasks have completed, and every
due wave row is resolved. The 21 are the 14 external-upstream and 7
needs-repro rows. If a condition remains, a final gate may still PASS with the
corresponding bounded larger open count, but it prints each remaining
condition; an accepted harness cut that is due cannot remain open.

## Exact count assertions

The checker reads the following assertions and recomputes every value from the
rows below. No approximate counts are used.

<!-- checker assertions
total=132
disposition.already-fixed=15
disposition.external-upstream=14
disposition.fix=43
disposition.instruction-only=50
disposition.needs-repro=7
disposition.resolved=3
wave.1=14
wave.2=7
wave.3=33
wave.4a=0
wave.5=12
wave.4b=4
wave.6=10
wave.7=25
wave.8=27
end_state.already-resolved=3
end_state.resolve-on-1=0
end_state.resolve-on-2=0
end_state.resolve-on-3=33
end_state.resolve-on-4a=0
end_state.resolve-on-4b=4
end_state.resolve-on-5=10
end_state.resolve-on-6=7
end_state.resolve-on-7=21
end_state.resolve-on-8=1
end_state.resolve-on-verification=29
end_state.stays-open-external=14
end_state.stays-open-needs-repro=7
end_state.stays-open-trey-task=3
-->

| Disposition | Count |
| --- | ---: |
| `fix` | 43 |
| `instruction-only` | 50 |
| `already-fixed` | 15 |
| `external-upstream` | 14 |
| `needs-repro` | 7 |
| `resolved` | 3 |
| **total** | **132** |

| Wave | Count |
| --- | ---: |
| `1` | 14 |
| `2` | 7 |
| `3` | 33 |
| `4a` | 0 |
| `5` | 12 |
| `4b` | 4 |
| `6` | 10 |
| `7` | 25 |
| `8` | 27 |
| **total** | **132** |

| Expected end state | Count |
| --- | ---: |
| `already-resolved` | 3 |
| `resolve-on-1` | 0 |
| `resolve-on-2` | 0 |
| `resolve-on-3` | 33 |
| `resolve-on-4a` | 0 |
| `resolve-on-5` | 10 |
| `resolve-on-4b` | 4 |
| `resolve-on-6` | 7 |
| `resolve-on-7` | 21 |
| `resolve-on-8` | 1 |
| `resolve-on-verification` | 29 |
| `stays-open-external` | 14 |
| `stays-open-needs-repro` | 7 |
| `stays-open-trey-task` | 3 |
| **total** | **132** |

## Named resolution conditions

Each condition is attached to one row. Shell conditions are the atomic
per-harness acceptance unit for that shell cut; `pc_02430da9ef6d` and `pc_88e09fdfbb7f` additionally
wait for their Wave 3 documentation correction because their end state is
`resolve-on-3`. The three task-blocked rows resolve only after their named
human task and their assigned Wave 7 work. The OPM row resolves only after its
live complete-part-set check.

| ID | Condition |
| --- | --- |
| `pc_9517658ecfbe` | `shell:claude` |
| `pc_944d374ac9c4` | `shell:claude` |
| `pc_ff863521e129` | `shell:codex` |
| `pc_211736ebcc47` | `shell:codex` |
| `pc_ada11ec02666` | `shell:claude` |
| `pc_8c05423be42a` | `shell:claude` |
| `pc_a5bac2dcb6b8` | `shell:claude` |
| `pc_17287b48a152` | `shell:claude` |
| `pc_bd55a6a719a4` | `shell:claude` |
| `pc_db9e8d6227fa` | `shell:claude` |
| `pc_246f9cd9b37b` | `shell:claude` |
| `pc_6157c26ecfce` | `shell:claude` |
| `pc_88a67e8cea07` | `shell:claude` |
| `pc_7892e011944e` | `shell:codex` |
| `pc_02430da9ef6d` | `shell:claude` |
| `pc_88e09fdfbb7f` | `shell:claude` |
| `pc_344b79d2e28e` | `verify:pc_344b79d2e28e` |
| `pc_9d8218775b5b` | `verify:pc_9d8218775b5b` |
| `pc_affa8f792f6c` | `verify:pc_affa8f792f6c` |
| `pc_3fee5e17ace2` | `verify:pc_3fee5e17ace2` |
| `pc_f7d578ff5f38` | `verify:pc_f7d578ff5f38` |
| `pc_a7973681d8f3` | `verify:pc_a7973681d8f3` |
| `pc_ca2cfb2732ae` | `verify:pc_ca2cfb2732ae` |
| `pc_13afdf97e5a6` | `verify:pc_13afdf97e5a6` |
| `pc_03d1e73413b7` | `verify:pc_03d1e73413b7` |
| `pc_506af2585d23` | `verify:pc_506af2585d23` |
| `pc_a3b6d83c3fb7` | `verify:pc_a3b6d83c3fb7` |
| `pc_c0598bce24d9` | `verify:pc_c0598bce24d9` |
| `pc_b967e7071e47` | `verify:pc_b967e7071e47` |
| `pc_3c1502703dcd` | `verify:pc_3c1502703dcd` |
| `pc_52e555b5a7d0` | `verify:pc_52e555b5a7d0` |
| `pc_b8fe2e571b1f` | `verify:opm-complete-part-set` |
| `pc_3d8f55856fe6` | `task:x-key` |
| `pc_b66efae3997d` | `task:x-key` |
| `pc_828f1dfa2edc` | `task:data-gov-key` |

## Post-snapshot live-log note

`pc_7e7d9f9a0385` was filed after the frozen diagnostic snapshot and is not
part of this manifest. The checker reports and ignores any live-log ID absent
from the frozen report; post-snapshot cuts never count as snapshot drift.

## ID manifest

| ID | Diagnostic disposition | Wave | Expected end state |
| --- | --- | --- | --- |
| `pc_4a608dca0dec` | `external-upstream` | `8` | `stays-open-external` |
| `pc_595745b3b43e` | `fix` | `4b` | `resolve-on-4b` |
| `pc_b7b1fb2f9854` | `external-upstream` | `8` | `stays-open-external` |
| `pc_c7f83ba034d1` | `fix` | `6` | `resolve-on-6` |
| `pc_42a766f4dfcc` | `fix` | `6` | `resolve-on-6` |
| `pc_9abc2154521c` | `resolved` | `8` | `already-resolved` |
| `pc_f821dfb7ca32` | `fix` | `7` | `resolve-on-7` |
| `pc_f8eb38d950f5` | `fix` | `6` | `resolve-on-6` |
| `pc_8c2350511589` | `external-upstream` | `8` | `stays-open-external` |
| `pc_222f7ad3b20d` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_828f1dfa2edc` | `fix` | `7` | `stays-open-trey-task` |
| `pc_f2720a4950c7` | `external-upstream` | `5` | `stays-open-external` |
| `pc_08099f2644cd` | `external-upstream` | `7` | `stays-open-external` |
| `pc_344b79d2e28e` | `already-fixed` | `6` | `resolve-on-verification` |
| `pc_e5038ed9b918` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_48bcd0653758` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_cb37997204ff` | `fix` | `5` | `resolve-on-5` |
| `pc_9d8218775b5b` | `already-fixed` | `6` | `resolve-on-verification` |
| `pc_3681878d4d1b` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_6ffe1c95444b` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_d1a5192425bc` | `fix` | `6` | `resolve-on-6` |
| `pc_affa8f792f6c` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_a385533b3e95` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_8a4580aae521` | `external-upstream` | `8` | `stays-open-external` |
| `pc_a1553455a3d4` | `fix` | `5` | `resolve-on-5` |
| `pc_3615c044abbd` | `instruction-only` | `5` | `resolve-on-5` |
| `pc_bf8ab691e65a` | `external-upstream` | `8` | `stays-open-external` |
| `pc_cc2d338911db` | `fix` | `5` | `resolve-on-5` |
| `pc_3fee5e17ace2` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_b8fe2e571b1f` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_8c05423be42a` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_6c5b407e3864` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_07522a6497fc` | `external-upstream` | `8` | `stays-open-external` |
| `pc_007de9088587` | `fix` | `4b` | `resolve-on-4b` |
| `pc_a5bac2dcb6b8` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_26ad9661d970` | `fix` | `7` | `resolve-on-7` |
| `pc_325e89b9af88` | `fix` | `7` | `resolve-on-7` |
| `pc_c027e2058acb` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_657859fb968d` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_b61350696e1c` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_abce1276d1ce` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_17287b48a152` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_0aef5be73d6b` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_2a32afa6a5d9` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_aff08102f981` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_dd0267276789` | `fix` | `7` | `resolve-on-7` |
| `pc_acd630d89fa5` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_f7d578ff5f38` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_63ff0a8d6ed3` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_a7973681d8f3` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_d5448baaf2f5` | `external-upstream` | `5` | `stays-open-external` |
| `pc_0ab19b19876d` | `fix` | `5` | `resolve-on-5` |
| `pc_086ff9f44d41` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_2413fc2383b5` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_98be51fc86c0` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_ee1f80f998cb` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_d09a98689667` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_2df63b1c0880` | `fix` | `5` | `resolve-on-5` |
| `pc_69f47212dc0a` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_bd9ede3cf94d` | `external-upstream` | `8` | `stays-open-external` |
| `pc_10e28695f5fa` | `external-upstream` | `8` | `stays-open-external` |
| `pc_dc0fd914fe93` | `fix` | `5` | `resolve-on-5` |
| `pc_ca2cfb2732ae` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_02430da9ef6d` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_51e571c07493` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_bd55a6a719a4` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_32ee1733d053` | `fix` | `7` | `resolve-on-7` |
| `pc_13afdf97e5a6` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_03d1e73413b7` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_db9e8d6227fa` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_bce78a0aff06` | `fix` | `5` | `resolve-on-5` |
| `pc_88e09fdfbb7f` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_9cb7c305959d` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_246f9cd9b37b` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_ae44fb08f5ce` | `fix` | `7` | `resolve-on-7` |
| `pc_b37f54ccfbe6` | `instruction-only` | `7` | `resolve-on-7` |
| `pc_dc81b1ac1f3f` | `fix` | `7` | `resolve-on-7` |
| `pc_b66b74817bba` | `fix` | `7` | `resolve-on-7` |
| `pc_8312d8ea11fd` | `fix` | `7` | `resolve-on-7` |
| `pc_aff4d7f9b134` | `fix` | `7` | `resolve-on-7` |
| `pc_b057ceb4523e` | `fix` | `5` | `resolve-on-5` |
| `pc_b90d50ead946` | `external-upstream` | `8` | `stays-open-external` |
| `pc_d741782a7167` | `fix` | `6` | `resolve-on-6` |
| `pc_fdd9d446d4c6` | `fix` | `7` | `resolve-on-7` |
| `pc_71fc5d5bea37` | `fix` | `7` | `resolve-on-7` |
| `pc_a96389c3cc68` | `external-upstream` | `8` | `stays-open-external` |
| `pc_278287fce683` | `fix` | `7` | `resolve-on-7` |
| `pc_db0db641f6cf` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_f8d120f7e054` | `fix` | `4b` | `resolve-on-4b` |
| `pc_183edfed93b6` | `instruction-only` | `8` | `resolve-on-8` |
| `pc_a342698f560f` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_41ba11574a6b` | `fix` | `4b` | `resolve-on-4b` |
| `pc_0aa764d04f6d` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_9517658ecfbe` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_506af2585d23` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_8d0d40377a6b` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_4ba151a66c8d` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_7a6283b8f24b` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_f9dba97b97ea` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_ceff5f2fafc5` | `fix` | `7` | `resolve-on-7` |
| `pc_a3b6d83c3fb7` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_944d374ac9c4` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_c0598bce24d9` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_df6af25a100a` | `fix` | `6` | `resolve-on-6` |
| `pc_a21c970bd217` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_6157c26ecfce` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_2f283876d668` | `fix` | `7` | `resolve-on-7` |
| `pc_ff863521e129` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_e0092074a509` | `needs-repro` | `2` | `stays-open-needs-repro` |
| `pc_f160678a51cc` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_e31354465446` | `fix` | `6` | `resolve-on-6` |
| `pc_b967e7071e47` | `already-fixed` | `6` | `resolve-on-verification` |
| `pc_26b94226e075` | `fix` | `5` | `resolve-on-5` |
| `pc_3c1502703dcd` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_8ae5f391206a` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_91911d7ae332` | `fix` | `7` | `resolve-on-7` |
| `pc_4642e6d76ee3` | `fix` | `7` | `resolve-on-7` |
| `pc_211736ebcc47` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_37830dd5b21e` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_e1e215f2bcc1` | `instruction-only` | `3` | `resolve-on-3` |
| `pc_b95946c00f3d` | `fix` | `7` | `resolve-on-7` |
| `pc_a782707f3a97` | `fix` | `7` | `resolve-on-7` |
| `pc_ada11ec02666` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_88a67e8cea07` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_52e555b5a7d0` | `already-fixed` | `8` | `resolve-on-verification` |
| `pc_8e017bfc8c63` | `external-upstream` | `8` | `stays-open-external` |
| `pc_7892e011944e` | `instruction-only` | `1` | `resolve-on-verification` |
| `pc_97ff0f165238` | `fix` | `7` | `resolve-on-7` |
| `pc_3d8f55856fe6` | `fix` | `7` | `stays-open-trey-task` |
| `pc_b66efae3997d` | `fix` | `7` | `stays-open-trey-task` |
| `pc_ca10740db917` | `resolved` | `8` | `already-resolved` |
| `pc_55fb196ef6ee` | `resolved` | `8` | `already-resolved` |

## Wave 3 execution reconciliation (2026-07-15)

The shared and local instruction evidence and the complete bounded source-log
union are recorded in `docs/reviews/papercuts-wave3-acceptance-2026-07-15.md`.
Resolve due IDs only in their source log with `papercuts --file <source> resolve`;
do not synthesize or rewrite cut events.

`pc_b8fe2e571b1f` remains open because the live OPM lookup did not prove a
complete latest part set. `pc_b37f54ccfbe6` remains Wave 7-owned until the
`test:web:file` script exists; its current typecheck guidance is staged only.

## Amendments (2026-07-16)

- **Ledger-lost IDs (4):** `pc_944d374ac9c4` (resolved, Wave 3 attestation), `pc_8c2350511589` (open, external-upstream), `pc_df6af25a100a` and `pc_f8eb38d950f5` (open, Wave 6 — delegate-agent session owns). Their live JSONL events were destroyed with deleted delegate worktrees after the diagnostic snapshot; the filings survive verbatim in the diagnostic report. `scripts/check-manifest.sh` counts them at these attested statuses and discloses it on every run. None affects a resolvable bucket: all four stay open or were already resolved.
- **Wave 6 ownership:** implemented by the delegate-agent session (see plan Amendments); its cuts resolve there, not via this repo's batches.

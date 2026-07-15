# Paper cuts diagnostic report

This report is the implementation handoff for every paper cut filed by July 15, 2026. It preserves each original filing, records the root cause found during the diagnostic pass, and states the proposed correction and the reason for it. Agents working from this report should verify the live state before editing because several findings concern external tools or behavior that has already changed.

The inventory contains 132 unique paper cuts from 141 cut events across the home log and repository logs. Worktree copies account for the duplicate events. The append-only logs contain 129 open cuts and 3 resolved cuts. The diagnostic pass grouped the cuts into 87 root cause families.

No product or configuration fixes were made during this review. Two filings were created by this report work itself: `pc_8a4580aae521` records dropped execution output during inventory, and `pc_6ffe1c95444b` records a regex replacement failure while preserving verbatim filings. Both are included below.

## Disposition summary

| Disposition | Count | Meaning |
| --- | ---: | --- |
| `fix` | 43 | A local code, configuration, or operational change is warranted. |
| `instruction-only` | 50 | The durable correction belongs in agent instructions, documentation, or a small helper. |
| `already-fixed` | 15 | Current source appears to contain the correction, or the filing was not a product defect. Verify acceptance, then resolve the log entry. |
| `external-upstream` | 14 | The failure belongs to an external runtime, service, or integration. Local work is limited to clearer fallback and diagnosis. |
| `needs-repro` | 7 | The evidence does not isolate the failing layer. Preserve a reproduction before changing code. |
| `resolved` | 3 | The append-only log contains a resolution and current evidence supports it. |

Confidence is evidence based: 75 diagnoses are confirmed, 45 are strong, and 12 are tentative. A tentative diagnosis is not implementation authorization. It is a request for a better reproduction.

## Reading and execution rules

The log status and diagnostic disposition are separate fields. An open cut marked `already-fixed` still needs acceptance verification and an appended resolution event. A cut marked `external-upstream` may justify local error handling, but it does not justify pretending the upstream behavior can be repaired locally. A cut marked `needs-repro` should not be assigned to an implementation lane until the reproduction identifies the failing layer.

Duplicate cuts remain separate entries because the original reports are part of the audit trail. Agents may implement one family-level correction when entries share a root cause, then verify and resolve every covered cut individually. Safety boundaries remain intact. External symlink containment, credential redaction, `__Host` cookie requirements, and destructive-command guards must not be weakened to make a paper cut disappear.

## Index

| No. | ID | Severity | Log status | Disposition | Confidence | Family |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | `pc_4a608dca0dec` | major | open | `external-upstream` | strong | `collaboration-thread-absence` |
| 2 | `pc_595745b3b43e` | major | open | `fix` | confirmed | `browser-use-skill-missing-executable` |
| 3 | `pc_b7b1fb2f9854` | major | open | `external-upstream` | strong | `collaboration-thread-absence` |
| 4 | `pc_c7f83ba034d1` | major | open | `fix` | strong | `delegate-empty-success-output` |
| 5 | `pc_42a766f4dfcc` | major | open | `fix` | confirmed | `delegate-safety-boundaries` |
| 6 | `pc_9abc2154521c` | major | resolved | `resolved` | confirmed | `walk-asset-pipeline` |
| 7 | `pc_f821dfb7ca32` | major | open | `fix` | confirmed | `radar-playwright-isolation` |
| 8 | `pc_f8eb38d950f5` | major | open | `fix` | confirmed | `delegate_external_skill_symlinks` |
| 9 | `pc_8c2350511589` | major | open | `external-upstream` | strong | `collaboration-thread-absence` |
| 10 | `pc_222f7ad3b20d` | major | open | `needs-repro` | tentative | `receipts_silent_invocation` |
| 11 | `pc_828f1dfa2edc` | major | open | `fix` | confirmed | `fec_demo_key_quota` |
| 12 | `pc_f2720a4950c7` | major | open | `external-upstream` | strong | `exa-contents-failure-semantics` |
| 13 | `pc_08099f2644cd` | major | open | `external-upstream` | strong | `collaboration-thread-absence` |
| 14 | `pc_344b79d2e28e` | major | open | `already-fixed` | confirmed | `delegate_codex_error_classification` |
| 15 | `pc_e5038ed9b918` | major | open | `needs-repro` | tentative | `codex_state_db_thread_lookup` |
| 16 | `pc_48bcd0653758` | major | open | `instruction-only` | confirmed | `zsh_startup_xtrace_secret_exposure` |
| 17 | `pc_cb37997204ff` | major | open | `fix` | strong | `exa_repo_no_live_guard` |
| 18 | `pc_9d8218775b5b` | major | open | `already-fixed` | confirmed | `delegate_codex_error_classification` |
| 19 | `pc_3681878d4d1b` | minor | open | `instruction-only` | confirmed | `memoryd-reindex-runbook-shorthand` |
| 20 | `pc_6ffe1c95444b` | minor | open | `instruction-only` | confirmed | `python-regex-replacement-escaping` |
| 21 | `pc_d1a5192425bc` | minor | open | `fix` | confirmed | `delegate-cross-workspace-run-handle` |
| 22 | `pc_affa8f792f6c` | minor | open | `already-fixed` | confirmed | `delegate-src-layout-smoke-imports` |
| 23 | `pc_a385533b3e95` | minor | open | `instruction-only` | strong | `python-set-union-oneoff` |
| 24 | `pc_8a4580aae521` | minor | open | `external-upstream` | tentative | `codex-exec-output-capture` |
| 25 | `pc_a1553455a3d4` | minor | open | `fix` | confirmed | `exa-contents-text-cap-contract` |
| 26 | `pc_3615c044abbd` | minor | open | `instruction-only` | strong | `exa-json-envelope-path-docs` |
| 27 | `pc_bf8ab691e65a` | minor | open | `external-upstream` | strong | `hyperframes-node26-sharp-install` |
| 28 | `pc_cc2d338911db` | minor | open | `fix` | confirmed | `exa-positional-urls-schema-drift` |
| 29 | `pc_3fee5e17ace2` | minor | open | `already-fixed` | confirmed | `exa-search-text-flag-guidance` |
| 30 | `pc_b8fe2e571b1f` | minor | open | `instruction-only` | tentative | `opm-current-file-discovery` |
| 31 | `pc_8c05423be42a` | minor | open | `instruction-only` | confirmed | `zsh-path-special-variable` |
| 32 | `pc_6c5b407e3864` | minor | open | `instruction-only` | strong | `bounded-agent-guidance-discovery` |
| 33 | `pc_07522a6497fc` | minor | open | `external-upstream` | strong | `agent-browser-hidden-page-animation-freeze` |
| 34 | `pc_007de9088587` | minor | open | `fix` | confirmed | `browser-use-skill-missing-executable` |
| 35 | `pc_a5bac2dcb6b8` | minor | open | `instruction-only` | confirmed | `zsh-path-special-variable` |
| 36 | `pc_26ad9661d970` | minor | open | `fix` | confirmed | `gavel-next-workspace-root` |
| 37 | `pc_325e89b9af88` | minor | open | `fix` | confirmed | `gavel-impeccable-interactive-gate` |
| 38 | `pc_c027e2058acb` | minor | open | `instruction-only` | strong | `shell-compound-check-diagnostics` |
| 39 | `pc_657859fb968d` | minor | open | `instruction-only` | confirmed | `nested-host-shell-interpolation` |
| 40 | `pc_b61350696e1c` | minor | open | `instruction-only` | strong | `non-git-workspace-verification` |
| 41 | `pc_abce1276d1ce` | minor | open | `instruction-only` | strong | `shell-oneoff-parser-error` |
| 42 | `pc_17287b48a152` | minor | open | `instruction-only` | confirmed | `zsh-unmatched-glob` |
| 43 | `pc_0aef5be73d6b` | minor | open | `instruction-only` | strong | `bounded-agent-guidance-discovery` |
| 44 | `pc_2a32afa6a5d9` | minor | open | `instruction-only` | confirmed | `non-git-workspace-verification` |
| 45 | `pc_aff08102f981` | minor | open | `needs-repro` | tentative | `apply-patch-multihunk-rejection` |
| 46 | `pc_dd0267276789` | minor | open | `fix` | confirmed | `contacts-email-key-normalization` |
| 47 | `pc_acd630d89fa5` | minor | open | `needs-repro` | tentative | `gog-gmail-id-type-ambiguity` |
| 48 | `pc_f7d578ff5f38` | minor | open | `already-fixed` | strong | `trash-not-rm-guidance` |
| 49 | `pc_63ff0a8d6ed3` | minor | open | `instruction-only` | strong | `shell-array-record-splitting` |
| 50 | `pc_a7973681d8f3` | minor | open | `already-fixed` | confirmed | `hearth-concurrent-test-fixture-mismatch` |
| 51 | `pc_d5448baaf2f5` | minor | open | `external-upstream` | tentative | `exa-source-domain-availability` |
| 52 | `pc_0ab19b19876d` | minor | open | `fix` | confirmed | `exa-contents-text-cap-contract` |
| 53 | `pc_086ff9f44d41` | minor | open | `instruction-only` | strong | `functions-exec-malformed-wrapper` |
| 54 | `pc_2413fc2383b5` | minor | open | `instruction-only` | confirmed | `rg-filename-prefix-path-extraction` |
| 55 | `pc_98be51fc86c0` | minor | open | `instruction-only` | strong | `functions-exec-malformed-wrapper` |
| 56 | `pc_ee1f80f998cb` | minor | open | `instruction-only` | strong | `shell-sed-range-quoting-error` |
| 57 | `pc_d09a98689667` | minor | open | `instruction-only` | strong | `functions-exec-malformed-wrapper` |
| 58 | `pc_2df63b1c0880` | minor | open | `fix` | confirmed | `exa-contents-text-cap-contract` |
| 59 | `pc_69f47212dc0a` | minor | open | `instruction-only` | confirmed | `repo-aware-test-discovery` |
| 60 | `pc_bd9ede3cf94d` | minor | open | `external-upstream` | tentative | `exa-public-social-post-fallback` |
| 61 | `pc_10e28695f5fa` | minor | open | `external-upstream` | tentative | `exa-wire-service-source-availability` |
| 62 | `pc_dc0fd914fe93` | minor | open | `fix` | confirmed | `exa-contents-text-cap-contract` |
| 63 | `pc_ca2cfb2732ae` | minor | open | `already-fixed` | confirmed | `pdf-dependency-resolution` |
| 64 | `pc_02430da9ef6d` | minor | open | `instruction-only` | strong | `invalid-orientation-loop` |
| 65 | `pc_51e571c07493` | minor | open | `instruction-only` | strong | `apply-patch-context-fragility` |
| 66 | `pc_bd55a6a719a4` | minor | open | `instruction-only` | confirmed | `zsh-unmatched-glob` |
| 67 | `pc_32ee1733d053` | minor | open | `fix` | strong | `radar-targeted-test-entrypoints` |
| 68 | `pc_13afdf97e5a6` | minor | open | `already-fixed` | strong | `exa-cli-contract-and-doc-drift` |
| 69 | `pc_03d1e73413b7` | minor | open | `already-fixed` | strong | `exa-cli-contract-and-doc-drift` |
| 70 | `pc_db9e8d6227fa` | minor | open | `instruction-only` | confirmed | `bash-array-indirection-under-zsh` |
| 71 | `pc_bce78a0aff06` | minor | open | `fix` | confirmed | `exa-contents-failure-semantics` |
| 72 | `pc_88e09fdfbb7f` | minor | open | `instruction-only` | confirmed | `unquoted-url-glob-expansion` |
| 73 | `pc_9cb7c305959d` | minor | open | `needs-repro` | tentative | `exa-contents-failure-semantics` |
| 74 | `pc_246f9cd9b37b` | minor | open | `instruction-only` | confirmed | `bash-array-indirection-under-zsh` |
| 75 | `pc_ae44fb08f5ce` | minor | open | `fix` | confirmed | `radar-live-ops-probe` |
| 76 | `pc_b37f54ccfbe6` | minor | open | `instruction-only` | confirmed | `radar-targeted-test-entrypoints` |
| 77 | `pc_dc81b1ac1f3f` | minor | open | `fix` | confirmed | `radar-live-ops-probe` |
| 78 | `pc_b66b74817bba` | minor | open | `fix` | confirmed | `radar-live-ops-probe` |
| 79 | `pc_8312d8ea11fd` | minor | open | `fix` | confirmed | `radar-env-loading` |
| 80 | `pc_aff4d7f9b134` | minor | open | `fix` | confirmed | `radar-live-ops-probe` |
| 81 | `pc_b057ceb4523e` | minor | open | `fix` | confirmed | `exa-positional-urls-schema-drift` |
| 82 | `pc_b90d50ead946` | minor | open | `external-upstream` | strong | `gog-calendar-update-integrity` |
| 83 | `pc_d741782a7167` | minor | open | `fix` | strong | `delegate-safety-boundaries` |
| 84 | `pc_fdd9d446d4c6` | minor | open | `fix` | strong | `radar-playwright-isolation` |
| 85 | `pc_71fc5d5bea37` | minor | open | `fix` | confirmed | `radar-playwright-isolation` |
| 86 | `pc_a96389c3cc68` | minor | open | `external-upstream` | confirmed | `agent-browser-contract-drift` |
| 87 | `pc_278287fce683` | minor | open | `fix` | confirmed | `gavel-local-auth-vqa` |
| 88 | `pc_db0db641f6cf` | minor | open | `instruction-only` | confirmed | `agent-safety-guard-friction` |
| 89 | `pc_f8d120f7e054` | minor | open | `fix` | confirmed | `skill-path-resolution` |
| 90 | `pc_183edfed93b6` | minor | open | `instruction-only` | confirmed | `walk-coordinate-diagnosis` |
| 91 | `pc_a342698f560f` | minor | open | `needs-repro` | tentative | `agent-browser-contract-drift` |
| 92 | `pc_41ba11574a6b` | minor | open | `fix` | confirmed | `browser-use-skill-missing-executable` |
| 93 | `pc_0aa764d04f6d` | minor | open | `instruction-only` | strong | `agent-exec-lifecycle` |
| 94 | `pc_9517658ecfbe` | minor | open | `instruction-only` | confirmed | `zsh-reserved-status-variable` |
| 95 | `pc_506af2585d23` | minor | open | `already-fixed` | confirmed | `walk-coordinate-diagnosis` |
| 96 | `pc_8d0d40377a6b` | minor | open | `instruction-only` | strong | `dropbox-workspace-review-guidance` |
| 97 | `pc_4ba151a66c8d` | minor | open | `instruction-only` | confirmed | `non-git-workspace-verification` |
| 98 | `pc_7a6283b8f24b` | minor | open | `instruction-only` | strong | `awk-label-file-argument-mixup` |
| 99 | `pc_f9dba97b97ea` | minor | open | `instruction-only` | strong | `apply-patch-context-fragility` |
| 100 | `pc_ceff5f2fafc5` | minor | open | `fix` | confirmed | `contacts-warmth-monotonicity` |
| 101 | `pc_a3b6d83c3fb7` | minor | open | `already-fixed` | confirmed | `agent-browser-contract-drift` |
| 102 | `pc_944d374ac9c4` | minor | open | `instruction-only` | confirmed | `zsh-reserved-status-variable` |
| 103 | `pc_c0598bce24d9` | minor | open | `already-fixed` | strong | `threejs_asset_validator_dependency` |
| 104 | `pc_df6af25a100a` | minor | open | `fix` | confirmed | `delegate_external_skill_symlinks` |
| 105 | `pc_a21c970bd217` | minor | open | `instruction-only` | strong | `apply-patch-context-fragility` |
| 106 | `pc_6157c26ecfce` | minor | open | `instruction-only` | strong | `macos_portable_shell_tools` |
| 107 | `pc_2f283876d668` | minor | open | `fix` | confirmed | `prompt_relative_path_ambiguity` |
| 108 | `pc_ff863521e129` | minor | open | `instruction-only` | confirmed | `zsh-reserved-status-variable` |
| 109 | `pc_e0092074a509` | minor | open | `needs-repro` | tentative | `blender_mcp_headless_shutdown` |
| 110 | `pc_f160678a51cc` | minor | open | `instruction-only` | strong | `archive_layout_assumption` |
| 111 | `pc_e31354465446` | minor | open | `fix` | confirmed | `delegate_run_output_tail_default` |
| 112 | `pc_b967e7071e47` | minor | open | `already-fixed` | confirmed | `delegate_codex_error_classification` |
| 113 | `pc_26b94226e075` | minor | open | `fix` | confirmed | `rust_msrv_gate` |
| 114 | `pc_3c1502703dcd` | minor | open | `already-fixed` | strong | `cargo_build_script_invalidation` |
| 115 | `pc_8ae5f391206a` | minor | open | `instruction-only` | confirmed | `cargo_single_test_filter` |
| 116 | `pc_91911d7ae332` | minor | open | `fix` | confirmed | `safe_credential_name_inspection` |
| 117 | `pc_4642e6d76ee3` | minor | open | `fix` | strong | `safe_credential_name_inspection` |
| 118 | `pc_211736ebcc47` | minor | open | `instruction-only` | confirmed | `zsh-reserved-status-variable` |
| 119 | `pc_37830dd5b21e` | minor | open | `instruction-only` | strong | `nested-host-shell-interpolation` |
| 120 | `pc_e1e215f2bcc1` | minor | open | `instruction-only` | strong | `nested_host_shell_interpolation` |
| 121 | `pc_b95946c00f3d` | minor | open | `fix` | strong | `safe_credential_name_inspection` |
| 122 | `pc_a782707f3a97` | minor | open | `fix` | confirmed | `memoryd_repo_runtime_default` |
| 123 | `pc_ada11ec02666` | minor | open | `instruction-only` | confirmed | `zsh-reserved-status-variable` |
| 124 | `pc_88a67e8cea07` | minor | open | `instruction-only` | strong | `macos_portable_shell_tools` |
| 125 | `pc_52e555b5a7d0` | minor | open | `already-fixed` | confirmed | `exa_explicit_output_precedence` |
| 126 | `pc_8e017bfc8c63` | minor | open | `external-upstream` | confirmed | `vercel_api_endpoint_parameter_discovery` |
| 127 | `pc_7892e011944e` | minor | open | `instruction-only` | confirmed | `macos_portable_shell_tools` |
| 128 | `pc_97ff0f165238` | minor | open | `fix` | confirmed | `transcribe_event_page_no_media` |
| 129 | `pc_3d8f55856fe6` | minor | open | `fix` | strong | `x_watch_credential_presence_vs_validity` |
| 130 | `pc_b66efae3997d` | minor | open | `fix` | confirmed | `x_watch_credential_presence_vs_validity` |
| 131 | `pc_ca10740db917` | minor | resolved | `resolved` | confirmed | `journal_cat_n_guard` |
| 132 | `pc_55fb196ef6ee` | minor | resolved | `resolved` | confirmed | `continuity_state_staleness` |

## Detailed findings

## 1. `pc_4a608dca0dec`

Family: `collaboration-thread-absence`
Severity: `major`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: Claude Code collaboration tool integration/runtime
Duplicate of: None

### Original filing

```text
collaboration.spawn_agent failed with 'no thread with id' during a skill-required post-implementation review; Delegate runs should either expose a valid collaboration thread or hide the tool.
```

### Root cause

Claude Code exposed collaboration.spawn_agent even though its runtime invocation was not attached to a valid collaboration thread. Calls therefore failed with no thread with id instead of being hidden or falling back.

### Proposed correction

Gate tool exposure on a successful collaboration-thread preflight. If the runtime has no thread, either create/bind one before exposing spawn_agent or return a structured unavailable result with an approved local Delegate fallback.

### Why this correction

Claude Code collaboration tool integration/runtime owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Two independent cuts from the same repo report the exact no thread with id failure during skill-required review work.
2. The tool was advertised and callable far enough to return a server-side thread lookup error, so this is capability/session wiring rather than an absent skill.
3. The current Codex collaboration surface works in this task, showing the defect is harness/session-specific.

### Risks and constraints

Automatically creating threads may have ownership, billing, and lifecycle implications; capability hiding plus explicit fallback is the safer first step.

## 2. `pc_595745b3b43e`

Family: `browser-use-skill-missing-executable`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: shared skill registry/loader and browser-use skill
Duplicate of: None

### Original filing

```text
browser-use skill was advertised as available, but the browser-use CLI is not installed; skill availability should include an executable prerequisite check or an automatic fallback.
```

### Root cause

A browser-use skill was installed/advertised, but its required browser-use executable was absent. Skill discovery checked files, not runtime prerequisites.

### Proposed correction

Add executable prerequisites to skill discovery and suppress or mark the skill unavailable when browser-use is missing. Prefer automatic routing to the installed agent-browser skill where semantics permit, otherwise surface a one-command install/setup path.

### Why this correction

The failure originates in shared skill registry/loader and browser-use skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. command -v browser-use returns no executable in the live environment.
2. The installed browser-use skill instructs agents to run browser-use and lists browser-use doctor only after activation.
3. agent-browser 0.30.0 is installed at /opt/homebrew/bin/agent-browser and provides overlapping browser automation, so a local fallback exists.

### Risks and constraints

The two installed browser-use skill copies describe incompatible CLIs, so silently aliasing to agent-browser would be unsafe without selecting one canonical browser contract.

## 3. `pc_b7b1fb2f9854`

Family: `collaboration-thread-absence`
Severity: `major`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: Claude Code collaboration tool integration/runtime
Duplicate of: `pc_4a608dca0dec`

### Original filing

```text
collaboration.spawn_agent failed with 'no thread with id' during a skill-required parallel code review; a supported availability preflight or graceful local fallback would prevent the dead end
```

### Root cause

Same harness/session wiring defect: collaboration.spawn_agent was exposed without a valid collaboration thread and failed with no thread with id.

### Proposed correction

Use the canonical fix from pc_4a608dca0dec: preflight thread binding and hide/fallback when unavailable.

### Why this correction

Claude Code collaboration tool integration/runtime owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Exact error matches pc_4a608dca0dec.
2. Both failures occurred in /Users/treygoff/Code/warroom during skill-required post-implementation review workflows.
3. The repeated failure rules out a one-off prompt typo.

### Risks and constraints

Same as canonical item; avoid silently degrading a required independent-review workflow without reporting the fallback.

## 4. `pc_c7f83ba034d1`

Family: `delegate-empty-success-output`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: delegate-agent runner capture/result quality contract
Duplicate of: None

### Original filing

```text
delegate grok/cursor safe lanes returned status=succeeded with empty text envelopes on a read-only review prompt (claims sweep, 2 of 2 lanes); same bug family as delegate-agent#12. Workaround: codex call --read-only returns text reliably; safe-mode lanes cannot write result files to the real tree so the file-output workaround is unavailable in safe mode.
```

### Root cause

Delegate can mark a child run succeeded from process/harness terminal status without enforcing a semantic non-empty text result. Safe-mode isolation also prevents the suggested file-output workaround from writing to the real tree.

### Proposed correction

For review/report prompts, fail or flag resultQuality=empty when the final text is blank despite success; retry once with an explicit final-answer request and preserve raw stdout/stderr diagnostics. Document call --read-only as the safe fallback until fixed.

### Why this correction

The failure originates in delegate-agent runner capture/result quality contract. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Cut reports 2/2 grok/cursor safe lanes returned status=succeeded with empty text envelopes while codex call --read-only returned text.
2. Installed delegate is 0.13.1 and GitHub issue #12 remains open for semantically empty succeeded workflow outputs.
3. Delegate source records succeeded terminal events independently of a non-empty rendered result; exact grok/cursor empty-text capture path was not reproduced live.
4. Delegate help now exposes stateless call --read-only, matching the reported reliable workaround.

### Risks and constraints

Some legitimate agent runs intentionally return no text after file mutations; enforce non-empty output only when the invocation contract requires a report or make it an explicit launch option.

## 5. `pc_42a766f4dfcc`

Family: `delegate-safety-boundaries`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: delegate work-mode isolation and machine command guards
Duplicate of: None

### Original filing

```text
delegate codex work lane on the real tree ran a 'temporary-QA cleanup' that moved the entire checkout to Trash mid-run (self-recovered, but left a corrupt commit-graph cache referencing pruned objects). Lanes should never relocate the working checkout; worktree isolation or a guard on trash/mv of the repo root would prevent it.
```

### Root cause

Delegate work mode defaults auto isolation to none, so the Codex child ran in the source checkout. Prompt-only safety cannot stop an agent from moving/trashing its own absolute workspace root.

### Proposed correction

Default edit-capable work runs to persistent worktree isolation for Git repos, with explicit opt-out for real-tree work; additionally deny trash/mv of source/execution roots at the launcher/hook boundary.

### Why this correction

The failure originates in delegate work-mode isolation and machine command guards. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. src/delegate_agent/isolation.py lines 123-127 maps auto to worktree only for safe mode and returns none for work mode.
2. Current docs offer explicit `--isolation worktree`, but it is opt-in for work.
3. Persistent-worktree prompt text forbids deleting/renaming the workspace, but there is no host guard against absolute-path moves; the complaint records source checkout relocation and git commit-graph fallout.

### Risks and constraints

Default worktrees may omit dirty/untracked context unless include-dirty behavior and user messaging are exact; guards must not block normal file moves.

## 6. `pc_9abc2154521c`

Family: `walk-asset-pipeline`
Severity: `major`
Log status: `resolved`
Diagnostic disposition: `resolved`
Confidence: `confirmed`
Owner surface: claude-space threejs-agent-kit asset tooling
Duplicate of: None

### Original filing

```text
walk/threejs-agent-kit scripts/optimize-glb.mjs spawns a bare 'gltf-transform' binary that is not installed anywhere (not in devDeps, not global) — npx -y @gltf-transform/cli optimize also exits silently with no output. Asset pipeline docs promise 'npm run assets:optimize' but it cannot work from a clean checkout; add @gltf-transform/cli to devDependencies
```

### Root cause

The clean checkout originally lacked @gltf-transform/cli even though scripts spawned its bare binary. That dependency has since been added.

### Proposed correction

No further change; retain the devDependency and clean-checkout asset pipeline test.

### Why this correction

The log contains a resolution for `walk-asset-pipeline` and current evidence supports it. No additional implementation is justified.

### Evidence

1. Cut status is resolved in the source log.
2. Current threejs-agent-kit package.json devDependencies includes @gltf-transform/cli ^4.4.1.
3. Current node_modules/.bin/gltf-transform exists, and git history includes `The asset pipeline works from a clean checkout again`.

### Risks and constraints

The script still spawns a bare binary, so it depends on npm-run PATH; direct `node scripts/optimize-glb.mjs` remains less portable.

## 7. `pc_f821dfb7ca32`

Family: `radar-playwright-isolation`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build Next/Playwright config
Duplicate of: None

### Original filing

```text
Playwright's isolated :3407 webServer still cannot start while an unrelated Next dev server owns web/.next at :3400; the test config's reuseExistingServer=false does not isolate Next's dev lock. A separate distDir/worktree test command would prevent this.
```

### Root cause

Playwright uses a different port but the same web/.next dist directory as any existing Next dev server; reuseExistingServer:false isolates the HTTP server, not Next’s filesystem lock.

### Proposed correction

Set an E2E-only distDir via environment in next.config and Playwright webServer command (for example .next-e2e-&lt;port&gt;), keeping production/default .next unchanged.

### Why this correction

The failure originates in prospera-radar-build Next/Playwright config. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. web/playwright.config.ts defaults to port 3407 and sets reuseExistingServer:false.
2. web/next.config.ts defines no E2E-specific distDir, so all dev servers use .next.
3. The complaint records an unrelated :3400 dev server owning the .next lock while :3407 startup failed.

### Risks and constraints

Dynamic dist directories can accumulate and tracing/caching behavior must be tested on Next 16.

## 8. `pc_f8eb38d950f5`

Family: `delegate_external_skill_symlinks`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: delegate-agent safe workspace materialization plus affected repositories' skill layout
Duplicate of: None

### Original filing

```text
Delegate worktree replaced the mandatory .claude/skills/clean-code symlink with an 'External symlink blocked' text stub, so agents cannot load the repo-required skill; preserve or vendor readable skill instructions inside isolated worktrees.
```

### Root cause

Delegate safe isolation intentionally replaces symlinks whose targets resolve outside the source repository with inert placeholder files. The project stored mandatory skill entrypoints as external symlinks, so the security boundary and the project instruction contract conflict.

### Proposed correction

Keep the escape protection, but materialize required project skills as validated read-only file copies inside the isolated tree (or vendor the small required skill entrypoints in the repo). Never preserve a live external symlink.

### Why this correction

The failure originates in delegate-agent safe workspace materialization plus affected repositories' skill layout. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The preserved worktree .claude/skills/clean-code, .codex/skills/clean-code, and .codex/skills/tdd paths are regular 53-byte files containing 'External symlink blocked by Delegate safe isolation.'
2. delegate-agent/src/delegate_agent/safe_workspace.py defines that exact placeholder and blocks symlinks resolving outside source_root.
3. delegate-agent/docs/worktrees.md documents that external symlinks are blocked.

### Risks and constraints

1. A broad symlink exception would reopen workspace escape and secret-write-through risks; copying must be bounded to explicit skill files and must not follow nested links blindly.

## 9. `pc_8c2350511589`

Family: `collaboration-thread-absence`
Severity: `major`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: Codex collaboration runtime plus Delegate/research prompt instructions
Duplicate of: `pc_4a608dca0dec`

### Original filing

```text
collaboration.spawn_agent failed with 'no thread with id' despite root thread; prevented requested parallel audit
```

### Root cause

A Delegate Codex lane is a standalone `codex exec` subprocess, not a child in the Codex app's collaboration tree. A prompt that asks that subprocess to call collaboration.spawn_agent can expose a collaboration surface without a resolvable root thread.

### Proposed correction

Do not require nested collaboration inside Delegate prompts. Have the parent orchestrate lanes or use Delegate workflow parallelism; additionally, Codex should hide or fail-fast the collaboration tool with an explicit 'standalone exec has no team thread' message.

### Why this correction

Codex collaboration runtime plus Delegate/research prompt instructions owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. delegate-agent/docs/cli-reference.md says Codex prompts are delivered on stdin to `codex exec`.
2. No Delegate source creates or registers Codex app collaboration child threads.
3. This diagnostic lane was successfully spawned from an actual Codex app root thread, showing the API itself works when a coordinator thread exists.
4. The exact historic 'no thread with id' runtime state was not retained for replay.

### Risks and constraints

1. Retrying spawn_agent cannot create missing coordinator state and wastes quota; changing Delegate alone cannot repair Codex thread registration.

## 10. `pc_222f7ad3b20d`

Family: `receipts_silent_invocation`
Severity: `major`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: receipts timeout/error path or the invoking runner, pending reproduction
Duplicate of: None

### Original filing

```text
receipts standard FEC verification returned an empty stdout file and no visible stderr or exit report after ~48s, despite doctor healthy; an explicit machine error envelope on all failures would prevent silent dead ends.
```

### Root cause

The retained evidence cannot determine whether receipts exited silently or an outer runner timed out/killed it and failed to preserve stderr/exit status. Current receipts source is fail-closed around structured envelopes, so the original empty capture needs a reproducible command and runner context.

### Proposed correction

Reproduce with the exact FEC question under `receipts --json --max-seconds N`, capturing stdout, stderr, and exit together. If receipts itself goes silent, add a forced provider-hang integration test and a top-level timeout error envelope; if the wrapper dropped channels, fix the wrapper instead.

### Why this correction

The evidence for `receipts_silent_invocation` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. Installed receipts is v0.2.1 and exposes `--max-seconds` plus JSON error-envelope contracts.
2. recon/src/envelope.rs emits failures on stderr and tests assert empty stdout plus a structured stderr envelope for usage/auth failures.
3. No original argv, exit code, stderr capture, or run artifact accompanies the complaint; only the empty stdout file and elapsed time are recorded.

### Risks and constraints

1. Fixing receipts without proving where output was lost may duplicate envelopes or mask a runner-level cancellation bug.

## 11. `pc_828f1dfa2edc`

Family: `fec_demo_key_quota`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: local ai-profile credential routing and FEC research helper
Duplicate of: None

### Original filing

```text
FEC OpenAPI DEMO_KEY was already over its 40-call hourly limit before this lane's verification, blocking a fresh API reconfirmation; a configured personal DATA.gov key would prevent this.
```

### Root cause

The research used FEC's shared DEMO_KEY, whose 40-call hourly pool was already exhausted, because no personal DATA.gov/FEC key was available in the active profile.

### Proposed correction

Create a personal DATA.gov API key, store it in the protected profile key file, add its name to the managed credential union, and make the FEC research helper prefer it before DEMO_KEY.

### Why this correction

The failure originates in local ai-profile credential routing and FEC research helper. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The research artifact states the DEMO_KEY exceeded its hourly limit and that the audit should be rerun with a personal DATA.gov key.
2. Current process has neither DATA_GOV_API_KEY nor FEC_API_KEY set.
3. No value was inspected; only key presence was checked.

### Risks and constraints

1. The key must never be committed or printed; the helper still needs explicit 429 handling because personal keys also have quotas.

## 12. `pc_f2720a4950c7`

Family: `exa-contents-failure-semantics`
Severity: `major`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: Exa API upstream and exa-agent-cli contents warning/fallback UX
Duplicate of: None

### Original filing

```text
Exa contents returned partial URL failures for valid live Cato pages with empty error objects, forcing direct fetch/search fallback; clearer crawl failure reasons or live retry would prevent dead ends.
```

### Root cause

Exa's contents endpoint can return HTTP success with per-URL crawl failures and sometimes an empty upstream error object. The CLI cannot recover a reason that Exa did not supply.

### Proposed correction

Preserve the full per-URL status payload, label an empty error as `upstream_reason_unavailable`, and offer an explicit one-retry/direct-fetch fallback command rather than silently dead-ending.

### Why this correction

Exa API upstream and exa-agent-cli contents warning/fallback UX owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. exa-agent-cli currently parses statuses[] and emits url_failed warnings; all-URL failure exits 10 with all_urls_failed.
2. Current tests cover partial and total URL failures and preserve status tags.
3. The specific valid Cato crawl and empty error object are complaint evidence; no live retry was performed in this read-only diagnosis.

### Risks and constraints

1. Automatic retries spend quota and may duplicate slow crawls; direct fetch can differ from Exa-rendered content and must be labeled as fallback evidence.

## 13. `pc_08099f2644cd`

Family: `collaboration-thread-absence`
Severity: `major`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: research skill instructions plus Codex collaboration runtime
Duplicate of: `pc_4a608dca0dec`

### Original filing

```text
Research skill required a background agent, but collaboration.spawn_agent failed with 'no thread' despite an active Delegate run; direct research had to continue without the mandated background pass.
```

### Root cause

The research skill mandated a collaboration child from inside a standalone Delegate Codex subprocess, which had no Codex app collaboration root thread.

### Proposed correction

Make background research conditional on a real collaboration coordinator; otherwise let the parent launch it or use Delegate workflow parallelism and continue directly.

### Why this correction

research skill instructions plus Codex collaboration runtime owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Delegate documents Codex execution through standalone `codex exec`.
2. The complaint records `no thread` in an active Delegate run, matching the architecture mismatch.
3. No retained runtime artifact proves the exact missing thread identifier.

### Risks and constraints

1. A mandatory background-pass rule that ignores runtime capability will continue to create false task failures.

## 14. `pc_344b79d2e28e`

Family: `delegate_codex_error_classification`
Severity: `major`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: delegate-agent Codex event normalization and synthesized report
Duplicate of: None

### Original filing

```text
delegate synthesized completion report says only 'harness_error' when codex fails on a subscription quota wall; the real reason (usage limit, retry time) is buried in --stdout JSON. Surface terminal error messages in the synthesized report
```

### Root cause

Older Delegate event handling reduced Codex terminal failures to harness_error without promoting error-event text into classification and synthesized reports.

### Proposed correction

No new implementation; close after running the existing usage-limit/auth regression tests against current main and installed Delegate.

### Why this correction

Current evidence indicates that the `delegate_codex_error_classification` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current harness_events.py explicitly ingests Codex `{type:error,message:...}` events for failover and synthesized reports.
2. Current runner classifies account quota text as usage_limit and appends a redacted `Harness error:` terminal reason.
3. Current tests cover usage-limit classification and synthesized failure reports.

### Risks and constraints

1. Novel Codex error wording can still miss classifiers; retain the redacted terminal message even when the coarse reason remains harness_error.

## 15. `pc_e5038ed9b918`

Family: `codex_state_db_thread_lookup`
Severity: `major`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: Codex CLI state store upstream, with Delegate classification/retry UX
Duplicate of: None

### Original filing

```text
Delegate Codex Wave F failed before execution because Codex state DB thread lookup discrepancies caused a harness_error; retry should recover cleanly or surface a repair command.
```

### Root cause

The complaint indicates Codex's local state DB could not resolve a thread before the child began execution. There is not enough retained evidence to distinguish stale restored state, a deleted thread, or a Codex regression.

### Proposed correction

On the next occurrence, preserve redacted Codex stdout/stderr and version, then have Delegate detect the known thread-lookup signature, retry once as a fresh non-resumed exec, and surface a Codex repair/login command if retry fails.

### Why this correction

The evidence for `codex_state_db_thread_lookup` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. The cut states failure occurred before execution and surfaced through Delegate as harness_error.
2. Current Delegate launches standalone codex exec and has no code that repairs Codex's internal state DB.
3. No failing stdout/state DB row or exact error string was retained in the cut.

### Risks and constraints

1. Blind retry can duplicate work if the first run actually started; retry only failures proven to occur before turn execution.

## 16. `pc_48bcd0653758`

Family: `zsh_startup_xtrace_secret_exposure`
Severity: `major`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared shell-testing instructions and ai-profile test helpers
Duplicate of: None

### Original filing

```text
zsh smoke-test diagnostics source global .zshenv and expose secret values under -x; provide a documented no-startup-files test mode
```

### Root cause

A zsh smoke command enabled xtrace while normal startup files, including .zshenv, could source credential assignments before the tested script disabled tracing.

### Proposed correction

Document and standardize secret-safe shell smokes as `zsh -df` (optionally under a minimal env); never combine `-x` with normal user startup files.

### Why this correction

The evidence identifies `zsh_startup_xtrace_secret_exposure` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. A nonempty ~/.zshenv exists on this machine.
2. `zsh -df -c ...` succeeds as a no-startup-files diagnostic mode; -f alone does not prevent .zshenv.
3. ai-profile launchers unset xtrace after they are sourced, which cannot protect lines traced earlier by shell startup.

### Risks and constraints

1. No-startup mode changes PATH/functions; tests must explicitly provide the minimal dependencies they need instead of silently falling back to the user's environment.

## 17. `pc_cb37997204ff`

Family: `exa_repo_no_live_guard`
Severity: `major`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: exa-agent-cli centralized transport and developer scripts
Duplicate of: None

### Original filing

```text
A docs-probe shell snippet used the installed exa-agent binary and made an unintended live Exa contents call; repo work should have an explicit no-live-call guard or a docs-only fetch helper.
```

### Root cause

Repo diagnostics invoked the installed exa-agent binary with a live read command. Read operations are networked by default, and there is no environment-level no-network guard for development probes.

### Proposed correction

Add a repo-test `EXA_AGENT_NO_NETWORK=1` guard at the centralized transport boundary and make docs probes set it; use offline robot-docs/schema commands or dry-run request previews for documentation checks.

### Why this correction

The failure originates in exa-agent-cli centralized transport and developer scripts. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. exa-agent documents capabilities/schema/doctor as offline and supports universal `--dry-run --print-request`, but live search/contents remain networked by default.
2. No EXA_AGENT_NO_NETWORK/OFFLINE execution guard was found in source.
3. The unintended call itself is complaint evidence; no request log was replayed.

### Risks and constraints

1. The guard must fail before credential resolution/network I/O and must be opt-in so intentional live dogfood remains possible.

## 18. `pc_9d8218775b5b`

Family: `delegate_codex_error_classification`
Severity: `major`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: delegate-agent Codex failure classification and synthesized completion report
Duplicate of: `pc_344b79d2e28e`

### Original filing

```text
delegate codex lanes failed twice mid-build (work-account quota, then an expired restored token) but both surfaced as generic harness_error; a quota/auth-specific failure reason would have saved ~30 minutes of diagnosis
```

### Root cause

Older Delegate collapsed both Codex work-account quota and revoked restored-token failures into generic harness_error, losing the decisive child error text.

### Proposed correction

No new implementation; verify current installed Delegate and close this as covered by the existing classifier/report regression suite.

### Why this correction

Current evidence indicates that the `delegate_codex_error_classification` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current Delegate separately classifies usage_limit and auth_failed from Codex stream/stderr signals.
2. Current synthesized reports include redacted terminal reason, auth remediation, and next actions.
3. Current tests cover both classifier families and guard against false-positive auth classification.

### Risks and constraints

1. Keep raw child messages redacted and bounded; actionable failure reporting must not leak tokens from stderr/stdout.

## 19. `pc_3681878d4d1b`

Family: `memoryd-reindex-runbook-shorthand`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: agent-memory live-repair packet and runbooks
Duplicate of: None

### Original filing

```text
live-repair runbook says 'doctor + reindex' but memoryd has no reindex/index-rebuild subcommand; index updates ride the import path and startup reconcile. Either add a maintenance reindex surface or fix the runbook language.
```

### Root cause

The live-repair packet says 'doctor + reindex' without naming the command or its scope. Memoryd does have an explicit repair surface, `memoryd doctor --reindex`, but it rebuilds the event-log mirror rather than exposing a general standalone reindex subcommand. The shorthand made a real flag look like a missing command.

### Proposed correction

Replace the shorthand with the exact `memoryd doctor --repo "$MEMORUM_REPO" --runtime "$MEMORUM_RUNTIME" --reindex` command and state that it rebuilds the SQLite event-log mirror from canonical JSONL. Do not add a second reindex subcommand unless a distinct full-index maintenance operation is actually needed.

### Why this correction

The evidence identifies `memoryd-reindex-runbook-shorthand` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. docs/reviews/memora-arc/trey-decision-packet-2026-07-12.md line 36 ends the live pass with 'doctor + reindex'.
2. crates/memoryd/src/cli/mod.rs defines DoctorArgs.reindex as a boolean CLI flag.
3. crates/memoryd/src/cli/daemon.rs calls doctor_reindex_events_log when the flag is set, and crates/memoryd/tests/doctor_reindex.rs pins the command contract.

### Risks and constraints

Calling this a full reindex would overstate the flag. The documentation must preserve the narrower event-log-mirror scope.

## 20. `pc_6ffe1c95444b`

Family: `python-regex-replacement-escaping`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: agent report-generation instructions and one-off transformation scripts
Duplicate of: None

### Original filing

```text
Python re.sub treated backslashes in verbatim paper-cut text as replacement escapes during Markdown report generation, causing a bad-escape retry; use a function replacement whenever inserting untrusted text
```

### Root cause

Python treats a string passed as the replacement argument to `re.sub` as a replacement-template language. A verbatim paper cut contained `\s`, so inserting it as the replacement raised `re.PatternError: bad escape \s` before the report could be written.

### Proposed correction

When untrusted or verbatim text is inserted by a regex transformation, pass a callable replacement such as `lambda _: replacement`, or avoid regex substitution and generate the section directly.

### Why this correction

The evidence identifies `python-regex-replacement-escaping` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The report-generation command failed at re.subn replacement parsing with `bad escape \s`.
2. The original `pc_ca10740db917` filing contains the regex fragment `'^\s*\d+\t'`.
3. The same replacement works when supplied through a callable because callable return values are inserted verbatim.

### Risks and constraints

Escaping only known backslashes is incomplete because future source text can contain other replacement-template syntax. Treat the entire replacement as data.

## 21. `pc_d1a5192425bc`

Family: `delegate-cross-workspace-run-handle`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: delegate-agent run registry and run-output resolution UX
Duplicate of: None

### Original filing

```text
delegate run-output with a bare harness name resolves per-workspace: launching with --cwd <worktree> registers there, and querying from the repo root silently returns a months-old run (cursor-29) instead of erroring — nearly caused stale-findings triage. A cross-workspace 'not found here, did you mean...' hint would have prevented it.
```

### Root cause

Delegate stores run registries under each invocation workspace. A bare harness handle such as `cursor` means the latest cursor run in the current workspace, not the latest cursor run machine-wide. Because the repository root contained an old local cursor run, lookup succeeded there instead of revealing that the intended run was registered in a worktree.

### Proposed correction

For bare-harness resolution, always surface the selected workspace, run ID, alias, and age. Warn when the selected run is stale and repeat the `--cwd` guidance. Launch output should provide a copyable inspection command with the exact source workspace and run ID. Add a machine-wide workspace locator only if the explicit command and stale-run warning prove insufficient.

### Why this correction

The failure originates in delegate-agent run registry and run-output resolution UX. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. run_output_commands.emit resolves only the current workspace registry before looking up the handle.
2. run_registry.resolve_handle treats a bare harness name as the latest run for that harness in the selected registry.
3. The existing unknown-handle error mentions per-workspace storage and `--cwd`, but that branch does not run when an old local harness match exists.

### Risks and constraints

Silently searching every repository would create ambiguity and privacy costs. Exact run IDs and explicit workspaces must remain the canonical lookup contract.

## 22. `pc_affa8f792f6c`

Family: `delegate-src-layout-smoke-imports`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: delegate-agent development workflow
Duplicate of: None

### Original filing

```text
Repo-local Python smoke probes fail without PYTHONPATH=src; a documented helper or dev-shell default would prevent the dead-end.
```

### Root cause

Delegate uses a standard Python `src` layout. Importing `delegate_agent` from an uninstalled checkout with the system interpreter therefore fails by design. The repository already documents creating a virtual environment and installing the package editable; the smoke probe bypassed that development environment.

### Proposed correction

Do not change package discovery or set a repository-wide PYTHONPATH. Run probes with the documented editable environment, preferably `.venv/bin/python`, or use `python3 bin/delegate.py` for CLI smoke tests. Add a wrapper only if agents repeatedly need module-level probes outside the virtual environment.

### Why this correction

Current evidence indicates that the `delegate-src-layout-smoke-imports` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. pyproject.toml sets package-dir to `src` and discovers packages under `src`.
2. README.md lines 44 through 52 document a virtual environment followed by `python -m pip install -e .` for local development and checkout smoke tests.
3. docs/agent-setup.md repeats the editable-install workflow, and AGENTS.md documents `python3 -m pip install -e ".[dev]"` for development tooling.

### Risks and constraints

A global `PYTHONPATH=src` can mask packaging and installation defects. Editable installation is the safer development contract.

## 23. `pc_a385533b3e95`

Family: `python-set-union-oneoff`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent-authored Python analysis snippets
Duplicate of: None

### Original filing

```text
Ad-hoc duplicate-token survey failed because the overlap-dedup loop tried to add a set to a set; a checked reusable clone-survey helper would prevent this avoidable retry.
```

### Root cause

The one-off duplicate-token survey used `set.add(other_set)`. `add` inserts one hashable element, while a mutable set is unhashable; the intended operation was set union through `update` or `|=`.

### Proposed correction

Use `seen.update(tokens)` or `seen |= tokens`, then leave one small assertion that proves overlaps are counted once. Create a reusable clone-survey helper only if this survey recurs.

### Why this correction

The evidence identifies `python-set-union-oneoff` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The filing records the exact set-to-set operation and the failed overlap-dedup step.
2. Python set.add accepts one element; set.update and the `|=` operator add members from another set.
3. The failed ad-hoc script was not retained, so the diagnosis is based on the recorded operation rather than source replay.

### Risks and constraints

A new permanent helper would be needless if this remains a one-off survey. The immediate correction is the proper set operation plus an assertion.

## 24. `pc_8a4580aae521`

Family: `codex-exec-output-capture`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `tentative`
Owner surface: Codex functions.exec / exec_command runtime
Duplicate of: None

### Original filing

```text
exec_command returned no captured output for successful multi-line inventory commands, forcing repeated simpler probes; preserving output or surfacing why it was dropped would avoid redundant diagnostics
```

### Root cause

A Codex exec bridge sometimes completed a multi-line command successfully but returned no captured stdout. The complaint establishes the symptom, but the capture/drop point is outside this repository and did not reproduce in this lane.

### Proposed correction

In the Codex exec bridge, preserve stdout whenever exit status is zero; if output is intentionally dropped or truncated, return an explicit reason and byte counts. Add a regression using a successful multi-line command with mixed stdout/stderr.

### Why this correction

Codex functions.exec / exec_command runtime owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Complaint reports successful multi-line inventory commands with empty captured output and successful simpler retries.
2. Current lane ran several multi-line exec_command calls and received stdout, so the failure is intermittent or context-specific rather than a universal shell behavior.
3. No papercuts repository code implements functions.exec or exec_command capture.

### Risks and constraints

Without a captured failing invocation or bridge logs, a fix could target the wrong layer (shell, PTY, truncation, or response serialization).

## 25. `pc_a1553455a3d4`

Family: `exa-contents-text-cap-contract`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent CLI schema/help/validation and exa-agent-cli skill
Duplicate of: None

### Original filing

```text
exa-agent contents schema omitted the 10,000-character --text cap, causing an avoidable invalid_value retry
```

### Root cause

Exa CLI validation enforces a 1..10000 numeric --text range, but schema output omits the range and current help incorrectly says contents numeric caps are uncapped. The skill also gives no 10000-limit warning.

### Proposed correction

Make one contract authoritative: either support full/0 as the help promises or document numeric 1..10000 in generated help, schema constraints, examples, and suggestedCommand. The CLI should reject locally before any network work and suggest --text full or 10000 for contents.

### Why this correction

The failure originates in exa-agent CLI schema/help/validation and exa-agent-cli skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. exa-agent 0.2.0 schema show contents lists text but no min/max metadata.
2. exa-agent contents --help says bare --text is uncapped and accepts N without naming a maximum.
3. exa-agent contents https://example.com --text 20000 --dry-run returns invalid_value with min 1 and max 10000.
4. The live exa-agent-cli skill contains contents --text examples but no 10000/10,000 cap text.

### Risks and constraints

Simply clamping silently could hide caller intent; an explicit local error plus correct alternatives is safer.

## 26. `pc_3615c044abbd`

Family: `exa-json-envelope-path-docs`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: exa-agent-cli skill documentation
Duplicate of: None

### Original filing

```text
exa-agent search --json envelope did not expose results at top level, causing six jq parse failures; skill should document the actual JSON response path
```

### Root cause

The current skill documents the generic success envelope, but it still omits a concrete, live-verified jq path for search results. That leaves callers to guess whether results are under `.data.results`, another nested field, or a spill file.

### Proposed correction

Live-verify the search response shape, then add one copyable jq example for the exact result path to the Exa skill and schema examples.

### Why this correction

The evidence identifies `exa-json-envelope-path-docs` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Current exa-agent-cli skill says one JSON envelope per call and identifies data as the payload.
2. The skill explicitly says request previews are at data.request.body and oversized payloads use dataPath.
3. Complaint-only evidence records six jq failures from assuming a top-level results field; the historical output was not preserved in the cut.

### Risks and constraints

The exact upstream search payload key was not exercised live to avoid a billed/network call; only the envelope location is confirmed.

## 27. `pc_bf8ab691e65a`

Family: `hyperframes-node26-sharp-install`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: hyperframes npm packaging and CLI launcher
Duplicate of: None

### Original filing

```text
hyperframes@0.7.58 npx install fails silently on node v26.3.1: sharp postinstall wants node-addon-api/node-gyp, npx caches the broken install and the CLI exits 1 with zero output. Workaround: npm install --prefix <dir> --ignore-scripts hyperframes@latest, run dist/cli.js directly (sharp prebuilds load fine at runtime).
```

### Root cause

hyperframes 0.7.58 declares Node &gt;=22 and depends on sharp 0.34.5. Sharp runs an install check and falls back to a native build; on Node 26.3.1 the reported prebuild check failed and the fallback lacked node-addon-api/node-gyp. npx then reused the incomplete cache while Hyperframes emitted no useful error.

### Proposed correction

Add Node 26 install smoke coverage; either narrow Hyperframes' engines until sharp install is supported, move sharp behind a lazy/optional path, or ensure native build prerequisites are declared. Never swallow npx/npm stderr, and detect/clear a failed npx cache before retry guidance.

### Why this correction

hyperframes npm packaging and CLI launcher owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Live Node version is v26.3.1.
2. npm registry metadata for hyperframes@0.7.58 declares engines node &gt;=22 and dependency sharp ^0.34.5.
3. npm registry metadata for sharp@0.34.5 defines install as node install/check.js || npm run build and exposes platform binaries as optional dependencies.
4. Complaint supplies a successful workaround: install Hyperframes with --ignore-scripts and invoke dist/cli.js, indicating runtime loading works while install lifecycle is the failure point.

### Risks and constraints

The exact failed npx cache was not retained, so the missing-prebuild reason is based on the detailed cut plus package lifecycle metadata rather than a fresh failed install.

## 28. `pc_cc2d338911db`

Family: `exa-positional-urls-schema-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent generated schema and contents argument parser
Duplicate of: None

### Original filing

```text
exa-agent schema show contents reports a --urls flag/body path, but the CLI rejects --urls as unknown; schema should expose the actual positional multi-URL syntax or a working flag
```

### Root cause

Generated schema labels urls as a flag/body field, while clap implements URLs only as positional arguments. Agents reasonably translate schema flag names into --urls, which the CLI rejects.

### Proposed correction

Represent positional inputs distinctly in schema (for example kind=positional, name=urls, variadic=true) or implement a real repeatable --urls alias. Update the skill with a multi-URL positional example.

### Why this correction

The failure originates in exa-agent generated schema and contents argument parser. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. exa-agent schema show contents emits a field with flag=urls and bodyPath=urls.
2. exa-agent contents --help shows Usage: contents [OPTIONS] [URLS]... and Arguments: [URLS]..., with no --urls option.
3. exa-agent contents --urls https://example.com --dry-run returns unknown_flag.

### Risks and constraints

Adding --urls must preserve positional compatibility and define how mixed positional/flag URLs are ordered and deduplicated.

## 29. `pc_3fee5e17ace2`

Family: `exa-search-text-flag-guidance`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: exa-agent CLI typo suggestions and exa-agent-cli skill
Duplicate of: None

### Original filing

```text
exa-agent search skill examples/assumptions led to unsupported --include-text flag; a discoverable search usage example would have prevented the retry
```

### Root cause

An older example or assumption used --include-text, which is not an Exa CLI flag. The current CLI and skill consistently use --text.

### Proposed correction

No CLI change is required. Retain the current --text examples; optionally map --include-text to a targeted suggestion for --text rather than --include-domain.

### Why this correction

Current evidence indicates that the `exa-search-text-flag-guidance` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. exa-agent search --help documents --text [N|full] and no --include-text.
2. Current exa-agent-cli skill recipes use --text, --text 4000, and --text full.
3. exa-agent search foo --include-text --dry-run returns unknown_flag and a did-you-mean unrelated to text.

### Risks and constraints

Old cached skill copies may still contain stale examples; installed-copy synchronization should be checked when implementing.

## 30. `pc_b8fe2e571b1f`

Family: `opm-current-file-discovery`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `tentative`
Owner surface: research skill/helper library for OPM datasets
Duplicate of: None

### Original filing

```text
OPM Federal Workforce Data file index returns the entire multi-decade catalog with no documented current-only example; a small helper/query recipe would prevent downloading and parsing a huge index just to locate the latest month.
```

### Root cause

The OPM Federal Workforce Data file index is a historical catalog, while the research task needed only the newest employment snapshot. No local recipe translated 'latest month' into a bounded index query or direct download path.

### Proposed correction

Add a short research recipe/helper that queries the file index once, sorts employment entries by year/month, selects the latest complete part set, and prints direct download URLs. Keep it local rather than requiring OPM to redesign its archive API.

### Why this correction

The evidence identifies `opm-current-file-discovery` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint-only evidence says the index returned a multi-decade catalog and required downloading/parsing it to find the latest month.
2. The resulting local DFC research cites direct OPM download paths shaped /api/v1/files/employment/YYYY/MM/part/download, showing a bounded recipe is possible once year/month/part are known.
3. No OPM current-only helper or recipe was found in the local research skill libraries.

### Risks and constraints

A naive max(year,month) selector can pick an incomplete publication; the helper must account for all expected parts and OPM corrections.

## 31. `pc_8c05423be42a`

Family: `zsh-path-special-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global agent shell instructions
Duplicate of: None

### Original filing

```text
Using a zsh loop variable named 'path' silently overwrites the special PATH array and makes every subsequent command disappear; shell guidance should warn agents to avoid  as a zsh variable.
```

### Root cause

In zsh, path is a special tied array for PATH. Assigning path=... replaces the executable search path, so later commands appear to vanish.

### Proposed correction

Add one global shell-guidance warning: never use path as a zsh variable; prefer file, target, or filepath. A shellcheck-style preflight may flag assignments to zsh special parameters if such a wrapper is later introduced.

### Why this correction

The evidence identifies `zsh-path-special-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. A live zsh probe showed commands[cat]=/bin/cat before path=demo and missing afterward, with PATH=demo.
2. The same behavior was independently filed from two workspaces.
3. This is zsh language behavior, not a repository defect.

### Risks and constraints

A broad textual lint can false-positive on quoted data or non-zsh scripts; keep the initial fix documentary.

## 32. `pc_6c5b407e3864`

Family: `bounded-agent-guidance-discovery`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global agent orientation instructions/helper
Duplicate of: None

### Original filing

```text
Repo guidance lookup with rg over '..' traversed every sibling repository and flooded output; a scoped upward-only AGENTS.md helper would prevent this.
```

### Root cause

The orientation command searched the recursive parent tree for AGENTS.md, traversing every sibling repository. The task only needed the finite ancestor chain from cwd to filesystem root.

### Proposed correction

Document or ship a tiny upward-only lookup that checks $PWD/AGENTS.md, then each parent directory, stopping at /. Do not use find/rg recursively over .. for instruction discovery.

### Why this correction

The evidence identifies `bounded-agent-guidance-discovery` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint reports rg over .. flooding output across sibling repositories.
2. The local repo has AGENTS.md at its root and global guidance is at /Users/treygoff/.codex/AGENTS.md; recursive sibling traversal is unnecessary.
3. No scoped upward-only AGENTS helper is documented in the current repo instructions.

### Risks and constraints

The helper must preserve precedence and include harness-specific global instruction locations rather than assuming all guidance lives on the ancestor chain.

## 33. `pc_07522a6497fc`

Family: `agent-browser-hidden-page-animation-freeze`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: agent-browser browser lifecycle/session runtime
Duplicate of: None

### Original filing

```text
agent-browser pages run visibility:hidden so framer-motion/rAF animations freeze mid-flight between screenshots — AnimatePresence-gated content (act switches) doesn't mount until you force frames with throwaway screenshots, which mimics real bugs and eats keypresses fired mid-transition. A flag to force page visibility (or an idle-frames-then-screenshot command) would have saved ~30 min of false-positive chasing.
```

### Root cause

The browser automation page runs with document visibility hidden, so requestAnimationFrame/framer-motion transitions can be throttled or frozen between screenshots. Input sent during the stalled transition is consumed before AnimatePresence-gated content mounts.

### Proposed correction

Add a launch/session flag that forces Page.setWebLifecycleState active and document visibility visible where Chromium permits, plus a deterministic settle command that advances animation frames until rAF/transition activity is idle before interaction or screenshots.

### Why this correction

agent-browser browser lifecycle/session runtime owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Complaint records that throwaway screenshots advanced frames and unblocked the UI, a strong signature of background/visibility-driven frame throttling.
2. agent-browser 0.30.0 exposes waits, screenshots, viewport, and scrollbar controls but its help and core skill expose no force-page-visible or advance-idle-frames control.
3. The current core skill recommends condition/network waits but does not address visibility-gated animation clocks.

### Risks and constraints

Forcing visibility changes real browser semantics and can mask legitimate background-tab bugs; make it explicit and off by default.

## 34. `pc_007de9088587`

Family: `browser-use-skill-missing-executable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: shared skill registry/loader and browser-use skill
Duplicate of: `pc_595745b3b43e`

### Original filing

```text
browser-use skill was listed as available but the browser-use CLI is not installed; a capability preflight or accurate skill availability signal would have avoided the dead end
```

### Root cause

Same skill-capability mismatch: browser-use was listed but no browser-use executable was installed.

### Proposed correction

Use the canonical prerequisite-aware skill discovery and explicit agent-browser fallback proposed for pc_595745b3b43e.

### Why this correction

The failure originates in shared skill registry/loader and browser-use skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Exact symptom matches pc_595745b3b43e.
2. command -v browser-use is empty in the current environment.
3. Both cuts were filed from the warroom workflow within hours.

### Risks and constraints

Same incompatible-skill-copy risk as the canonical item.

## 35. `pc_a5bac2dcb6b8`

Family: `zsh-path-special-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global agent shell instructions
Duplicate of: `pc_8c05423be42a`

### Original filing

```text
In zsh, using 'path' as a local shell variable overwrites PATH and makes later commands like cat disappear; a shell-scripting helper or lint warning would prevent this footgun.
```

### Root cause

Same zsh special-parameter footgun: assigning local variable path overwrote PATH.

### Proposed correction

Use the canonical global zsh warning proposed for pc_8c05423be42a.

### Why this correction

The evidence identifies `zsh-path-special-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint exactly matches pc_8c05423be42a from a second workspace.
2. Live zsh probe confirmed path assignment removes command lookup.
3. The behavior is deterministic across zsh invocations.

### Risks and constraints

None beyond keeping the guidance zsh-specific.

## 36. `pc_26ad9661d970`

Family: `gavel-next-workspace-root`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: gavel next.config.ts
Duplicate of: None

### Original filing

```text
Gavel's Next.js build warns that it inferred /Users/treygoff as the workspace root because of multiple lockfiles; setting turbopack.root in next.config.ts would remove ambiguity and the warning.
```

### Root cause

Gavel's Next.js config does not set turbopack.root, and multiple lockfiles above/around the project cause Next.js to infer /Users/treygoff as the workspace root.

### Proposed correction

Set turbopack.root to the Gavel project directory in next.config.ts using the shortest Next-supported config form, then verify npm run build no longer warns.

### Why this correction

The failure originates in gavel next.config.ts. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. /Users/treygoff/Code/gavel/next.config.ts defines headers, poweredByHeader, and productionBrowserSourceMaps only; no turbopack.root exists.
2. Gavel uses Next 16.2.10.
3. The cut records the exact Next.js workspace-root warning and inferred /Users/treygoff path.

### Risks and constraints

An incorrectly resolved ESM dirname can break builds; use Next's documented config pattern and verify from both repo root and npm scripts.

## 37. `pc_325e89b9af88`

Family: `gavel-impeccable-interactive-gate`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: gavel package.json quality gate
Duplicate of: None

### Original filing

```text
npm run gate is not noninteractive: impeccable detect src prompted 'Continue? [Y/n]' on the full repo, requiring a TTY response; the gate should pass a yes/noninteractive flag.
```

### Root cause

Gavel's gate runs impeccable detect src in human mode. Impeccable explicitly prompts Continue? when scanning more than 50 files on a TTY unless --json or --quiet is used; its detect command has no --yes flag.

### Proposed correction

Change Gavel's design:detect script to impeccable detect --quiet src (or --json if downstream parsing is desired). This bypasses the prompt without inventing an unsupported --yes flag.

### Why this correction

The failure originates in gavel package.json quality gate. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Gavel package.json defines design:detect as impeccable detect src and gate invokes it.
2. Installed impeccable source prompts when files.length &gt; 50, stdin is a TTY, and neither jsonMode nor quietMode is set.
3. impeccable detect --help exposes --json and --quiet but no yes/noninteractive flag.

### Risks and constraints

Quiet mode still needs to preserve a nonzero exit on findings; verify gate failure semantics before adopting it.

## 38. `pc_c027e2058acb`

Family: `shell-compound-check-diagnostics`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global shell verification guidance
Duplicate of: None

### Original filing

```text
A combined render-verification shell check exited without a diagnostic, requiring separate assertions to identify the failed condition.
```

### Root cause

A compact compound render check used silent predicates/short-circuiting, so its nonzero exit identified failure but not which assertion failed.

### Proposed correction

Use named assertions that print a short failure label to stderr before exiting, or run independent checks with accumulated status. Add a reusable snippet to verification guidance rather than a product abstraction.

### Why this correction

The evidence identifies `shell-compound-check-diagnostics` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint-only evidence says separate assertions immediately identified the failed condition.
2. The temporary /private/tmp/pact-wf workspace no longer exists, so the exact command cannot be inspected.
3. Shell test, grep -q, and chained &amp;&amp; checks are silent by default on false predicates.

### Risks and constraints

Overly verbose wrappers can obscure the original command; keep labels one line and preserve exit codes.

## 39. `pc_657859fb968d`

Family: `nested-host-shell-interpolation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: functions.exec documentation/examples
Duplicate of: None

### Original filing

```text
functions.exec JavaScript template literals collide with zsh parameter expansion such as ${target:h}; a clearer example or escaping note would prevent a dead-end verification call
```

### Root cause

A zsh command containing ${target:h} was embedded unescaped in a JavaScript template literal. JavaScript tries to parse ${...} before zsh sees it, and target:h is invalid JavaScript syntax.

### Proposed correction

Document a safe rule: pass shell commands as ordinary quoted JS strings or escape the dollar sign before shell interpolation inside template literals. Prefer one-purpose exec_command calls over composing shell-heavy JavaScript wrappers.

### Why this correction

The evidence identifies `nested-host-shell-interpolation` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The cut names the exact collision between functions.exec JavaScript template literals and zsh ${target:h} expansion.
2. functions.exec accepts raw JavaScript, so template literal interpolation is evaluated in the orchestration layer first.
3. zsh uses ${name:h} as a pathname modifier, making this a predictable two-language delimiter collision.

### Risks and constraints

Escaping guidance differs between JS strings, template literals, and heredocs; examples must name the exact quoting layer.

## 40. `pc_b61350696e1c`

Family: `non-git-workspace-verification`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global verification workflow guidance
Duplicate of: `pc_2a32afa6a5d9`

### Original filing

```text
The task workspace is not a Git repository, so a scoped git status verification cannot run; a repository-presence check before status would prevent this dead-end.
```

### Root cause

The verification recipe assumed the task workspace was a Git repository and ran git status without preflighting repository presence.

### Proposed correction

Use the canonical preflight and snapshot/hash fallback proposed for pc_2a32afa6a5d9.

### Why this correction

The evidence identifies `non-git-workspace-verification` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The original /private/tmp/pact-wf directory no longer exists, so only the cut preserves this instance.
2. The same failure family is live in the PACT Act workspace, where git rev-parse reports not a git repository.
3. Git status/diff cannot provide scoped change verification in non-Git directories.

### Risks and constraints

Same as canonical item: a fallback snapshot must be scoped to intended files to avoid expensive tree-wide comparisons.

## 41. `pc_abce1276d1ce`

Family: `shell-oneoff-parser-error`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global shell recipe library
Duplicate of: None

### Original filing

```text
A compact awk frontmatter-extraction snippet double-printed the delimiter under an action-order mistake; a tested helper for copying YAML frontmatter would prevent accidental body duplication.
```

### Root cause

The AWK action ordering printed the closing YAML delimiter twice. This was an untested one-off parser rather than a missing product feature.

### Proposed correction

Add one tested, copyable frontmatter extraction recipe using a single state transition and exit at the second delimiter. Do not build a general parser unless structured YAML transformation is actually needed.

### Why this correction

The evidence identifies `shell-oneoff-parser-error` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint says the delimiter duplication came from an action-order mistake and was found during frontmatter extraction.
2. The temporary workspace is gone, so the exact AWK program is unavailable for source-level confirmation.
3. YAML frontmatter is delimiter-sensitive; multiple matching AWK actions can both print the same delimiter unless they use next/exit or exclusive conditions.

### Risks and constraints

A delimiter-only helper will not handle BOMs or nonstandard frontmatter; document that narrow contract.

## 42. `pc_17287b48a152`

Family: `zsh-unmatched-glob`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global agent shell instructions
Duplicate of: None

### Original filing

```text
A guarded '*.md' loop still failed under zsh nomatch before the in-loop existence check; a shell-safe empty-directory listing helper or documented nullglob pattern would have prevented it.
```

### Root cause

zsh expands unmatched globs before entering the loop and raises nomatch, so an in-loop [[ -e $f ]] guard can never run for an empty directory.

### Proposed correction

Document the zsh-local null-glob form *.md(N), or avoid glob loops by using a bounded find/rg --files pipeline that handles zero results. Prefer the native (N) qualifier for short zsh-only snippets.

### Why this correction

The evidence identifies `zsh-unmatched-glob` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. A live zsh probe over an empty temporary directory failed with zsh: no matches found before loop execution.
2. The cut describes the same failure for a guarded *.md loop.
3. This is deterministic zsh expansion behavior.

### Risks and constraints

The (N) qualifier is zsh-specific and will fail under bash; examples must state the shell.

## 43. `pc_0aef5be73d6b`

Family: `bounded-agent-guidance-discovery`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global agent orientation instructions/helper
Duplicate of: `pc_6c5b407e3864`

### Original filing

```text
A compound repo-orientation command stopped after 'find .. -name AGENTS.md' with no error or remaining command output; a bounded AGENTS lookup would have prevented the dead end.
```

### Root cause

Same unbounded orientation pattern: find .. -name AGENTS.md traversed sibling repositories and blocked the rest of a compound command.

### Proposed correction

Use the upward-only AGENTS lookup proposed for pc_6c5b407e3864 and keep orientation probes as separate bounded calls.

### Why this correction

The evidence identifies `bounded-agent-guidance-discovery` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The cut names find .. -name AGENTS.md as the stopping command.
2. The task needed ancestor guidance, not a recursive subtree scan.
3. This matches pc_6c5b407e3864's flooded rg-over-parent failure.

### Risks and constraints

Same precedence/global-location risk as the canonical item.

## 44. `pc_2a32afa6a5d9`

Family: `non-git-workspace-verification`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global verification workflow guidance
Duplicate of: None

### Original filing

```text
Attempted a scoped git diff verification in the PACT Act workspace, but the directory is not a Git repository; a local snapshot or documented non-Git verification helper would prevent this dead end.
```

### Root cause

The workflow assumed every workspace supports git diff. The PACT Act Dropbox directory has no .git repository, so Git cannot provide before/after evidence.

### Proposed correction

Preflight with git rev-parse --is-inside-work-tree. For non-Git workspaces, record explicit target-file hashes/copies before mutation and compare only those paths afterward; otherwise state that verification is content-based, not diff-based.

### Why this correction

The evidence identifies `non-git-workspace-verification` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live git -C /Users/treygoff/Library/CloudStorage/Dropbox/Prospera/Policy/pact-act rev-parse returns not a git repository.
2. The cut records a failed scoped git diff in that same workspace.
3. A second cut reports the same assumption in a temporary non-Git workspace.

### Risks and constraints

Tree-wide snapshots are expensive and noisy in Dropbox; keep fallback scope explicit and narrow.

## 45. `pc_aff08102f981`

Family: `apply-patch-multihunk-rejection`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: Codex apply_patch tool
Duplicate of: None

### Original filing

```text
apply_patch rejected valid multi-hunk patches in campaign.ts; applying the same logical edits as smaller hunks was required
```

### Root cause

apply_patch rejected a logically valid multi-hunk patch but accepted the same edits as smaller hunks. The exact patch and parser error were not retained, so context drift, hunk formatting, or a tool parser defect cannot be distinguished.

### Proposed correction

Capture the next failing patch payload, target file hash, and parser error; add it as an apply_patch regression. In the meantime, keep the small-hunk retry guidance because it is safe and worked.

### Why this correction

The evidence for `apply-patch-multihunk-rejection` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. Complaint reports same logical edits succeeded when split into smaller hunks.
2. No failed patch payload or apply_patch error text is stored in the cut.
3. No papercuts repository code owns apply_patch.

### Risks and constraints

Changing parser tolerance without a minimal repro can accept ambiguous patches or apply hunks at unintended locations.

## 46. `pc_dd0267276789`

Family: `contacts-email-key-normalization`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: contacts CLI touch command and override writer
Duplicate of: None

### Original filing

```text
contacts touch preserves mixed-case email keys instead of normalizing them, so touching Akers.IN04@mail.house.gov created an override that did not merge into the canonical lowercase contact; normalize email targets before writing overrides
```

### Root cause

contacts touch preserves the caller's exact email casing as the override YAML key instead of resolving and writing the canonical lowercase contact key. YAML then contains distinct mixed-case and lowercase mappings that do not merge.

### Proposed correction

At the touch trust boundary, lowercase/trim exact email targets before lookup and before override serialization, then merge into the resolved canonical contact key. Add one regression that touches mixed case and proves only the lowercase key is emitted.

### Why this correction

The failure originates in contacts CLI touch command and override writer. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. contacts 0.1.0 touch mixed-case --dry-run echoes target Akers.IN04@mail.house.gov unchanged.
2. curated-overrides.yaml contains both Akers.IN04@mail.house.gov and akers.in04@mail.house.gov keys.
3. contacts.jsonl canonical email is lowercase akers.in04@mail.house.gov and its audit records a case-normalized correction.

### Risks and constraints

Email local parts are theoretically case-sensitive; for this contact store the existing canonicalization is already lowercase, but normalization should be documented as store policy.

## 47. `pc_acd630d89fa5`

Family: `gog-gmail-id-type-ambiguity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: gog Gmail command validation and help
Duplicate of: None

### Original filing

```text
gog gmail thread get returned the opaque error 'empty threadId' when passed a valid search result id; a clearer distinction between thread and message identifiers would prevent retry ambiguity
```

### Root cause

A value presented as a search result id was passed to gmail thread get, which requires a Gmail threadId. Either the caller used a message id from a different result shape or gog failed to extract a valid thread id and surfaced the internal empty threadId error.

### Proposed correction

Improve the error to state whether the supplied value is empty, a message id, or a thread id; include the exact jq/select recipe from gmail search to gmail thread get. Add a regression using both message and thread result envelopes.

### Why this correction

The evidence for `gog-gmail-id-type-ambiguity` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. gog v0.21.0 help says gmail search searches threads and gmail thread get requires &lt;threadId&gt;.
2. The cut says a valid search-result id produced opaque empty threadId, but does not preserve the search command or JSON result shape.
3. No live mailbox call was made because reproducing requires selecting user mail data and the identifier is absent.

### Risks and constraints

Automatically converting message ids to thread ids would require an API lookup and can hide caller mistakes; diagnose first.

## 48. `pc_f7d578ff5f38`

Family: `trash-not-rm-guidance`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `strong`
Owner surface: global Codex/Claude instructions and deletion hook
Duplicate of: None

### Original filing

```text
Smoke setup used rm for a temp directory and was hook-blocked; the shell environment should surface repo deletion guidance before command execution
```

### Root cause

Current global instructions prove the trash-not-rm rule exists now, but the filing does not establish whether the original run predated that guidance or the harness failed to load or follow it.

### Proposed correction

No further code change: retain the current global rule and ensure all harnesses load it before command generation. The command-time hook should continue naming trash in its rejection message.

### Why this correction

Current evidence indicates that the `trash-not-rm-guidance` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current /Users/treygoff/.codex/AGENTS.md has an explicit top-level Deletions: trash, never rm rule and says rm is hook-blocked machine-wide.
2. Current /Users/treygoff/.claude/CLAUDE.md contains the same rule.
3. The cut predates or bypassed that now-prominent guidance.

### Risks and constraints

If a harness fails to load global guidance, the hook remains the final safeguard; verify its error text is actionable when this recurs.

## 49. `pc_63ff0a8d6ed3`

Family: `shell-array-record-splitting`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global shell recipe guidance
Duplicate of: None

### Original filing

```text
Multi-file shell range loop split each file/range tuple into separate words and produced misleading no-such-file output; a documented helper or safer array pattern would prevent it
```

### Root cause

The loop encoded file/range records as whitespace-delimited strings, so shell word splitting separated each tuple into unrelated tokens and generated fake missing-file paths.

### Proposed correction

Use parallel arrays or tab-delimited records read with IFS=$'\t' read -r file range; for a one-off, separate commands are safer. Add one copyable multi-file range recipe to shell guidance.

### Why this correction

The evidence identifies `shell-array-record-splitting` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Complaint explicitly identifies tuple word splitting as the source of no-such-file output.
2. No failed command was retained, so delimiter/array syntax cannot be inspected.
3. Shell whitespace splitting is expected unless records are quoted or encoded with a non-ambiguous delimiter.

### Risks and constraints

Delimiter recipes still fail if paths contain that delimiter; newline/NUL records are needed for fully arbitrary paths, but likely unnecessary for this narrow workflow.

## 50. `pc_a7973681d8f3`

Family: `hearth-concurrent-test-fixture-mismatch`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: hearth src/shared/validate.ts and validate.test.ts
Duplicate of: None

### Original filing

```text
Full npm test is blocked by concurrent src/shared/validate.ts lane: validate.test.ts strips slotsUsed key 3 but keeps usesSpent junk, so the fixture's expected sanitized character does not match.
```

### Root cause

A concurrent Hearth lane temporarily left parseUsesSpent behavior and the expected sanitized fixture out of sync. The current test intentionally preserves arbitrary usesSpent keys and now matches implementation.

### Proposed correction

No fix remains for this cut. Preserve lane ownership discipline and rerun the full suite after concurrent edits merge.

### Why this correction

Current evidence indicates that the `hearth-concurrent-test-fixture-mismatch` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current parseUsesSpent copies every bounded key/value, including junk.
2. Current validate.test.ts expects usesSpent to contain both Arcane Recovery and junk while slotsUsed drops junk.
3. npm test -- --run src/shared/validate.test.ts passes 23/23 tests.

### Risks and constraints

This conclusion covers the reported mismatch only; it does not decide whether preserving arbitrary usesSpent keys is the desired long-term schema.

## 51. `pc_d5448baaf2f5`

Family: `exa-source-domain-availability`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `tentative`
Owner surface: exa-agent search error handling and research skill recipe
Duplicate of: None

### Original filing

```text
exa-agent search treats site:domain.com queries as domain filters and returns SOURCE_NOT_AVAILABLE for several common publishers, forcing broader queries and secondary discovery.
```

### Root cause

Exa/upstream source access treats some site-constrained queries as unavailable rather than returning ordinary zero results. Embedding site: in the query provides no stable fallback when that publisher is unsupported.

### Proposed correction

Document --include-domain as the supported domain restriction path, classify SOURCE_NOT_AVAILABLE distinctly from zero hits, and suggest a broader search plus result-domain filtering when the source is unsupported.

### Why this correction

exa-agent search error handling and research skill recipe owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Cut reports SOURCE_NOT_AVAILABLE for several common publishers when using site:domain.com queries.
2. Current exa-agent search has explicit --include-domain and --exclude-domain flags, but the installed skill's main recipe does not demonstrate them.
3. No live publisher query was run, so current upstream domain availability is unverified.

### Risks and constraints

Broad fallback searches can surface mirrors with weaker provenance; preserve source labeling and do not treat syndicated copies as the original.

## 52. `pc_0ab19b19876d`

Family: `exa-contents-text-cap-contract`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent CLI schema/help/validation and exa-agent-cli skill
Duplicate of: `pc_a1553455a3d4`

### Original filing

```text
exa-agent contents rejects --text 12000 even though the research task needs long pages; the CLI error only surfaces after the call, so the skill should state the current 1-10000 cap.
```

### Root cause

Same undocumented numeric contents cap: --text 12000 exceeds the live 10000 validator limit, while help/schema do not state it.

### Proposed correction

Use the canonical schema/help/validation fix proposed for pc_a1553455a3d4.

### Why this correction

The failure originates in exa-agent CLI schema/help/validation and exa-agent-cli skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Live dry-run with --text 20000 confirms max=10000.
2. Current help omits the numeric maximum.
3. The cut specifically records 12000 rejection.

### Risks and constraints

Same silent-clamping risk as canonical item.

## 53. `pc_086ff9f44d41`

Family: `functions-exec-malformed-wrapper`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: functions.exec documentation and client-side validation
Duplicate of: `pc_d09a98689667`

### Original filing

```text
A second malformed functions.exec wrapper expression failed before execution; simpler one-purpose invocations would reduce avoidable orchestration errors.
```

### Root cause

Same orchestration-layer syntax family: an over-composed functions.exec JavaScript expression failed before the intended shell command executed.

### Proposed correction

Use the canonical direct-call/examples and pre-execution syntax validation proposal from pc_d09a98689667.

### Why this correction

The evidence identifies `functions-exec-malformed-wrapper` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Cut labels this as the second malformed wrapper expression.
2. Two earlier cuts in the same research workflow report the same pre-execution failure.
3. No payload/error is preserved for finer parsing diagnosis.

### Risks and constraints

Same as canonical: clearer validation cannot make arbitrary generated JavaScript semantically correct.

## 54. `pc_2413fc2383b5`

Family: `rg-filename-prefix-path-extraction`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global shell/data-extraction guidance
Duplicate of: None

### Original filing

```text
rg -o spill-path extraction included the searched filename prefix, producing malformed file:/path arguments for trash; use --no-filename or avoid parsing path-bearing JSON when cleaning
```

### Root cause

rg -o prefixes matches with filenames when searching files, so parsing the output as a bare spill path produced file:/path rather than /path.

### Proposed correction

Prefer jq on the JSON field. If rg is necessary, add --no-filename and print/inspect targets before passing them to trash.

### Why this correction

The evidence identifies `rg-filename-prefix-path-extraction` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The cut identifies the searched filename prefix and the resulting malformed file:/path argument.
2. ripgrep's normal multi-file output includes filename prefixes unless --no-filename/-h is used.
3. The cleanup target came from path-bearing JSON, which should be parsed structurally rather than regexed when possible.

### Risks and constraints

Any text parser can mis-handle escaped JSON paths; jq is the safer default.

## 55. `pc_98be51fc86c0`

Family: `functions-exec-malformed-wrapper`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: functions.exec documentation and client-side validation
Duplicate of: `pc_d09a98689667`

### Original filing

```text
A malformed tool-orchestration snippet failed before running a research command; a simpler direct tool invocation would reduce avoidable syntax risk.
```

### Root cause

Same pre-execution orchestration syntax failure caused by composing an unnecessary wrapper around a direct research command.

### Proposed correction

Use the canonical one-purpose tool invocation and syntax-validation change proposed for pc_d09a98689667.

### Why this correction

The evidence identifies `functions-exec-malformed-wrapper` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Cut says the snippet failed before running the research command.
2. It occurred seconds after pc_d09a98689667 in the same workflow.
3. No command payload was retained.

### Risks and constraints

Same as canonical.

## 56. `pc_ee1f80f998cb`

Family: `shell-sed-range-quoting-error`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: global shell guidance
Duplicate of: None

### Original filing

```text
A quoted sed range typo caused a needless dead-end during evidence extraction; a shell-safe helper or checked command wrapper would prevent this.
```

### Root cause

A malformed quoted sed range was an agent-authored command typo, not a data or product failure.

### Proposed correction

Prefer rg -n with -C context or separate, simple sed -n 'START,ENDp' calls. Do not add a product helper for a one-off typo unless this pattern repeats.

### Why this correction

The evidence identifies `shell-sed-range-quoting-error` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Cut explicitly attributes the dead end to a quoted sed range typo.
2. No command/error payload remains, so the exact quoting error cannot be reproduced.
3. The task was evidence extraction, which can usually use rg with context or sed -n with a validated numeric range.

### Risks and constraints

None material; over-standardizing simple reads would add more syntax than it removes.

## 57. `pc_d09a98689667`

Family: `functions-exec-malformed-wrapper`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: functions.exec documentation and client-side validation
Duplicate of: None

### Original filing

```text
A malformed functions.exec wrapper expression failed before execution; a clearer validation error or typed helper would prevent losing a research turn.
```

### Root cause

The intended command was wrapped in generated JavaScript that was syntactically invalid, so functions.exec could not evaluate the module and the nested tool never ran.

### Proposed correction

For one tool call, document the minimal direct pattern await tools.exec_command({...}); text(r.output). Add client-side JavaScript parse errors that include line/column and do not consume a research turn where possible.

### Why this correction

The evidence identifies `functions-exec-malformed-wrapper` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Cut states failure occurred before execution and asks for validation/typed helpers.
2. Two follow-on cuts from the same workflow report repeated malformed wrapper expressions.
3. functions.exec accepts raw JavaScript, creating an additional syntax layer unnecessary for a single exec_command call.

### Risks and constraints

A typed builder would reduce syntax errors but add API surface; start with minimal examples and better parser diagnostics.

## 58. `pc_2df63b1c0880`

Family: `exa-contents-text-cap-contract`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent CLI schema/help/validation and exa-agent-cli skill
Duplicate of: `pc_a1553455a3d4`

### Original filing

```text
exa-agent contents rejected --text 12000 with an error instead of honoring the documented-style cap; the schema/help should make the contents limit obvious or clamp it
```

### Root cause

Same 10000 numeric --text validator limit omitted from generated help/schema and normal skill guidance.

### Proposed correction

Use the canonical contract fix from pc_a1553455a3d4; do not silently clamp without warning.

### Why this correction

The failure originates in exa-agent CLI schema/help/validation and exa-agent-cli skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Live CLI rejects numeric values above 10000.
2. The cut records --text 12000 rejection.
3. Current contents help still describes N without its maximum.

### Risks and constraints

Same as canonical item.

## 59. `pc_69f47212dc0a`

Family: `repo-aware-test-discovery`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: global repo-orientation/testing guidance
Duplicate of: None

### Original filing

```text
rg against the conventional tests/ path failed because this repo keeps tests beside source under src; a repo-aware test-root hint would avoid the false error
```

### Root cause

The verification command assumed a conventional top-level tests/ directory, but Hearth colocates Vitest files beside source under src.

### Proposed correction

Discover tests with rg --files -g '*.{test,spec}.*' or read the package test script before probing conventional directories. Add this as orientation guidance, not a repo helper.

### Why this correction

The evidence identifies `repo-aware-test-discovery` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. /Users/treygoff/Code/hearth/tests does not exist.
2. Relevant tests are at /Users/treygoff/Code/hearth/src/shared/validate.test.ts.
3. A focused Vitest run of that colocated file passes 23 tests.

### Risks and constraints

Broad filename globs may include generated/vendor tests; retain normal exclusion patterns.

## 60. `pc_bd9ede3cf94d`

Family: `exa-public-social-post-fallback`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `tentative`
Owner surface: exa-agent contents/search fallback guidance and Exa source connectors
Duplicate of: None

### Original filing

```text
exa-agent contents can render the public Truth Social profile but timed out on the account's public statuses API, leaving the original July 13 post URL unavailable through direct crawl. The tool should expose a stable fallback for public social post lookup.
```

### Root cause

Exa could render a public Truth Social profile but timed out on the statuses API needed to resolve a specific post. The contents path lacks a stable post-level alternate lookup when direct crawl fails.

### Proposed correction

Classify social API timeout separately and suggest a bounded search for exact account/date/text snippets, cached/indexed result URLs, or an alternate public endpoint. Preserve that the fallback is secondary discovery, not direct-source confirmation.

### Why this correction

exa-agent contents/search fallback guidance and Exa source connectors owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Cut records profile success, public statuses API timeout, and inability to recover the July 13 post URL.
2. Current Exa help documents per-URL warnings and all_urls_failed behavior but no social-post fallback.
3. No live Truth Social request was made, so current source availability is unverified.

### Risks and constraints

Social mirrors can be incomplete or spoofed; do not upgrade fallback content to verified original without matching source metadata.

## 61. `pc_10e28695f5fa`

Family: `exa-wire-service-source-availability`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `tentative`
Owner surface: Exa source availability/error taxonomy and exa-agent rendering
Duplicate of: None

### Original filing

```text
exa-agent rejects Reuters and AP as unavailable domains, so direct wire-domain searches cannot run even though syndicated AP and Reuters text is discoverable through PBS, Al-Monitor, Baird Maritime, and other mirrors. The CLI should distinguish temporary source unavailability from auth/reauth errors.
```

### Root cause

Direct Reuters/AP domains are unavailable to the Exa source layer, but the error does not clearly distinguish a publisher access restriction from authentication failure or temporary outage. Syndicated copies remain discoverable elsewhere.

### Proposed correction

Return a stable source_unavailable or publisher_restricted code with retryable=false/true as appropriate, distinct from auth errors, and suggest broader syndicated discovery while preserving wire-service attribution.

### Why this correction

Exa source availability/error taxonomy and exa-agent rendering owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Cut reports Reuters and AP direct-domain searches rejected while syndicated text was found on PBS, Al-Monitor, Baird Maritime, and others.
2. Current Exa error contract includes categories such as not_authenticated and upstream/source failures, but no live Reuters/AP call was made to inspect the current exact code.
3. The installed skill advises reading error.code/category/retryable, making a precise source-unavailable code the natural fix.

### Risks and constraints

Syndicated copies may be edited or truncated; the workflow must cite the accessible publisher and separately attribute the wire service.

## 62. `pc_dc0fd914fe93`

Family: `exa-contents-text-cap-contract`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent CLI schema/help/validation and exa-agent-cli skill
Duplicate of: `pc_a1553455a3d4`

### Original filing

```text
exa-agent contents rejected a reasonable 20000-character cap even though the skill says to use contents --text full; the error only surfaced after the network call attempt. Help or schema should make the 10000 cap explicit in the normal recipe.
```

### Root cause

Same numeric cap mismatch: --text 20000 is rejected at max 10000 even though the normal skill recipe and help imply full/uncapped contents are supported without explaining the numeric ceiling.

### Proposed correction

Use the canonical help/schema/validation correction proposed for pc_a1553455a3d4.

### Why this correction

The failure originates in exa-agent CLI schema/help/validation and exa-agent-cli skill. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Live dry-run reproduces --text 20000 invalid_value with max=10000.
2. Current skill says contents --text and help says bare/full are uncapped but neither states numeric max 10000.
3. The cut records the error surfacing only after attempted use.

### Risks and constraints

Same as canonical item.

## 63. `pc_ca2cfb2732ae`

Family: `pdf-dependency-resolution`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: PACT fly-in followups PDF builder
Duplicate of: None

### Original filing

```text
The meeting-briefs README says build-simple.js needs no NODE_PATH, but its require('pdf-lib') is not locally resolvable from that directory; document the actual shared module location or vendor the dependency.
```

### Root cause

The original builder relied on Node module resolution for a dependency installed only in a shared toolchain, so require("pdf-lib") could not resolve from the followups directory. The current builder no longer does that.

### Proposed correction

No further change. Keep the explicit shared-toolchain path (or give the PDF toolchain its own package.json if it ever needs clean-checkout portability).

### Why this correction

Current evidence indicates that the `pdf-dependency-resolution` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current followups/pdf-build/build.js lines 11-19 computes the shared 06-production/toolchains path and requires pdf-lib by its explicit absolute path.
2. A live require.resolve using that computed path resolves to pact-witness-invites-pdf/node_modules/pdf-lib/cjs/index.js.
3. The complained-about build-simple.js no longer exists in the current followups/pdf-build directory.

### Risks and constraints

The explicit shared path is machine/workspace-layout dependent, but that dependency is now honest and diagnosable.

## 64. `pc_02430da9ef6d`

Family: `invalid-orientation-loop`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent shell guidance
Duplicate of: None

### Original filing

```text
Initial skill-read probe used an invalid zsh loop/glob and failed before doing work; a simpler explicit cat command would avoid this dead end.
```

### Root cause

A one-off agent-authored zsh probe combined loop/glob syntax without first validating it; this is command construction error, not a Hearth defect.

### Proposed correction

Instruction-only: use explicit cat paths or rg --files for optional files; validate compound shell snippets with zsh -n or avoid the loop entirely.

### Why this correction

The evidence identifies `invalid-orientation-loop` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The cut contains no failing argv or script path, so the exact syntax is complaint-only and cannot be source-reproduced.
2. Live zsh is configured to error on unmatched globs, confirming that speculative glob probes can abort before their body runs.

### Risks and constraints

Adding a repo helper for a one-off inspection would create more surface than it removes.

## 65. `pc_51e571c07493`

Family: `apply-patch-context-fragility`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent editing guidance
Duplicate of: `pc_a21c970bd217`

### Original filing

```text
A combined apply_patch assumed package.json dependency ordering after npm install; patching package scripts separately avoids order-sensitive context
```

### Root cause

One combined patch used package.json context captured before npm install reordered/rewrote dependencies, so exact-context patch matching correctly failed.

### Proposed correction

Instruction-only: run dependency mutation first, reread package.json, then patch scripts in a small independent hunk.

### Why this correction

The evidence identifies `apply-patch-context-fragility` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint identifies npm install between context capture and apply_patch; apply_patch is exact-context and atomic.
2. No enduring Hearth source defect or failing product behavior is identified.

### Risks and constraints

Relaxing patch context matching would risk editing the wrong repeated block.

## 66. `pc_bd55a6a719a4`

Family: `zsh-unmatched-glob`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: agent shell guidance
Duplicate of: `pc_17287b48a152`

### Original filing

```text
zsh unmatched-glob expansion aborted a config inspection command; use nullglob-safe explicit paths or rg --files for optional config discovery
```

### Root cause

zsh nomatch semantics abort commands when an optional glob has no matches; the probe assumed bash/nullglob-like behavior.

### Proposed correction

Instruction-only: discover optional files with rg --files/find or use explicit paths; quote literal wildcard text.

### Why this correction

The evidence identifies `zsh-unmatched-glob` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live `zsh -fc "print -r -- definitely-no-match-*.xyz"` exits 1 with `no matches found`.
2. No Hearth application code participates in pathname expansion performed by the invoking shell.

### Risks and constraints

Globally enabling nonomatch/nullglob can hide typos in unrelated commands.

## 67. `pc_32ee1733d053`

Family: `radar-targeted-test-entrypoints`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: prospera-radar-build package scripts and agent guidance
Duplicate of: None

### Original filing

```text
Targeted web test command was easy to misaddress from the repo root:  from web needs , not ; a documented targeted-test helper would avoid this dead end.
```

### Root cause

The repo has separate root and web package contexts, while targeted-test examples rely on npm argument forwarding and cwd-sensitive paths; the filed text also lost the two command fragments, making the intended distinction unrecoverable.

### Proposed correction

Add one root script such as `test:web:file` that invokes `tsx --test` in web with the passed path, and document one root-relative example.

### Why this correction

The failure originates in prospera-radar-build package scripts and agent guidance. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The raw .papercuts.jsonl record itself contains blanks where the compared commands should be, so exact reproduction is unavailable.
2. docs/agent-guidance.md line 170 now documents `npm --prefix web test -- <file>` for targeted web logic checks.
3. web/package.json test already supplies broad test globs, so an appended file is not a clean dedicated single-file entrypoint.

### Risks and constraints

A poorly designed helper could still run the full globbed suite or mishandle paths with spaces.

## 68. `pc_13afdf97e5a6`

Family: `exa-cli-contract-and-doc-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `strong`
Owner surface: exa-agent CLI documentation
Duplicate of: None

### Original filing

```text
exa-agent global --output did not create the expected file when placed after the subcommand; a clear positional/output example would prevent agents from assuming stdout was persisted
```

### Root cause

The current parser and tests accept global --output before or after the subcommand, so the historical failure is not reproducible. It may have been an older binary, an unsupported command path, or an invocation-specific issue.

### Proposed correction

No code change. Add a single robot-docs example showing `exa-agent -o FILE search ...`; if a specific command still ignores it, refile with argv and envelope.

### Why this correction

Current evidence indicates that the `exa-cli-contract-and-doc-drift` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Installed exa-agent is 0.2.0. Live dry-run parsing succeeds with `--output /dev/null` both before and after `search test`.
2. exa-agent --help labels -o/--output a global option.
3. Current exa-agent-cli tests include `explicit_output_wins_over_max_output_bytes_and_confirms_file`.

### Risks and constraints

`when supported` remains vague; unsupported command-specific output still needs a precise reproduction.

## 69. `pc_03d1e73413b7`

Family: `exa-cli-contract-and-doc-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `strong`
Owner surface: exa-agent CLI skill/runtime sync
Duplicate of: None

### Original filing

```text
exa-agent skill recipe documents a fetch macro, but the installed 0.2.0 binary exposes no fetch command; contents works as the supported replacement.
```

### Root cause

The current 0.2.0 binary and source expose the fetch macro, while the filing says an installed 0.2.0 did not. Version skew or a stale binary is likely, but the historical mismatch was not preserved well enough to prove which.

### Proposed correction

No further change; keep skill and binary version-matched and make the skill preflight `exa-agent --version` when command drift is suspected.

### Why this correction

Current evidence indicates that the `exa-cli-contract-and-doc-drift` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current `exa-agent --help` lists `fetch Macro -> contents URL...`.
2. Current exa-agent-cli src/cli/mod.rs defines Command::Fetch and tests parse/execute the macro.
3. Current exa-agent-cli skill documents both `exa-agent contents` and `exa-agent fetch`.

### Risks and constraints

Multiple installed skill copies can regress into version skew if global sync is bypassed.

## 70. `pc_db9e8d6227fa`

Family: `bash-array-indirection-under-zsh`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: research recipe guidance
Duplicate of: None

### Original filing

```text
The recon shell recipe used Bash indirect array expansion (${!array[@]}), which fails under the configured zsh shell; provide shell-portable examples or detect the shell.
```

### Root cause

The recipe used bash-only indirect array index expansion under zsh.

### Proposed correction

Instruction-only: write zsh-native loops (`for q in $queries`) or a portable line-oriented while-read loop; label bash recipes and invoke bash explicitly.

### Why this correction

The evidence identifies `bash-array-indirection-under-zsh` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live `zsh -fc` with `${!a[@]}` fails with `bad substitution`.
2. The configured shell for this task is zsh.

### Risks and constraints

Claiming shell portability without testing both shells will recreate the failure.

## 71. `pc_bce78a0aff06`

Family: `exa-contents-failure-semantics`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent contents/fetch response contract
Duplicate of: None

### Original filing

```text
exa-agent contents returns exit 10 with ok=true and an empty results array on HTTP 403 crawl failures; a clearer nonzero envelope or prominent status field would prevent false-success handling
```

### Root cause

exa-agent models a successfully completed API call separately from per-URL retrieval success: total URL failure sets process exit 10 and warning all_urls_failed, but retains top-level ok:true and an empty result list.

### Proposed correction

Make total URL failure unambiguous in-band: either top-level ok:false with a structured upstream_fetch_failed error, or a required outcome/status field that distinguishes request success from content success.

### Why this correction

The failure originates in exa-agent contents/fetch response contract. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Current exa-agent-cli src/lib.rs emits warning code `all_urls_failed` for zero results when every requested URL failed.
2. Current skill lines 166-169 explicitly warn that exit 10 plus ok:true/count:0 is not empty-page success.
3. The complaint is the observed runtime envelope; no live paid/network call was repeated.

### Risks and constraints

Changing ok semantics is a breaking contract for clients already branching on exit code and warnings.

## 72. `pc_88e09fdfbb7f`

Family: `unquoted-url-glob-expansion`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: agent shell and exa-agent usage guidance
Duplicate of: None

### Original filing

```text
zsh globbing broke exa-agent contents when a URL with query parameters was passed unquoted; a URL-safe wrapper or shellcheck hint would prevent this
```

### Root cause

The shell expanded `?` in an unquoted URL as a filename pattern before exa-agent saw the argument.

### Proposed correction

Instruction-only: quote every URL argument in examples; add a prominent quoted-query-string example to the Exa skill.

### Why this correction

The evidence identifies `unquoted-url-glob-expansion` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live zsh reports `no matches found` for an unquoted URL containing `?a=1`.
2. exa-agent contents accepts positional URL strings; URL parsing occurs only after shell tokenization.

### Risks and constraints

A wrapper cannot recover characters already consumed or split by the shell.

## 73. `pc_9cb7c305959d`

Family: `exa-contents-failure-semantics`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: exa-agent fetch diagnostics and Congress.gov upstream
Duplicate of: None

### Original filing

```text
exa-agent contents returned HTTP 403 for a public Congress.gov hearing event page; a Congress.gov-aware fetch hint or fallback would prevent a dead-end call
```

### Root cause

Congress.gov rejected Exa crawler retrieval with HTTP 403; this is an upstream fetch limitation, and the CLI did not route the user to an official alternate surface.

### Proposed correction

On 403 from congress.gov, emit a nextAction pointing to Congress.gov API/event JSON or direct official-browser retrieval; do not silently retry as generic crawl.

### Why this correction

The evidence for `exa-contents-failure-semantics` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. The 403 and target class are complaint-only; the exact hearing URL is not present, so current reproduction was not possible.
2. Current exa-agent failure machinery can report per-URL statuses/all_urls_failed but contains no Congress.gov-specific fallback in the inspected CLI/skill.

### Risks and constraints

Domain-specific fallbacks can become stale and must not imply the API contains the same rendered content.

## 74. `pc_246f9cd9b37b`

Family: `bash-array-indirection-under-zsh`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: research sweep recipe guidance
Duplicate of: `pc_db9e8d6227fa`

### Original filing

```text
The recon shell used Bash array-indirection syntax (${!queries[@]}) under zsh and failed before launching searches; a shell-portable Exa sweep helper would prevent this.
```

### Root cause

Same bash-only `${!queries[@]}` array-indirection recipe was executed by zsh and failed before any Exa call.

### Proposed correction

Replace the duplicated shell sweep with one checked-in line-oriented helper or explicitly run a bash-labeled recipe under bash.

### Why this correction

The evidence identifies `bash-array-indirection-under-zsh` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live zsh rejects `${!a[@]}` with `bad substitution`.
2. The complaint explicitly records configured zsh and zero searches launched.

### Risks and constraints

A helper that embeds paid searches must preserve explicit spend confirmation and sequential limits.

## 75. `pc_ae44fb08f5ce`

Family: `radar-live-ops-probe`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build ops tooling and secret bootstrap
Duplicate of: None

### Original filing

```text
Patched agent-sweep live probe could not reuse the working exa-agent credential because app code requires EXA_API_KEY and .env.local lacks it. A shared credential bridge or read-only probe command would let ops validate the production Exa adapter without copying secrets.
```

### Root cause

Radar app code reads EXA_API_KEY from process.env, while exa-agent authenticates from its private credentials file; .env.local has no EXA_API_KEY, so the two working credential surfaces cannot interoperate for a live adapter probe.

### Proposed correction

Add a read-only agent-sweep probe accepting the key via stdin or the existing machine secret bootstrap; never copy the credential into repo files or print it.

### Why this correction

The failure originates in prospera-radar-build ops tooling and secret bootstrap. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. web/lib/radar/agent-sweep/search-client.ts lines 175-182 defaults to process.env.EXA_API_KEY and throws when absent.
2. A key-name-only inspection confirms root .env.local has no EXA_API_KEY.
3. `exa-agent auth status --json` reports authenticated via ~/.config/exa-agent-cli/credentials.json.

### Risks and constraints

Any credential bridge must avoid argv, logs, shell history, child-process listings, and accidental persistence.

## 76. `pc_b37f54ccfbe6`

Family: `radar-targeted-test-entrypoints`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: prospera-radar-build agent guidance
Duplicate of: None

### Original filing

```text
Used npm exec tsc with the wrong separator, which printed compiler help instead of checking the web project. Repo gate commands should be preferred over ad hoc npm exec syntax.
```

### Root cause

An ad hoc npm exec invocation used the wrong `--` boundary, so tsc received no project check request and printed help.

### Proposed correction

Instruction-only: use `npm run typecheck` from root or `npm --prefix web run typecheck`; do not synthesize npm exec tsc commands.

### Why this correction

The evidence identifies `radar-targeted-test-entrypoints` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Root package.json already defines `typecheck` as `npm --prefix web run typecheck && npm run typecheck:scripts`.
2. web/package.json defines `typecheck` as `tsc --noEmit`.
3. No repo defect is required to explain npm argument-forwarding misuse.

### Risks and constraints

Duplicating gate syntax in more places can itself drift.

## 77. `pc_dc81b1ac1f3f`

Family: `radar-live-ops-probe`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build ops scripts/schema
Duplicate of: None

### Original filing

```text
Supabase schema uses radar_watch_area_entities rather than the natural radar_watch_area_entity_bindings name; an ad hoc live watch-area join failed. A documented schema map or lookup script would avoid this ops dead end.
```

### Root cause

An ad hoc production query guessed a naturalized join-table name instead of using the actual schema name radar_watch_area_entities; no typed/schema-derived ops lookup exists for this question.

### Proposed correction

Add one typed read-only Radar lookup script for item/watch-area/entity inspection, deriving/selecting only known columns; document it as the production spot-check path.

### Why this correction

The failure originates in prospera-radar-build ops scripts/schema. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. supabase/migrations/20260514100000_radar_refactor.sql creates public.radar_watch_area_entities.
2. Repo-wide SQL and tests consistently reference radar_watch_area_entities; no radar_watch_area_entity_bindings table was found.
3. Existing scripts/lib/supabase-script-shared.ts supplies a typed REST client but no generic read-only lookup command.

### Risks and constraints

Service-role lookup tooling must be read-only, narrowly projected, and redact sensitive fields.

## 78. `pc_b66b74817bba`

Family: `radar-live-ops-probe`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build ops read model
Duplicate of: None

### Original filing

```text
Live spot-check query hit a schema naming trap: radar_items has no source_published_at despite publication timing being operationally important. A typed read-only lookup script would prevent ad hoc column drift.
```

### Root cause

The query guessed source_published_at on radar_items, but the canonical table stores first_seen_at/updated_at and publication metadata lives through other read-model/evidence paths.

### Proposed correction

Cover publication timing in the same typed read-only lookup, with an explicit stable output field mapped from the correct table/view rather than exposing raw guessed columns.

### Why this correction

The failure originates in prospera-radar-build ops read model. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The base product migration defines radar_items with first_seen_at and updated_at, not source_published_at.
2. Repo search finds no radar_items source_published_at definition.
3. Operational timing is consumed in typed code as published_at/first_seen_at depending on the model, making raw-table guessing unsafe.

### Risks and constraints

Choosing first_seen_at as publication time would silently conflate discovery time with source publication time.

## 79. `pc_8312d8ea11fd`

Family: `radar-env-loading`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build env/runbook tooling
Duplicate of: None

### Original filing

```text
Repo .env.local cannot be sourced by zsh (parse error near line 56), so the obvious live-probe path fails; keep env syntax sourceable or provide a checked-in dotenv loader command.
```

### Root cause

.env.local is valid dotenv but not valid zsh because at least one value contains shell-significant syntax; agents assumed dotenv files were sourceable shell scripts.

### Proposed correction

Document “never source .env.local”; route TypeScript ops scripts through scripts/lib/env-local.ts and provide a tiny checked-in launcher for non-TS probes.

### Why this correction

The failure originates in prospera-radar-build env/runbook tooling. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Live `zsh -n .env.local` exits 1 with a parse error near line 56.
2. scripts/lib/env-local.ts provides a checked-in dotenv-style assignment parser and auto-loads root .env.local without shell sourcing.

### Risks and constraints

Making the secret file shell-sourceable may require unsafe quoting changes and encourages secret leakage into shell state.

## 80. `pc_aff4d7f9b134`

Family: `radar-live-ops-probe`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build ops scripts
Duplicate of: None

### Original filing

```text
Root-level live DB probe assumed dotenv was installed, but this repo keeps JS dependencies under web; a documented one-command production read probe would prevent this resolution dead end.
```

### Root cause

The repo is not an npm workspace and dotenv is not a root dependency, so a root-level improvised Node probe could not import it; dependencies and existing env loading are intentionally split.

### Proposed correction

Do not add dotenv. Add the typed read-only lookup using the existing stdlib env-local and supabase-script-shared modules, with a root package script.

### Why this correction

The failure originates in prospera-radar-build ops scripts. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Root package.json has no dotenv dependency and no workspaces field.
2. web/package.json owns web dependencies; scripts/lib/env-local.ts is the installed-dependency-free root env loader.
3. No current package script exposes the requested one-command production read probe.

### Risks and constraints

A generic database console would broaden service-role blast radius; keep queries enumerated.

## 81. `pc_b057ceb4523e`

Family: `exa-positional-urls-schema-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent embedded schema generator
Duplicate of: `pc_cc2d338911db`

### Original filing

```text
exa-agent schema show contents advertises a --urls flag, but exa-agent contents rejects --urls; built-in recipe uses positional URLs. Schema/CLI contract mismatch caused a dead-end call.
```

### Root cause

The embedded schema exposes request-body field names as if each were a CLI flag. For contents it advertises flag `urls`, while clap defines URLs as positional arguments.

### Proposed correction

Represent positional inputs explicitly in schema (for example `inputKind:"argument", name:"urls"`) and reserve `flag` for accepted `--...` tokens; add schema-to-clap parity tests.

### Why this correction

The failure originates in exa-agent embedded schema generator. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Live `exa-agent schema show contents` returns a field `{flag:"urls", bodyPath:"urls"}`.
2. Live `exa-agent contents --urls https://example.com --dry-run` exits 1 with structured unknown_flag.
3. Live `exa-agent contents --help` shows `[URLS]...` as positional arguments.

### Risks and constraints

Schema consumers may already special-case `flag`; version the schema or preserve a compatibility field.

## 82. `pc_b90d50ead946`

Family: `gog-calendar-update-integrity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `strong`
Owner surface: gogcli calendar update
Duplicate of: None

### Original filing

```text
gog calendar update --attendees="" claims 'set empty to clear' but silently no-ops (returned envelope even showed attendees:[] while the event kept them); also updating attendees on an event you don't organize fails silently with a stale 'updated' timestamp — both need honest errors
```

### Root cause

The calendar updater appears to serialize an empty attendee replacement ambiguously and trusts the update response without read-after-write validation or surfacing organizer permission constraints.

### Proposed correction

Upstream gog should distinguish flag-not-set from explicitly-empty, reject non-organizer attendee mutations before/after API response, and read back attendee changes before emitting success.

### Why this correction

gogcli calendar update owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Installed gog is v0.21.0; current help still promises `--attendees ... set empty to clear`.
2. The complaint records an envelope showing attendees:[] while the event retained attendees and a non-organizer mutation returning apparent success; no destructive calendar mutation was repeated.
3. Binary strings include attendee replacement validation but no visible read-after-write mismatch error contract.

### Risks and constraints

Read-after-write adds quota/latency; retries must not duplicate notifications or mutate recurring instances unexpectedly.

## 83. `pc_d741782a7167`

Family: `delegate-safety-boundaries`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: delegate Devin adapter
Duplicate of: None

### Original filing

```text
delegate devin safe failed instantly with 'A tool was rejected by the user' on a plain read-only survey task in ~/Code/hearth — devin lane appears to need interactive tool approval under delegate safe mode
```

### Root cause

Delegate Devin safe mode denies edit/write/exec/mcp tools via generated agent config. A “read-only survey” that Devin implements through a generic exec tool is therefore rejected non-interactively even though the intended shell command is read-only.

### Proposed correction

Add a Devin-safe read-only command policy/allowlist if Devin supports one, or fail preflight with a clear “filesystem survey unsupported in Devin safe; use another safe harness” message.

### Why this correction

The failure originates in delegate Devin adapter. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Current delegate command_help and describe state that Devin safe passes a deny-list for edit/write/exec and mcp__* with permission-mode auto.
2. src/delegate_agent/argv_builders.py lines 403-412 applies that agent config to every Devin safe run.
3. The complaint records Devin’s immediate `A tool was rejected by the user` on a plain survey; exact child trace is unavailable.

### Risks and constraints

Allowing generic exec based on command text is not a robust read-only boundary and can create shell escapes.

## 84. `pc_fdd9d446d4c6`

Family: `radar-playwright-isolation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: prospera-radar-build E2E tooling
Duplicate of: None

### Original filing

```text
Isolating Playwright in a temporary worktree also fails if web/node_modules is symlinked back to the original checkout: Turbopack rejects dependency symlinks outside its filesystem root. A documented clone-safe E2E helper would avoid both the shared .next lock and external-node_modules failure.
```

### Root cause

A worktree with node_modules symlinked outside its root conflicts with Next/Turbopack filesystem-root constraints; dependency reuse by external symlink is not equivalent to an isolated checkout.

### Proposed correction

Provide a documented E2E worktree helper that installs dependencies inside the isolated tree (using the warm npm cache), never symlinks node_modules, and selects an isolated distDir/port.

### Why this correction

The failure originates in prospera-radar-build E2E tooling. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. web/next.config.ts sets turbopack.root to the repo root and outputFileTracingRoot to the repo root.
2. The complaint records Turbopack rejecting node_modules symlinked to the original checkout.
3. No clone/worktree-safe E2E helper exists in current package scripts.

### Risks and constraints

Per-worktree installs cost disk/time; helper cleanup must preserve caches and avoid deleting active worktrees.

## 85. `pc_71fc5d5bea37`

Family: `radar-playwright-isolation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: prospera-radar-build QA documentation
Duplicate of: None

### Original filing

```text
Local visual-QA server and Playwright both default to port 3407, so targeted e2e fails unless the manual server is stopped; a documented alternate QA port would avoid this collision.
```

### Root cause

Manual visual QA and Playwright both selected 3407; the config supports an override, but the alternate-port workflow is not documented alongside QA commands.

### Proposed correction

Document one command using RADAR_PLAYWRIGHT_PORT for targeted E2E and reserve a separate conventional manual-QA port; integrate it into the isolation helper.

### Why this correction

The failure originates in prospera-radar-build QA documentation. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. web/playwright.config.ts reads RADAR_PLAYWRIGHT_PORT with default 3407.
2. No RADAR_PLAYWRIGHT_PORT example appears in AGENTS.md, README.md, QUALITY_GATES.md, or docs/agent-guidance.md.

### Risks and constraints

Port separation alone does not solve the shared .next lock.

## 86. `pc_a96389c3cc68`

Family: `agent-browser-contract-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `confirmed`
Owner surface: agent-browser bundled skills/docs
Duplicate of: None

### Original filing

```text
agent-browser core advertises a mobile specialized skill, but 'agent-browser skills get mobile' returns Skill not found; document the actual viewport workflow or ship the skill.
```

### Root cause

The installed agent-browser README advertises a bundled mobile skill that the actual 0.30.0 skill registry does not ship.

### Proposed correction

Upstream agent-browser should either ship/version the mobile skill or remove it from README; core should explicitly show `agent-browser set viewport` and `set device` as the supported workflow.

### Why this correction

agent-browser bundled skills/docs owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Live `agent-browser skills list` exposes agentcore, core, dogfood, electron, slack, and vercel-sandbox only.
2. Live `agent-browser skills get mobile` exits 1 with `Skill not found: mobile`.
3. Installed README still lists `mobile — Viewport/device/geolocation/media, touch, swipe...`.

### Risks and constraints

Documentation must stay generated from the same registry as `skills list` to prevent repeat drift.

## 87. `pc_278287fce683`

Family: `gavel-local-auth-vqa`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: gavel local VQA harness
Duplicate of: None

### Original filing

```text
Chromium rejected the local app's __Host-session cookie over plain HTTP, so browser VQA could not authenticate even though the curl acceptance matrix passed with the same fixture credentials; a local HTTPS fixture mode or non-__Host development cookie would unblock visual QA.
```

### Root cause

Gavel intentionally uses the `__Host-` cookie prefix with secure:true for production security; Chromium correctly refuses such a cookie over plain HTTP, while curl does not enforce browser cookie-prefix rules.

### Proposed correction

Keep the production cookie invariant. Add a local HTTPS VQA launcher/fixture (trusted or browser-launched with ignore-HTTPS-errors) rather than weakening the cookie name/options in development.

### Why this correction

The failure originates in gavel local VQA harness. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. src/lib/auth/session.ts lines 5 and 15-21 define `__Host-session`, path /, httpOnly, sameSite lax, secure true.
2. BUILD_GOAL.md explicitly says the __Host-session payload/verification is an invariant.
3. The complaint reports curl acceptance success but browser authentication failure on HTTP, matching browser prefix enforcement.

### Risks and constraints

A development-only non-__Host cookie can mask production behavior; local certificates and ignore-error flags require careful scoping.

## 88. `pc_db0db641f6cf`

Family: `agent-safety-guard-friction`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: machine agent deletion guidance
Duplicate of: None

### Original filing

```text
The local fixture acceptance harness was blocked because its cleanup trap recursively removed a mktemp directory; the guard requires enumerating known temporary files even when the directory is freshly created.
```

### Root cause

The agent attempted recursive deletion of a fresh mktemp directory, while the machine deletion guard intentionally requires recoverable trash or narrowly enumerated cleanup.

### Proposed correction

Instruction-only: use `trash "$tmpdir"` in agent-authored cleanup or enumerate known files; add a reusable safe temp-dir cleanup snippet to machine guidance.

### Why this correction

The evidence identifies `agent-safety-guard-friction` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Current global repo instructions say deletions use `trash`, never rm, including tmp cleanup.
2. The complaint identifies the cleanup trap—not product behavior—as the blocked operation.
3. gavel scripts include both file-level mktemp cleanup and some existing rm -rf traps, showing inconsistent historical patterns.

### Risks and constraints

Weakening the deletion guard for mktemp paths creates path-confusion and symlink risks.

## 89. `pc_f8d120f7e054`

Family: `skill-path-resolution`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: global Impeccable skill distribution
Duplicate of: None

### Original filing

```text
Impeccable skill setup points to .agents/skills/impeccable/scripts/context.mjs, but the repo has no .agents directory; the installed skill lives under ~/.agents/skill-library.
```

### Root cause

The loaded Impeccable instructions assumed a project-local .agents skill path, while the skill was installed globally. The active skill has since added a runtime-provided base-directory fallback and a live .agents/skills copy exists.

### Proposed correction

No project change. Finish syncing/deleting stale duplicate skill copies so all launch surfaces use the runtime base-directory contract.

### Why this correction

The failure originates in global Impeccable skill distribution. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Current ~/.agents/skills/impeccable/SKILL.md line 13 says to use `<skill-base-dir>/scripts/context.mjs` when the runtime supplies it.
2. Current ~/.agents/skills/impeccable/scripts/context.mjs exists.
3. The older skill-library copy still references .claude/skills, confirming prior path drift across copies.

### Risks and constraints

A stale skill-library copy can reintroduce the dead path on another harness.

## 90. `pc_183edfed93b6`

Family: `walk-coordinate-diagnosis`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: papercuts usage / threejs-agent-kit docs
Duplicate of: `pc_506af2585d23`

### Original filing

```text
RESOLUTION of pc_506af2585d23: there was never a teleport bug. World coords are CENTERED — domain x[-975,975] z[-670,670], origin at sheet middle. All 'failing' coords were off-sheet; falling+respawn is correct edge behavior. __THREE_DEBUG__.player() readback added in Experience.ts for future diagnosis. Lesson: check the coordinate system before filing physics bugs
```

### Root cause

This is a resolution note accidentally filed as a new cut, not a new defect. It records that centered world coordinates made the tested positions off-sheet.

### Proposed correction

Resolve this meta-cut and the original as no-bug; future resolutions should use `papercuts resolve`, not `papercuts add`.

### Why this correction

The evidence identifies `walk-coordinate-diagnosis` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The cut text explicitly begins `RESOLUTION of pc_506af2585d23`.
2. WORLD_BRIEF.md now documents centered domain x[-975,975], z[-670,670] and says off-map teleports fall/respawn correctly.
3. WORLD_BRIEF.md documents __THREE_DEBUG__.player() for diagnosis.

### Risks and constraints

Leaving the meta-cut open makes aggregate counts misleading.

## 91. `pc_a342698f560f`

Family: `agent-browser-contract-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: agent-browser screenshot/session behavior
Duplicate of: None

### Original filing

```text
agent-browser screenshot captures the top of the page regardless of prior window.scrollTo/scrollIntoView eval calls — had to use --full + crop to inspect below-fold regions during VQA; a --at-scroll-position behavior or a scroll subcommand that persists into screenshot would prevent it
```

### Root cause

The reported scroll state either used a different/expired browser session or exposed an agent-browser screenshot bug; current docs say scroll and scrollintoview are persistent core commands, but no exact URL/session/argv was filed.

### Proposed correction

Reproduce with a minimal static tall page and explicit shared `--session`; if confirmed, upstream should test screenshot clip origin after eval/scroll and expose scrollY in JSON diagnostics.

### Why this correction

The evidence for `agent-browser-contract-drift` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. Current agent-browser help lists `scroll`, `scrollintoview`, and `screenshot`.
2. Installed README examples treat scroll and screenshot as commands in the same persistent session.
3. The complaint contains no version, session name, URL, or exact command sequence; reproducing would write screenshot artifacts and was not justified in this read-only lane.

### Risks and constraints

Fixing screenshot origin without isolating a session-mismatch cause could break full-page and element screenshots.

## 92. `pc_41ba11574a6b`

Family: `browser-use-skill-missing-executable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: global browser-use skill installation contract
Duplicate of: `pc_595745b3b43e`

### Original filing

```text
The loaded browser-use skill advertises a browser-use CLI that is not installed on this machine; a prerequisite check in the skill preamble would prevent the dead end.
```

### Root cause

The browser-use skill unconditionally instructs invocation of a browser-use CLI that is absent from PATH and performs no prerequisite check.

### Proposed correction

Add an opening `command -v browser-use` preflight with a clear install/fallback path; preferably do not activate this skill on machines without its runtime.

### Why this correction

The failure originates in global browser-use skill installation contract. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. `command -v browser-use` fails on the live machine.
2. Current ~/.agents/skills/browser-use/SKILL.md lines 16-23 immediately prescribe `browser-use` invocation.
3. The skill’s only setup pointer is an external install.md link; no `command -v` preflight appears.

### Risks and constraints

Auto-installing a browser runtime would be network-heavy and should require user approval.

## 93. `pc_0aa764d04f6d`

Family: `agent-exec-lifecycle`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent exec/session guidance
Duplicate of: None

### Original filing

```text
Backgrounded next start launched from a one-shot exec was reaped when the shell exited; the local acceptance workflow needs a documented persistent-session launcher.
```

### Root cause

A background process launched inside a one-shot execution shell inherits that shell/session lifecycle and is reaped when the tool call ends; this is harness behavior, not Next behavior.

### Proposed correction

Instruction-only: launch long-lived servers in the execution tool’s persistent session and poll that session; document this in agent runtime guidance, not app code.

### Why this correction

The evidence identifies `agent-exec-lifecycle` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint identifies a backgrounded `next start` from a one-shot exec and immediate shell exit.
2. Gavel BUILD_GOAL documents the server command but not a persistent exec/PTY/session lifecycle.

### Risks and constraints

nohup/disown workarounds can leak orphan servers; use first-class session handles.

## 94. `pc_9517658ecfbe`

Family: `zsh-reserved-status-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: agent shell guidance
Duplicate of: `pc_944d374ac9c4`

### Original filing

```text
Acceptance wrapper used zsh's read-only 'status' special variable; a documented shell-safe result variable would prevent the false start.
```

### Root cause

zsh reserves `status` as a read-only special parameter, so an acceptance wrapper using it as a result variable failed before reporting.

### Proposed correction

Instruction-only: use rc/exit_code in zsh wrappers, or invoke scripts with their declared bash interpreter.

### Why this correction

The evidence identifies `zsh-reserved-status-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live `zsh -fc "status=1"` exits 1 with `read-only variable: status`.
2. Current gavel scripts/acceptance.sh uses a local shell variable named status inside bash, where it is valid; the failed wrapper was separate agent-authored zsh.

### Risks and constraints

Renaming app script variables is unnecessary and would not prevent external wrapper mistakes.

## 95. `pc_506af2585d23`

Family: `walk-coordinate-diagnosis`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: threejs-agent-kit debug UX
Duplicate of: None

### Original filing

```text
walk/threejs-agent-kit debug teleport: object-position teleports to mid-tidal-flats coords (e.g. [1030,400],[1045,415],[1062,428],[985,480]) silently respawn at the prospect spawn; same-call teleports near built structures ([925,371],[1205,562]) work. Suspect the y<-20 fall-respawn masking a capsule fall-through; needs a player() position readback hook in __THREE_DEBUG__ to diagnose
```

### Root cause

There was no teleport/physics defect: supplied x/z coordinates exceeded the centered sheet domain, so the player correctly fell off terrain and the y&lt;-20 guard respawned it.

### Proposed correction

Resolve as no-bug; keep the coordinate-domain documentation and player readback. Optionally validate/debug-warn when teleport coordinates are outside world bounds.

### Why this correction

Current evidence indicates that the `walk-coordinate-diagnosis` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. src/app/hurst.ts documents centered coordinates [-975..975] x [-670..670].
2. src/app/PlayerController.ts line 96 respawns when capsule end y &lt; -20.
3. WORLD_BRIEF.md now explicitly records this cut as no-bug and documents player() readback.

### Risks and constraints

Hard rejection could block intentional off-map physics tests; a warning is safer than changing teleport semantics.

## 96. `pc_8d0d40377a6b`

Family: `dropbox-workspace-review-guidance`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: PACT Dropbox workspace agent guidance
Duplicate of: None

### Original filing

```text
A broad rg on the Packard directory swept raw transcript artifacts and produced multi-megabyte output; the workspace needs a documented owned-file glob for review lanes.
```

### Root cause

A broad recursive text search included large raw transcripts because the policy workspace has heterogeneous authored and artifact files with no default owned-file scope.

### Proposed correction

Instruction-only/workspace runbook: document review globs for authored .md/.txt sources and explicit exclusions for raw transcripts/artifacts; use rg --files first.

### Why this correction

The evidence identifies `dropbox-workspace-review-guidance` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint records multi-megabyte output from raw transcript artifacts.
2. The workspace is not a Git repository, so git-owned-file enumeration is unavailable.
3. No exact Packard directory or command was filed, limiting current size/path confirmation.

### Risks and constraints

Overly narrow globs can omit authoritative source artifacts when they are actually in scope.

## 97. `pc_4ba151a66c8d`

Family: `non-git-workspace-verification`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: PACT Dropbox workspace agent guidance
Duplicate of: `pc_2a32afa6a5d9`

### Original filing

```text
This Dropbox policy workspace is not a Git worktree, so file-scoped diff and whitespace verification require ad hoc backup comparisons instead of git diff --check.
```

### Root cause

The Dropbox policy workspace is intentionally plain files, not a Git worktree, so Git diff and whitespace checks cannot operate there.

### Proposed correction

Instruction-only: use explicit before/after copies plus cmp/diff -u and a file-scoped whitespace check; do not initialize Git in the shared workspace merely for agent convenience.

### Why this correction

The evidence identifies `non-git-workspace-verification` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live `git -C .../pact-act rev-parse --is-inside-work-tree` exits 128: not a git repository.
2. The complaint identifies ad hoc backup comparison as the current fallback.

### Risks and constraints

Temporary comparison copies can contain sensitive policy material; keep them local, named, permissioned, and cleaned recoverably.

## 98. `pc_7a6283b8f24b`

Family: `awk-label-file-argument-mixup`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent shell/reporting guidance
Duplicate of: None

### Original filing

```text
My markdown verification count command passed its protected-content check but then fed a display label to awk as a filename; a small reusable diff-count helper would prevent this avoidable reporting failure.
```

### Root cause

A one-off awk verification command mixed a human display label into the filename argument list after the substantive content check had already passed.

### Proposed correction

Instruction-only: separate machine filenames from display labels and print labels with printf; only add a helper if this exact count recurs.

### Why this correction

The evidence identifies `awk-label-file-argument-mixup` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint records the protected-content check passing before awk failed.
2. No reusable script or exact argv was filed, so source-level reproduction is unavailable.

### Risks and constraints

A bespoke diff-count helper can obscure the underlying verification and become another stale tool.

## 99. `pc_f9dba97b97ea`

Family: `apply-patch-context-fragility`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent editing guidance
Duplicate of: `pc_a21c970bd217`

### Original filing

```text
A large apply_patch failed atomically because a duplicated context block made the hunk non-unique; smaller scoped hunks would make long editorial passes less error-prone.
```

### Root cause

A large patch hunk used duplicated surrounding text, so exact context did not identify a unique insertion point and the atomic patch correctly failed.

### Proposed correction

Instruction-only: split editorial changes into small hunks anchored by unique headings/lines and reread after each batch.

### Why this correction

The evidence identifies `apply-patch-context-fragility` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint identifies duplicated context and atomic failure.
2. No Gavel product code defect resulted; this is an editing-operation failure.

### Risks and constraints

Automating fuzzy selection risks editing the wrong repeated prose block.

## 100. `pc_ceff5f2fafc5`

Family: `contacts-warmth-monotonicity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: contacts-cli touch mutation contract
Duplicate of: None

### Original filing

```text
contacts touch --warmth downgrades silently: touching a 'relationship'-warmth contact with --warmth contacted overwrote the higher state (had to re-touch to restore). touch should either refuse warmth downgrades or require an explicit --force-downgrade
```

### Root cause

contacts touch directly honors an explicit --warmth value without comparing it to the existing ordinal state, so a lower value silently overwrites a warmer relationship.

### Proposed correction

Reject explicit warmth downgrades with a structured usage/conflict error; add `--force-downgrade` only if a real workflow needs intentional correction, with --why required.

### Why this correction

The failure originates in contacts-cli touch mutation contract. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. contacts-cli src/store.rs lines 335-339 maps explicit args.warmth directly and only preserves the warmer current state when the flag is omitted.
2. Warmth enum order is cold, contacted, replied, met, relationship in src/cli.rs.
3. Existing parity test explicitly touches Alice with --warmth contacted and does not assert downgrade refusal.

### Risks and constraints

Ordinal warmth may not capture every relationship correction; forced downgrades need auditability and parity with the Python rebuild path.

## 101. `pc_a3b6d83c3fb7`

Family: `agent-browser-contract-drift`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: agent-browser CLI compatibility/help
Duplicate of: None

### Original filing

```text
agent-browser: 'viewport'/'resize'/'device <name>' commands listed in help but return 'Unknown command' in installed version — blocked 390px mobile emulation during GAVEL VQA; had to fall back to full-res asset inspection
```

### Root cause

The filed commands used obsolete top-level `viewport`/`resize`/`device` syntax; the current CLI implements browser emulation under `set viewport` and `set device`, while bare commands still fail.

### Proposed correction

No CLI code change; ensure all skills/help surfaces use the current `set ...` syntax and add an unknown-command hint mapping viewport/resize/device to it.

### Why this correction

Current evidence indicates that the `agent-browser-contract-drift` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Live agent-browser 0.30.0 help documents Browser Settings: `agent-browser set viewport <w> <h>` and `set device <name>`.
2. Live bare `agent-browser viewport`, `resize`, and `device <name>` return unknown-command/subcommand errors.
3. Installed README now shows the `set viewport` and `set device` forms.

### Risks and constraints

A compatibility alias named device conflicts with the existing iOS `device list` command; hints are safer than aliases.

## 102. `pc_944d374ac9c4`

Family: `zsh-reserved-status-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared agent shell instructions and reusable diagnostic snippets
Duplicate of: None

### Original filing

```text
zsh reserves the status parameter; reusable shell diagnostics should use exit_code instead.
```

### Root cause

The diagnostic was authored as generic shell while running under zsh, where status is a special read-only parameter.

### Proposed correction

Add one shell-authoring rule to shared agent instructions/examples: never assign zsh special parameters; use rc, exit_code, or http_code.

### Why this correction

The evidence identifies `zsh-reserved-status-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. A live `zsh -f -c 'status=1'` exits 1 with `read-only variable: status`.
2. The complaint identifies assignment to status; no reusable source file was named, so recurrence beyond the recorded snippet is complaint-derived.

### Risks and constraints

1. Renaming only one snippet will not prevent recurrence if the rule is not on the shared instruction surface.

## 103. `pc_c0598bce24d9`

Family: `threejs_asset_validator_dependency`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `strong`
Owner surface: claude-space/walk/threejs-agent-kit package manifest and CI/gate
Duplicate of: None

### Original filing

```text
threejs-agent-kit scripts/validate-assets.mjs fails ERR_MODULE_NOT_FOUND out of the box — its import isn't in package.json deps; the kit ships a broken gate
```

### Root cause

The asset validator imported gltf-validator without declaring it in the kit manifest at the time of the complaint.

### Proposed correction

No new code: retain the declared dependency and make the clean-install gate run `npm ci` followed by `npm run assets:validate` so manifest drift cannot recur.

### Why this correction

Current evidence indicates that the `threejs_asset_validator_dependency` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current scripts/validate-assets.mjs imports `gltf-validator`.
2. Current threejs-agent-kit/package.json declares `gltf-validator` in devDependencies and node_modules is installed.
3. The historical missing declaration itself is supported by the complaint, not a retained failing checkout.

### Risks and constraints

1. A gate run only against an existing node_modules directory can hide future undeclared imports.

## 104. `pc_df6af25a100a`

Family: `delegate_external_skill_symlinks`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: delegate-agent safe workspace materialization plus affected repository skill layout
Duplicate of: `pc_f8eb38d950f5`

### Original filing

```text
Delegate worktree contains broken .codex/skills/clean-code and tdd symlinks; worktree creation should preserve or relink project skills so mandatory skill loading works
```

### Root cause

The mandatory clean-code and tdd entrypoints were external symlinks, and Delegate's safe isolation correctly replaced them with placeholders.

### Proposed correction

Use the family fix: materialize bounded skill contents or vendor the required skill entrypoints; do not weaken external-symlink containment.

### Why this correction

The failure originates in delegate-agent safe workspace materialization plus affected repository skill layout. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The retained 20260711T044830Z worktree has regular placeholder files at both .codex skill paths and .claude/skills/clean-code.
2. The placeholder content matches delegate-agent's SAFE_BLOCKED_SYMLINK_PLACEHOLDER constant.

### Risks and constraints

1. Relinking to the same external target merely recreates the security violation.

## 105. `pc_a21c970bd217`

Family: `apply-patch-context-fragility`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: shared agent editing instructions
Duplicate of: None

### Original filing

```text
A multi-hunk apply_patch failed entirely because one long HSGAC URL context omitted 'of' from the live file; smaller contextual hunks would have prevented the all-or-nothing retry.
```

### Root cause

A large multi-hunk patch depended on an unnecessary long-line context match. apply_patch is atomic, so one stale context line rejected every otherwise valid hunk.

### Proposed correction

Change editing guidance: split unrelated hunks and use the shortest unique context, especially around long URLs or generated prose.

### Why this correction

The evidence identifies `apply-patch-context-fragility` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint records the precise omitted word in the URL context and the all-or-nothing retry.
2. The local apply_patch surface accepts patch text and rejects invalid patch/context as a unit; no product source defect was identified.

### Risks and constraints

1. Non-atomic partial application would be more dangerous than the retry; do not redesign apply_patch to partially succeed.

## 106. `pc_6157c26ecfce`

Family: `macos_portable_shell_tools`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: shared macOS shell/research helper instructions
Duplicate of: None

### Original filing

```text
macOS csplit rejected GNU-style '{*}' repetition while splitting Exa JSON page bodies; a documented portable JSON-to-files helper would avoid this minor detour.
```

### Root cause

The snippet assumed GNU csplit repetition syntax on macOS/BSD csplit.

### Proposed correction

Document one portable JSON splitter using jq (for example, `jq -c '.[]'` or indexed `jq` writes) rather than csplit dialect extensions.

### Why this correction

The evidence identifies `macos_portable_shell_tools` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. macOS csplit's local manual lists BSD operands and does not document GNU `{*}` repetition.
2. The complaint records the rejected `{*}` invocation; the exact command was not retained.

### Risks and constraints

1. A helper must define whether input is an array, JSONL, or concatenated JSON; treating them interchangeably can corrupt page boundaries.

## 107. `pc_2f283876d668`

Family: `prompt_relative_path_ambiguity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: Packard lane prompt generator/brief files
Duplicate of: None

### Original filing

```text
Lane prompt local corpus paths used ../../lane-B-written-corpus.md and ../../transcripts from credibility-card, but files are one level up at ../; corrected manually after directory inspection.
```

### Root cause

The lane prompt used paths relative to its own briefs directory, while the agent executed from credibility-card. Relative-path base was implicit, so `../../` resolved one level too high from the actual cwd.

### Proposed correction

Put absolute corpus paths in generated lane prompts, or state and enforce a required cwd before listing relative paths.

### Why this correction

The failure originates in Packard lane prompt generator/brief files. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. From credibility-card, ../lane-B-written-corpus.md and ../transcripts exist while both ../../ variants are missing.
2. The two lane brief files literally instruct ../../lane-B-written-corpus.md and ../../transcripts.
3. Those paths would be correct only if interpreted relative to credibility-card/briefs, not the lane cwd.

### Risks and constraints

1. Absolute paths reduce portability; if portability matters, emit a repo-root variable plus paths relative to that declared root.

## 108. `pc_ff863521e129`

Family: `zsh-reserved-status-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared agent shell instructions and credential-validation snippets
Duplicate of: `pc_944d374ac9c4`

### Original filing

```text
zsh reserves the variable name 'status' as read-only; credential-validation shell snippet failed before storing anything. Use http_code or another non-special name.
```

### Root cause

A credential-validation snippet assigned zsh's read-only status parameter.

### Proposed correction

Use http_code for HTTP status and add the shared no-status-assignment rule.

### Why this correction

The evidence identifies `zsh-reserved-status-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live zsh confirms status assignment exits before subsequent checks.
2. The complaint says the snippet failed before storing anything; no reusable source file was named.

### Risks and constraints

1. Credential validation must continue to avoid printing response bodies or secret-bearing headers.

## 109. `pc_e0092074a509`

Family: `blender_mcp_headless_shutdown`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `needs-repro`
Confidence: `tentative`
Owner surface: Blender MCP extension/upstream
Duplicate of: None

### Original filing

```text
Blender's documented Ctrl-C exit for 'blender --background --command blender_mcp' required a second Ctrl-C, which force-killed the process and emitted unfreed-memory warnings; provide a graceful shutdown path for headless MCP smoke tests.
```

### Root cause

The Blender MCP background command appears not to complete a graceful shutdown on the first SIGINT, leaving the host process alive until a second interrupt force-terminates it. Source-level ownership between Blender and the MCP extension is not established from retained artifacts.

### Proposed correction

Reproduce under a bounded smoke harness, then add an MCP shutdown RPC or signal handler that stops the server/event loop and asks Blender to quit; document a timed SIGTERM fallback only after graceful timeout.

### Why this correction

The evidence for `blender_mcp_headless_shutdown` does not isolate the failing layer. A complete reproduction is needed to avoid fixing the wrong component.

### Evidence

1. Blender 5.1.2 and the installed MCP extension are present locally.
2. The complaint records two Ctrl-C events and unfreed-memory warnings.
3. No MCP source, process trace, or reproducible smoke command output was retained in the referenced repo.

### Risks and constraints

1. Calling Blender quit while writes are active can lose scene changes; smoke mode must use a disposable file and bounded timeout.

## 110. `pc_f160678a51cc`

Family: `archive_layout_assumption`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: shared artifact-validation instructions/smoke script
Duplicate of: None

### Original filing

```text
Blender MCP release ZIP stores blender_manifest.toml at archive root, so wildcard path validation failed; inspect ZIP layout before asserting nested paths.
```

### Root cause

Validation assumed blender_manifest.toml lived below a nested extension directory instead of inspecting the release archive's actual root layout.

### Proposed correction

Make release validation enumerate archive entries first and assert on basename plus expected cardinality, not a guessed wildcard directory depth.

### Why this correction

The evidence identifies `archive_layout_assumption` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The currently installed extension has blender_manifest.toml at the MCP extension root.
2. The release ZIP itself was not retained in the searched claude-space tree; its historical archive-root layout is complaint evidence.

### Risks and constraints

1. Basename-only checks must reject duplicate manifests and still validate the selected manifest's extension id/version.

## 111. `pc_e31354465446`

Family: `delegate_run_output_tail_default`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: delegate-agent run-output parser/help/tests
Duplicate of: None

### Original filing

```text
delegate run-output --tail without --stdout/--stderr errors with invalid_option_combination instead of defaulting to stdout; costs a retry every time
```

### Root cause

The parser treats `--tail`/`--max-chars` as modifiers that require an explicit stream; the user's common shorthand expected stdout to be implied.

### Proposed correction

When `--tail` or `--max-chars` is supplied with no output selector, imply `--stdout`; keep explicit --stderr and completion-report behavior unchanged and document the shorthand.

### Why this correction

The failure originates in delegate-agent run-output parser/help/tests. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Current cli_parser.py deliberately raises invalid_option_combination when tail/max-chars is present without stdout/stderr/raw.
2. Current help says the flags require a stream, so this is current product policy rather than a transient bug.

### Risks and constraints

1. Changing an intentional validation rule is a CLI behavior change; scripts expecting an error must be updated, and stderr must never be silently selected.

## 112. `pc_b967e7071e47`

Family: `delegate_codex_error_classification`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: delegate-agent Codex auth classifier and synthesized report
Duplicate of: `pc_344b79d2e28e`

### Original filing

```text
delegate codex failing with opaque 'harness_error' when the Codex OAuth token is revoked — completion report and stderr are both empty, so the agent burns cycles suspecting its own syntax. A 'codex auth expired/revoked: run codex login' line in the failure reason would have saved two retries.
```

### Root cause

Older Delegate logic did not classify revoked/expired Codex auth from the child stream or synthesize actionable login remediation.

### Proposed correction

No new code; verify the installed Delegate matches current main, then resolve as covered by the classifier/report fix.

### Why this correction

Current evidence indicates that the `delegate_codex_error_classification` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current runner.py includes revoked-token auth patterns, sets failureReason auth_failed, and recommends `delegate profiles` plus `codex login`.
2. Current runner tests assert auth_failed synthesized report/remediation behavior.

### Risks and constraints

1. Do not classify arbitrary child-auth discussion as harness auth failure; current tests specifically guard a false-positive 401 case.

## 113. `pc_26b94226e075`

Family: `rust_msrv_gate`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: exa-agent-cli CI and developer gate
Duplicate of: None

### Original filing

```text
Rust 1.96 accepted std::fs::File::lock, but the repo MSRV 1.85 gate only surfaced via all-features clippy; an MSRV compile helper would catch new-stdlib API drift earlier.
```

### Root cause

Development used stable Rust newer than the declared MSRV, while CI also runs only stable, so adoption of a newer std API was not caught until a separate 1.85/all-features check.

### Proposed correction

Add one CI/helper command that compiles or clippies all features with Rust 1.85, alongside stable; keep the fs2 implementation until MSRV is intentionally raised.

### Why this correction

The failure originates in exa-agent-cli CI and developer gate. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. exa-agent-cli Cargo.toml declares rust-version 1.85.
2. Current Cargo.toml uses fs2 rather than the newer std file-lock API, so the immediate incompatibility is repaired.
3. Current .github/workflows/ci.yml installs only dtolnay/rust-toolchain@stable; no 1.85/MSRV job was found.

### Risks and constraints

1. Pinning old Rust may expose dependencies that raised their MSRV; use the lockfile and treat that as a real compatibility failure, not a reason to skip the gate.

## 114. `pc_3c1502703dcd`

Family: `cargo_build_script_invalidation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `strong`
Owner surface: exa-agent-cli build.rs and codegen regression tests
Duplicate of: None

### Original filing

```text
Cargo kept executing a stale exa-agent build-script binary after build.rs changed (the binary lacked the new BuilderId string even after touch); a scoped build-script invalidation helper would avoid confusing overlay validation failures.
```

### Root cause

Cargo reused a build-script artifact because the script/input rerun dependencies were not sufficiently explicit during the earlier validation workflow.

### Proposed correction

No helper now: current explicit rerun-if-changed coverage is the minimal fix. Keep a regression assertion that generated registry output changes after a build.rs/input edit.

### Why this correction

Current evidence indicates that the `cargo_build_script_invalidation` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current build.rs emits `cargo:rerun-if-changed=build.rs` plus all registry/spec inputs and git HEAD/ref paths.
2. The complaint records a stale build-script binary lacking a new BuilderId string; no stale target directory was retained for replay.

### Risks and constraints

1. A generic cargo clean helper would be slow and conceal missing dependency declarations; use targeted invalidation only as emergency diagnosis.

## 115. `pc_8ae5f391206a`

Family: `cargo_single_test_filter`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared Rust test instructions and generated wave prompts
Duplicate of: None

### Original filing

```text
cargo test accepts only one positional test filter, so Wave E focused tests need one combined filter or separate invocations
```

### Root cause

The command passed multiple positional test-name filters even though cargo test accepts exactly one TESTNAME before the test-binary argument separator.

### Proposed correction

Use one shared substring filter when names permit; otherwise invoke cargo test separately for each focused test/target.

### Why this correction

The evidence identifies `cargo_single_test_filter` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Current `cargo test --help` shows a single optional `[TESTNAME]` positional argument.
2. The recorded Wave E command itself was not retained.

### Risks and constraints

1. A broad common substring can accidentally run more tests than intended; separate commands are clearer when the set is small.

## 116. `pc_91911d7ae332`

Family: `safe_credential_name_inspection`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: ~/.ai-profiles local tooling
Duplicate of: None

### Original filing

```text
Broad recursive auth-schema search under ~/.ai-profiles included live and backup credential payloads; a safe helper should exclude credential filenames by default.
```

### Root cause

Credential topology work lacked a first-class name/presence-only inspection command, so agents recursively searched directories containing live, backup, and runtime credential payloads.

### Proposed correction

Add an `ai-profile credential-names`/doctor mode that emits only managed variable names plus presence/source/equality booleans, excludes backups/runtime by construction, and never reads values into output.

### Why this correction

The failure originates in ~/.ai-profiles local tooling. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. ~/.ai-profiles contains protected key files, backup files, runtime session data, and a canonical managed-env-vars.zsh name list.
2. model-performance-journal.md records a prior faulty redaction leak and explicitly says future inspections must emit variable names only.
3. No dedicated name-only credential inventory helper was found in the live profile scripts.

### Risks and constraints

1. Even hashes or prefixes can become correlatable secret material; output only names and booleans, and keep file paths non-sensitive.

## 117. `pc_4642e6d76ee3`

Family: `safe_credential_name_inspection`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: ~/.ai-profiles local tooling and credential-work instructions
Duplicate of: `pc_91911d7ae332`

### Original filing

```text
Credential-masking probe printed an assignment RHS; provide a safe built-in redact-by-key helper or make shell inspection patterns less error-prone.
```

### Root cause

An ad hoc masking pipeline processed assignment text and accidentally printed the right-hand side because there was no safe structured inspection surface.

### Proposed correction

Use the family helper that parses names internally and returns only name/presence/source/equality booleans; prohibit generic redact-after-print probes for credential files.

### Why this correction

The failure originates in ~/.ai-profiles local tooling and credential-work instructions. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The complaint reports an assignment RHS was printed.
2. The local model-performance journal independently records faulty redaction and mandates name-only inspection.
3. A canonical managed variable-name list exists, but no output-safe inspection command was found.

### Risks and constraints

1. Regex masking is not an adequate safety boundary for arbitrary shell syntax or quoted/multiline values.

## 118. `pc_211736ebcc47`

Family: `zsh-reserved-status-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared validation wrapper templates/instructions
Duplicate of: `pc_944d374ac9c4`

### Original filing

```text
Validation wrapper used zsh's read-only status parameter as a mutable variable, so checks did not run; use rc or exit_code for shell status aggregation.
```

### Root cause

The validation wrapper used zsh's read-only status parameter as an accumulator, so execution stopped before its checks.

### Proposed correction

Rename the accumulator to exit_code/rc and apply the shared zsh-special-parameter instruction.

### Why this correction

The evidence identifies `zsh-reserved-status-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live zsh status assignment fails immediately.
2. The complaint reports checks did not run; no reusable wrapper file was identified.

### Risks and constraints

1. Ensure the wrapper still captures each command's code before another command overwrites `$?`.

## 119. `pc_37830dd5b21e`

Family: `nested-host-shell-interpolation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent tool-call construction instructions
Duplicate of: `pc_657859fb968d`

### Original filing

```text
Quoted heredoc execution was still wrapped in a JavaScript template literal, so shell parameter syntax triggered host interpolation; escape parameter expansions or use a non-template command builder.
```

### Root cause

A quoted shell heredoc was embedded inside a JavaScript template literal, so `${...}` was interpreted by the host JavaScript before the shell ever received the quoted heredoc.

### Proposed correction

Avoid template literals for shell programs containing shell parameter expansion: pass a plain string/argv to exec, load a script file, or escape host interpolation before relying on shell heredoc quoting.

### Why this correction

The evidence identifies `nested-host-shell-interpolation` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint identifies host template interpolation as the pre-execution failure.
2. No reusable command-builder source was named; source-level recurrence is therefore not confirmed.

### Risks and constraints

1. Fixing only shell quoting cannot protect against interpolation performed one language layer earlier.

## 120. `pc_e1e215f2bcc1`

Family: `nested_host_shell_interpolation`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: agent tool-call construction instructions
Duplicate of: `pc_37830dd5b21e`

### Original filing

```text
Protected-profile edit command failed before execution because nested shell quoting broke the generated script; use a quoted heredoc for local transformation scripts.
```

### Root cause

The protected-profile transformation was packed into nested shell quoting, and the generated script broke before execution.

### Proposed correction

Use one quoted heredoc or a checked-in/local script file, with the outer tool call avoiding a second interpolation language.

### Why this correction

The evidence identifies `nested_host_shell_interpolation` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint records a pre-execution nested-quoting failure.
2. No durable script was identified, supporting an authoring/instruction issue rather than a live product defect.

### Risks and constraints

1. Credential transformations must never embed secret values in command text or tool logs.

## 121. `pc_b95946c00f3d`

Family: `safe_credential_name_inspection`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: ~/.ai-profiles local tooling
Duplicate of: `pc_91911d7ae332`

### Original filing

```text
Shell inspection command failed due to nested quote escaping; a name-only config-case helper would prevent fragile ad hoc quoting.
```

### Root cause

A name-only credential/config inspection still depended on fragile nested shell quoting because no structured name-only helper existed.

### Proposed correction

Use the family credential-names helper rather than generating an ad hoc quoted parser.

### Why this correction

The failure originates in ~/.ai-profiles local tooling. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. The complaint records quote-escaping failure during inspection.
2. The profile tree exposes a canonical managed variable-name list but no dedicated safe inventory command.

### Risks and constraints

1. A helper that shells out with reconstructed assignment text would preserve the same quoting and disclosure hazards; parse files directly and emit only approved fields.

## 122. `pc_a782707f3a97`

Family: `memoryd_repo_runtime_default`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: agent-memory memoryd dream CLI argument resolution
Duplicate of: None

### Original filing

```text
memoryd dream now --repo <path> still requires a separate --runtime; the runtime default is ./.memoryd relative to cwd instead of following --repo, so the obvious invocation fails with 'local-device.yaml is missing'
```

### Root cause

Dream subcommand structs declare runtime as a PathBuf with clap default `.memoryd`, so the default is fixed relative to process cwd and cannot be derived from an explicitly supplied --repo.

### Proposed correction

Change dream repo/runtime args to Option&lt;PathBuf&gt;, resolve them through the existing shared resolver, and add one repo-only CLI test asserting `<repo>/.memoryd`.

### Why this correction

The failure originates in agent-memory memoryd dream CLI argument resolution. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. agent-memory/crates/memoryd/src/cli/mod.rs DreamNowArgs and sibling dream structs use `#[arg(long, default_value = ".memoryd")] pub runtime: PathBuf`.
2. agent-memory/src/paths.rs already provides resolve_repo_runtime_paths(repo: Option&lt;PathBuf&gt;, runtime: Option&lt;PathBuf&gt;) and correctly defaults runtime to repo/.memoryd.
3. Dream CLI tests currently pass both --repo and --runtime rather than covering repo-only defaulting.

### Risks and constraints

1. Apply consistently across dream status/now/scheduled/cleanup/review/toggle variants so command defaults do not diverge.

## 123. `pc_ada11ec02666`

Family: `zsh-reserved-status-variable`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared gate-wrapper instructions
Duplicate of: `pc_944d374ac9c4`

### Original filing

```text
Gate wrapper assigned to zsh's read-only status parameter; using a neutral exit_code variable would have preserved the captured gate tail.
```

### Root cause

The gate wrapper assigned zsh's read-only status parameter, aborting before it could preserve the gate tail.

### Proposed correction

Use exit_code and capture the tail before any subsequent command changes `$?`; apply the shared shell rule.

### Why this correction

The evidence identifies `zsh-reserved-status-variable` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. Live zsh confirms the reserved parameter behavior.
2. The complaint identifies the lost captured tail; no durable wrapper source was named.

### Risks and constraints

1. Pipeline exit status semantics differ unless pipefail is deliberate; preserve the intended command's code explicitly.

## 124. `pc_88a67e8cea07`

Family: `macos_portable_shell_tools`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `strong`
Owner surface: shared macOS shell/runtime verification instructions
Duplicate of: None

### Original filing

```text
Runtime verification script used Bash lowercase parameter expansion under the repo's zsh shell; a shell-neutral temp-name helper would have prevented the retry.
```

### Root cause

A script running under zsh used Bash-only lowercase parameter expansion.

### Proposed correction

Use a shell-neutral temp name (`mktemp`) or portable `tr '[:upper:]' '[:lower:]'`; declare bash explicitly only when bash is actually required.

### Why this correction

The evidence identifies `macos_portable_shell_tools` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. The complaint names Bash lowercase expansion under the repo's zsh shell.
2. No reusable runtime script containing the expression was found in the referenced gavel source during this lane.

### Risks and constraints

1. macOS ships an old Bash; switching shebangs casually can introduce a different compatibility problem.

## 125. `pc_52e555b5a7d0`

Family: `exa_explicit_output_precedence`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `already-fixed`
Confidence: `confirmed`
Owner surface: exa-agent-cli output routing
Duplicate of: None

### Original filing

```text
exa-agent contents --output /tmp/file still auto-spilled elsewhere at the 48 KiB ceiling, left the requested output path absent, and made the follow-up rg fail; explicit --output should win or the envelope should state why it did not
```

### Root cause

Older output-ceiling logic applied auto-spill before honoring an explicit --output destination, so the caller's requested path was absent.

### Proposed correction

No new code; run the explicit-output regression and resolve this complaint against current main/installed binary.

### Why this correction

Current evidence indicates that the `exa_explicit_output_precedence` behavior has already been corrected or was not a defect. More implementation would duplicate existing behavior; verification and formal resolution are the remaining work.

### Evidence

1. Current exa-agent-cli tests include `explicit_output_wins_over_max_output_bytes_and_confirms_file` with a 60KB response and a one-byte ceiling.
2. Current test asserts the requested file exists, is untruncated, and stdout confirms dataPath.
3. Current output source has dedicated explicit output writers.

### Risks and constraints

1. Keep atomic file writes and structured path errors; explicit output must not partially overwrite an existing destination on command failure.

## 126. `pc_8e017bfc8c63`

Family: `vercel_api_endpoint_parameter_discovery`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `external-upstream`
Confidence: `confirmed`
Owner surface: Vercel CLI beta api command upstream
Duplicate of: None

### Original filing

```text
Vercel CLI API endpoint list exposes firewall config routes but not the required query/path parameters, and GET /v1/security/firewall/config/active with projectId returns an opaque 404; parameter discovery should be available from endpoint help.
```

### Root cause

Vercel CLI's beta `api list` exposes endpoint routes but not per-endpoint path/query parameter help, leaving callers to guess project/team identifiers and receiving an opaque 404.

### Proposed correction

Upstream should add `vercel api describe <endpoint>` (or include OpenAPI parameters in JSON list output) with required path/query fields and examples; locally, use the cached OpenAPI spec as the interim source.

### Why this correction

Vercel CLI beta api command upstream owns the failing behavior. Local code cannot repair that upstream state reliably; the local work should make the failure explicit and preserve a safe fallback.

### Evidence

1. Installed Vercel CLI 54.7.1 marks api as beta.
2. `vercel api list --help` offers only table/JSON format and refresh; it has no endpoint-detail/help selector.
3. Top-level `vercel api --help` documents generic -F/-f fields but not operation-specific required parameters.

### Risks and constraints

1. Examples must distinguish projectId, project name, and team scope; guessing among them can query the wrong tenant or produce misleading 404s.

## 127. `pc_7892e011944e`

Family: `macos_portable_shell_tools`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `instruction-only`
Confidence: `confirmed`
Owner surface: shared macOS shell guidance and ai-profile helper
Duplicate of: None

### Original filing

```text
macOS awk rejected the GNU-style match capture array syntax during credential-name inspection; a portable grep/sed extraction helper would avoid shell-specific retry work.
```

### Root cause

The extraction used GNU awk's third match() capture-array argument, which macOS awk rejects.

### Proposed correction

For simple assignment names, use portable sed/grep or the proposed structured credential-names helper instead of awk capture arrays.

### Why this correction

The evidence identifies `macos_portable_shell_tools` as an agent-use or guidance failure, not a product defect. Updating the instruction or helper prevents the same mistake without changing product behavior.

### Evidence

1. A live local awk probe using `match(s,/.../,m)` fails with syntax errors and exit 2.
2. The complaint records the same failure during credential-name inspection.

### Risks and constraints

1. Text regexes over shell assignments remain incomplete; the structured helper is safer for credentials.

## 128. `pc_97ff0f165238`

Family: `transcribe_event_page_no_media`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: ~/.local/bin/transcribe-url and its installed usage docs
Duplicate of: None

### Original filing

```text
transcribe-url dead-ends on think-tank event pages (e.g. cato.org/multimedia/events/*): ElevenLabs source_url 400s and yt-dlp reports unsupported URL, exit 0 with only a tiny error raw.json left behind. Two fixes worth having: non-zero exit / loud failure when no transcript.md is produced, and a hint in the header docs to resolve event pages to their podcast/YouTube mirror (Cato events live on the Cato Event Podcast via Acast) before calling.
```

### Root cause

transcribe-url treats arbitrary event pages as media URLs: it tries ElevenLabs source_url, then yt-dlp. Think-tank event landing pages may expose neither direct media nor a yt-dlp extractor even when a podcast/YouTube mirror exists elsewhere.

### Proposed correction

Keep the now-loud nonzero failure, add a regression asserting no transcript means nonzero, and add an actionable error/header hint for unsupported event landing pages to resolve their podcast/YouTube/Acast mirror first.

### Why this correction

The failure originates in ~/.local/bin/transcribe-url and its installed usage docs. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. Current ~/.local/bin/transcribe-url uses source_url first for generic HTTP URLs, then yt-dlp fallback.
2. Current script begins with set -euo pipefail, so a current yt-dlp failure is nonzero; the historic exit-0 behavior is no longer present in this file.
3. Current header docs do not mention resolving event pages to podcast/YouTube mirrors or the Cato Event Podcast.

### Risks and constraints

1. Do not silently scrape and choose a mirror without showing the resolved source; citation provenance must retain both landing page and actual media URL.

## 129. `pc_3d8f55856fe6`

Family: `x_watch_credential_presence_vs_validity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `strong`
Owner surface: X developer app credentials plus x-watch doctor
Duplicate of: `pc_b66efae3997d`

### Original filing

```text
x-watch pc_b66efae3997d diagnosed 2026-07-10: NOT just a stale bearer token — the full app credential set is dead. doctor passes locally (token found+readable), live search returns 401, and minting a fresh app-only bearer from X_API_KEY/X_API_SECRET via POST /oauth2/token fails with code 99 'Unable to verify your credentials' — consumer keys revoked/regenerated upstream. Fix (Trey, ~2 min): developer.x.com portal → the x-watch app → regenerate keys+bearer → paste into ~/.codex/secrets/x-watch.env (X_WATCH_BEARER_TOKEN is the one the CLI reads; update X_API_KEY/X_API_SECRET too). Smoke: X_WATCH_LIVE=1 npm run x-watch -- search --query 'from:thetreygoff' --json --limit 1 --since 7d
```

### Root cause

This is the diagnostic follow-up to the original x-watch complaint: local files could be present while the full upstream app credential set had been revoked/regenerated. The local diagnosis is plausible but was not re-probed to avoid an unrequested live API call.

### Proposed correction

Regenerate the app keys/bearer in the X developer portal as the operational fix, then add an opt-in `doctor --online` minimal authenticated probe that distinguishes present from accepted credentials.

### Why this correction

The failure originates in X developer app credentials plus x-watch doctor. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. ~/.codex/secrets/x-watch.env exists, checked without printing values.
2. Current x-watch doctor marks api_reachable skipped and only validates token presence/permissions.
3. The reported OAuth code 99 and required portal regeneration come from the complaint's prior live diagnosis, not this read-only pass.

### Risks and constraints

1. Regeneration invalidates old credentials everywhere; update all protected profiles atomically and never print tokens. Online doctor consumes rate limit and must be opt-in.

## 130. `pc_b66efae3997d`

Family: `x_watch_credential_presence_vs_validity`
Severity: `minor`
Log status: `open`
Diagnostic disposition: `fix`
Confidence: `confirmed`
Owner surface: x-watch CLI doctor/config/error contract
Duplicate of: None

### Original filing

```text
x-watch: auth status ok (env file found, perms ok) but live search returns upstream 401 — bearer token expired/revoked. Doctor should distinguish 'credential present' from 'credential valid' with a live probe.
```

### Root cause

x-watch doctor defines health as credential found/readable and explicitly skips API reachability, so revoked credentials can produce doctor ok followed by live 401.

### Proposed correction

Add `doctor --online` with one minimal recent-search/auth probe, classify 401 as credential_invalid with portal/login remediation, and keep default doctor offline/presence-only.

### Why this correction

The failure originates in x-watch CLI doctor/config/error contract. The proposed change addresses that layer directly, so every caller benefits from one correction.

### Evidence

1. x-watch/src/cli.ts doctor sets ok to status.found and emits api_reachable status skipped with a note to run a separate live search.
2. The credential config's canonical key is X_WATCH_BEARER_TOKEN.
3. The historic 401 is complaint evidence; no live API call was made in this diagnosis.

### Risks and constraints

1. Online doctor consumes quota and can reveal query/account metadata; make it explicit, minimal, redacted, and separately report network vs auth failure.

## 131. `pc_ca10740db917`

Family: `journal_cat_n_guard`
Severity: `minor`
Log status: `resolved`
Diagnostic disposition: `resolved`
Confidence: `confirmed`
Owner surface: claude-space pre-commit hook
Duplicate of: None

### Original filing

```text
JOURNAL.md append convention has baked in cat -n line numbers three separate times (fixed 5/8, 5/29, 6/9); the file format has no guard. A pre-commit grep for '^\s*\d+\t' on JOURNAL.md would end the class.
```

### Root cause

The append workflow allowed numbered `cat -n` output to be pasted into continuity files without a staged-content guard.

### Proposed correction

No change; retain the verified hook or migrate the same check into a versioned repo hook/CI if durability across clones becomes necessary.

### Why this correction

The log contains a resolution for `journal_cat_n_guard` and current evidence supports it. No additional implementation is justified.

### Evidence

1. The cut is already resolved with a recorded note.
2. claude-space/.git/hooks/pre-commit currently rejects staged JOURNAL.md/STATE.md additions matching leading number plus tab.
3. The hook's error text explains the cat -n failure class.

### Risks and constraints

1. The current .git hook is local-only and not cloned; that is acceptable for this personal repo but should be versioned if more checkouts/users appear.

## 132. `pc_55fb196ef6ee`

Family: `continuity_state_staleness`
Severity: `minor`
Log status: `resolved`
Diagnostic disposition: `resolved`
Confidence: `confirmed`
Owner surface: claude-space STATE.md and context hydrator
Duplicate of: None

### Original filing

```text
hydrator briefing pointed me at the right letter file by exact name, but STATE.md still says the paper is the top open loop — a session arriving for paper work would trust a stale sitrep. STATE.md wants an 'updated' staleness tripwire the hydrator can flag louder.
```

### Root cause

STATE.md had no sufficiently visible freshness signal, so a correct hydrator pointer could coexist with a stale top-open-loop summary.

### Proposed correction

No change; retain the Updated field and hydrator staleness warning behavior.

### Why this correction

The log contains a resolution for `continuity_state_staleness` and current evidence supports it. No additional implementation is justified.

### Evidence

1. The cut is already resolved with a recorded STATE rewrite note.
2. Current STATE.md has an explicit Updated timestamp and says hydrator staleness is measured/flagged.
3. Current content reflects later July 12 state rather than the stale paper-only sitrep.

### Risks and constraints

1. A timestamp alone can be fresh while content is wrong; the hydrator should continue comparing substantive repo/session state, not only file mtime.

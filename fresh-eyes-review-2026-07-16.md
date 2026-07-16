# Fresh-eyes adversarial review — 2026-07-16

- **Repo:** `/Users/treygoff/Code/papercuts`
- **Range reviewed:** `origin/main..main` (`ffba2bd453ab0faeadf4f923fc727586958c8d6f..e7b38aec64aa720f21a83e676ccb568c40bdd2c2`), 41 changed files
- **Execution:** `cargo build --release`; `cargo test --all-features` five times; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo fmt --check`; `git diff --check`; `sh -n` and ShellCheck on `scripts/check-manifest.sh`; `papercuts doctor` and JSONL parsing on `.papercuts.jsonl`; Gitleaks; the manifest diagnostic and Wave 8 state gate over the documented 17-log union; focused black-box redaction, persistence, and duplicate-add probes.
- **Review constraint:** Native subagent spawning was attempted but failed before launch with `no thread with id`. The local Delegate fallback was not invoked because its mandatory journal append would have violated the instruction that this report be the only write.

## Findings

No blocker findings.

1. **major | `src/commands/add.rs:284-287` | VERIFIED — token-only URL userinfo is never redacted.** `url_userinfo_spans` requires a colon before `@`, while the high-entropy pass exempts values that look like URLs. A black-box filing of `git clone https://ghp_AbCdEf0123456789GhIjKlMnOpQrStUv@github.com/acme/private.git` returned and stored the complete token unchanged. **Concrete failure scenario:** an agent attaches a common token-authenticated Git URL through `--cmd`, `--evidence`, or `--stderr-file`; papercuts persists the credential in stdout and the append-only JSONL log despite the advertised redaction boundary.

2. **major | `src/commands/add.rs:322-343` | VERIFIED — lowercase compound credential keys bypass assignment redaction.** The key matcher recognizes delimiters, camel-case boundaries, and a few exact names, but not concatenated lowercase forms. Black-box filings stored `clientsecret=short-value`, `accesskey=short-value`, `authtoken=short-value`, and `dbpassword=also-short` unchanged. **Concrete failure scenario:** a tool normalizes config keys to lowercase before emitting an error; a short client secret or database password lacks enough entropy for the fallback detector and is permanently written to the log.

3. **major | `scripts/check-manifest.sh:375-380` | VERIFIED — two hard-coded `open` ledger replacements cannot observe their required Wave 6 resolution.** The checker injects `pc_df6af25a100a` and `pc_f8eb38d950f5` as open whenever their deleted source ledgers are absent, but the manifest assigns both `resolve-on-6` (`docs/plans/papercuts-remediation-manifest.md:206,302`). The current Wave 8 state-gate run failed on both exact mismatches, and the amendment's statement that no lost ID is in a resolvable bucket is contradicted by those rows. **Concrete failure scenario:** the delegate-agent Wave 6 work completes and appends resolve events elsewhere; without the deleted cut events, `papercuts list` treats those resolves as orphans, the checker re-injects both IDs as open, and every Wave 6-or-later gate remains red until the script's asserted history is manually edited.

4. **major | `scripts/check-manifest.sh:413-422` | VERIFIED — condition-backed rows can pass after an unverified resolution.** When a due row has a verification or human-task condition but the caller supplies no attestation, the checker sets `expected="either"`; therefore an already-resolved row passes without evidence that its condition ever succeeded. This contradicts the manifest's promise that `resolve-on-verification` rows remain open until their named condition is attested. **Concrete failure scenario:** a bulk resolve accidentally includes `pc_52e555b5a7d0`, whose cited regression check is documented as failed, or a Trey-task row before its key exists; later state gates omit the corresponding flag and still accept the premature resolution.

5. **minor | `src/commands/add.rs:109-116` | VERIFIED — duplicate adds without evidence changed an existing warning string.** The pre-range output was `duplicate papercut; existing record returned`; current code returns `duplicate_cut: existing record returned`, even when no evidence flag is used. The remediation spec says absent evidence must leave existing emissions byte-identical (`docs/plans/papercuts-remediation-2026-07-15.md:78`), and the new test pins only the new value rather than comparing it to the baseline. **Concrete failure scenario:** an existing agent or golden fixture matches the prior `meta.warnings` value to identify a duplicate retry and breaks after upgrading despite using none of the new functionality.

## Verified OK

- The complete Rust gate passed. All 59 tests passed five consecutive times, including the concurrency tests; release build, Clippy with warnings denied, and formatting also passed.
- Multi-ID resolve held up under trace and tests: state-dependent validation occurs before append inside one exclusive lock; invalid or ambiguous batches append nothing; IDs and candidates are sorted and deduplicated; single-ID response shape remains `{changed,record}`; concurrent batches append one resolution per cut; injected partial-write failure rolls the file back.
- Evidence file handling held up outside the redaction gaps above: regular-handle validation, symlink-to-regular-file support, FIFO/device/directory rejection, 1 MiB input bound, UTF-8 validation, redaction-before-4096-byte truncation, and structured 65/66/74/77 error mapping are exercised by meaningful black-box tests.
- `.papercuts.jsonl` is append-only in the range, parses as 51 JSON records, and passes `papercuts doctor` with no findings. `git diff --check` passed.
- The manifest checker passes shell syntax and ShellCheck, reconciles exactly 132 unique report/manifest IDs, validates visible count tables, and correctly rejects current due-wave state mismatches. Its later-state and attestation semantics remain defective as described above.
- Gitleaks found only the intentional fake-token fixtures in `tests/cli.rs`; no apparent live credential was found in the reviewed tree.

## Severity summary

- **Blocker:** 0
- **Major:** 4
- **Minor:** 1

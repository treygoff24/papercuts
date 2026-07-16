# Papercuts Wave 4a acceptance

**Decision:** Accepted on 2026-07-16. Wave 4a is blocker/major clean. The
doctor framework is accepted; its live findings are the planned Wave 4b input.

## Artifact

- Standalone script: `/Users/treygoff/.local/bin/claude-skill`
- Prior SHA-256: `b221b7e4972a72f7888191bd507ea390ac820f937791c8135b24796711f8a6e5`
- Final SHA-256: `5f5fcdd51c7ba30f276f7d443941cb7f41a872134275fe493b14bc09d94eebfb`
- The script is outside a Git repository. This acceptance evidence is therefore
  committed in this repository rather than alongside the standalone artifact.

## Accepted contract

The implementation provides the documented `requires:` grammar; fixed,
shell-free `--version` probes; trusted canonical GNU path and sanitized
environment; bounded process and output handling; topology and realpath sweep;
length-framed content fingerprints; fail-closed scan errors; stable IDs and
deduplication; allowlist integrity checks; JSON output and exit semantics; and
privileged Bash `BASH_ENV` purity.

## Evidence

- Governing scope: `docs/plans/papercuts-remediation-2026-07-15.md:137-155`.
- Final acceptance evidence: `model-performance-journal.md:1099-1119`.
- Sol final review, run `codex-12`: **SHIP**, no blocker or major.
- Grok final review, run `cursor-6`: **SHIP**, no blocker or major.
- Live command: `env -u BASH_ENV /Users/treygoff/.local/bin/claude-skill doctor --json`
  returned exit `1` with 331 skills, 20 duplicate divergences, 12 scan errors,
  and 32 unacknowledged findings. This is the expected Wave 4b input, not a
  Wave 4a framework failure.
- Final JSON artifact: `/tmp/claude-skill-wave4a-final.json`; its summary matches
  the live inventory above.

## Residuals and boundary

- A trusted executable that deliberately daemonizes via `setsid` can escape
  process-group cleanup. This is minor under the contract's trusted-executable
  threat boundary. Robust future containment requires Seatbelt deny-fork or a
  disposable PID-isolated VM/container, not PID polling.
- The host-specific Bash path and Bash 3.2 portability remain outside this
  local-tool wave.

**Next:** Wave 5, then Wave 4b reconciliation.

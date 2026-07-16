# Preserve complete command output or report deterministic truncation

> Draft only. Do not post without Trey approval. Papercut: `pc_8a4580aae521`; Family: `codex-exec-output-capture`; Proposed upstream owner: Codex functions.exec / exec_command runtime

## Problem

```text
exec_command returned no captured output for successful multi-line inventory commands, forcing repeated simpler probes; preserving output or surfacing why it was dropped would avoid redundant diagnostics
```

A Codex exec bridge sometimes completed a multi-line command successfully but returned no captured stdout. The complaint establishes the symptom, but the capture/drop point is outside this repository and did not reproduce in this lane.

## Requested change

In the Codex exec bridge, preserve stdout whenever exit status is zero; if output is intentionally dropped or truncated, return an explicit reason and byte counts. Add a regression using a successful multi-line command with mixed stdout/stderr.

## Evidence

1. Complaint reports successful multi-line inventory commands with empty captured output and successful simpler retries.
2. Current lane ran several multi-line exec_command calls and received stdout, so the failure is intermittent or context-specific rather than a universal shell behavior.
3. No papercuts repository code implements functions.exec or exec_command capture.

## Constraints

Without a captured failing invocation or bridge logs, a fix could target the wrong layer (shell, PTY, truncation, or response serialization).

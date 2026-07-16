# Make research prompts safe in Codex sessions without collaboration threads

> Draft only. Do not post without Trey approval. Papercut: `pc_08099f2644cd`; Family: `collaboration-thread-absence`; Proposed upstream owner: research skill instructions plus Codex collaboration runtime

## Problem

```text
Research skill required a background agent, but collaboration.spawn_agent failed with 'no thread' despite an active Delegate run; direct research had to continue without the mandated background pass.
```

The research skill mandated a collaboration child from inside a standalone Delegate Codex subprocess, which had no Codex app collaboration root thread. The child therefore could not perform the required background pass.

## Requested change

Make background research conditional on a real collaboration coordinator; otherwise let the parent launch it or use Delegate workflow parallelism and continue directly.

## Evidence

1. Delegate documents Codex execution through standalone `codex exec`.
2. The complaint records `no thread` in an active Delegate run, matching the architecture mismatch.
3. No retained runtime artifact proves the exact missing thread identifier.

## Constraints

1. A mandatory background-pass rule that ignores runtime capability will continue to create false task failures.

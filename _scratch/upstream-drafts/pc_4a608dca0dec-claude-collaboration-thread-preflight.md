# Hide or preflight collaboration tools when no thread is bound

> Draft only. Do not post without Trey approval. Papercut: `pc_4a608dca0dec`; Family: `collaboration-thread-absence`; Proposed upstream owner: Claude Code collaboration tool integration/runtime

## Problem

```text
collaboration.spawn_agent failed with 'no thread with id' during a skill-required post-implementation review; Delegate runs should either expose a valid collaboration thread or hide the tool.
```

Claude Code exposed collaboration.spawn_agent even though its runtime invocation was not attached to a valid collaboration thread. Calls therefore failed with no thread with id instead of being hidden or falling back.

## Requested change

Gate tool exposure on a successful collaboration-thread preflight. If the runtime has no thread, either create/bind one before exposing spawn_agent or return a structured unavailable result with an approved local Delegate fallback.

## Evidence

1. Two independent cuts from the same repo report the exact no thread with id failure during skill-required review work.
2. The tool was advertised and callable far enough to return a server-side thread lookup error, so this is capability/session wiring rather than an absent skill.
3. The current Codex collaboration surface works in this task, showing the defect is harness/session-specific.

## Constraints

Automatically creating threads may have ownership, billing, and lifecycle implications; capability hiding plus explicit fallback is the safer first step.

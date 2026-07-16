# Return a supported fallback when collaboration has no thread

> Draft only. Do not post without Trey approval. Papercut: `pc_b7b1fb2f9854`; Family: `collaboration-thread-absence`; Proposed upstream owner: Claude Code collaboration tool integration/runtime

## Problem

```text
collaboration.spawn_agent failed with 'no thread with id' during a skill-required parallel code review; a supported availability preflight or graceful local fallback would prevent the dead end
```

The collaboration tool was exposed without a valid collaboration thread and failed with no thread with id. The tool was available but unusable in that session.

## Requested change

Use the canonical fix from pc_4a608dca0dec: preflight thread binding and hide/fallback when unavailable.

## Evidence

1. Exact error matches pc_4a608dca0dec.
2. Both failures occurred in /Users/treygoff/Code/warroom during skill-required post-implementation review workflows.
3. The repeated failure rules out a one-off prompt typo.

## Constraints

Same as canonical item; avoid silently degrading a required independent-review workflow without reporting the fallback.

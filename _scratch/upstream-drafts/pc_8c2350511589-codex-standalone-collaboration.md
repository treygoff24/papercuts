# Hide collaboration spawning in standalone Codex exec sessions

> Draft only. Do not post without Trey approval. Papercut: `pc_8c2350511589`; Family: `collaboration-thread-absence`; Proposed upstream owner: Codex collaboration runtime plus Delegate/research prompt instructions

## Problem

```text
collaboration.spawn_agent failed with 'no thread with id' despite root thread; prevented requested parallel audit
```

A Delegate Codex lane is a standalone `codex exec` subprocess, not a child in the Codex app's collaboration tree. A prompt that asks that subprocess to call collaboration.spawn_agent can expose a collaboration surface without a resolvable root thread.

## Requested change

Do not require nested collaboration inside Delegate prompts. Have the parent orchestrate lanes or use Delegate workflow parallelism. Codex should hide or fail-fast the collaboration tool with an explicit 'standalone exec has no team thread' message.

## Evidence

1. delegate-agent/docs/cli-reference.md says Codex prompts are delivered on stdin to `codex exec`.
2. No Delegate source creates or registers Codex app collaboration child threads.
3. This diagnostic lane was successfully spawned from an actual Codex app root thread, showing the API itself works when a coordinator thread exists.
4. The exact historic 'no thread with id' runtime state was not retained for replay.

## Constraints

1. Retrying spawn_agent cannot create missing coordinator state and wastes quota; changing Delegate alone cannot repair Codex thread registration.

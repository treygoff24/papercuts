# Prevent hidden-page animation from freezing agent-browser captures

> Draft only. Do not post without Trey approval. Papercut: `pc_07522a6497fc`; Family: `agent-browser-hidden-page-animation-freeze`; Proposed upstream owner: agent-browser browser lifecycle/session runtime

## Problem

```text
agent-browser pages run visibility:hidden so framer-motion/rAF animations freeze mid-flight between screenshots — AnimatePresence-gated content (act switches) doesn't mount until you force frames with throwaway screenshots, which mimics real bugs and eats keypresses fired mid-transition. A flag to force page visibility (or an idle-frames-then-screenshot command) would have saved ~30 min of false-positive chasing.
```

The browser automation page runs with document visibility hidden, so requestAnimationFrame/framer-motion transitions can be throttled or frozen between screenshots. Input sent during the stalled transition is consumed before AnimatePresence-gated content mounts.

## Requested change

Add a launch/session flag that forces Page.setWebLifecycleState active and document visibility visible where Chromium permits, plus a deterministic settle command that advances animation frames until rAF/transition activity is idle before interaction or screenshots.

## Evidence

1. Complaint records that throwaway screenshots advanced frames and unblocked the UI, a strong signature of background/visibility-driven frame throttling.
2. agent-browser 0.30.0 exposes waits, screenshots, viewport, and scrollbar controls but its help and core skill expose no force-page-visible or advance-idle-frames control.
3. The current core skill recommends condition/network waits but does not address visibility-gated animation clocks.

## Constraints

Forcing visibility changes real browser semantics and can mask legitimate background-tab bugs; make it explicit and off by default.

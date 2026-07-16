# Support or fail clearly on HyperFrames sharp installation under Node 26

> Draft only. Do not post without Trey approval. Papercut: `pc_bf8ab691e65a`; Family: `hyperframes-node26-sharp-install`; Proposed upstream owner: hyperframes npm packaging and CLI launcher

## Problem

```text
hyperframes@0.7.58 npx install fails silently on node v26.3.1: sharp postinstall wants node-addon-api/node-gyp, npx caches the broken install and the CLI exits 1 with zero output. Workaround: npm install --prefix <dir> --ignore-scripts hyperframes@latest, run dist/cli.js directly (sharp prebuilds load fine at runtime).
```

hyperframes 0.7.58 declares Node &gt;=22 and depends on sharp 0.34.5. Sharp runs an install check and falls back to a native build; on Node 26.3.1 the reported prebuild check failed and the fallback lacked node-addon-api/node-gyp. npx then reused the incomplete cache while Hyperframes emitted no useful error.

## Requested change

Add Node 26 install smoke coverage; either narrow Hyperframes' engines until sharp install is supported, move sharp behind a lazy/optional path, or ensure native build prerequisites are declared. Never swallow npx/npm stderr, and detect/clear a failed npx cache before retry guidance.

## Evidence

1. Live Node version is v26.3.1.
2. npm registry metadata for hyperframes@0.7.58 declares engines node &gt;=22 and dependency sharp ^0.34.5.
3. npm registry metadata for sharp@0.34.5 defines install as node install/check.js || npm run build and exposes platform binaries as optional dependencies.
4. Complaint supplies a successful workaround: install Hyperframes with --ignore-scripts and invoke dist/cli.js, indicating runtime loading works while install lifecycle is the failure point.

## Constraints

The exact failed npx cache was not retained, so the missing-prebuild reason is based on the detailed cut plus package lifecycle metadata rather than a fresh failed install.

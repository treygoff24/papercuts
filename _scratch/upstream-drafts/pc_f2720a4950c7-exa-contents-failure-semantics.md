# Return structured Exa contents failures with a safe fallback

> Draft only. Do not post without Trey approval. Papercut: `pc_f2720a4950c7`; Family: `exa-contents-failure-semantics`; Proposed upstream owner: Exa API upstream and exa-agent-cli contents warning/fallback UX

## Problem

```text
Exa contents returned partial URL failures for valid live Cato pages with empty error objects, forcing direct fetch/search fallback; clearer crawl failure reasons or live retry would prevent dead ends.
```

Exa's contents endpoint can return HTTP success with per-URL crawl failures and sometimes an empty upstream error object. The CLI cannot recover a reason that Exa did not supply.

## Requested change

Preserve the full per-URL status payload, label an empty error as `upstream_reason_unavailable`, and offer an explicit one-retry/direct-fetch fallback command rather than silently dead-ending.

## Evidence

1. exa-agent-cli currently parses statuses[] and emits url_failed warnings; all-URL failure exits 10 with all_urls_failed.
2. Current tests cover partial and total URL failures and preserve status tags.
3. The specific valid Cato crawl and empty error object are complaint evidence; no live retry was performed in this read-only diagnosis.

## Constraints

1. Automatic retries spend quota and may duplicate slow crawls; direct fetch can differ from Exa-rendered content and must be labeled as fallback evidence.

# Expose a reliable fallback for public social posts unavailable through Exa

> Draft only. Do not post without Trey approval. Papercut: `pc_bd9ede3cf94d`; Family: `exa-public-social-post-fallback`; Proposed upstream owner: exa-agent contents/search fallback guidance and Exa source connectors

## Problem

```text
exa-agent contents can render the public Truth Social profile but timed out on the account's public statuses API, leaving the original July 13 post URL unavailable through direct crawl. The tool should expose a stable fallback for public social post lookup.
```

Exa could render a public Truth Social profile but timed out on the statuses API needed to resolve a specific post. The contents path lacks a stable post-level alternate lookup when direct crawl fails.

## Requested change

Classify social API timeout separately and suggest a bounded search for exact account/date/text snippets, cached/indexed result URLs, or an alternate public endpoint. Preserve that the fallback is secondary discovery, not direct-source confirmation.

## Evidence

1. Cut records profile success, public statuses API timeout, and inability to recover the July 13 post URL.
2. Current Exa help documents per-URL warnings and all_urls_failed behavior but no social-post fallback.
3. No live Truth Social request was made, so current source availability is unverified.

## Constraints

Social mirrors can be incomplete or spoofed; do not upgrade fallback content to verified original without matching source metadata.

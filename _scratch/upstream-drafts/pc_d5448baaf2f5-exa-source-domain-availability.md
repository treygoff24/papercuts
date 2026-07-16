# Explain unavailable source-domain connectors in Exa results

> Draft only. Do not post without Trey approval. Papercut: `pc_d5448baaf2f5`; Family: `exa-source-domain-availability`; Proposed upstream owner: exa-agent search error handling and research skill recipe

## Problem

```text
exa-agent search treats site:domain.com queries as domain filters and returns SOURCE_NOT_AVAILABLE for several common publishers, forcing broader queries and secondary discovery.
```

Exa/upstream source access treats some site-constrained queries as unavailable rather than returning ordinary zero results. Embedding site: in the query provides no stable fallback when that publisher is unsupported.

## Requested change

Document --include-domain as the supported domain restriction path, classify SOURCE_NOT_AVAILABLE distinctly from zero hits, and suggest a broader search plus result-domain filtering when the source is unsupported.

## Evidence

1. Cut reports SOURCE_NOT_AVAILABLE for several common publishers when using site:domain.com queries.
2. Current exa-agent search has explicit --include-domain and --exclude-domain flags, but the installed skill's main recipe does not demonstrate them.
3. No live publisher query was run, so current upstream domain availability is unverified.

## Constraints

Broad fallback searches can surface mirrors with weaker provenance; preserve source labeling and do not treat syndicated copies as the original.

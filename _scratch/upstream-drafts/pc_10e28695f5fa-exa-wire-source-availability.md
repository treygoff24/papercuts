# Distinguish unavailable wire-service sources from empty Exa results

> Draft only. Do not post without Trey approval. Papercut: `pc_10e28695f5fa`; Family: `exa-wire-service-source-availability`; Proposed upstream owner: Exa source availability/error taxonomy and exa-agent rendering

## Problem

```text
exa-agent rejects Reuters and AP as unavailable domains, so direct wire-domain searches cannot run even though syndicated AP and Reuters text is discoverable through PBS, Al-Monitor, Baird Maritime, and other mirrors. The CLI should distinguish temporary source unavailability from auth/reauth errors.
```

Direct Reuters/AP domains are unavailable to the Exa source layer, but the error does not clearly distinguish a publisher access restriction from authentication failure or temporary outage. Syndicated copies remain discoverable elsewhere.

## Requested change

Return a stable source_unavailable or publisher_restricted code with retryable=false/true as appropriate, distinct from auth errors, and suggest broader syndicated discovery while preserving wire-service attribution.

## Evidence

1. Cut reports Reuters and AP direct-domain searches rejected while syndicated text was found on PBS, Al-Monitor, Baird Maritime, and others.
2. Current Exa error contract includes categories such as not_authenticated and upstream/source failures, but no live Reuters/AP call was made to inspect the current exact code.
3. The installed skill advises reading error.code/category/retryable, making a precise source-unavailable code the natural fix.

## Constraints

Syndicated copies may be edited or truncated; the workflow must cite the accessible publisher and separately attribute the wire service.

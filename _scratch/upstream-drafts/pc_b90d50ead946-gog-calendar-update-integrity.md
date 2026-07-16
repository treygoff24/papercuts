# Preserve calendar event integrity during gog updates

> Draft only. Do not post without Trey approval. Papercut: `pc_b90d50ead946`; Family: `gog-calendar-update-integrity`; Proposed upstream owner: gogcli calendar update

## Problem

```text
gog calendar update --attendees="" claims 'set empty to clear' but silently no-ops (returned envelope even showed attendees:[] while the event kept them); also updating attendees on an event you don't organize fails silently with a stale 'updated' timestamp — both need honest errors
```

The calendar updater appears to serialize an empty attendee replacement ambiguously. It then trusts the update response without read-after-write validation or surfacing organizer permission constraints.

## Requested change

Upstream gog should distinguish flag-not-set from explicitly-empty, reject non-organizer attendee mutations before/after API response, and read back attendee changes before emitting success.

## Evidence

1. Installed gog is v0.21.0; current help still promises `--attendees ... set empty to clear`.
2. The complaint records an envelope showing attendees:[] while the event retained attendees and a non-organizer mutation returning apparent success; no destructive calendar mutation was repeated.
3. Binary strings include attendee replacement validation but no visible read-after-write mismatch error contract.

## Constraints

Read-after-write adds quota/latency; retries must not duplicate notifications or mutate recurring instances unexpectedly.

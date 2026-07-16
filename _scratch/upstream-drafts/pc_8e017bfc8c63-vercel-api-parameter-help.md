# Show required parameters for Vercel CLI API endpoints

> Draft only. Do not post without Trey approval. Papercut: `pc_8e017bfc8c63`; Family: `vercel_api_endpoint_parameter_discovery`; Proposed upstream owner: Vercel CLI beta api command upstream

## Problem

```text
Vercel CLI API endpoint list exposes firewall config routes but not the required query/path parameters, and GET /v1/security/firewall/config/active with projectId returns an opaque 404; parameter discovery should be available from endpoint help.
```

Vercel CLI's beta `api list` exposes endpoint routes but not per-endpoint path/query parameter help, leaving callers to guess project/team identifiers. The reported call returned only an opaque 404.

## Requested change

Upstream should add `vercel api describe <endpoint>` (or include OpenAPI parameters in JSON list output) with required path/query fields and examples; locally, use the cached OpenAPI spec as the interim source.

## Evidence

1. Installed Vercel CLI 54.7.1 marks api as beta.
2. `vercel api list --help` offers only table/JSON format and refresh; it has no endpoint-detail/help selector.
3. Top-level `vercel api --help` documents generic -F/-f fields but not operation-specific required parameters.

## Constraints

1. Examples must distinguish projectId, project name, and team scope; guessing among them can query the wrong tenant or produce misleading 404s.

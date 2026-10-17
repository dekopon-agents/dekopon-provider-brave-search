# POI Descriptions

**GET `https://api.search.brave.com/res/v1/local/descriptions`**
LLM-generated descriptions of identified places. This operation belongs to the documented local retrieval family but its text is generated, not verbatim page evidence.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/web/poi_descriptions/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `ids` | query | array | yes | unspecified | Temporary place identifiers obtained from search, valid for about eight hours; repeated query keys, one to twenty IDs per Web guide. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

Up to 20 repeated ids, approximately eight-hour lifetime; no pagination. Must label text generated and retain the associated ID. Initial provider includes an explicit operation rather than silently enriching every place with generated prose.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](poi-descriptions-response.md) · [documented-shape response schema](../../schemas/upstream/poi-descriptions.response.schema.json) · [synthetic response](../../examples/poi-descriptions.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "local_descriptions",
  "results": [
    {
      "type": "local_description",
      "id": "synthetic-place-01",
      "description": "Synthetic generated description: a demonstration garden with an educational display."
    }
  ]
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/poi-descriptions.minimal.provider.json) · [decoded GET request](../../examples/poi-descriptions.minimal.request.json) · [encoded URL](../../examples/poi-descriptions.minimal.url.txt)

```json
{
  "operation": "poi-descriptions",
  "native": {
    "ids": [
      "synthetic-place-01"
    ]
  }
}
```

```text
https://api.search.brave.com/res/v1/local/descriptions?ids=synthetic-place-01
```

### Rich

[Provider JSON](../../examples/poi-descriptions.rich.provider.json) · [decoded GET request](../../examples/poi-descriptions.rich.request.json) · [encoded URL](../../examples/poi-descriptions.rich.url.txt)

```json
{
  "operation": "poi-descriptions",
  "native": {
    "ids": [
      "synthetic-place-01",
      "synthetic-place-02"
    ]
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  }
}
```

```text
https://api.search.brave.com/res/v1/local/descriptions?ids=synthetic-place-01&ids=synthetic-place-02
```


## Planned behavior

Capability/CLI subcommand `poi-descriptions` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/poi-descriptions.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

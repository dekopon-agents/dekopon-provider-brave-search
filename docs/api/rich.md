# Rich Search

**GET `https://api.search.brave.com/res/v1/web/rich`**
Intent-specific real-time structured data using a key from Web Search with enable_rich_callback. Choose for a qualifying weather, financial, calculation or sports result.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/web/rich_search/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `callback_key` | query | string | yes | unspecified | Opaque continuation supplied by Web rich.hint.callback_key; percent-encode verbatim. Never parse it as credentials or execute an arbitrary callback URL. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

Requires Search plan. Read rich.hint.callback_key, then call the fixed /v1/web/rich URL explicitly. No documented key expiry or pagination contract; do not invent one. response_callback_info is metadata, not authority to call arbitrary URLs. Preserve provider attribution.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](rich-response.md) · [documented-shape response schema](../../schemas/upstream/rich.response.schema.json) · [synthetic response](../../examples/rich.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "rich",
  "results": [
    {
      "type": "rich",
      "subtype": "calculator",
      "provider": {
        "name": "Example calculator",
        "url": "https://example.com/calculator"
      },
      "calculator": {
        "expression": "2+2",
        "answer": 4
      }
    }
  ]
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/rich.minimal.provider.json) · [decoded GET request](../../examples/rich.minimal.request.json) · [encoded URL](../../examples/rich.minimal.url.txt)

```json
{
  "operation": "rich",
  "native": {
    "callback_key": "synthetic-key-01"
  }
}
```

```text
https://api.search.brave.com/res/v1/web/rich?callback_key=synthetic-key-01
```

### Rich

[Provider JSON](../../examples/rich.rich.provider.json) · [decoded GET request](../../examples/rich.rich.request.json) · [encoded URL](../../examples/rich.rich.url.txt)

```json
{
  "operation": "rich",
  "native": {
    "callback_key": "synthetic-key-01"
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  }
}
```

```text
https://api.search.brave.com/res/v1/web/rich?callback_key=synthetic-key-01
```


## Planned behavior

Capability/CLI subcommand `rich` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/rich.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

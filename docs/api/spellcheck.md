# Spellcheck

**GET `https://api.search.brave.com/res/v1/spellcheck/search`**
Return candidate corrected phrases without running a web search. Useful when the caller wants to control whether a correction is applied.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/other/spell_check/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `q` | query | string | yes | unspecified | Search phrase; nonempty, at most 400 characters and 50 words where required. Place Search also accepts omission or an empty phrase. |
| `lang` | query | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Language hint for completion or spelling, using the listed language codes. |
| `country` | query | string | no | `AR`, `AU`, `AT`, `BE`, `BR`, `CA`, `CL`, `DK`, `FI`, `FR`, `DE`, `GR`, `HK`, `IN`, `ID`, `IT`, `JP`, `KR`, `MY`, `MX`, `NL`, `NZ`, `NO`, `CN`, `PL`, `PT`, `PH`, `RU`, `SA`, `ZA`, `ES`, `SE`, `CH`, `TW`, `TR`, `GB`, `US`, `ALL`; default `US` | Result-country selection using the listed country codes; ALL means worldwide. For suggest/spellcheck it is a ranking hint, not a hard geographic boundary. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

No count, offset or cursor. lang/country are hints. Reused query model lists fields that may not appear: the prose promises only original query. A correction is a suggestion, not a fetched document; no automatic follow-up search.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](spellcheck-response.md) · [documented-shape response schema](../../schemas/upstream/spellcheck.response.schema.json) · [synthetic response](../../examples/spellcheck.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "spellcheck",
  "query": {
    "original": "orbitl gardens"
  },
  "results": [
    {
      "query": "orbital gardens"
    }
  ]
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/spellcheck.minimal.provider.json) · [decoded GET request](../../examples/spellcheck.minimal.request.json) · [encoded URL](../../examples/spellcheck.minimal.url.txt)

```json
{
  "operation": "spellcheck",
  "native": {
    "q": "orbitl gardens"
  }
}
```

```text
https://api.search.brave.com/res/v1/spellcheck/search?q=orbitl+gardens
```

### Rich

[Provider JSON](../../examples/spellcheck.rich.provider.json) · [decoded GET request](../../examples/spellcheck.rich.request.json) · [encoded URL](../../examples/spellcheck.rich.url.txt)

```json
{
  "operation": "spellcheck",
  "native": {
    "q": "orbitl gardens",
    "lang": "en",
    "country": "GB"
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  }
}
```

```text
https://api.search.brave.com/res/v1/spellcheck/search?q=orbitl+gardens&lang=en&country=GB
```


## Planned behavior

Capability/CLI subcommand `spellcheck` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/spellcheck.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

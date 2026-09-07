# Local POIs

**GET `https://api.search.brave.com/res/v1/local/pois`**
Detailed place records for temporary IDs from Web or Place Search. Choose this when a search card needs address, hours, ratings or reviews.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/web/local_pois/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `ids` | query | array | yes | unspecified | Temporary place identifiers obtained from search, valid for about eight hours; repeated query keys, one to twenty IDs per Web guide. |
| `search_lang` | query | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Preferred content language, not interface language. Codes identify languages; regional/script suffixes narrow the preference. |
| `ui_lang` | query | string | no | `es-AR`, `en-AU`, `de-AT`, `nl-BE`, `fr-BE`, `pt-BR`, `en-CA`, `fr-CA`, `es-CL`, `da-DK`, `fi-FI`, `fr-FR`, `de-DE`, `el-GR`, `zh-HK`, `en-IN`, `en-ID`, `it-IT`, `ja-JP`, `ko-KR`, `en-MY`, `es-MX`, `nl-NL`, `en-NZ`, `no-NO`, `zh-CN`, `pl-PL`, `en-PH`, `ru-RU`, `en-ZA`, `es-ES`, `sv-SE`, `fr-CH`, `de-CH`, `zh-TW`, `tr-TR`, `en-GB`, `en-US`, `es-US`; default `en-US` | Language-region locale for presentation and metadata, not a translation request. |
| `units` | query | string | no | `imperial`, `metric` | metric uses kilometres/Celsius; imperial uses miles/Fahrenheit. Response quantities carry their own unit fields. |
| `x-loc-lat` | header | string | no | `-90`-`90` | User latitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `x-loc-long` | header | string | no | `-180`-`180` | User longitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

Up to 20 repeated ids per request per Web guide; IDs expire in approximately eight hours. No cursor/pagination is documented. A missing record is not evidence the place does not exist. Refresh IDs by an explicit new search, never an automatic fallback.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](local-pois-response.md) · [documented-shape response schema](../../schemas/upstream/local-pois.response.schema.json) · [synthetic response](../../examples/local-pois.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "local_pois",
  "results": [
    {
      "type": "location_result",
      "title": "Example Garden",
      "url": "https://example.com/garden",
      "provider_url": "https://example.com/places/1",
      "id": "synthetic-place-01",
      "coordinates": [
        51.5,
        -0.12
      ],
      "postal_address": {
        "displayAddress": "Example Square, Example City"
      },
      "rating": {
        "ratingValue": 4.2,
        "bestRating": 5,
        "reviewCount": 12
      },
      "opening_hours": {
        "current_day": [
          {
            "abbr_name": "Mon",
            "full_name": "Monday",
            "opens": "09:00",
            "closes": "17:00"
          }
        ]
      }
    }
  ]
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/local-pois.minimal.provider.json) · [decoded GET request](../../examples/local-pois.minimal.request.json) · [encoded URL](../../examples/local-pois.minimal.url.txt)

```json
{
  "operation": "local-pois",
  "native": {
    "ids": [
      "synthetic-place-01"
    ]
  }
}
```

```text
https://api.search.brave.com/res/v1/local/pois?ids=synthetic-place-01
```

### Rich

[Provider JSON](../../examples/local-pois.rich.provider.json) · [decoded GET request](../../examples/local-pois.rich.request.json) · [encoded URL](../../examples/local-pois.rich.url.txt)

```json
{
  "operation": "local-pois",
  "native": {
    "ids": [
      "synthetic-place-01",
      "synthetic-place-02"
    ],
    "search_lang": "en",
    "ui_lang": "en-GB",
    "units": "metric"
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  },
  "localization": {
    "x-loc-lat": "51.50",
    "x-loc-long": "-0.12"
  }
}
```

```text
https://api.search.brave.com/res/v1/local/pois?ids=synthetic-place-01&ids=synthetic-place-02&search_lang=en&ui_lang=en-GB&units=metric
```


## Planned behavior

Capability/CLI subcommand `local-pois` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/local-pois.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`, `links`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

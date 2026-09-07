# LLM Context

**GET `https://api.search.brave.com/res/v1/llm/context`**

**POST `https://api.search.brave.com/res/v1/llm/context`** (same fields in JSON body; guide-supported).
Query-selected extracted chunks for model grounding, including text, tables, code and serialized structured data. Prefer this to ordinary snippets for relevance-focused source material.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/summarizer/llm_context/get/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `q` | query (GET) / body (POST) | string | yes | unspecified | Search phrase; nonempty, at most 400 characters and 50 words where required. Place Search also accepts omission or an empty phrase. |
| `country` | query (GET) / body (POST) | string | no | `AR`, `AU`, `AT`, `BE`, `BR`, `CA`, `CL`, `DK`, `FI`, `FR`, `DE`, `GR`, `HK`, `IN`, `ID`, `IT`, `JP`, `KR`, `MY`, `MX`, `NL`, `NZ`, `NO`, `CN`, `PL`, `PT`, `PH`, `RU`, `SA`, `ZA`, `ES`, `SE`, `CH`, `TW`, `TR`, `GB`, `US`, `ALL`; default `US` | Result-country selection using the listed country codes; ALL means worldwide. For suggest/spellcheck it is a ranking hint, not a hard geographic boundary. |
| `search_lang` | query (GET) / body (POST) | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Preferred content language, not interface language. Codes identify languages; regional/script suffixes narrow the preference. |
| `count` | query (GET) / body (POST) | integer | no | `1`-`50`; default `20` | Requested result ceiling, not a guaranteed number; Web applies it only to web.results, Context uses it for candidate selection. |
| `safesearch` | query (GET) / body (POST) | string | no | `off`, `moderate`, `strict` | Adult-content policy; endpoint-specific off/moderate/strict semantics are explained below. |
| `spellcheck` | query (GET) / body (POST) | boolean | no | default `true` | Allow upstream spelling changes before searching; inspect query.altered rather than pretending the original phrase was executed unchanged. |
| `maximum_number_of_urls` | query (GET) / body (POST) | integer | no | `1`-`50`; default `20` | Upper target for distinct context source URLs. |
| `maximum_number_of_tokens` | query (GET) / body (POST) | integer | no | `1024`-`32768`; default `8192` | Approximate combined context token budget; not characters or bytes and not a serialized-response ceiling. |
| `maximum_number_of_snippets` | query (GET) / body (POST) | integer | no | `1`-`256`; default `50` | Nominal total chunk count; current guide says token budgets take precedence and this may not constrain output. |
| `context_threshold_mode` | query (GET) / body (POST) | string | no | `disabled`, `strict`, `lenient`, `balanced` | strict favors precision; balanced trades coverage against relevance; lenient favors recall; disabled removes relevance thresholding, not extraction limits. Omission uses a calibrated vendor default. |
| `maximum_number_of_tokens_per_url` | query (GET) / body (POST) | integer | no | `512`-`8192`; default `4096` | Approximate context token budget for one source URL. |
| `maximum_number_of_snippets_per_url` | query (GET) / body (POST) | integer | no | `1`-`100`; default `50` | Nominal per-source chunk count; not binding on the current pipeline when token budget permits more. |
| `goggles` | query (GET) / body (POST) | string | no | unspecified | One inline ruleset or registered hosted ruleset URL, or up to three such values; changes ranking/filtering rather than searching a different index. |
| `freshness` | query (GET) / body (POST) | string | no | default `` | Page-date filter: pd=24 hours, pw=7 days, pm=31 days, py=365 days, or inclusive-looking start/end syntax YYYY-MM-DDtoYYYY-MM-DD (boundary inclusivity unspecified). Empty means no age filter. |
| `enable_local` | query (GET) / body (POST) | string | no | unspecified | true forces local recall; false forces standard ranking; omitted/null auto-detects from location headers. Local recall remains strict for safety. |
| `enable_source_metadata` | query (GET) / body (POST) | boolean | no | default `false` | Enrich sources keyed by URL with site_name, favicon, thumbnail and query-independent description. |
| `x-loc-lat` | header | string | no | `-90`-`90` | User latitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `x-loc-long` | header | string | no | `-180`-`180` | User longitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `x-loc-city` | header | string | no | unspecified | User city name for local ranking. |
| `x-loc-state` | header | string | no | unspecified | First-level region code, at most three characters; ISO 3166-2 subdivision component. |
| `x-loc-state-name` | header | string | no | unspecified | First-level region display name for local ranking. |
| `x-loc-country` | header | string | no | `AD`, `AE`, `AF`, `AG`, `AI`, `AL`, `AM`, `AO`, `AQ`, `AR`, `AS`, `AT`, `AU`, `AW`, `AX`, `AZ`, `BA`, `BB`, `BD`, `BE`, `BF`, `BG`, `BH`, `BI`, `BJ`, `BL`, `BM`, `BN`, `BO`, `BQ`, `BR`, `BS`, `BT`, `BV`, `BW`, `BY`, `BZ`, `CA`, `CC`, `CD`, `CF`, `CG`, `CH`, `CI`, `CK`, `CL`, `CM`, `CN`, `CO`, `CR`, `CU`, `CV`, `CW`, `CX`, `CY`, `CZ`, `DE`, `DJ`, `DK`, `DM`, `DO`, `DZ`, `EC`, `EE`, `EG`, `EH`, `ER`, `ES`, `ET`, `FI`, `FJ`, `FK`, `FM`, `FO`, `FR`, `GA`, `GB`, `GD`, `GE`, `GF`, `GG`, `GH`, `GI`, `GL`, `GM`, `GN`, `GP`, `GQ`, `GR`, `GS`, `GT`, `GU`, `GW`, `GY`, `HK`, `HM`, `HN`, `HR`, `HT`, `HU`, `ID`, `IE`, `IL`, `IM`, `IN`, `IO`, `IQ`, `IR`, `IS`, `IT`, `JE`, `JM`, `JO`, `JP`, `KE`, `KG`, `KH`, `KI`, `KM`, `KN`, `KP`, `KR`, `KW`, `KY`, `KZ`, `LA`, `LB`, `LC`, `LI`, `LK`, `LR`, `LS`, `LT`, `LU`, `LV`, `LY`, `MA`, `MC`, `MD`, `ME`, `MF`, `MG`, `MH`, `MK`, `ML`, `MM`, `MN`, `MO`, `MP`, `MQ`, `MR`, `MS`, `MT`, `MU`, `MV`, `MW`, `MX`, `MY`, `MZ`, `NA`, `NC`, `NE`, `NF`, `NG`, `NI`, `NL`, `NO`, `NP`, `NR`, `NU`, `NZ`, `OM`, `PA`, `PE`, `PF`, `PG`, `PH`, `PK`, `PL`, `PM`, `PN`, `PR`, `PS`, `PT`, `PW`, `PY`, `QA`, `RE`, `RO`, `RS`, `RU`, `RW`, `SA`, `SB`, `SC`, `SD`, `SE`, `SG`, `SH`, `SI`, `SJ`, `SK`, `SL`, `SM`, `SN`, `SO`, `SR`, `SS`, `ST`, `SV`, `SX`, `SY`, `SZ`, `TC`, `TD`, `TF`, `TG`, `TH`, `TJ`, `TK`, `TL`, `TM`, `TN`, `TO`, `TR`, `TT`, `TV`, `TW`, `TZ`, `UA`, `UG`, `UM`, `US`, `UY`, `UZ`, `VA`, `VC`, `VE`, `VG`, `VI`, `VN`, `VU`, `WF`, `WS`, `YE`, `YT`, `ZA`, `ZM`, `ZW` | User country using the enumerated ISO 3166-1 alpha-2 codes; separate from result-country preference. |
| `x-loc-postal-code` | header | string | no | unspecified | User postal code; do not derive from machine location automatically. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |
| `content-type` | header | string | POST only | `application/json` | POST Context requires application/json; not sent for bodyless GET. |

## Restrictions and continuation

GET and POST are documented; POST accepts the same fields in a JSON body, not simultaneous query/body duplicates. Current token budgets override nominal snippet-count controls. sources[url].age has four renderings or an empty array. No cursor or original-full-page retrieval operation is documented. Empty generic data can mean no relevant context; optional local data is independent.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](context-response.md) · [documented-shape response schema](../../schemas/upstream/context.response.schema.json) · [synthetic response](../../examples/context.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "grounding": {
    "generic": [
      {
        "url": "https://example.com/gardens",
        "title": "Orbital Garden Handbook",
        "snippets": [
          "A synthetic orbital garden uses recirculated water.",
          "{\"bed_count\": 4, \"synthetic\": true}"
        ]
      }
    ],
    "map": [],
    "poi": null
  },
  "sources": {
    "https://example.com/gardens": {
      "title": "Orbital Garden Handbook",
      "hostname": "example.com",
      "age": [
        "Sunday, September 6, 2026",
        "2026-09-06",
        "1 day ago",
        "2026-09-06T12:00:00Z"
      ],
      "site_name": "Example Research",
      "description": "A synthetic handbook."
    }
  }
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/context.minimal.provider.json) · [decoded GET request](../../examples/context.minimal.request.json) · [encoded URL](../../examples/context.minimal.url.txt)

```json
{
  "operation": "context",
  "native": {
    "q": "orbital gardens"
  }
}
```

```text
https://api.search.brave.com/res/v1/llm/context?q=orbital+gardens
```

### Rich

[Provider JSON](../../examples/context.rich.provider.json) · [decoded GET request](../../examples/context.rich.request.json) · [encoded URL](../../examples/context.rich.url.txt)

```json
{
  "operation": "context",
  "native": {
    "q": "orbital gardens",
    "count": 12,
    "maximum_number_of_urls": 4,
    "maximum_number_of_tokens": 4096,
    "maximum_number_of_snippets": 8,
    "maximum_number_of_tokens_per_url": 1024,
    "maximum_number_of_snippets_per_url": 3,
    "context_threshold_mode": "strict",
    "country": "GB",
    "search_lang": "en",
    "safesearch": "strict",
    "spellcheck": false,
    "freshness": "py",
    "enable_local": false,
    "enable_source_metadata": true,
    "goggles": [
      "$boost=2,site=example.com",
      "$downrank,site=example.org"
    ]
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
https://api.search.brave.com/res/v1/llm/context?q=orbital+gardens&count=12&maximum_number_of_urls=4&maximum_number_of_tokens=4096&maximum_number_of_snippets=8&maximum_number_of_tokens_per_url=1024&maximum_number_of_snippets_per_url=3&context_threshold_mode=strict&country=GB&search_lang=en&safesearch=strict&spellcheck=false&freshness=py&enable_local=false&enable_source_metadata=true&goggles=%24boost%3D2%2Csite%3Dexample.com&goggles=%24downrank%2Csite%3Dexample.org
```


## Planned behavior

Capability/CLI subcommand `context` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/context.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`, `links`, `relevant`, `page-text`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

### POST JSON

[Minimal POST](../../examples/context.minimal.post.request.json) · [Feature-rich POST](../../examples/context.rich.post.request.json). Set `transport: "POST"` in the proposed provider input. The `body` member of each wire fixture is exactly the JSON body; the wrapper itself is never sent. Content-Type is application/json, while localization stays in headers. Null enable_local is permitted in POST by the guide; GET omission is the canonical auto mode. No search parameters belong in both locations.

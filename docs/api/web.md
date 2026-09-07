# Web Search

**GET `https://api.search.brave.com/res/v1/web/search`**
General ranked links and snippets, with optional vertical enrichments. Choose this for human-facing search cards.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/web/search/get/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `q` | query | string | yes | unspecified | Search phrase; nonempty, at most 400 characters and 50 words where required. Place Search also accepts omission or an empty phrase. |
| `country` | query | string | no | `AR`, `AU`, `AT`, `BE`, `BR`, `CA`, `CL`, `DK`, `FI`, `FR`, `DE`, `GR`, `HK`, `IN`, `ID`, `IT`, `JP`, `KR`, `MY`, `MX`, `NL`, `NZ`, `NO`, `CN`, `PL`, `PT`, `PH`, `RU`, `SA`, `ZA`, `ES`, `SE`, `CH`, `TW`, `TR`, `GB`, `US`, `ALL`; default `US` | Result-country selection using the listed country codes; ALL means worldwide. For suggest/spellcheck it is a ranking hint, not a hard geographic boundary. |
| `search_lang` | query | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Preferred content language, not interface language. Codes identify languages; regional/script suffixes narrow the preference. |
| `ui_lang` | query | string | no | `es-AR`, `en-AU`, `de-AT`, `nl-BE`, `fr-BE`, `pt-BR`, `en-CA`, `fr-CA`, `es-CL`, `da-DK`, `fi-FI`, `fr-FR`, `de-DE`, `el-GR`, `zh-HK`, `en-IN`, `en-ID`, `it-IT`, `ja-JP`, `ko-KR`, `en-MY`, `es-MX`, `nl-NL`, `en-NZ`, `no-NO`, `zh-CN`, `pl-PL`, `en-PH`, `ru-RU`, `en-ZA`, `es-ES`, `sv-SE`, `fr-CH`, `de-CH`, `zh-TW`, `tr-TR`, `en-GB`, `en-US`, `es-US`; default `en-US` | Language-region locale for presentation and metadata, not a translation request. |
| `count` | query | integer | no | `1`-`20`; default `20` | Requested result ceiling, not a guaranteed number; Web applies it only to web.results, Context uses it for candidate selection. |
| `offset` | query | integer | no | `0`-`9`; default `0` | Zero-based PAGE number, not a row offset. Keep count fixed across pages; duplicate results can occur. |
| `safesearch` | query | string | no | `off`, `moderate`, `strict`; default `moderate` | Adult-content policy; endpoint-specific off/moderate/strict semantics are explained below. |
| `spellcheck` | query | boolean | no | default `true` | Allow upstream spelling changes before searching; inspect query.altered rather than pretending the original phrase was executed unchanged. |
| `freshness` | query | string | no | default `` | Page-date filter: pd=24 hours, pw=7 days, pm=31 days, py=365 days, or inclusive-looking start/end syntax YYYY-MM-DDtoYYYY-MM-DD (boundary inclusivity unspecified). Empty means no age filter. |
| `text_decorations` | query | boolean | no | default `true` | Allow upstream emphasis/highlight markers in display strings. Disable for undecorated snippets; never treat returned markup as trusted HTML. |
| `result_filter` | query | string | no | unspecified | Comma-separated vertical selector: discussions=forum clusters; faq=question/answer pairs; infobox=entity panels; news=articles; query=query metadata; summarizer=legacy generated-answer key; videos=video cards; web=web links; locations=places. query and type remain present. Availability depends on plan and query. |
| `units` | query | string | no | `imperial`, `metric` | metric uses kilometres/Celsius; imperial uses miles/Fahrenheit. Response quantities carry their own unit fields. |
| `goggles_id` | query | string | no | unspecified | Legacy Goggles identifier; vendor-deprecated in favor of goggles. Proposed provider rejects this alias with a migration error. |
| `goggles` | query | string | no | unspecified | One inline ruleset or registered hosted ruleset URL, or up to three such values; changes ranking/filtering rather than searching a different index. |
| `extra_snippets` | query | string | no | unspecified | Enable up to five additional alternative excerpts per result; boolean meaning, serialized as true/false in GET despite reference string type. |
| `summary` | query | string | no | unspecified | Legacy switch to generate a Summarizer key, not page text. Deferred: generated-answer service is outside the initial search capability. |
| `enable_rich_callback` | query | boolean | no | default `false` | Request a rich-result continuation key when the query qualifies. Search plan required; does not automatically perform the second call. |
| `include_fetch_metadata` | query | boolean | no | default `false` | Ask for fetch provenance/timestamps; availability and actual freshness are not guaranteed. |
| `operators` | query | boolean | no | default `true` | Interpret operator syntax within q when true; false requests literal query treatment. Operator behavior remains experimental. |
| `x-loc-lat` | header | string | no | `-90`-`90` | User latitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `x-loc-long` | header | string | no | `-180`-`180` | User longitude in decimal degrees; a nonsecret, potentially sensitive location hint. |
| `x-loc-timezone` | header | string | no | unspecified | User IANA timezone name; Web only in the fetched references. |
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

## Restrictions and continuation

Web count limits only web.results. Pagination is offset 0–9; consult query.more_results_available and never infer exhaustion only from a short page. result_filter cannot activate an unavailable plan feature. summary and goggles_id are documented but excluded from initial provider execution. Rich and local follow-ups are explicit separate invocations.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](web-response.md) · [documented-shape response schema](../../schemas/upstream/web.response.schema.json) · [synthetic response](../../examples/web.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "search",
  "query": {
    "original": "orbital gardens",
    "more_results_available": true
  },
  "web": {
    "type": "search",
    "results": [
      {
        "title": "Orbital Garden Handbook",
        "url": "https://example.com/gardens",
        "description": "A synthetic guide to growing plants in an orbital habitat.",
        "extra_snippets": [
          "The demonstration garden uses recirculated water."
        ],
        "profile": {
          "name": "Example Research",
          "url": "https://example.com"
        }
      }
    ]
  },
  "rich": {
    "type": "rich",
    "hint": {
      "vertical": "calculator",
      "callback_key": "synthetic-key-01"
    }
  }
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/web.minimal.provider.json) · [decoded GET request](../../examples/web.minimal.request.json) · [encoded URL](../../examples/web.minimal.url.txt)

```json
{
  "operation": "web",
  "native": {
    "q": "orbital gardens"
  }
}
```

```text
https://api.search.brave.com/res/v1/web/search?q=orbital+gardens
```

### Rich

[Provider JSON](../../examples/web.rich.provider.json) · [decoded GET request](../../examples/web.rich.request.json) · [encoded URL](../../examples/web.rich.url.txt)

```json
{
  "operation": "web",
  "native": {
    "q": "orbital gardens site:example.com",
    "count": 3,
    "offset": 1,
    "country": "GB",
    "search_lang": "en",
    "ui_lang": "en-GB",
    "freshness": "pw",
    "safesearch": "strict",
    "spellcheck": false,
    "text_decorations": false,
    "extra_snippets": true,
    "goggles": [
      "$boost=2,site=example.com"
    ],
    "result_filter": "web,news,locations",
    "units": "metric",
    "enable_rich_callback": true,
    "include_fetch_metadata": true,
    "operators": true
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
https://api.search.brave.com/res/v1/web/search?q=orbital+gardens+site%3Aexample.com&count=3&offset=1&country=GB&search_lang=en&ui_lang=en-GB&freshness=pw&safesearch=strict&spellcheck=false&text_decorations=false&extra_snippets=true&goggles=%24boost%3D2%2Csite%3Dexample.com&result_filter=web%2Cnews%2Clocations&units=metric&enable_rich_callback=true&include_fetch_metadata=true&operators=true
```


## Planned behavior

Capability/CLI subcommand `web` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/web.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`, `links`, `relevant`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

# Place Search

**GET `https://api.search.brave.com/res/v1/local/place_search`**
Discover geographic points of interest and area entities. Omit q to browse general places in an area rather than search a phrase.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/web/place_search/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `radius` | query | string | no | min `0` | Nonnegative radius BIAS in metres, not a strict distance cutoff. Without radius the guide/reference describes global search. |
| `count` | query | integer | no | `1`-`100`; default `20` | Requested result ceiling, not a guaranteed number; Web applies it only to web.results, Context uses it for candidate selection. |
| `country` | query | string | no | `AR`, `AU`, `AT`, `BE`, `BR`, `CA`, `CL`, `DK`, `FI`, `FR`, `DE`, `GR`, `HK`, `IN`, `ID`, `IT`, `JP`, `KR`, `MY`, `MX`, `NL`, `NZ`, `NO`, `CN`, `PL`, `PT`, `PH`, `RU`, `SA`, `ZA`, `ES`, `SE`, `CH`, `TW`, `TR`, `GB`, `US`, `ALL`; default `US` | Result-country selection using the listed country codes; ALL means worldwide. For suggest/spellcheck it is a ranking hint, not a hard geographic boundary. |
| `search_lang` | query | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Preferred content language, not interface language. Codes identify languages; regional/script suffixes narrow the preference. |
| `ui_lang` | query | string | no | `es-AR`, `en-AU`, `de-AT`, `nl-BE`, `fr-BE`, `pt-BR`, `en-CA`, `fr-CA`, `es-CL`, `da-DK`, `fi-FI`, `fr-FR`, `de-DE`, `el-GR`, `zh-HK`, `en-IN`, `en-ID`, `it-IT`, `ja-JP`, `ko-KR`, `en-MY`, `es-MX`, `nl-NL`, `en-NZ`, `no-NO`, `zh-CN`, `pl-PL`, `en-PH`, `ru-RU`, `en-ZA`, `es-ES`, `sv-SE`, `fr-CH`, `de-CH`, `zh-TW`, `tr-TR`, `en-GB`, `en-US`, `es-US`; default `en-US` | Language-region locale for presentation and metadata, not a translation request. |
| `units` | query | string | no | `imperial`, `metric`; default `metric` | metric uses kilometres/Celsius; imperial uses miles/Fahrenheit. Response quantities carry their own unit fields. |
| `safesearch` | query | string | no | `off`, `moderate`, `strict`; default `strict` | Adult-content policy; endpoint-specific off/moderate/strict semantics are explained below. |
| `spellcheck` | query | boolean | no | default `true` | Allow upstream spelling changes before searching; inspect query.altered rather than pretending the original phrase was executed unchanged. |
| `geoloc` | query | string | no | unspecified | User coordinates formatted latitudexlongitude for displayed distances; distinct from the search-centre coordinates. |
| `q` | query | string | no | default `` | Search phrase; nonempty, at most 400 characters and 50 words where required. Place Search also accepts omission or an empty phrase. |
| `latitude` | query | string | no | `-90`-`90` | Search-centre latitude in decimal degrees. |
| `longitude` | query | string | no | `-180`-`180` | Search-centre longitude in decimal degrees. |
| `location` | query | string | no | unspecified | Place-name alternative to latitude/longitude. US: city state country; elsewhere: city country. Case/commas unnecessary; common local language or English preferred. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

location is an alternative to latitude/longitude; proposed provider rejects both together and requires coordinate pairs when either is supplied. radius is a bias, not a geofence; geoloc affects distance display. No offset/cursor is documented. Area categories are returned in separate collections with mixed ordering references.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](place-response.md) · [documented-shape response schema](../../schemas/upstream/place.response.schema.json) · [synthetic response](../../examples/place.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "local_search",
  "query": {
    "original": "gardens"
  },
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
      "distance": {
        "value": 0.2,
        "units": "km"
      }
    }
  ]
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/place.minimal.provider.json) · [decoded GET request](../../examples/place.minimal.request.json) · [encoded URL](../../examples/place.minimal.url.txt)

```json
{
  "operation": "place",
  "native": {}
}
```

```text
https://api.search.brave.com/res/v1/local/place_search
```

### Rich

[Provider JSON](../../examples/place.rich.provider.json) · [decoded GET request](../../examples/place.rich.request.json) · [encoded URL](../../examples/place.rich.url.txt)

```json
{
  "operation": "place",
  "native": {
    "q": "gardens",
    "latitude": "51.50",
    "longitude": "-0.12",
    "radius": "2500",
    "geoloc": "51.51x-0.11",
    "count": 5,
    "country": "GB",
    "search_lang": "en",
    "ui_lang": "en-GB",
    "units": "metric",
    "safesearch": "strict",
    "spellcheck": false
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  }
}
```

```text
https://api.search.brave.com/res/v1/local/place_search?q=gardens&latitude=51.50&longitude=-0.12&radius=2500&geoloc=51.51x-0.11&count=5&country=GB&search_lang=en&ui_lang=en-GB&units=metric&safesearch=strict&spellcheck=false
```


## Planned behavior

Capability/CLI subcommand `place` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/place.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`, `links`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

# Videos Search

**GET `https://api.search.brave.com/res/v1/videos/search`**
Video-page discovery with duration, creator, publisher and thumbnail metadata. Does not download videos or guarantee transcripts.

Primary [reference](https://api-dashboard.search.brave.com/api-reference/videos/video_search/get/index.html.md) · [source conflicts](../sources.md) · [endpoint index](../endpoints.md)

## Inputs

Every table constraint/default is upstream, **not** the proposed projection default. Blank/unspecified means the source does not establish it. Header names are case-insensitive. Auth is required on the real wire but absent in all synthetic examples. Request schemas represent decoded wire components, not an HTTP client. See [encoding and enums](../request-controls.md).

| Field | Location | Vendor type | Required | Vendor default / range / enum | Meaning |
|---|---|---|---|---|---|
| `x-subscription-token` | header | string | yes | unspecified | Required vendor secret header. Broker-only integration gate; not a provider argument or fixture value. |
| `q` | query | string | yes | unspecified | Search phrase; nonempty, at most 400 characters and 50 words where required. Place Search also accepts omission or an empty phrase. |
| `search_lang` | query | string | no | `ar`, `eu`, `bn`, `bg`, `ca`, `zh-hans`, `zh-hant`, `hr`, `cs`, `da`, `nl`, `en`, `en-gb`, `et`, `fi`, `fr`, `gl`, `de`, `el`, `gu`, `he`, `hi`, `hu`, `is`, `it`, `ja`, `jp`, `kn`, `ko`, `lv`, `lt`, `ms`, `ml`, `mr`, `nb`, `pl`, `pt-br`, `pt-pt`, `pa`, `ro`, `ru`, `sr`, `sk`, `sl`, `es`, `sv`, `ta`, `te`, `th`, `tr`, `uk`, `vi`; default `en` | Preferred content language, not interface language. Codes identify languages; regional/script suffixes narrow the preference. |
| `ui_lang` | query | string | no | `es-AR`, `en-AU`, `de-AT`, `nl-BE`, `fr-BE`, `pt-BR`, `en-CA`, `fr-CA`, `es-CL`, `da-DK`, `fi-FI`, `fr-FR`, `de-DE`, `el-GR`, `zh-HK`, `en-IN`, `en-ID`, `it-IT`, `ja-JP`, `ko-KR`, `en-MY`, `es-MX`, `nl-NL`, `en-NZ`, `no-NO`, `zh-CN`, `pl-PL`, `en-PH`, `ru-RU`, `en-ZA`, `es-ES`, `sv-SE`, `fr-CH`, `de-CH`, `zh-TW`, `tr-TR`, `en-GB`, `en-US`, `es-US`; default `en-US` | Language-region locale for presentation and metadata, not a translation request. |
| `country` | query | string | no | `AR`, `AU`, `AT`, `BE`, `BR`, `CA`, `CL`, `DK`, `FI`, `FR`, `DE`, `GR`, `HK`, `IN`, `ID`, `IT`, `JP`, `KR`, `MY`, `MX`, `NL`, `NZ`, `NO`, `CN`, `PL`, `PT`, `PH`, `RU`, `SA`, `ZA`, `ES`, `SE`, `CH`, `TW`, `TR`, `GB`, `US`, `ALL`; default `US` | Result-country selection using the listed country codes; ALL means worldwide. For suggest/spellcheck it is a ranking hint, not a hard geographic boundary. |
| `safesearch` | query | string | no | `off`, `moderate`, `strict`; default `moderate` | Adult-content policy; endpoint-specific off/moderate/strict semantics are explained below. |
| `count` | query | integer | no | `1`-`50`; default `20` | Requested result ceiling, not a guaranteed number; Web applies it only to web.results, Context uses it for candidate selection. |
| `offset` | query | integer | no | `0`-`9`; default `0` | Zero-based PAGE number, not a row offset. Keep count fixed across pages; duplicate results can occur. |
| `spellcheck` | query | boolean | no | default `true` | Allow upstream spelling changes before searching; inspect query.altered rather than pretending the original phrase was executed unchanged. |
| `freshness` | query | string | no | default `` | Page-date filter: pd=24 hours, pw=7 days, pm=31 days, py=365 days, or inclusive-looking start/end syntax YYYY-MM-DDtoYYYY-MM-DD (boundary inclusivity unspecified). Empty means no age filter. |
| `include_fetch_metadata` | query | boolean | no | default `false` | Ask for fetch provenance/timestamps; availability and actual freshness are not guaranteed. |
| `operators` | query | boolean | no | default `true` | Interpret operator syntax within q when true; false requests literal query treatment. Operator behavior remains experimental. |
| `api-version` | header | string | no | unspecified | Date-form API version YYYY-MM-DD; omission follows latest. Operator selects any explicit pin; old Context pipeline pin is 2026-02-06. |
| `accept` | header | string | no | `application/json`, `*/*`; default `application/json` | application/json requests JSON; */* accepts the available representation. Provider fixes JSON. |
| `cache-control` | header | string | no | `no-cache` | no-cache asks upstream to avoid cached content on a best-effort basis; no retention or origin-fetch guarantee. |
| `user-agent` | header | string | no | unspecified | Originating agent/device description can change presentation; operator-owned, not caller-supplied arbitrary identity. |
| `accept-encoding` | header | string | no | `gzip` | Compression negotiation shown as gzip in public examples; operator-only and deferred until bounded decompression behavior is verified. |

## Restrictions and continuation

count 1–50 and page offset 0–9; pages can overlap. No cursor. Subscription flags describe source access, not this API subscription. No video bytes, transcript fetch or image-page controls are implied.

Safety: off disables adult filtering (Images still excludes illegal content); moderate filters explicit material while Web/Videos can retain adult domains; strict excludes adult material (News also describes suggestive material). Context omission applies no filtering except strict local recall. Place defaults strict. These classifications are vendor behavior, not a provider guarantee.

## Output and errors

[Complete reference field-path catalog](videos-response.md) · [documented-shape response schema](../../schemas/upstream/videos.response.schema.json) · [synthetic response](../../examples/videos.response.json).

Nested model optionality, provenance, scores, attribution, byte caps, errors and absence semantics are specified in [response semantics](../response-semantics.md). No cross-endpoint comparable relevance score or per-result price is established. This endpoint uses the shared error envelope; HTTP status and body are separate.

```json
{
  "type": "videos",
  "query": {
    "original": "orbital gardens"
  },
  "results": [
    {
      "type": "video_result",
      "title": "Orbital Garden Demonstration",
      "url": "https://example.com/watch/garden",
      "description": "Synthetic instructional video result.",
      "video": {
        "duration": "04:12",
        "views": 123,
        "creator": "Example Research",
        "requires_subscription": false
      },
      "thumbnail": {
        "src": "https://example.com/video-thumb.jpg"
      }
    }
  ],
  "extra": {}
}
```

## Synthetic requests

These are invented documentation fixtures, not live responses or valid temporary keys. The GET URLs intentionally omit authentication; do not run them as an integration test. Feature-rich is representative, not every combinatorial option.

### Minimal

[Provider JSON](../../examples/videos.minimal.provider.json) · [decoded GET request](../../examples/videos.minimal.request.json) · [encoded URL](../../examples/videos.minimal.url.txt)

```json
{
  "operation": "videos",
  "native": {
    "q": "orbital gardens"
  }
}
```

```text
https://api.search.brave.com/res/v1/videos/search?q=orbital+gardens
```

### Rich

[Provider JSON](../../examples/videos.rich.provider.json) · [decoded GET request](../../examples/videos.rich.request.json) · [encoded URL](../../examples/videos.rich.url.txt)

```json
{
  "operation": "videos",
  "native": {
    "q": "orbital gardens",
    "country": "GB",
    "search_lang": "en",
    "ui_lang": "en-GB",
    "count": 5,
    "offset": 1,
    "freshness": "pm",
    "safesearch": "strict",
    "spellcheck": false,
    "include_fetch_metadata": true,
    "operators": true
  },
  "output": {
    "mode": "raw",
    "max_results": 10,
    "max_bytes": 32768
  }
}
```

```text
https://api.search.brave.com/res/v1/videos/search?q=orbital+gardens&country=GB&search_lang=en&ui_lang=en-GB&count=5&offset=1&freshness=pm&safesearch=strict&spellcheck=false&include_fetch_metadata=true&operators=true
```


## Planned behavior

Capability/CLI subcommand `videos` maps to this fixed endpoint, never an arbitrary URL/body/header pass-through. [Provider input schema](../../schemas/provider/videos.input.schema.json) enforces typed options for this proposal. Available output modes: `raw`, `brief`, `links`. Mode meanings and operator ceilings: [provider plan](../provider-plan.md). All native controls, including deferred controls and nested items, are mapped in [coverage inventory](../../coverage.json). A supported inventory entry means planned support, conditional on the auth integration gate—not a shipped feature.

# Request controls and wire encoding

Primary sources: [references and dated guides](sources.md). These are documented vendor controls, distinct from the proposed `output` namespace. The [inventory](../coverage.json) lists every request field and array item, its method/location and status. No nested JSON request objects are documented for the eleven search operations; Context POST moves the same flat controls into a body. Goggles and IDs can be arrays of strings.

## Encoding contract

- GET has no body. Percent-encode each UTF-8 query name/value once. Spaces may be `%20` or `+`; a literal `+` operator must be `%2B`, not `+`. Encode quotes, newlines, ampersands and colons rather than constructing a URL with string concatenation.
- Serialize booleans as lowercase `true`/`false`. The Web guide also demonstrates `enable_rich_callback=1`, but the reference types it boolean: the proposed serializer uses `true`. `extra_snippets` is string-typed in Web/News references but semantically boolean; proposed typed booleans become strings on GET. `summary=1` is a legacy guide example, not an initial provider feature.
- Encode lists by repeated keys: `ids=synthetic-a&ids=synthetic-b`, `goggles=...&goggles=...`. A Goggles inline ruleset is one value even when it contains multiple newline-separated instructions; newline becomes `%0A`. IDs have maximum 20 items from the Web guide; Goggles maximum 3. Do not comma-join these arrays.
- `result_filter` is different: one comma-separated string, e.g. `web,news`, encoded as `web%2Cnews`. It is not a JSON array. `query` and `type` still identify the response even when not requested. Category availability depends on plan and query; `count` never caps all the mixed verticals together.
- Context supports POST to the identical fixed URL, with `Content-Type: application/json`. Send the fixture's `body` object, not the fixture wrapper. Numbers/booleans remain JSON numbers/booleans; Goggles can be a string or JSON string array. Localization remains in headers. Reject simultaneous query/body controls in an eventual serializer rather than choosing an undocumented precedence.
- Context `enable_local` is string-typed in the GET reference, but boolean/null in the guide. Proposed `true`/`false` encode accordingly; null means omit on GET and can be JSON null on POST. No null string (`"null"`) is planned for GET. Guide-only null for calibrated `context_threshold_mode`/safesearch is represented by omission; use explicit string enums otherwise.
- Header names are case-insensitive. Wire headers are strings even when latitude/longitude represent numeric degrees. Reject CR/LF in an eventual serializer. No arbitrary URL, request body or caller header bag is proposed. The upstream request schemas describe decoded components solely for documentation/testing.
- Examples show `Accept-Encoding: gzip`. Negotiation is **deferred**, not silently assumed safe: compressed response size, expanded size and host decompression behavior must be verified before it is enabled. JSON `Accept` is operator-fixed. `api-version`, `cache-control` and `user-agent` are operator-owned. User-provided localization is narrowly typed; it is not discovered from the host.

## Enum meaning and defaults

Every endpoint table carries its exact reference enum spelling, including apparently redundant codes. Vendor defaults are **not inserted by the documentation validator**. Proposed provider output defaults are independent.

- `country`: supported ISO country codes select the result market; `ALL` is the explicitly supported worldwide exception to two-character spelling. `x-loc-country` instead gives the user's physical country. Its larger enumerated set does not include `ALL`.
- Language codes identify the named language: `ar` Arabic; `eu` Basque; `bn` Bengali; `bg` Bulgarian; `ca` Catalan; `zh-hans` Simplified Chinese; `zh-hant` Traditional Chinese; `hr` Croatian; `cs` Czech; `da` Danish; `nl` Dutch; `en` English; `en-gb` British English; `et` Estonian; `fi` Finnish; `fr` French; `gl` Galician; `de` German; `el` Greek; `gu` Gujarati; `he` Hebrew; `hi` Hindi; `hu` Hungarian; `is` Icelandic; `it` Italian; `ja` Japanese; `jp` appears alongside `ja` in the reference, with no distinct semantics established; `kn` Kannada; `ko` Korean; `lv` Latvian; `lt` Lithuanian; `ms` Malay; `ml` Malayalam; `mr` Marathi; `nb` Norwegian Bokmål; `pl` Polish; `pt-br` Brazilian Portuguese; `pt-pt` European Portuguese; `pa` Punjabi; `ro` Romanian; `ru` Russian; `sr` Serbian; `sk` Slovak; `sl` Slovenian; `es` Spanish; `sv` Swedish; `ta` Tamil; `te` Telugu; `th` Thai; `tr` Turkish; `uk` Ukrainian; `vi` Vietnamese. Preserve the endpoint enum, do not normalize `jp` away without vendor clarification.
- UI locales combine a language with a country/region (`fr-CA` French for Canada, `en-US` English for the US). `no-NO` is Norwegian presentation; `zh-HK`, `zh-CN`, `zh-TW` select regional Chinese presentation. They are metadata/presentation preferences, not a request to translate every page. Endpoint tables enumerate all supported pairs.
- Safety: Web/Videos default `moderate`, retaining adult domains but filtering explicit content; News and Place default `strict`; Images defaults `strict` and does **not** accept moderate. `off` removes adult filtering (Images still removes illegal material). News describes moderate as excluding explicit and strict as excluding explicit plus suggestive material. Context defaults to no adult filtering, **except strict local recall**. Never silently substitute one endpoint's default for another's.
- `freshness`: `pd`=24h, `pw`=7d, `pm`=31d, `py`=365d; custom `YYYY-MM-DDtoYYYY-MM-DD` uses page-reported dates. It does not mean last crawl and does not force a fetch. Date validity, ordering and range-edge inclusion are not established by the subset schemas.
- `units`: metric means kilometre/Celsius conventions; imperial means mile/Fahrenheit conventions. Place specifies metric default; other references leave it unspecified. Inspect response unit labels.
- `context_threshold_mode`: strict prioritizes relevance; balanced trades precision/recall; lenient admits weaker matches; disabled removes that threshold. Omission uses vendor calibration, not necessarily balanced. This controls inclusion, not an exposed numerical score.
- `result_filter`: discussions selects forum clusters; faq selects Q/A pairs; infobox selects entity panels; news selects articles; query requests query metadata (always present); summarizer selects a **legacy generated-answer key**, not text; videos selects video cards; web selects ordinary links; locations selects places. Missing categories are not guaranteed empty lists. Unknown categories are rejected by the proposed validator.
- `accept`: application/json chooses JSON; `*/*` expresses no specific preference. `cache-control: no-cache` is only a best-effort vendor cache request, not a guarantee of original-site retrieval.

## Search operator language (experimental)

The [operator guide](https://api-dashboard.search.brave.com/documentation/resources/search-operators/index.html.md) explicitly warns that behavior and availability are experimental. These are **inside q**, not extra API fields:

| Syntax | Meaning | Synthetic example |
|---|---|---|
| `ext:` | Match file extension | `garden ext:pdf` |
| `filetype:` | Match document type | `garden filetype:pdf` |
| `intitle:` | Term in title | `intitle:garden` |
| `inbody:` | Term/phrase in body | `inbody:"water reuse"` |
| `inpage:` | Term in title or body | `inpage:habitat` |
| `lang:` / `language:` | Content language, ISO 639-1 | `garden lang:en` |
| `loc:` / `location:` | Country/region origin | `garden loc:gb` |
| `site:` | Domain including subdomains | `garden site:example.com` |
| `+` | Require term in title/body | `garden +water` |
| `-` | Exclude term | `garden -plastic` |
| `"phrase"` | Exact ordered phrase | `"orbital garden"` |
| `AND` | Require both conditions | `garden AND habitat` |
| `OR` | Either condition | `garden OR greenhouse` |
| `NOT` | Exclude condition | `garden NOT site:example.org` |

Logical operators are uppercase. No precedence guarantee beyond guide examples is established. Parameter `operators` is explicitly documented on Web/News/Videos; do not invent it for Images, Context or local endpoints. A restrictive phrase may legitimately produce no results. Conflicts between phrase-level lang/loc and explicit native preferences have no documented precedence: preserve them and flag the ambiguity, never rewrite q silently.

## Goggles semantics

The [Goggles guide](https://api-dashboard.search.brave.com/documentation/resources/goggles/index.html.md) covers Web, News and Context. It reranks/filters the existing index; it does not create an exclusive private index or retrieve an entire origin page. Up to three values may mix hosted URLs and inline rulesets. Hosted files must already be submitted to Brave; registration is outside this provider plan. The provider does not fetch, host or register Goggles.

`$boost` raises rank, `$downrank` lowers it, `$discard` removes a match. Optional `=N` strengths are 1–10. `site=` targets domains, path patterns target paths, `*` matches arbitrary characters. Multiple inline rules are newline-separated. Matching conflict precedence is specific discard (except generic discard), then boost over downrank, then higher strength. Do not interpret strength as comparable relevance probability.

Vendor file limits: 2 MB per file (unit precision not clarified), 100,000 instructions, 500 characters per instruction, at most two `*` and two `^` per instruction. These are not provider byte ceilings. Hosted metadata requires `name`, `description`, `public`, `author`; optional `homepage`, `issues`, `avatar`, `license` are file metadata, **not request JSON fields**. The proposed validator checks value count and types, not the entire Goggles DSL or registration state. Legacy `goggles_id` is rejected with guidance to use `goggles`, not silently ignored.

# Response models, provenance and errors

Every endpoint has a complete **fetched reference field-path catalog** and a recursively nested, permissive documented-shape response schema. These are not exhaustive vendor validators: references flatten variable models, omit many field descriptions and leave `any`/generic arrays unconstrained. Unknown fields are allowed; requiredness is deliberately not enforced. `?` means optional vendor notation; accepting null conservatively is not proof null is emitted for every such field. All fixtures are invented, not observations from the service.

## Shared interpretation

- Preserve absence separately from null, empty arrays and empty strings. An absent vertical may mean no matching data, plan restrictions, a changed version or an omitted field; do not assert one explanation. No-results is not a network error.
- Query metadata can contain original, altered and cleaned phrases, language, location, operator extraction, safety warnings and ranking hints. `should_fallback` does not authorize provider fallback. `more_results_available` in a reused model does not create pagination where no request control exists.
- A result URL normally identifies the source page. Image `properties.url` identifies the image asset, `thumbnail.src` the served preview, and `results[].url` the source page. Profile/provider/attribution URLs identify different sources; keep them distinguishable. Never fetch any of them automatically.
- `page_age` comes from content publication/modification; `page_fetched` and fetched timestamps describe retrieval provenance. Relative `age` is presentation, not an exact timestamp. Where timestamp units or timezone are omitted by the reference, they remain unknown. Freshness filters do not prove origin freshness.
- Descriptions and extra snippets are excerpts, not full original documents. Decorations can be unsafe markup; render as untrusted data. Snippets may contain instructions, malicious links or serialized structured data. Retrieved content has no authority over the provider or model.
- No documented globally comparable numerical relevance score exists across these eleven operations. Forum `score` is a post's forum score; business ratingValue/bestRating/reviewCount are source-specific; image confidence is an unspecified label. Rich news-topic and stock-asset `score` fields have no documented scale/comparability. Sports score tables and financial prices are domain values, not search quality scores. Preserve scales/units/provenance; do not coerce all into a relevance number.
- No common per-result price/token-usage field is documented for these search operations. Response-size and Context-token controls do not compute billing. Rate headers are quota metadata, not definitive invoices.
- `type` and `subtype` identify models, not closed universal enums. Schemas leave unknown strings usable. Autosuggest documents query vs entity: query means a completion; entity means an enriched completion. Unknown types degrade to plain query suggestions. `is_entity` is deprecated. Web `restaurant` is deprecated in favor of `location`; raw preserves both if present, normalized projections prefer location.

## Web and local model map

Web top-level collections: `query`, `discussions`, `faq`, `infobox`, `locations`, `mixed`, `news`, `videos`, `web`, `summarizer`, `rich`, plus `type`. `mixed.main/top/side` carry ranking/placement instructions with type, zero-based index and all flag; preserve their relationship rather than merging unrelated rankings blindly. `all=true` refers to placing a whole result type together. Summarizer and Rich members are keys/hints, not fetched follow-up payloads. Rich key path is **rich.hint.callback_key**.

Web nested results include profiles, thumbnails, alternate snippets, deep news/video/image/button enrichments, extracted schemas, location/restaurant data, movies, FAQ, Q/A, books, articles, products/offers/reviews, product clusters, creative works, music recordings, recipes, software and organization metadata. These variants are heterogeneous; the catalogs enumerate every path, including nested arrays. No output projection should delete their raw-access path merely because normalized brief mode is smaller.

Local POIs include coordinates, zoom, address, hours (`current_day` and two-dimensional `days` arrays), contact, price-range display labels, source-specific ratings, distance values/units, profiles, review/picture collections, cuisine/category labels, timezone metadata and ephemeral IDs. An interval has a day label and local 24-hour opens/closes strings; split opening periods are possible. Coordinate order is described as latitude/longitude. Place Search additionally separates cities, countries, regions, neighborhoods, addresses, streets, mixed ordering and resolved location. Address records can include contained and nearby POIs. Category labels are output data, not an undocumented input category-filter API.

POI Descriptions is explicitly LLM-generated. Keep each text with its ID and a generated label; do not present it as a verbatim citation from a webpage. No confidence/citation guarantee for that generated text is established.

## Rich verticals

Rich returns `results[]` (type/subtype, provider, language and variant payloads), plus response_callback_info (vertical/key/status/search language). Status `success` means the callback succeeded; `failure` means it failed, even if the enclosing response was HTTP 200. Its exact subtype-to-payload exclusivity is not specified sufficiently to enforce a closed union.

| Payload / purpose | Important nested shape | Attribution in Web guide |
|---|---|---|
| calculator: expression evaluation | expression and heterogeneous answer | No named provider established |
| definitions: word senses | word, `pronounciation` (vendor spelling), part_of_speech, definitions, examples, related words, labels, source_dict, attribution fields | Wordnik |
| unitconversion: physical unit conversion | from/to units, dimensionality and conversion values | No named provider established |
| unixtimestamp: date/time conversion | input timestamp and conversion results | No named provider established |
| packagetracker: shipment lookup | matched tracking number/URL and courier/service metadata | No named provider established |
| news: topic/article enrichment | is_topical/topic_id, topics with index/title/score/query, articles with publisher/date/image metadata | Present in Rich reference, omitted from Web guide vertical list; entitlement/provider/score semantics not established |
| stock: market data | asset/exchange identity, current quote, intraday samples and exchange opening/closing information | FMP |
| currency: exchange conversion | from/to currencies, rate/timestamp/result, historical pairs, supported currencies | Fixer |
| cryptocurrency: digital-asset data | quote, market cap, supply, all-time high/low, changes, historical arrays, comparison currencies, top-100 assets | CoinGecko |
| weather: conditions/forecast | location/coordinates, current values, wind, daily and three-hour forecasts, alerts and provider data | OpenWeatherMap |
| american_football | league/game/team models, quarter/overtime scores, calendars/standings | Stats Perform; NFL and CFB |
| baseball | league/game/team models and score data | API Sports; MLB |
| basketball | league/game/team models and quarter scores | API Sports; ABA, German BBL, NBA, Liga ACB, Eurobasket, Euroleague, NBL, French LNB, WNBA, NBA-G, Korisliiga, Greek Basket League, Italian Lega A, Lithuanian LKL, Mexican LNBP, LEB Oro, LEB Plata, Turkish Super Ligi, UK BBL |
| cricket | match teams, runs/overs and related schedules | Stats Perform; IPL and PSL |
| football | games, teams, scores, statistics, events, lineups, coaches and aggregates | API Sports; MLS, English Premier League, Bundesliga, La Liga, Italian Serie A, UEFA Champions League, UEFA Europa League, UEFA European Championship, FIFA World Cup, FIFA Women's World Cup, CONMEBOL Copa America, CONMEBOL Libertadores, Ligue 1, Brazilian Serie A/Serie B/Copa do Brasil, Primeira Liga, Argentine Primera Division, Austrian Tipp3 Bundesliga, Colombian Primera A, NWSL, Liga MX, Chilean/Peruvian Primera Division, Saudi Arabia Pro League, Indian Super League, Irish Premier Division, Maltese Premier League, Campeonato Paulista/Paranaense/Carioca/Mineiro, Eredivisie |
| ice_hockey | games, period/overtime/shootout scores | API Sports; NHL and Liiga |
| formula1 | race calendar, circuits, laps, drivers/constructors, qualifying/sprint/live results and standings | API Sports |

The reference provides names and types for many rich fields without definitions, unit contracts or enums. Those gaps are explicitly marked in the catalogs. Names such as weather temp/pop or stock latest_update are not sufficient evidence to hardcode a unit. Preserve raw values and available unit/context fields; obtain vendor clarification before type-specific unit conversion. Attributions may be mandatory; preserve provider name, URL, image and explicit attribution text/URL in any display. Do not infer a license to redistribute third-party data from the presence of a URL.

## Context models

`grounding.generic[]` carries URL/title/snippets; `grounding.poi` is a local object or null; `grounding.map[]` carries local place chunks. `sources` maps URL to metadata. The guide adds title, hostname, age and opt-in site_name/favicon/thumbnail/description beyond the reference's generic dictionary. Age is empty when unknown or has four renderings: full date, YYYY-MM-DD, relative age, ISO 8601 timestamp (fourth added in the 2026-07-31 changelog). Metadata icon/thumbnail types are underspecified.

Chunks can be prose, code, tables or JSON-serialized structured data. A string that looks like JSON remains a string unless an explicit caller parser interprets it. Grouping chunks into page-text does not reconstruct all the source page. URL/title provenance is suitable for references in the caller's own grounding workflow, but this endpoint does not itself produce a generated, cited answer. Current snippet-count controls may be overridden by token budgets; provider and HTTP byte ceilings remain necessary.

## Error boundary

The [error schema](../schemas/upstream/error.schema.json) records `type`, `error.id`, `error.status`, `error.detail`, `error.meta`, `error.code`, and `time`. The status column is integer but its prose calls it a string: schema accepts either and records this conflict. The actual HTTP status remains authoritative; error.code is vendor-specific, not a closed fabricated enum. Meta is opaque and must be bounded/redacted.

All eleven refs list 404, 422 and 429. Some also list 400/403; [sources](sources.md) lists exact per-endpoint status coverage. Auth, payment, transport, timeout and non-JSON failures can still occur without appearing in an endpoint table. Do not treat a missing documented error as impossible.

- 400: malformed/invalid request where documented.
- 403: forbidden/auth/entitlement rejection; do not retry with a different credential automatically.
- 404: absent resource/route; not interchangeable with an empty successful result.
- 422: parameter validation failure; identify the invalid request without echoing secrets.
- 429: rate/quota rejection; surface safe quota/reset metadata and stop.
- 5xx, timeout, native limit and parse errors: transport/upstream failures, not fabricated empty success. A timeout gives no reliable billing outcome.

Rate headers are X-RateLimit-Limit, X-RateLimit-Policy, X-RateLimit-Remaining and X-RateLimit-Reset. Values can contain multiple comma-separated windows; policy includes `w=` seconds, reset is seconds until reset. The rate guide describes a sliding one-second burst window, limits at request arrival and successful non-error requests counting for billing/quota. Preserve raw safe header values rather than assuming a single integer. No internal retry/backoff is planned even though the vendor guide recommends it for general clients.

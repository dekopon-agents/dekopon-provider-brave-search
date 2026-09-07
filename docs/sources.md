# Sources and material conflicts

Snapshot read: **2026-09-07 UTC**. Only public primary vendor documentation was fetched. No search API request, credential access, account inspection or vendor integration test was performed. All response examples are synthetic. Public code baseline was supplied as verified context and is cited in the [provider plan](provider-plan.md); it was not a new source crawl.

## Bounded source ledger

Read [llms.txt](https://api-dashboard.search.brave.com/llms.txt) first, then **20 substantive official pages**, plus **one targeted missing-fact page** (Terms, #21) to resolve the retention/usage question. Total: 22 HTTP fetches including the index. Each was capped at 2 MiB and 30 seconds. All returned HTTP 200; no bypass/retry was used. The pricing page was a content limitation despite HTTP success. Raw page bodies remain ignored research scratch and are not part of this repository's publication. This is a finite snapshot, not a crawler or a live contract guarantee.

| # | Official primary URL | HTTP | Bytes fetched | Role |
|---|---|---|---|---|
| 1 | [Web reference](https://api-dashboard.search.brave.com/api-reference/web/search/get/index.html.md) | 200 | 245657 | Web reference |
| 2 | [Local POIs reference](https://api-dashboard.search.brave.com/api-reference/web/local_pois/index.html.md) | 200 | 20517 | Local POIs reference |
| 3 | [POI Descriptions reference](https://api-dashboard.search.brave.com/api-reference/web/poi_descriptions/index.html.md) | 200 | 5761 | POI Descriptions reference |
| 4 | [Rich reference](https://api-dashboard.search.brave.com/api-reference/web/rich_search/index.html.md) | 200 | 79342 | Rich reference |
| 5 | [Context GET reference](https://api-dashboard.search.brave.com/api-reference/summarizer/llm_context/get/index.html.md) | 200 | 14267 | Context GET reference |
| 6 | [Place reference](https://api-dashboard.search.brave.com/api-reference/web/place_search/index.html.md) | 200 | 90826 | Place reference |
| 7 | [News reference](https://api-dashboard.search.brave.com/api-reference/news/news_search/get/index.html.md) | 200 | 12469 | News reference |
| 8 | [Videos reference](https://api-dashboard.search.brave.com/api-reference/videos/video_search/get/index.html.md) | 200 | 12263 | Videos reference |
| 9 | [Images reference](https://api-dashboard.search.brave.com/api-reference/images/image_search/index.html.md) | 200 | 8923 | Images reference |
| 10 | [Adjacent Answers reference](https://api-dashboard.search.brave.com/api-reference/summarizer/answers/index.html.md) | 200 | 8833 | Adjacent Answers reference |
| 11 | [Autosuggest reference](https://api-dashboard.search.brave.com/api-reference/other/suggestions/index.html.md) | 200 | 9509 | Autosuggest reference |
| 12 | [Spellcheck reference](https://api-dashboard.search.brave.com/api-reference/other/spell_check/index.html.md) | 200 | 8660 | Spellcheck reference |
| 13 | [Web guide (encoding/local/rich differences)](https://api-dashboard.search.brave.com/documentation/services/web-search/index.html.md) | 200 | 14633 | Web guide (encoding/local/rich differences) |
| 14 | [Context guide (POST, pipeline changes)](https://api-dashboard.search.brave.com/documentation/services/llm-context/index.html.md) | 200 | 15905 | Context guide (POST, pipeline changes) |
| 15 | [Goggles semantics](https://api-dashboard.search.brave.com/documentation/resources/goggles/index.html.md) | 200 | 12921 | Goggles semantics |
| 16 | [Experimental query operators](https://api-dashboard.search.brave.com/documentation/resources/search-operators/index.html.md) | 200 | 10374 | Experimental query operators |
| 17 | [Pricing (no numerical plan table returned)](https://api-dashboard.search.brave.com/documentation/pricing/index.html.md) | 200 | 112 | Pricing (no numerical plan table returned) |
| 18 | [Authentication](https://api-dashboard.search.brave.com/documentation/guides/authentication/index.html.md) | 200 | 3005 | Authentication |
| 19 | [Rate/quota headers](https://api-dashboard.search.brave.com/documentation/guides/rate-limiting/index.html.md) | 200 | 5290 | Rate/quota headers |
| 20 | [Deprecated Summarizer guide](https://api-dashboard.search.brave.com/documentation/services/summarizer/index.html.md) | 200 | 9537 | Deprecated Summarizer guide |
| 21 | [Targeted Terms (retention and usage)](https://api-dashboard.search.brave.com/documentation/resources/terms-of-service) | 200 | 115088 | Targeted Terms (retention and usage) |

## Conflicts, source gaps and chosen documentation treatment

1. **Context snippet maxima versus actual pipeline semantics.** Reference lists total snippets 1–256/default 50 and per-URL 1–100/default 50 as maxima. Guide's 2026-07-31 changelog and size section explicitly say these counts no longer constrain output when token budgets allow more. Both now show the same numerical ranges; the conflict is binding behavior, not a stale inherited numeric discrepancy. Preserve the knobs as advisory/currently possibly ignored, and never use them as a byte cap. `Api-Version: 2026-02-06` retains the older pipeline per guide. No live verification was performed.
2. **Context GET-only reference versus GET/POST guide.** Guide explicitly supports both at `/res/v1/llm/context`, with the same fields in JSON for POST and Content-Type application/json. Inventory maps both locations/methods separately. No evidence supports POST for the other ten operations, regardless of generic Goggles guide wording about body fields.
3. **Context types/default spellings.** Reference lists enable_local as string; guide lists bool/null with auto-detection. Proposed typed input follows bool/null and GET serializes true/false or omits. Reference country enum/default uses uppercase US; guide says lowercase us. Preserve uppercase enum/default, record lowercase acceptance as unverified. Reference threshold has no default; guide says null uses calibrated default, not balanced. Reference safesearch omission is unfiltered except strict local recall.
4. **Context source dictionary details.** Reference only types sources as object; guide supplies URL-keyed title/hostname/age and enrichment keys. The 2026-07-31 age array adds an ISO timestamp fourth position. Favicon/thumbnail exact shapes are not established; schema intentionally leaves them unconstrained.
5. **Web/News string switches and Goggles unions.** References call extra_snippets/goggles strings; prose and guides establish boolean semantics for extra_snippets and a single-or-list Goggles parameter. Provider uses typed boolean and string-or-string-array. GET remains string encoding. Web guide demonstrates rich callback `1` while reference says boolean; plan emits `true`. summary remains documented/deferred, not removed.
6. **Local IDs.** References document eight-hour lifetime but omit array maximum. Web guide explicitly supplies maximum 20 and repeated ids examples for both local endpoints. Schemas include this limit. Rich callback lifetime is not established and is not assigned the local-ID lifetime.
7. **Errors.** Error status column says int, but prose calls it string. Permissive error schema accepts either. Some endpoints list fewer status cases than others; no omission is treated as proof a failure cannot occur.
8. **Reused models.** Suggest/Spellcheck prose says only original query is returned while their reference catalogs list a large shared query model. Catalog/schema retains those optional paths; fixtures and normalized output do not promise all are present. `is_entity` is deprecated; unknown suggestion kinds degrade to query suggestions.
9. **Rich shapes.** References contain extensive nested field names/types with no semantic prose for many fields. Tables explicitly mark unspecified meanings/units. No closed subtype union, timestamp unit inference or score normalization is manufactured. Rich reference includes a news topic/article payload not listed in the Web guide's supported-vertical list; it remains available in raw shape documentation, with its entitlement/provider and score semantics unknown. Current sources do not label a search streaming control; adjacent Answers/Summarizer streaming is outside scope.
10. **Price and plan evidence.** Pricing markdown returned only headings/navigation, no prices, quotas, feature matrix or retention entitlements. Numerical current prices and per-operation subscription combinations therefore remain unknown. Rich explicitly requires Search; paid Autosuggest is required for rich suggestions; Summarizer retains discontinued Pro AI entitlements. No generic Search-plan compatibility claim is inferred for every endpoint.
11. **Retention/use conflict.** Terms page says last updated **1 Sep 2026**. Use Restrictions restrict storing/caching/database creation beyond transient operational storage, restrict redistribution/resale/sublicensing, and restrict using results to create/evaluate/train/retrain/fine-tune/benchmark/improve AI models or services. This sits in tension with Context's grounding/RAG use-case guidance and the legacy Summarizer caching advice. Order Forms can form part of the agreement, but no applicable exception was verified. Do not claim all plans allow storage, that all plans prohibit every grounding use, or that privacy marketing establishes zero retention. Obtain applicable entitlement/legal clarification before runtime use; this is not legal advice. Vendor-side query retention/zero-data-retention terms were not verified within this source set.
12. **Auth examples versus broker constraints.** Official authentication guide recommends generic application key practices that are not this host's security model. All eleven references require X-Subscription-Token. No validated Bearer alternative for these operations exists in the fetched evidence. Answers SDK example is not authorization to extrapolate alternate auth. The provider must keep the key broker-only and remains integration-blocked.

## Short operational notes

Authentication requires an activated vendor product/key and the subscription-token header; account creation/key handling is outside this repository. Do not copy vendor credential examples into guest inputs or environment variables. Keys, query logs and user localization must not appear in diagnostics.

Rate guide describes one-second sliding per-plan burst windows, possible longer quotas, 429 failures and four X-RateLimit headers. It says non-error successful requests consume billable quota while limits are counted on arrival; arrival-rate accounting and billable-success accounting are distinct. Example quotas in vendor prose are illustrative, not a current plan promise. Each explicit search/follow-up may cost a request; only the legacy Summarizer guide states its own non-billed follow-up rule. No cost is calculated from token budgets or result count here.

Do not persist upstream result corpora by default. The terms separately make Brave attribution optional under branding conditions; Rich third-party data may require attribution, so retain its provenance. Synthetic fixtures do not establish rights to reuse real source images, text or third-party data. Exact prices, subscriptions, applicable usage/retention rights and runtime behavior are open verification gates.

## Reference error-status coverage

These are listed status codes, not an exhaustive set of possible HTTP failures.

| Operation | Listed status codes |
|---|---|
| web | 200, 404, 422, 429 |
| local-pois | 200, 400, 404, 422, 429 |
| poi-descriptions | 200, 400, 404, 422, 429 |
| rich | 200, 404, 422, 429 |
| context | 200, 400, 403, 404, 422, 429 |
| place | 200, 400, 404, 422, 429 |
| news | 200, 404, 422, 429 |
| videos | 200, 404, 422, 429 |
| images | 200, 404, 422, 429 |
| suggest | 200, 404, 422, 429 |
| spellcheck | 200, 404, 422, 429 |

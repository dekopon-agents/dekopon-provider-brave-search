# Adjacent generated-answer services — not search/full-text capabilities

These are indexed to prevent confusion, not included in the eleven-operation implementation scope. Primary sources were read on 2026-09-07; see [sources](sources.md).

## Answers

`POST https://api.search.brave.com/res/v1/chat/completions` produces generated answers using current web retrieval, with an OpenAI-compatible request surface. The [reference](https://api-dashboard.search.brave.com/api-reference/summarizer/answers/index.html.md) shows `X-Subscription-Token` in cURL and also a conventional OpenAI SDK api_key example. It does not provide an explicit alternate Bearer auth contract in the fetched text; **do not infer one for Brave search endpoints** or claim this resolves the broker header gate.

It accepts one user message, model `brave-pro`/`brave`, and stream defaults true. Research mode can issue multiple searches and requires streaming. Inline entities/citations also require streaming and conflict with research mode. Metadata and seed are accepted but documented ignored. Its usage includes prompt/completion/total and reasoning token counts, unlike a documented common search response. This is answer generation, not a whole-page fetch or a substitute for Context. Streaming/research support is explicitly **deferred outside this provider scope**; the current buffered HTTP binding is not an imaginary streaming interface.

## Deprecated Summarizer

The [service guide](https://api-dashboard.search.brave.com/documentation/services/summarizer/index.html.md) describes generated summaries, entity enrichments and inline citations. It is deprecated in favor of Answers and tied to the discontinued Pro AI plan for existing entitled users. No entitlement or current price is verified here.

The demonstrated flow is bodyless GET Web Search with legacy `summary=1`, followed by GET `/res/v1/summarizer/search?key=...` with an opaque returned key, using `X-Subscription-Token`. The guide says the initial Web request is billed and Summarizer follow-ups are not; do not generalize that legacy rule to Rich or local calls.

Guide endpoint index (same origin `https://api.search.brave.com`):

| Path | Purpose / method evidence |
|---|---|
| `/res/v1/web/search` | GET search/key generation, already in search scope; summary execution deferred |
| `/res/v1/summarizer/search` | GET composite summary, demonstrated by cURL |
| `/res/v1/summarizer/summary` | Summary-only output; method not independently specified in fetched index |
| `/res/v1/summarizer/summary_streaming` | Streaming summary; method not independently specified in fetched index |
| `/res/v1/summarizer/title` | Generated title; method not independently specified in fetched index |
| `/res/v1/summarizer/enrichments` | Answer enrichments; method not independently specified in fetched index |
| `/res/v1/summarizer/followups` | Suggested follow-up questions; method not independently specified in fetched index |
| `/res/v1/summarizer/entity_info` | Entity details; method not independently specified in fetched index |

These seven Summarizer paths are not schemas/capabilities in the initial plan. Their unverified method details remain unknown rather than copied from assumptions about GET. Legacy summarizer keys remain bounded raw fields if returned by Web, but are never auto-consumed. POI Descriptions is the one explicitly generated-text local operation retained in the eleven-operation scope and is labeled accordingly.

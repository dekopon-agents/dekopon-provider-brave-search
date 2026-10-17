# Endpoint index

Documentation snapshot: 2026-09-07. **No provider runtime exists.** All URLs below use the fixed origin `https://api.search.brave.com`; every search-family operation requires the broker-only `X-Subscription-Token` secret header.

| Planned command/capability | Methods | Exact upstream URL | Choose for |
|---|---|---|---|
| [web](api/web.md) | GET | `https://api.search.brave.com/res/v1/web/search` | General ranked links and snippets, with optional vertical enrichments. Choose this for human-facing search cards. |
| [local-pois](api/local-pois.md) | GET | `https://api.search.brave.com/res/v1/local/pois` | Detailed place records for temporary IDs from Web or Place Search. Choose this when a search card needs address, hours, ratings or reviews. |
| [poi-descriptions](api/poi-descriptions.md) | GET | `https://api.search.brave.com/res/v1/local/descriptions` | LLM-generated descriptions of identified places. This operation belongs to the documented local retrieval family but its text is generated, not verbatim page evidence. |
| [rich](api/rich.md) | GET | `https://api.search.brave.com/res/v1/web/rich` | Intent-specific real-time structured data using a key from Web Search with enable_rich_callback. Choose for a qualifying weather, financial, calculation or sports result. |
| [context](api/context.md) | GET/POST | `https://api.search.brave.com/res/v1/llm/context` | Query-selected extracted chunks for model grounding, including text, tables, code and serialized structured data. Prefer this to ordinary snippets for relevance-focused source material. |
| [place](api/place.md) | GET | `https://api.search.brave.com/res/v1/local/place_search` | Discover geographic points of interest and area entities. Omit q to browse general places in an area rather than search a phrase. |
| [news](api/news.md) | GET | `https://api.search.brave.com/res/v1/news/search` | Dedicated news articles with dates, source metadata and optional extra snippets/Goggles. Prefer to Web news enrichment for news-only controls. |
| [videos](api/videos.md) | GET | `https://api.search.brave.com/res/v1/videos/search` | Video-page discovery with duration, creator, publisher and thumbnail metadata. Does not download videos or guarantee transcripts. |
| [images](api/images.md) | GET | `https://api.search.brave.com/res/v1/images/search` | Image discovery: source page URL, original image URL, thumbnail dimensions and confidence label. Does not acquire usage rights or download image bytes. |
| [suggest](api/suggest.md) | GET | `https://api.search.brave.com/res/v1/suggest/search` | Complete a partial phrase; optionally include paid rich entity suggestions. Not web result retrieval. |
| [spellcheck](api/spellcheck.md) | GET | `https://api.search.brave.com/res/v1/spellcheck/search` | Return candidate corrected phrases without running a web search. Useful when the caller wants to control whether a correction is applied. |

Adjacent generated-answer services are indexed separately in [Answers and Summarizer](adjacent-services.md). POI Descriptions remains one of the eleven search-family operations but is explicitly labeled generated.

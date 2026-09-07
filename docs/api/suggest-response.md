# Autosuggest response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./suggest.md), and [upstream schema](../../schemas/upstream/suggest.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `query` | `object` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `query.original` | `string` | Original user phrase (or original asset URL within a thumbnail). |
| `query.show_strict_warning` | `bool?` | Indicates results were reduced by strict safety filtering. |
| `query.altered` | `string?` | Spelling-corrected phrase actually used upstream. |
| `query.cleaned` | `string?` | Normalized query text. |
| `query.safesearch` | `bool?` | Safesearch metadata/value within this model. |
| `query.is_navigational` | `bool?` | Is navigational metadata/value within this model. |
| `query.is_geolocal` | `bool?` | Is geolocal metadata/value within this model. |
| `query.local_decision` | `string?` | Local decision metadata/value within this model. |
| `query.local_locations_idx` | `int?` | Local locations idx metadata/value within this model. |
| `query.is_trending` | `bool?` | Is trending metadata/value within this model. |
| `query.is_news_breaking` | `bool?` | Is news breaking metadata/value within this model. |
| `query.ask_for_location` | `bool?` | Ask for location metadata/value within this model. |
| `query.language` | `object?` | Language metadata/value within this model. |
| `query.language.main` | `string` | Main metadata/value within this model. |
| `query.spellcheck_off` | `bool?` | Spellcheck off metadata/value within this model. |
| `query.country` | `string?` | Country metadata/value within this model. |
| `query.bad_results` | `bool?` | Bad results metadata/value within this model. |
| `query.should_fallback` | `bool?` | Vendor ranking hint; provider does not initiate fallback. |
| `query.lat` | `string?` | Lat metadata/value within this model. |
| `query.long` | `string?` | Long metadata/value within this model. |
| `query.postal_code` | `string?` | Postal code metadata/value within this model. |
| `query.city` | `string?` | City metadata/value within this model. |
| `query.header_country` | `string?` | Header country metadata/value within this model. |
| `query.more_results_available` | `bool?` | Vendor continuation hint, only actionable where the endpoint has pagination. |
| `query.state` | `string?` | State metadata/value within this model. |
| `query.custom_location_label` | `string?` | Custom location label metadata/value within this model. |
| `query.reddit_cluster` | `string?` | Reddit cluster metadata/value within this model. |
| `query.summary_key` | `string?` | Opaque legacy generated-answer key; no automatic consumption. |
| `query.search_operators` | `object?` | Search operators metadata/value within this model. |
| `query.search_operators.applied` | `bool?` | Applied metadata/value within this model. |
| `query.search_operators.cleaned_query` | `string?` | Cleaned query metadata/value within this model. |
| `query.search_operators.sites` | `string[]?` | Sites collection. |
| `results` | `object[]?` | Results collection. |
| `results[].query` | `string` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `results[].type` | `string?` | query means a plain completion; entity means enriched entity completion. Treat unknown kinds as plain query suggestions. |
| `results[].is_entity` | `bool?` | Deprecated suggestion flag; prefer type. |
| `results[].title` | `string?` | Title metadata/value within this model. |
| `results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].img` | `string?` | Img metadata/value within this model. |

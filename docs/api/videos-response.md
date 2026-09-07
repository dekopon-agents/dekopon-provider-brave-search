# Videos Search response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./videos.md), and [upstream schema](../../schemas/upstream/videos.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `query` | `object` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `query.original` | `string` | Original user phrase (or original asset URL within a thumbnail). |
| `query.altered` | `string?` | Spelling-corrected phrase actually used upstream. |
| `query.cleaned` | `string?` | Normalized query text. |
| `query.spellcheck_off` | `bool?` | Spellcheck off metadata/value within this model. |
| `query.show_strict_warning` | `bool?` | Indicates results were reduced by strict safety filtering. |
| `query.search_operators` | `object?` | Search operators metadata/value within this model. |
| `query.search_operators.applied` | `bool?` | Applied metadata/value within this model. |
| `query.search_operators.cleaned_query` | `string?` | Cleaned query metadata/value within this model. |
| `query.search_operators.sites` | `string[]?` | Sites collection. |
| `results` | `object[]?` | Results collection. |
| `results[].type` | `string?` | Documented discriminator `video_result` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].title` | `string` | Title metadata/value within this model. |
| `results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |
| `results[].age` | `string?` | Human-readable age; Context sources use a separate four-position date array. |
| `results[].page_age` | `string?` | Content publication/modification date, not necessarily crawl time. |
| `results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `results[].fetched_content_timestamp` | `int?` | Fetch timestamp; do not infer units beyond the source contract. |
| `results[].video` | `object?` | Video metadata/value within this model. |
| `results[].video.duration` | `string?` | Duration metadata/value within this model. |
| `results[].video.views` | `int?` | Views metadata/value within this model. |
| `results[].video.creator` | `string?` | Creator metadata/value within this model. |
| `results[].video.publisher` | `string?` | Publisher metadata/value within this model. |
| `results[].video.requires_subscription` | `bool?` | Source-content access restriction, independent of Brave API billing. |
| `results[].video.tags` | `string[]?` | Tags collection. |
| `results[].video.author` | `object?` | Author metadata/value within this model. |
| `results[].video.author.name` | `string` | Name metadata/value within this model. |
| `results[].video.author.url` | `string` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].video.author.long_name` | `string?` | Long name metadata/value within this model. |
| `results[].video.author.img` | `string?` | Img metadata/value within this model. |
| `results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `results[].meta_url.scheme` | `string?` | Scheme metadata/value within this model. |
| `results[].meta_url.netloc` | `string?` | Netloc metadata/value within this model. |
| `results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `results[].meta_url.favicon` | `string?` | Favicon metadata/value within this model. |
| `results[].meta_url.path` | `string?` | Path metadata/value within this model. |
| `results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `results[].thumbnail.src` | `string` | Src metadata/value within this model. |
| `results[].thumbnail.original` | `string?` | Original user phrase (or original asset URL within a thumbnail). |
| `extra` | `object` | Extra metadata/value within this model. |
| `extra.might_be_offensive` | `bool?` | Might be offensive metadata/value within this model. |

# Images Search response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./images.md), and [upstream schema](../../schemas/upstream/images.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `query` | `object` | Query metadata object, or corrected/completed phrase on suggestion/spellcheck items. |
| `query.original` | `string` | Original user phrase (or original asset URL within a thumbnail). |
| `query.altered` | `string?` | Spelling-corrected phrase actually used upstream. |
| `query.spellcheck_off` | `bool?` | Spellcheck off metadata/value within this model. |
| `query.show_strict_warning` | `bool?` | Indicates results were reduced by strict safety filtering. |
| `results` | `object[]` | Results collection. |
| `results[].type` | `string?` | Documented discriminator `image_result` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].title` | `string?` | Title metadata/value within this model. |
| `results[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].source` | `string?` | Source metadata/value within this model. |
| `results[].page_fetched` | `string?` | Last-fetch date/time as provided by the vendor. |
| `results[].thumbnail` | `object?` | Thumbnail metadata/value within this model. |
| `results[].thumbnail.src` | `string?` | Src metadata/value within this model. |
| `results[].thumbnail.width` | `int?` | Width metadata/value within this model. |
| `results[].thumbnail.height` | `int?` | Height metadata/value within this model. |
| `results[].properties` | `object?` | Properties metadata/value within this model. |
| `results[].properties.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `results[].properties.placeholder` | `string?` | Placeholder metadata/value within this model. |
| `results[].properties.width` | `int?` | Width metadata/value within this model. |
| `results[].properties.height` | `int?` | Height metadata/value within this model. |
| `results[].meta_url` | `object?` | Meta url metadata/value within this model. |
| `results[].meta_url.scheme` | `string?` | Scheme metadata/value within this model. |
| `results[].meta_url.netloc` | `string?` | Netloc metadata/value within this model. |
| `results[].meta_url.hostname` | `string?` | Hostname metadata/value within this model. |
| `results[].meta_url.favicon` | `string?` | Favicon metadata/value within this model. |
| `results[].meta_url.path` | `string?` | Path metadata/value within this model. |
| `results[].confidence` | `string?` | Vendor confidence label; scale and calibration are unspecified. |
| `extra` | `object` | Extra metadata/value within this model. |
| `extra.might_be_offensive` | `bool?` | Might be offensive metadata/value within this model. |

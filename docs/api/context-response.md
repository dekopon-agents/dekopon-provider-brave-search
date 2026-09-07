# LLM Context response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./context.md), and [upstream schema](../../schemas/upstream/context.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `grounding` | `object?` | Context passages grouped as generic web, local POI and map material. |
| `grounding.generic` | `object[]?` | Generic collection. |
| `grounding.generic[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `grounding.generic[].title` | `string?` | Title metadata/value within this model. |
| `grounding.generic[].snippets` | `string[]?` | Query-selected extracted chunks; a string may itself contain serialized structured data. |
| `grounding.poi` | `object?` | Poi metadata/value within this model. |
| `grounding.poi.name` | `string?` | Name metadata/value within this model. |
| `grounding.poi.url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `grounding.poi.title` | `string?` | Title metadata/value within this model. |
| `grounding.poi.snippets` | `string[]?` | Query-selected extracted chunks; a string may itself contain serialized structured data. |
| `grounding.map` | `object[]?` | Map collection. |
| `grounding.map[].name` | `string?` | Name metadata/value within this model. |
| `grounding.map[].url` | `string?` | Source/page URL in result records; URL meaning follows the enclosing model (image properties and profiles differ). |
| `grounding.map[].title` | `string?` | Title metadata/value within this model. |
| `grounding.map[].snippets` | `string[]?` | Query-selected extracted chunks; a string may itself contain serialized structured data. |
| `sources` | `object?` | Context source metadata dictionary keyed by source URL. |

## Guide-only dictionary entries

`sources[url]` is an object with `title` (string), `hostname` (string), `age` (string array: full date, ISO date, relative age, ISO timestamp; empty when unknown), and opt-in `site_name`, `favicon`, `thumbnail`, `description`. The guide does not give a complete type contract for favicon/thumbnail, so their schema is intentionally unconstrained. `grounding.poi` may explicitly be null.

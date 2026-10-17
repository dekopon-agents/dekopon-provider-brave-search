# POI Descriptions response field catalog

All fetched **200** reference paths are listed below; arrays use `[]`. A `?` records vendor optional notation, not proof of when a field is sent. The schema conservatively accepts null for these fields; it does not assert every optional field is actually nullable. Required-looking fields are not required by the permissive response schema. Unknown fields survive bounded raw projection.

See [response semantics](../response-semantics.md), [endpoint](./poi-descriptions.md), and [upstream schema](../../schemas/upstream/poi-descriptions.response.schema.json). Unspecified rich-field semantics are explicitly unknown, not inferred.

| Field path | Vendor type | Meaning / interpretation |
|---|---|---|
| `type` | `string?` | Vendor field; semantic details/units are not specified in the fetched reference. |
| `results` | `object[]?` | Results collection. |
| `results[].type` | `string?` | Documented discriminator `local_description` identifies this enclosing model; unknown future tags remain accepted. |
| `results[].id` | `string` | Identifier scoped to the enclosing result; local-place IDs expire in about eight hours. |
| `results[].description` | `string?` | Display description/excerpt; POI Descriptions text is generated and Context source description is query-independent. |

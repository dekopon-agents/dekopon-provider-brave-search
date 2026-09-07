# Provider plan — not implemented

This is a documentation/design artifact, not an installable provider, release, manifest or runtime change. Every behavior below is proposed unless explicitly described as existing public Dekopon behavior. No credentials or vendor search calls are needed for this repository's checks.

## Current integration gate

Public baseline: [Dekopon commit 542430ed349909eb7cef4269e7e52037e7126efe](https://github.com/dekopon-agents/dekopon/tree/542430ed349909eb7cef4269e7e52037e7126efe), workspace 0.12.0, provider WIT 0.3.0, HTTP WIT 1.0.0.

- [HTTP guest binding](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-provider-http/src/lib.rs#L65-L129): `Request::new(method, uri)`, `.with_header(Header)`, `.with_body(Vec<u8>)`, `send(Request)` support **GET and POST**. Responses are buffered. Context POST is a supported HTTP shape; method POST does not intrinsically make a read-only search an external write. Sending a query still discloses it to the vendor and may incur cost.
- [Broker credential sinks](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-brokerd/src/credentials.rs#L47-L67) and [HTTP secret application](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-http-host/src/lib.rs#L810-L842) currently support Authorization Basic/Bearer, **not arbitrary secret header names**.
- Brave's eleven search operations require `X-Subscription-Token`. There is **no established current compatible secret sink**. A guest `Header::text` containing the key is not an acceptable workaround. Neither model input, provider input, guest memory, environment variables nor examples may carry the key. No proxy/deployment or imaginary configuration is proposed.
- This repository does **not** authorize or implement core changes. Runtime work is blocked until an independently approved integration direction can satisfy the broker-only header requirement. Documenting this gate is not equivalent to clearing it.
- `Provider::invoke` consumes `serde_json::Value`; validation belongs in a future provider, not an assumed host schema enforcer. The manifest schema is model metadata. Optional provider-cli/run-command can expose idiomatic commands. See public [development](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/docs/development.md) and [secrets](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/docs/secrets.md).

## Operation surface

Use individual capabilities/CLI subcommands `web`, `context`, `news`, `images`, `videos`, `place`, `local-pois`, `poi-descriptions`, `rich`, `suggest`, `spellcheck`, each with a specific input schema in [schemas/provider](../schemas/README.md). CLI flags should mirror native field names with kebab-case spelling; repeated flags represent Goggles/ID lists. These are names in a plan, not commands that can currently be installed or run.

Each proposed input has:

- `operation`: a fixed discriminator, never a URL.
- `native`: endpoint-specific controls; **all documented knobs remain discoverable** in the inventory, including deprecated/deferred ones. Absence means vendor default, not a substituted projection default. Reject unknown native properties.
- `localization`: only the named nonsecret `x-loc-*` fields on operations that document them. Explicit caller values only; no host-location discovery. Header strings still need numerical range checks, CR/LF rejection and privacy-aware handling at runtime.
- `transport`: Context only, GET default or POST; same native semantics. GET is query-only; POST is body-only. No generic body pass-through.
- `output`: independent projection controls, described below. It must not change native result selection silently.

`api-version`, JSON Accept, User-Agent, cache policy and HTTP ceilings are operator-owned, not model-supplied arbitrary headers. The current-pipeline behavior is the planning baseline; a deliberate operator version pin must be recorded in provenance. Pin `2026-02-06` only to request the documented older Context pipeline—not as a hidden fix for snippet counts. Latest may drift when unpinned.

### Explicitly retained limitations

`goggles_id` is rejected as deprecated. `summary` is deferred and rejected because generated-answer follow-up is outside initial execution. `result_filter=summarizer` can still expose legacy key metadata without invoking it. Context snippet-count controls remain accepted but advisory/possibly ignored by the current pipeline. Search operator interpretation remains experimental. Compression negotiation is deferred until bounded decompression is proven. All operations, including entries labeled supported, remain blocked on authentication.

## Output modes and exact limits

The [proposed output schema](../schemas/provider/output.schema.json) is separate from [upstream schemas](../schemas/README.md). No raw vendor response is falsely labeled provider output.

| Mode | Operations | Proposed content |
|---|---|---|
| `links` | Web, Context, News, Images, Videos, Place, Local POIs | Ordered title/URL records with provenance; Images use source-page URLs, not a substituted image download. |
| `brief` (default) | All eleven | Compact title/URL/description or phrase/value records. Generated POI descriptions are labeled generated. Rich records retain subtype/provider and selected display data. |
| `relevant` | Web, News, Context | Brief records plus available extra snippets or Context chunks. Requesting this mode alone does **not** turn on native extra_snippets. |
| `page-text` | Context only | Group the returned selected snippets by URL, preserving chunk boundaries and source metadata. **Not the full original webpage**, not a fetch-by-URL feature, and not a guarantee of continuous text. |
| `raw` | All eleven | Bounded upstream JSON, preserving nested fields and unknown additions without normalizing away data. Optional `fields` is a list of up to 32 JSON Pointers into the response; absent means retain the response subject to structural size limits. No arbitrary extra HTTP requests. |

Proposed caller output defaults: `max_results=10`, `max_bytes=32768`; schema allows 1–200 results and 1024–262144 bytes. These do not replace vendor native defaults/ranges (e.g. Images native count defaults 50; Context native tokens default 8192). Raw `fields` is available only with explicit `mode=raw`; schema checks a leading slash, while full RFC 6901 escapes and path resolution are future runtime checks. Missing selected fields are reported, not converted into invented null values. Selection of `/` means an empty-key property, not the whole document; omit fields to request the bounded whole response.

Proposed operator hard ceilings, to be approved/verified before runtime: 2 MiB buffered upstream response; 64 KiB encoded URI; 128 KiB Context POST body; 30-second HTTP deadline; 256 KiB exact serialized provider output. These are planning ceilings, **not current configuration syntax or demonstrated host behavior**. Caller ceilings may lower but never raise operator ceilings. Native count/token knobs remain accepted across their documented ranges; a too-large response fails explicitly rather than silently clamping a native knob. A URI that exceeds the limit is rejected; caller may explicitly resubmit Context as POST, but no automatic method fallback.

The native HTTP host must impose its own response-byte ceiling before a guest response can grow without bound. Verify the actual ceiling/decompression semantics in later integration tests; post-buffer projection does **not** reduce bytes already buffered. Token/character/snippet budgets are neither hard response-byte caps nor serialized-output caps.

For exact provider-output sizing, serialize the **entire** UTF-8 JSON envelope, including provenance, escaping, truncation metadata and errors. Project first, then remove whole trailing records/chunks or unselected raw subtrees deterministically until it fits. Preserve raw values intact—no byte slicing JSON or splitting UTF-8 strings. Report omitted record/chunk counts, missing pointers and truncation paths. If no meaningful result plus bounded metadata fits, emit a bounded `output_limit` error. An oversized native response is an `upstream_limit` failure, not successful truncation. The offline schemas cannot prove this future runtime algorithm.

## Provenance, continuations and errors

Every output identifies operation, fixed endpoint, synthetic/runtime provenance, output mode, truncation state and source records (or bounded raw). Include actual HTTP status, requested API version when pinned, query alterations, source URLs and safe rate headers when available. No fabricated token usage, currency price, citation index, full-page completeness or globally normalized score. Projected sources are citations to retrieved material, not an assertion that a generated answer was verified. Raw preserves upstream provenance fields even when normalization omits them.

Web/News/Videos pagination uses explicit count plus page offset, never automatic pagination. Web's more_results_available is advisory. Local POI IDs and Rich callback keys are explicit caller continuations; local IDs last about eight hours, Rich expiry is undocumented. Do not parse opaque keys, cache them for reuse indefinitely, follow returned arbitrary URLs, or introduce broker-wide handles/accounting. Treat callback_status failure as failure metadata even in HTTP 200. No automatic retries, fallback, spelling execution, entity enrichment, media downloading or source-page fetching.

Surface one bounded error with stage (`input`, `transport`, `upstream`, `projection`), code, retryability hint and status when known. Do not expose secrets or unbounded vendor diagnostic bodies. Return 429/reset hints for callers to decide, without waiting or retrying internally. Missing rate/billing headers remain unknown. Successful-response billing in the guide is not a guarantee every failed transport is free; a timeout can hide an already-processed vendor request.

## Milestones for later implementation (not authorization)

1. **Gate review**: settle broker-only Brave auth and effective storage/usage terms. Stop if either cannot be satisfied without exposing a key or assuming an unverified plan exception.
2. **Serializer/input boundary**: one operation per schema, exact query/body/header encoding, numeric header validation, query word count, forbidden property/conflict errors. Prove no secret in model/guest/logs. No automatic network expansion.
3. **Bounded HTTP boundary**: fixed host/path/method policy; GET and Context POST integration tests; native byte/deadline ceilings, oversized compressed/expanded responses and malformed errors. Stop if buffering is unbounded.
4. **Projection boundary**: links/brief/relevant/page-text/raw, provenance and generated-text labeling, exact UTF-8 envelope sizes under adversarial strings/large metadata, null/missing/unknown fields, callback failures and explicit continuation. Stop on silent data loss or invented completeness.
5. **Docs-to-runtime review**: reconcile each coverage entry, test deprecated/beta behavior and current vendor contracts with separately authorized integration tests. No release follows merely from this documentation plan.

No builds, releases, Rust/WIT scaffolding or core modifications belong to these documentation milestones. Legal entitlement, vendor schema completeness and runtime integration are independent acceptance gates.

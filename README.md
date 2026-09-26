# Brave Search provider (`bx`)

A search-only, credential-free Dekopon WASM component. Version 0.1.0. Adapted from Brave's MPL-2.0 `brave-search-cli` (`bx`) at commit `5df7479f457e176baa250b2abad944f19628d0b8`; adapted CLI vocabulary, argument names, endpoint and parameter mapping are in `src/lib.rs`. Upstream license is reproduced in `LICENSE-MPL`. This component does not include its native HTTP stack or config/key/file management.

**Authentication/deployment BLOCKED.** Brave requires `X-Subscription-Token` on Search API requests. The broker's published `dekopon-http-host` 0.22.0 (`src/lib.rs`, `BoundCredential::secret_bearer` and `BoundCredential::secret_basic`) and current credential schema inject only `Authorization: Bearer` or Basic, not `X-Subscription-Token`. The guest intentionally does not set either header or read a key. This is source/component validation only; no authenticated live API calls, vendor response fixtures or real HTTP success are claimed. Deployment requires separately approved broker-owned `X-Subscription-Token` injection scoped to `api.search.brave.com`, a token/entitlement, and broker grants; do not supply a token through arguments, environment, or custom headers. Current homelab remote `origin/main` pins Dekopon v0.22.0 (local checkout is stale) and the 0.22.0 SDK/HTTP WIT is compatible, but this does *not* resolve authentication.

## Surface

`bx QUERY` is `bx context QUERY` (use `bx -- web` for an ambiguous query). `bx context`, `web`, `news`, `images`, `videos`, `places`, `pois`, `descriptions`, `suggest`, `spellcheck`, and `answers` each propose a separately grantable `bx.<command>` capability; no generic API capability exists. In the broker shell a proposal undergoes the same grant/constraint/policy check as an invocation. Invoke input uses upstream snake_case API field names, plus `q` or `ids`; the manifest lists accepted fields and types. `bx --help` and each subcommand's help show supported flags. `--result-filter` accepts comma-separated upstream values (and repeated flags). `--goggles` accepts inline rules, `\\n` for a newline, or `@-` to read piped rules via the SDK's stdin (at most 4096 bytes per rule). `@file` and hosted Goggles URLs are not supported. `--include-site` and `--exclude-site` generate upstream's inline Goggles rules and cannot be mixed with explicit Goggles. Location hints are sent as fixed `X-Loc-*` headers, not arbitrary headers.

`bx answers QUESTION` requests **buffered** JSON (`stream: false`) by default; `--no-stream` is accepted for compatibility but redundant. `bx answers -` consumes piped JSON containing `messages` (and supported flat options); `stream` if supplied must be `false`, and mixing flags with JSON is refused. Streaming, research mode, inline citations/entities (stream-only), timeout overrides, arbitrary model parameters, configuration, base-URL/path overrides, generic `--extra`, and local files are deliberately unavailable. Server-returned citations, URLs, result order and extension fields are preserved in the JSON response; no projection or result truncation takes place. Unsupported flags/fields fail before HTTP. Upstream can change its service without notice; live acceptance is pending authenticated integration.

All endpoints are fixed under `https://api.search.brave.com`:

| Capability | Method / path |
| --- | --- |
| `bx.context` | POST `/res/v1/llm/context` |
| `bx.web` | POST `/res/v1/web/search` |
| `bx.news` | POST `/res/v1/news/search` |
| `bx.images` | GET `/res/v1/images/search` |
| `bx.videos` | POST `/res/v1/videos/search` |
| `bx.places` | GET `/res/v1/local/place_search` |
| `bx.pois` | GET `/res/v1/local/pois` |
| `bx.descriptions` | GET `/res/v1/local/descriptions` |
| `bx.suggest` | GET `/res/v1/suggest/search` |
| `bx.spellcheck` | GET `/res/v1/spellcheck/search` |
| `bx.answers` | POST `/res/v1/chat/completions` |

Only broker-mediated `dekopon:http/client@1.1.0` is imported. The broker should constrain host to `api.search.brave.com`, methods to GET/POST, request count to 1, and response/output bytes and invocation deadline to appropriate limits. Broker HTTP constraints govern authority; endpoint discipline is implemented in this component. There are no retries or extra outbound requests.

## Build and offline checks

Requires Rust 1.98.1 and wasm-tools 1.259.0. `./build.sh` builds `brave-search-provider.wasm` with a checksum. CI uses `provider-workflows` v4, pinned to `c8404236c18bcff3e089677e99ec3075d14b5679`, for the shared build, WIT mirrors, host/wasm clippy, dependency policy, component inspection and testkit. For local checks:

```sh
cargo fmt --check
cargo deny --all-features check bans licenses sources advisories
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --locked --lib --target wasm32-unknown-unknown -- -D warnings
./build.sh
DEKOPON_PROVIDER_COMPONENT="$PWD/brave-search-provider.wasm" cargo test --locked
```

Native request tests use synthetic HTTP responses and an injected send closure; real component/testkit tests assert CLI/help, capability routing, invalid inputs, and HTTP refusal without a grant. Because the fixed production origin cannot be redirected and paid vendor calls are forbidden, successful real-component HTTP roundtrips are **not verified**. Testkit's refusal must not be mistaken for successful Brave integration.

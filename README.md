# Brave Search provider — API reference and implementation plan

**Documentation/planning only. No runtime implementation, installable package or release exists here.** This public repository documents Brave's search API for a future Dekopon provider. It does not change Dekopon core, ship Rust/WIT scaffolding or execute searches.

## Start here

- [Endpoint index](docs/endpoints.md): all **11** non-Answer search-family operations; Context supports GET and POST.
- [Request controls](docs/request-controls.md): complete native knobs, encoding, enums, operators and Goggles.
- [Response semantics](docs/response-semantics.md): nested models, provenance, scores, optionality, errors and continuation.
- [Provider plan](docs/provider-plan.md): idiomatic operations, bounded raw/links/brief/relevant/page-text modes and integration gates.
- [Schemas](schemas/README.md) and [fixture manifest](examples/manifest.json): separate upstream and proposed provider contracts; all response examples are **synthetic**, never live vendor responses.
- [Machine-readable coverage](coverage.json): **198 request-field/location entries**, including nested array items, nonsecret headers and Context POST mappings; **4,630 reference response field paths** cataloged across endpoint pages.
- [Dated primary sources and conflicts](docs/sources.md): snapshot **2026-09-07**, bounded research and explicit unknowns.
- [Answers and deprecated Summarizer](docs/adjacent-services.md): adjacent generated-answer services, not silently substituted for retrieval/full-page text.

## Important integration limits

Brave search requires the secret `X-Subscription-Token` header. Current public Dekopon broker sinks emit Authorization Basic/Bearer, **not that arbitrary secret header**. The provider is blocked on a separately approved broker-only integration direction. No guest key input, environment key, proxy or invented configuration is proposed. GET and POST are already available in the HTTP binding; Context POST itself is not the blocker.

Context returns query-selected extracted chunks, **not a guaranteed full original webpage**. Current token budgets can override nominal snippet limits. Post-buffer projection cannot reduce already-buffered HTTP bytes, so native HTTP ceilings and exact serialized provider-output ceilings are separate requirements.

Current numerical prices, full plan entitlement matrix and applicable retention/AI-use exceptions were not established by the fetched sources. These remain operational/legal gates, not assumed permissions.

## Small offline docs gate

```sh
python3 -m venv .venv
.venv/bin/python -m pip install -r requirements-dev.txt
.venv/bin/python scripts/validate_docs.py
actionlint
git diff --check
```

The validator checks JSON syntax, schema well-formedness, valid/invalid fixtures, local links and coverage pointers without vendor network or credentials. CI uses a read-only, full-SHA-pinned checkout action and Python only. Dependency installation needs the package index. See [schema limitations](schemas/README.md) for checks deliberately not claimed as runtime proof.

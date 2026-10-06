---
schema: 1
id: E-M9-030
---

# E-M9-030

<!-- evidence-field:id markdown -->
## Id

E-M9-030
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

receipt-backed-review-commands
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/review_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/mutations.rs
- crates/jj-store-http/src/review_mutations.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/review_mutations.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/review_commands.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/review_tests.rs#collaboration_contract_exposes_receipt_backed_review_commands
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs#local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-http contract::review_tests::collaboration_contract_exposes_receipt_backed_review_commands --locked -- --exact
```

**Notes**

The test-first contract regression failed because the typed ReviewStore command and durable-status routes were absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-http contract::review_tests::collaboration_contract_exposes_receipt_backed_review_commands --locked -- --exact; cargo test -p jj-store-http contract:: --locked
```

**Notes**

The exact new contract and all 11 collaboration/shared HTTP contract tests pass. The strict request has seven typed command variants and generated apply/status response schemas.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle http_collaboration_review_acceptance::local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail --locked -- --exact
```

**Notes**

The LocalTrusted fixture applies create and reply, reconstructs HTTP composition, returns the durable receipt from status and identical replay, rejects changed intent and stale workflow state, denies read-only publication, and preserves the root-pinned read behavior.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CC=<pinned-clang> CXX=<pinned-clang++> CMAKE=<qualified-cmake> BOOST_ROOT=<qualified-boost> pnpm contracts:generate; pnpm contracts:check
```

**Notes**

Generated OpenAPI and TypeScript match; 70 actual HTTP/SSE responses including committed review commands, durable status, replay and not_known conform, and all six schema/client tests pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check; cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Rust formatting and every affected WAL, service, HTTP and CLI target pass. New production modules contain 237 and 55 physical lines; the extended lifecycle regression remains below 500.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm check:architecture; pnpm spec:validate
```

**Notes**

All 13 package boundaries, 198 governed web modules and 89 canonical requirements pass. The service consumes ReviewStore commands only through the WAL facade.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm exec prettier --check <changed specification files>
```

**Notes**

The first final governance run identified two missing mapped chapter updates and formatting drift in three edited canonical files.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed specification files>
```

**Notes**

All 15 protected files map to eight updated canonical chapters, all 61 progress tasks validate and every changed specification file is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- User-facing collaboration CLI commands and live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5.
- Real harness and cold native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewCommandV1 and ReviewCommandResultV1 schema v1; ReviewCommand request/outcome/status response v1; OpenAPI 3.1; generated TypeScript contracts.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds JSON POST /v1/repos/{repo}/reviews/commands and GET /v1/repos/{repo}/reviews/commands/{request_id}. The service derives the authenticated principal and jj-store/review-publication/v1 publication ID, validates canonical result bytes and returns the existing manifest-selected receipt. Native objects, manifest format 2, WAL and ReviewStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



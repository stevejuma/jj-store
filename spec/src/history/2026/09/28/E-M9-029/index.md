---
schema: 1
id: E-M9-029
---

# E-M9-029

<!-- evidence-field:id markdown -->
## Id

E-M9-029
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

root-pinned-review-threads
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/review_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/review_cursors.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/tests/review_threads.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/review_threads.rs#review_threads_are_root_pinned_and_details_resolve_selected_objects
- crates/jj-store-service/tests/review_threads.rs#review_thread_detail_limits_combined_visible_body_bytes
- crates/jj-store-http/src/contract/review_tests.rs#collaboration_contract_exposes_root_pinned_review_threads
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
cargo test -p jj-store-http contract::review_tests::collaboration_contract_exposes_root_pinned_review_threads --locked -- --exact
```

**Notes**

The test-first OpenAPI regression failed because ReviewStore thread collection and stable-thread detail routes were absent.
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
cargo test -p jj-store-service --test review_threads --locked
```

**Notes**

The two focused service regressions prove cursor root continuity after a later review publication, current stable-thread detail, ordered anchor/comment/event resolution, the eight MiB combined visible-body ceiling, missing-thread behavior and rejection of tampered or page-size-mismatched cursors.
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
cargo test -p jj-store-http contract:: --locked
```

**Notes**

All ten focused HTTP contract tests pass, including both additive ReviewStore paths and service-owned page/detail schemas.
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
cargo test -p jj-store-cli --test native_lifecycle http_collaboration_review_acceptance::local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail --locked -- --exact
```

**Notes**

The registered LocalTrusted handlers page the retained ReviewStore root without a bearer credential, resolve current detail after publication and return 404 for a missing stable thread.
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
CC=<pinned-clang> CXX=<pinned-clang++> CMAKE=<qualified-cmake> BOOST_ROOT=<qualified-boost> pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript regenerate with the additive ReviewStore thread routes and owned response DTOs.
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
cargo fmt --all -- --check; cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Rust formatting and every affected WAL, service, HTTP and CLI target pass. New production modules contain at most 295 counted lines; focused tests stay below 500 physical lines.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

Generated OpenAPI and TypeScript match; 66 actual HTTP/SSE responses, including both review pages and current detail, conform and all six schema/client tests pass.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

The first boundary run rejected a production service dependency on jj-review, proving the collaboration model must remain behind the WAL facade.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; cargo test -p jj-store-service --test review_threads --locked; cargo test -p jj-store-http contract::review_tests::collaboration_contract_exposes_root_pinned_review_threads --locked -- --exact
```

**Notes**

The WAL facade now exports only the validated review value types required by the service; all 13 package boundaries and the exact service/contract regressions pass after removing the invalid production dependency.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

All 198 governed web modules, 89 requirements, 21 protected files across nine canonical chapters and 61 progress tasks validate.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ReviewStore typed writes, collaboration CLI commands, durable receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5.
- Real harness and cold native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewStore schema v1; OpenAPI 3.1; TypeScript generated contracts; review thread page/detail response v1; cursor token jsr1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds two JSON GET review-thread routes and generated schemas. Collection cursors bind repository, epoch, publication position, manifest revision, exact ReviewStore root/state version, page size and last thread radix key while retaining the audited closure in bounded process memory. Detail resolves only the current stable-thread selection and its exact anchor, selected comments/bodies and event stream, with combined visible bodies limited to eight MiB. Native objects, manifest format 2, WAL, receipts and ReviewStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



---
schema: 1
id: E-HMAC-KEYINIT-001
---

# E-HMAC-KEYINIT-001

<!-- evidence-field:id markdown -->
## Id

E-HMAC-KEYINIT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-OBJ-001
- JST-API-001
- JST-MUT-001
- JST-ADAPT-001
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-object/src/s3.rs
- crates/jj-store-http/src/application_events/authorization.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-service/src/collaboration/agent_cursors.rs
- crates/jj-store-service/src/collaboration/review_cursors.rs
- crates/jj-store-service/src/collab_pages/cursors.rs
- crates/jj-store-service/src/native_inventory/cursors.rs
- crates/jj-store-service/src/pagination/cursors.rs
- crates/jj-store-tantivy/src/paging/cursors.rs
- docs/maintenance/coverage.json
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-object/src/s3.rs#sigv4_matches_independent_python_vector
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
rustfmt --edition 2024 --check crates/jj-store-object/src/s3.rs crates/jj-store-http/src/local_browser.rs crates/jj-store-http/src/application_events/authorization.rs crates/jj-store-http/src/repositories.rs crates/jj-store-service/src/native_inventory/cursors.rs crates/jj-store-service/src/collab_pages/cursors.rs crates/jj-store-service/src/pagination/cursors.rs crates/jj-store-service/src/collaboration/agent_cursors.rs crates/jj-store-service/src/collaboration/review_cursors.rs crates/jj-store-tantivy/src/paging/cursors.rs
```

**Notes**

rustfmt accepts the KeyInit imports.
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
node scripts/managed-cargo.mjs test --locked -p jj-store-object --lib sigv4_matches_independent_python_vector
```

**Notes**

The independent SigV4 vector passed against hmac 0.13.0. A combined check of jj-store-object, jj-store-http, jj-store-service, and jj-store-tantivy still stops in jj-store-jj with 146 errors against jj-lib 0.46.0. That API migration is outside this import.
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
pnpm check:progress && pnpm spec:first -- --base 753e9a8006ebf3100350d3fd6453287961fa4496 --head @
```

**Notes**

Progress passed with 186 tasks and 982 evidence records. Spec-first passed with 43 protected files and 17 canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

hmac 0.13.0 and sha2 0.11.0. No package version change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. SigV4 and cursor MAC bytes stay the same.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Bring KeyInit into scope so hmac 0.13 new_from_slice compiles for S3 SigV4 and the HTTP, service, and search cursor signers. Signing inputs and outputs stay the same.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

jj-store-jj still needs a jj-lib 0.46.0 API migration before required CI can pass. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-OBJ-001, JST-API-001, JST-MUT-001, JST-ADAPT-001, and JST-DOC-001/002 keep the same behavior. The import only satisfies hmac 0.13.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. The KeyInit import does not complete release qualification, and jj-lib 0.46.0 still fails to compile in jj-store-jj.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



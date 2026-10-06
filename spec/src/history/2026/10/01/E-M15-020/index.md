---
schema: 1
id: E-M15-020
---

# E-M15-020

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change msszrzpn
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; Vite 8.3.1; CARGO_INCREMENTAL=0 for Rust commands. Community source revisions 741b8285e2081f1c9af58afe5a9b72f538fad781 and d859f33a484a9238bb11f974f4ead8e9be729ec6 under Apache-2.0. Mira 0.0.2 from registry; exact registry bits-ui 2.18.1. Inherited Design Core local hook resolves 0.1.2 despite declared ^0.2.0; this does not qualify registry-only foundations.
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-M15-020
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
- M15.6
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-current-application-content-search
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive GET /v1/repos/{repo}/content/search and V1 ContentQuery/ContentPage/ContentSnapshot/ContentHit contracts; current Thread/WorkStore projection, full native identities and revisions, bounded excerpts, empty-query recent entities, scoped filters and opaque five-minute query-pinned cursors. Existing code Tantivy pins remain independent. Additive authorized Thread directory summaries; no native manifest, companion, canonical object or receipt format change.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Only operation-pinned code search exists. Current Thread messages/replies and WorkStore descriptions/comments have no common recent/scoped query facade; directory rows have no current message/contributor metadata.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs
- crates/jj-store-http/src/content_search.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/content_search.rs
- crates/jj-store-service/src/content_search/cursors.rs
- crates/jj-store-service/src/content_search/extract.rs
- crates/jj-store-service/src/content_search/models.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/thread_directory.rs
- crates/jj-store-service/tests/collab_pagination.rs
- crates/jj-store-service/tests/content_search.rs
- schemas/openapi.json
- spec/src/adapters.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/content_search.rs#current_full_bodies_edits_tombstones_and_scopes_are_manifest_selected
- crates/jj-store-service/tests/content_search.rs#recent_entities_and_cursor_binding_survive_writes_and_expire
- crates/jj-store-service/tests/content_search.rs#query_limits_and_foreign_repository_cursor_fail_closed
- crates/jj-store-service/tests/content_search.rs#fresh_and_resumed_content_never_disclose_after_token_revocation
- crates/jj-store-service/tests/content_search.rs#source_inventory_limits_apply_before_filtering_results
- crates/jj-store-service/tests/content_search.rs#thread_directory_keeps_latest_message_preview_after_rename
- crates/jj-store-service/src/content_search/cursors.rs#capacity_restart_and_query_deadline_fail_closed_without_eviction
- crates/jj-store-service/src/content_search/extract.rs#excerpt_keeps_unicode_expansion_match_in_original_text
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs#http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search --test native_collab --test collab_pagination --locked
```

**Notes**

Initial signature and stale custom-emoji fixture compile errors were repaired; exact definition version object references retained. Earlier final run: 5 content + 9 collaboration + 8 pagination tests pass; /tmp/jj-content-service-tests8.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service content_search:: --lib --locked
```

**Notes**

2 tests pass for Unicode lowercase-expansion excerpts and bounded cursor capacity/restart/fixed query deadline; /tmp/jj-alignment-content-units2.log. Earlier wrong digest namespace/WalPosition constructor in new tests corrected.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts --locked
```

**Notes**

Exact native HTTP Thread/reaction/content-search acceptance passes; /tmp/jj-alignment-http-thread.log. Isolated MemoryStore authority, not real RustFS.
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
CARGO_INCREMENTAL=0 pnpm contracts:check
```

**Notes**

Generated contracts match; 121 actual HTTP/SSE responses conform, 7 contract tests and TypeScript pass; /tmp/jj-alignment-contracts-check.log. Wider captures justified because route inventory and generated contracts are shared.
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
node scripts/browser-fixtures.mjs
```

**Notes**

Actual native API fixture capture passes; /tmp/jj-alignment-browser-fixtures.log; .scratch/web-responses.json remains ignored.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search thread_directory_keeps_latest_message_preview_after_rename --locked -- --exact
```

**Notes**

Directory preview became empty after a later rename despite a visible message. /tmp/jj-alignment-directory-rename-before.log. Separate newest-message selection now preserves preview while the Thread activity timestamp advances.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search thread_directory_keeps_latest_message_preview_after_rename --locked -- --exact
```

**Notes**

Product preview repaired, but new test assumed the authorization fixture clock was publication acceptance time. Native command timestamps use server acceptance time; assertion now uses the actual selected renamed revision. /tmp/jj-alignment-directory-rename-after.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search thread_directory_keeps_latest_message_preview_after_rename --locked -- --exact
```

**Notes**

Exact regression passes, preserving the visible preview and actual newer Thread activity timestamp. /tmp/jj-alignment-directory-rename-after2.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search --test native_collab --test collab_pagination --locked
```

**Notes**

Final direct consumers: 6 content + 9 native collaboration + 8 pagination tests pass; /tmp/jj-content-service-tests-final.log.
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

<!-- evidence-field:scope markdown -->
## Scope

All search reads authorize before body/count/identity disclosure and reauthorize continuations. Inventory bounds apply before filtering. Pages retain immutable query pins and authority epoch; later paging does not extend expiry. No runner launch or new membership ACL.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain selected-current search, native directory and direct transport evidence. M15.7 real RustFS and runner qualification remain independently pending.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



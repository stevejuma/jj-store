---
schema: 1
id: E-READ-002
---

# E-READ-002

<!-- evidence-field:id markdown -->
## Id

E-READ-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mulxnmqr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Local operation metadata and page-only bookmark target hydration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire or persisted format changes. Metadata-only queries bypass graph/index construction while retaining exact operation and reference membership.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/native.rs
- crates/jj-store-jj/src/local_browser/references.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_metadata_and_bookmark_pages_do_not_load_history_or_index
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#registered_large_repository_metadata_smoke
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_and_bookmark_pages_do_not_load_history_or_index -- --exact bounded_reads::local_metadata_and_bookmark_pages_do_not_load_history_or_index
```

**Notes**

Before: QueryInfo failed Limit under max_history_visits=1 with 1000 bookmarks and a two-commit chain.
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_and_bookmark_pages_do_not_load_history_or_index -- --exact bounded_reads::local_metadata_and_bookmark_pages_do_not_load_history_or_index
```

**Notes**

Initial post-change 1000-bookmark regression passed. Fixture then expanded to 10000 bookmarks and corrupt nested index segments.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

Escalated to all direct local browser consumers after changing shared Context setup: 8 passed, 1 opt-in smoke ignored, 36.39s. Includes 10000 bookmarks and corrupt JJ segment files, exact pins, history aliases, conflicts, registration, filesystem runtime and untouched dirty files/heads.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo target/debug/deps/local_browser-893d8c4057fcb950 --ignored --exact bounded_reads::registered_large_repository_metadata_smoke --nocapture
```

**Notes**

Linux read-only QueryInfo + first bookmark succeeds in 1.7358s; bookmark stats: 3 metadata nodes, 2936748 charged bytes, 6 work units. No checkout/metadata publication or service restart.
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
cargo check -p jj-store-jj -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package all-target checks pass.
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
pnpm spec:first
```

**Notes**

Initial evidence missed CLI ownership chapter; added local browser conformance requirements without changing governance.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md
```

**Notes**

Mapped canonical chapters, 136-task ledger, 167 requirements and focused formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Scoped file/commit reads, graph preparation, native reference paging and
  UI/collaboration changes remain subsequent slices. UI and full aggregate lanes
  out of scope for this metadata-only Rust slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement scoped file reads, direct directory windows and independent metadata loading; complete remaining large-repository plan.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



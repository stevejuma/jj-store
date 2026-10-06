---
schema: 1
id: E-READ-036
---

# E-READ-036 details 3

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
cargo check -p jj-store-service -p jj-store-http -p jj-store-cli --lib --locked; cargo fmt -p jj-store-service -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

Affected service/HTTP/CLI libraries and direct transitive consumers compile/22.82s; Rust format passes. work-activity-packages-036.log/work-activity-rust-format-036.log. Inherited oversized __eh_frame linker warning remains.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Required mapped architecture/CLI/compatibility/mutations chapters were missing. Added owning boundary/version/fixture/read-only requirements in this change. work-activity-spec-first-036.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial)
```

**Notes**

Two browser evidence references lacked named test fragments. Corrected to exact source test names. work-activity-progress-036.log.
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
pnpm spec:validate (initial); pnpm exec prettier --write spec/src/api.md spec/src/projects.md spec/src/local-jj.md spec/src/ui.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

5validators/23chapters/172requirements and canonical formatting pass. work-activity-spec-036.log/work-activity-doc-format-final-036.log. Additional mapped chapters and final ledger check follow.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/mutations.md spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; jj --no-pager diff --git plus added-line whitespace validation (corrected)
```

**Notes**

29protected files/9mapped chapters,5validators/23chapters/172requirements,138-task bookkeeping, canonical formatting and added-line whitespace pass. work-activity-doc-format-corrected-036.log/work-activity-spec-first-corrected-036.log/work-activity-spec-corrected-036.log/work-activity-progress-corrected-036.log/work-activity-diff-final-036.patch. Focused delivery only; complete qualification remains deferred.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Code and review card summaries still fetch bounded graph/thread pages; their
  lightweight summaries remain high-priority delivery work.
- Local WorkStore projection/state reuse still needs correct cross-request
  companion admission. Existing native disk projection decoding still retains
  namespace; local operation/prefix/comparison/deep/predecessor/copy/search and
  paged native RPC remain delivery work.
- Combined integration/contracts, Linux/native scale fixtures, isolated
  RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review qualification remain deferred. No milestone promotion,
  original dev service or main/Linux source changes; visual baselines remain
  unapproved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue lightweight code/review summaries and admitted local root reuse, then remaining native disk recovery and indexed lookup/RPC delivery; complete combined qualification in its later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Body-free complete-root task aggregates for native/local visible repository cards; current authorization, independent pins and cross-home/project activity semantics retained.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/work/browse_tests.rs#task_activity_counts_the_complete_root_and_pages_only_compact_groups
- crates/jj-store-service/tests/work_store/cache.rs#work_first_pages_share_selected_roots_and_enforce_fresh_limits
- crates/jj-store-service/tests/local_work.rs#local_task_activity_pages_pin_complete_counts_and_reject_foreign_or_changed_queries
- crates/jj-store-service/tests/local_work.rs#forged_continuations_cannot_read_staged_metadata_candidates
- crates/jj-store-http/src/contract/tests.rs#work_activity_contracts_are_body_free_and_pin_independent_profiles
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs#http_workstore_pages_commands_permissions_and_contract_capture
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/projects-repositories.test.ts#native directory reads compact
  complete task groups without fetching task summaries
- web/tests/vite/projects-repository-visibility.test.ts#canceling a local card
  during a shared snapshot read starts no collaboration or graph work
<!-- /evidence-field -->


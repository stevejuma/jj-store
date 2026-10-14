---
schema: 1
id: E-CUSTOM-INDEX-001
---

# E-CUSTOM-INDEX-001 details 3

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
pnpm dev:cargo -- fmt --all
```

**Notes**

Rust formatting after implementation and lint cleanup.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write Cargo.toml crates/jj-store-read-index/Cargo.toml crates/jj-store-service/Cargo.toml
```

**Notes**

Prettier has no TOML parser. Rust/Cargo syntax uses cargo checks; supported authored files were formatted separately.
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
pnpm exec prettier --write scripts/check-boundaries.mjs spec-validator.config.mjs docs/maintenance/coverage.json docs/site/technical/architecture.md docs/site/technical/publication.md docs/site/technical/reads.md docs/site/reference/configuration.md spec/src/current/contracts/adapters/03-requirements.md spec/src/current/contracts/adapters/12-custom-data-read-indexes.md spec/src/current/contracts/architecture/22-custom-data-indexes.md spec/src/current/contracts/cli/39-custom-data-indexes.md spec/src/current/contracts/mutations/25-custom-data-indexes.md spec/src/current/contracts/publication/09-workflowstore.md spec/src/current/contracts/collaboration/25-root-pinned-review-store.md spec/src/current/contracts/documentation/10-workflows.md spec/src/current/progress/M29/index.md spec/src/current/governance/spec-governance.md
```

**Notes**

Supported source/document formatting completed.
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
pnpm test:governance
pnpm check:boundaries
pnpm spec:validate --only fileLimits
```

**Notes**

Before final evidence: 68 governance tests, 19 package boundaries and authored file limits passed.
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
ps -p 91448,91453 -o pid,ppid,stat,command
kill -INT 91453
ps -ax -o pid,ppid,stat,command
```

**Notes**

Verified ownership, stopped the active serial server suite at the user request, and confirmed the runner and its custom-index test processes exited.
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
jj rebase -r nxyuuzpy -o xqllnwon
```

**Notes**

Integrated onto main parent xqllnwon/15906b72. Only the day history navigation conflicted; rebuilt the two day navigation files preserving all main and indexing evidence entries. No production source conflict.
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
pnpm check:progress
pnpm spec:validate --only fileLimits
```

**Notes**

At the verification pause: 154 tasks, 797 evidence records pass bookkeeping; authored file limits pass for 3,805 chapters. This is documentation validation, not resumed Rust or full qualification.
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
jj --no-pager diff --from 6c9eb07d --to @ -- Cargo.toml Cargo.lock crates
jj --no-pager diff --from xqllnwon --to @ -- web catalog desktop
```

**Notes**

Rebase retained the pre-integration Rust source tree exactly and changed no main UI/catalog/desktop files. Main application documentation coverage is equal to the main parent.
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
pnpm docs:review configuration
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/current/progress/M29/index.md
```

**Notes**

Refreshed only configuration source coverage and formatted changed bookkeeping documents. Full docs build deferred.
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
pnpm dev:cargo -- test -p jj-collab -p jj-store-read-index --all-targets --locked --target-dir .local/build-targets/custom-index
```

**Notes**

model-tests: 63 passed, 0 failed, 0 ignored across 14 binaries. See scope for focused lane boundaries.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-jj --lib --test local_stacks --test local_evidence --test native_codec --test compatibility --locked --target-dir .local/build-targets/custom-index
```

**Notes**

native-tests: 132 passed, 1 failed, 3 ignored across 4 binaries. Parallel local-stack Busy; exact case and complete serial lane subsequently pass.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-jj --test local_stacks --locked --target-dir .local/build-targets/custom-index -- --exact multisided_text_conflicts_offer_in_app_resolution_and_preserve_other_paths
```

**Notes**

native-conflict-regression: 1 passed, 0 failed, 0 ignored across 1 binaries. See scope for focused lane boundaries.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-jj --lib --test local_stacks --test local_evidence --test native_codec --test compatibility --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

native-tests-serial: 141 passed, 0 failed, 3 ignored across 5 binaries. Serial execution avoids global transaction/resource contention; 3 existing ignored tests remain separately governed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-service --lib --test workflow_store --test work_store --test content_search --test collab_pagination --test agent_sessions --test review_threads --test agent_checkpoints --test agent_relations --test native_collab --test local_work --test local_collab --test local_agent --test local_agent_pages --locked --target-dir .local/build-targets/custom-index
```

**Notes**

service-tests: 170 passed, 0 failed, 0 ignored across 14 binaries. See scope for focused lane boundaries.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-wal --test workflow_store -p jj-store-rpc --test work --test collab --test review --test application_mutations -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index
```

**Notes**

transport-tests: 49 passed, 1 failed, 1 ignored across 1 binaries. See scope for focused lane boundaries. Pre-existing path-count fixture expected 196 while both checked-in OpenAPI and source expose 200; corrected counts and final transport lane passes.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index -- --exact contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies
```

**Notes**

http-contract-regression: 1 passed, 0 failed, 0 ignored across 1 binaries. See scope for focused lane boundaries.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-wal --test workflow_store -p jj-store-rpc --test work --test collab --test review --test application_mutations -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index
```

**Notes**

transport-tests-final: 86 passed, 0 failed, 1 ignored across 8 binaries. See scope for focused lane boundaries.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index
```

**Notes**

server-tests: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Interrupted after concurrent fixture startup failures; three owned cold-process test children terminated.
<!-- /run -->
<!-- /evidence-field -->


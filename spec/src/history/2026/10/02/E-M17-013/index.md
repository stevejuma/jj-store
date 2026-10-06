---
schema: 1
id: E-M17-013
---

# E-M17-013

<!-- evidence-field:id markdown -->
## Id

E-M17-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bound-discovery-to-codex-reader-composition
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_discovery.rs
- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/codex_transcript/poll.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/tests/codex_bound_source.rs
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_bound_source.rs#discovery_candidate_qualifies_exact_surface_before_any_source_poll
- crates/jj-store-capture/tests/codex_bound_source.rs#replaced_candidate_is_unavailable_before_header_inspection
- crates/jj-store-capture/tests/codex_bound_source.rs#bound_poll_withholds_replaced_file_even_with_a_pending_exact_batch
- crates/jj-store-capture/tests/codex_bound_source.rs#bound_poll_withholds_rebound_root_and_symlinked_file
- crates/jj-store-capture/tests/codex_bound_source.rs#durable_pending_ack_survives_unavailability_and_rediscovery_preserves_generation_progress
- crates/jj-store-capture/src/codex_transcript/file.rs#replaced_physical_identity_is_rejected_before_header_parsing
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
cargo test -p jj-store-capture --test codex_bound_source --locked > /tmp/jj-store-agent-capture-m172-bound-source-before.log 2>&1
```

**Notes**

Before state: the bound-source composition API is absent; one unresolved-import compile failure proves the required boundary does not exist.
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
cargo fmt -p jj-store-capture
```

**Notes**

Formatted the bound reader, expected physical-identity inspection and polling extraction below the 300-line production module budget.
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
cargo test -p jj-store-capture --test codex_bound_source --locked > /tmp/jj-store-agent-capture-m172-bound-source-after.log 2>&1
```

**Notes**

Initial four tests pass. The extraction left an unused private inspect wrapper warning; removed that wrapper rather than suppressing the diagnostic.
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
cargo fmt -p jj-store-capture
```

**Notes**

Removed the unused wrapper and formatted exact pre-parse identity rejection plus durable rediscovery recovery.
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
cargo test -p jj-store-capture --test codex_bound_source --test codex_discovery --test codex_transcript --locked > /tmp/jj-store-agent-capture-m172-bound-source-runtime.log 2>&1
```

**Notes**

All 18 direct runtime tests pass: five bound-source, four metadata-discovery and nine existing transcript-reader tests. Existing exact batches, partial tails, limits, prefix verification and unbound replacement semantics are preserved.
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
cargo test -p jj-store-capture --lib codex_transcript::file::tests::replaced_physical_identity_is_rejected_before_header_parsing --locked -- --exact > /tmp/jj-store-agent-capture-m172-bound-source-identity.log 2>&1
```

**Notes**

Exact identity regression passes: a replaced inode is rejected with a constant binding error before parsing its invalid/private header.
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
cargo check -p jj-store-capture --all-targets --locked > /tmp/jj-store-agent-capture-m172-bound-source-check.log 2>&1
```

**Notes**

The affected capture package and its compiled owned boundaries pass all-target checks.
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
cargo clippy -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-bound-source-clippy.log 2>&1
```

**Notes**

Strict all-target lint passes; no warning suppression or inherited repair.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-bound-source-format.log 2>&1
```

**Notes**

Pinned Rust formatting passes, including extracted poll/verification code.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-bound-source-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; this in-memory source composition adds no dependency or provider authority.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/milestones.md
```

**Notes**

Formatted the bound-source canonical contract and same-change evidence.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-bound-source-progress.log 2>&1
```

**Notes**

129 tasks validate; 268 evidence entries retain all 267 parent entries and M17.2/aggregate remain In progress.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-bound-source-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-bound-source-first.log 2>&1
```

**Notes**

Spec-first passes: seven protected files and two canonical chapters.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-bound-source-doc-format.log 2>&1
```

**Notes**

Changed canonical chapter and ledger pass formatting.
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

Rust 1.98.1; JJ 0.45.1; exact Codex CLI 0.156.1 and Desktop 0.159.2 headers; disposable Unix files and sanitized-omission spool fixtures on macOS ARM64. No real harness body or model edit qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-013 details 2](details-02.md)

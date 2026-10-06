---
schema: 1
id: E-M10-001
---

# E-M10-001

<!-- evidence-field:id markdown -->
## Id

E-M10-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

audit
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/check-boundaries.mjs
- scripts/check-progress.mjs
- scripts/governance.test.mjs
- spec-validator.config.mjs
- spec/src/SUMMARY.md
- spec/src/index.md
- spec/src/milestones.md
- spec/src/replication.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/governance.test.mjs#accepts_multi_digit_milestone_tasks
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
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
cargo test -p jj-store-wal --test native_copy --locked native_copy_restores_code_agent_review_and_installs_manifest_last -- --exact
```

**Notes**

The before-state gate proves the existing copy primitive installs one complete manifest-selected code, AgentStore and ReviewStore closure into an empty destination; it does not merge into an independently writable replica.
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
pnpm spec:check
```

**Notes**

Specification validation and the catalog cold-browser lane passed, then the configured browser-interaction lane stopped while building the Kingfisher Vectorscan dependency because cmake is not installed on this host.
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
pnpm test:governance && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries && pnpm format:check
```

**Notes**

All 29 governance tests pass; 18 canonical chapters and 102 requirements validate; the 69-task progress ledger, 13 package boundaries and all tracked formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Trace every foreground CLI, browser and capture binding and prove it cannot
  obtain an upstream client after local attachment.
- Implement M10.2 through M10.7 and run their focused, aggregate and real RustFS
  gates.
- Rerun the complete specification checker after cmake is available for the
  Kingfisher Vectorscan build.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib and jj-cli 0.45.1; manifest format 2; AgentStore and ReviewStore schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds canonical local-first replication requirements and M10 gates only. Existing native objects, manifest format 2, WAL, receipts, RPC and application contracts remain unchanged in this audit slice.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



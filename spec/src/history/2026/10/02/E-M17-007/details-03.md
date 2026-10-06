---
schema: 1
id: E-M17-007
---

# E-M17-007 details 3

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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes.
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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

Final changed chapter and ledger formatting passes.
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

JJ 0.45.1; Rust/Cargo 1.98.1; owner-only file and lease evidence on macOS ARM64 Unix.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional private PendingCaptureV1.reader_metadata schema 1 and ReaderProgressV1 schema 1. Absent optional metadata preserves legacy pending bytes. No AgentStore objects, source IDs, native/local prepared request, receipt or wire schema changes. Journal limit 4096 records, each at most 8 KiB.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rrxpuxnx
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared capture queue/open/completion changes require all directly affected native/local spool and recovery binaries, runtime tests, affected package checks and strict clippy. Sanitizer/hooks, server startup, UI, real RustFS and root aggregate lanes are out of scope because their behavior is unchanged. This slice qualifies cursor durability only, not a real reader, watcher or worker. Future M17.2 gates remain qualified source readers, generation/partial-tail semantics, active backfill, verified alias enrichment, composition, controls and privacy. Recovery scans journal records once and compares bounded pending records in memory; normal admission validates the bounded journal and retained pending queue and does not claim constant I/O. Cursor retention/purge remains M17.7.
<!-- /evidence-field -->


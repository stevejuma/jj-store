---
schema: 1
id: E-M13-001
---

# E-M13-001 details 3

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test local_metadata
```

**Notes**

11 exact tests pass after adding immediate previous-commit checks and bounded history materialization/immutable receipt validation.
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

<!-- evidence-field:deferred list -->
## Deferred

- M13.2-M13.6 remain planned; no CLI, UI, portable anchor or import/export
  completion claim.
- Unrelated UI, native RustFS, root aggregate lanes and visual baselines are out
  of scope for the storage foundation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; gix 0.87.1; Git 2.55.0; Rust 1.98.1; macOS ARM64. Shared ignored Cargo target cache; source edits confined to local-jj workspace.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local metadata commit/tree/ref v1; canonical bounded RootV1; typed 20-byte SHA-1 GitOid; existing native identities/codecs unchanged. Ref CAS selects roots and receipts; parent/tree edges retain history and evidence. Explicit limits: 16 MiB/object, 4 MiB root, 16,384 receipt/retention entries, 100,000 closure objects/history steps, 256 MiB retained closure. Budget exhaustion rejects rather than truncates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

M13.1 metadata primitive gate complete. Implement M13.2 shared WorkStore service and direct CLI in the isolated local-jj workspace.
<!-- /evidence-field -->


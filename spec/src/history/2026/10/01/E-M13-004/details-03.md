---
schema: 1
id: E-M13-004
---

# E-M13-004 details 3

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
pnpm check:progress
```

**Notes**

102 tasks pass evidence bookkeeping.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/mutations.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/milestones.md
```

**Notes**

Changed canonical documents pass formatting.
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

- Portable agent code associations/checkpoints, review provider and local
  capture CLI remain incomplete; M13.3 is not Done.
- UI, conversion, real-RustFS and unrelated aggregate lanes are out of scope for
  this agent projection slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; gix 0.87.1; Git 2.55.0; Rust 1.98.1; macOS ARM64; isolated local-jj workspace and shared ignored target cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native AgentStore V1 record envelopes and IDs unchanged. Local AgentStore descriptor v1 retains exact immutable framed records in Git blobs; local typed command/receipt and snapshot report explicit local Git profile. Native code-anchor V1 records fail closed without native evidence.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify and commit shared AgentStore/local-provider slice, then continue portable associations, ReviewStore and backend-aware capture integration.
<!-- /evidence-field -->


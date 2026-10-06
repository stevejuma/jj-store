---
schema: 1
id: E-M13-010
---

# E-M13-010 details 3

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test review_projector
```

**Notes**

Both exact independent-provider regressions re-pass after style-only guard change.
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
pnpm check:progress
```

**Notes**

Final evidence bookkeeping.
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
pnpm exec prettier --check spec/src/publication.md spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Final changed-document formatting.
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

- Local ReviewStore provider, retained portable review anchors and CLI remain
  incomplete; M13.3 is not Done.
- "Only direct review consumers widened: shared command/audit code affects
  native publication, copying and replicated graph reduction. Unrelated UI,
  real-RustFS and root aggregates out of scope."
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; native owned ReviewStore V1; independent immutable Records provider in disposable MemoryStore.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native V1 root/object/command and receipt bytes unchanged. Shared ReviewProjector stages immutable records and returns a root value, typed delta and result; no publication capability. Native wrappers keep authorization, receipt-first replay, external code/AgentStore closure verification and manifest CAS.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit qualified shared projector, then integrate portable anchors and local ReviewStore publication.
<!-- /evidence-field -->


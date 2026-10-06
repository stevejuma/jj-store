---
schema: 1
id: E-M17-003
---

# E-M17-003 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Ledger JSON was edited after formatting; only milestones.md formatting drift. Format the final ledger after recording actual checks.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final actual-run ledger formatting.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

Final changed canonical document formatting.
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
pnpm check:progress
```

**Notes**

Final ledger after successful run bookkeeping.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M17.1 explicit bound-gap handling remains pending.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust/Cargo 1.98.1; existing qualified V1 normalizers and source receipts.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive transcript_tail_page object variant with page schema 2 and existing TranscriptPage semantic kind. V1 forward pages, manifests, record envelopes and request/receipt bytes remain unchanged. Strong prefix edges preserve mixed legacy chains and independently reconstructible immutable history. Rebuildable ordering witnesses derive from the validated pinned closure and fail closed on invalid sequence/identity/count; no new authority.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wnuzkruo
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared projection append and native/local direct consumers. Append reads at most one selected tail and one prior chunk; legacy adoption reads one page and zero prior chunks. Persistence providers still audit complete closure; total publication I/O is not claimed bounded. UI, real RustFS, real harness and root aggregate lanes are out of scope.
<!-- /evidence-field -->


---
schema: 1
id: E-M13-011
---

# E-M13-011 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 32
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review
```

**Notes**

Before implementation: local provider types/methods absent.
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
cargo fmt -p jj-store-service
```

**Notes**

Owning crate and tests formatted; repeated after compilation fixes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review
```

**Notes**

Initial draft: internal serde command tag collided with payload name; service uses WAL owned model reexports rather than a new production model dependency.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review
```

**Notes**

Draft test glob imports made Work Request ambiguous; fixed fixture imports.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review
```

**Notes**

Initial three provider regressions pass: no read initialization, domain preservation/pinned stale semantics/GC/restart receipts and unknown ref publication.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review
```

**Notes**

Five provider regressions pass, including foreign/unqualified anchor and missing selected record-closure rejection.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_review --test local_agent --test local_agent_pages --test local_work
```

**Notes**

22 pass across review and directly shared local agent/work state.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-service --lib --test local_review --test local_agent --test local_agent_pages --test local_work --locked -- -D warnings
```

**Notes**

Focused owning-package lint clean.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-cli --locked
```

**Notes**

Direct local facade/error consumer compiles.
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
cargo fmt -p jj-store-service -- --check
```

**Notes**

Final Rust formatting.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Changed canonical and evidence documents formatted.
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
pnpm spec:first
```

**Notes**

Seven protected files covered by same-change canonical chapters and ledger.
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
pnpm spec:validate
```

**Notes**

Four validators; 21 chapters and 129 requirements.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries.
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
pnpm check:progress
```

**Notes**

Final evidence bookkeeping.
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
pnpm exec prettier --check spec/src/local-jj.md spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
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

- Only repository-wide anchors qualified in this provider; portable code/agent
  review anchors and review CLI remain unsupported/incomplete. M13.3 is not
  Done.
- Filesystem server/UI, conversion, automatic agent code causality and native
  portable evidence carrier remain separate gaps.
- Unrelated UI, real-RustFS and root aggregate lanes out of scope under targeted
  verification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; disposable colocated Git-backed JJ folders using explicit stock development fixture executable.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

LocalReviewRootV1 descriptor selects canonical repository-bound ReviewRootV1 records and a retained record-to-Git-blob inventory. Local request/receipt V1 binds actor, stable request, canonical intent and typed native review results. Companion ref CAS selects state; native formats and code operation heads unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit qualified repository-wide review provider, then integrate retained portable review anchors and backend-aware CLI.
<!-- /evidence-field -->


---
schema: 1
id: E-CHAT-WORKSPACE-003
---

# E-CHAT-WORKSPACE-003 details 3

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
cargo test --locked -p jj-store-wal --test collab_commands
```

**Notes**

Ten tests pass. Native qualified publication requires the explicit reader, denial preserves the selected root, restart receipt replay is stable, and foreign reviews add no home review dependencies.
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
cargo check --locked -p jj-store-service -p jj-store-server
```

**Notes**

Changed model/service/native/local execution boundary compiles.
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
pnpm contracts:generate
```

**Notes**

Generated the additive repository_review_comment union in API contracts. Existing variants retain their encodings.
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
pnpm web:check
```

**Notes**

Both pre-generation and generated-consumer checks pass with zero errors/warnings.
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
pnpm evidence:new -- --id E-CHAT-WORKSPACE-003 --task M24 --task M13.3 --requirement JST-COLLAB-034 --requirement JST-COLLAB-035 --requirement JST-COLLAB-036
```

**Notes**

Created fresh evidence and explicit milestone reviews. Navigation regeneration touched unrelated M25 whitespace, which was restored.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Frozen workspace draft capture, queue eligibility, publication recovery and
  turn evidence retention remain subsequent work.
- Footer/Files/Changes integration, exact source/reply navigation and isolated
  cold native/local browser acceptance are not yet implemented.
- ResizeObserver warnings are not yet rechecked; visual baselines remain
  unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; cargo 1.98.1; JJ 0.45.1. Enclosing Jujutsu change is the revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive repository_review_comment references and optional omitted FrozenComment.file context. Existing reference encodings and native/local publication authority unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository-qualified references, independent authority ports, retained file context and exact reply proof. Direct native/local publication consumers and generated contracts justify their focused suites. No main workspace or live fixture/service changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Freeze file drafts and implement eligible-queue source capture/publication, then connect the shared footer and inspector and complete cold acceptance.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Collaboration/API contracts specify independently authorized foreign
  review references, exact immutable source context and delivered answer proof.
  Generic calls refuse the carrier without explicit authority.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M13.3 remain In progress. This slice qualifies foreign
  references only; complete Send/UI/restart acceptance remains outstanding.

```
<!-- /evidence-field -->


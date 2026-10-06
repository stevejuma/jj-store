---
schema: 1
id: E-M15-012
---

# E-M15-012 details 3

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
cargo fmt -p jj-collab -p jj-store-wal -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting.
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
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/publication.md spec/src/milestones.md (initial)
```

**Notes**

Ledger JSON updated after last formatter; wrapped long evidence strings differed. Reformat the completed ledger.
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
pnpm exec prettier --write spec/src/milestones.md && pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/publication.md spec/src/milestones.md && pnpm check:progress (final)
```

**Notes**

Completed ledger reformatted; focused document formatting and progress gates pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Shared AgentDeltaRequest codec and publication affect domain models, native RPC publication and durable CLI spool consumers. Before widening, select shared domain suite, complete RPC package, exact capture spool/drain tests and Agent/Thread WAL catalog/command/persistence tests, plus strict affected package lint. Root aggregates, unrelated UI and full Rust workspace remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Provider atomicity and durable encoding are qualified over MemoryStore; bound
  native prepare RPC, captured spool binding and worker support, local atomic
  provider, runner launch/continuation remain pending.
- Reducer 3, canonical cross-source timeline/search, released Community channel
  browser template, cold process/browser and real-RustFS full flow remain
  pending. WorkStore origin freeze and reducer-3 guard are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Connect optional explicit binding through native preparation RPC and durable captured spool entries before implementing local provider and runner integration.
<!-- /evidence-field -->


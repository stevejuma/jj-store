---
schema: 1
id: E-M15-014
---

# E-M15-014 details 2

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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test local_agent bound_local_capture_ --locked (before)
```

**Notes**

Zero tests match; local bound provider behavior absent. Before-state evidence only; /tmp/jj-bound-local-before.log.
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
CARGO_INCREMENTAL=0 cargo check -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package and direct consumer targets compile; /tmp/jj-bound-local-check.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test local_agent_binding --locked
```

**Notes**

Three real Git companion provider tests: atomic Agent/Thread selection with archived lifecycle, complete session closure through Git GC/reopen/receipt replay, stale and multiple-session rejection with no Agent selection, and primary reassignment retaining inactive history; /tmp/jj-bound-local-test.log.
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
cargo fmt -p jj-store-service -p jj-store-cli
```

**Notes**

Focused Rust formatting after new provider, journal and regressions.
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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md
```

**Notes**

Canonical provider, atomic ref publication and spool contracts formatted; /tmp/jj-bound-local-format.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test local_agent_recovery bound_local_spool_ --locked
```

**Notes**

One bound local journal regression: captured and prepared requests survive reopen, changed binding retry is rejected, unknown preserves exact original typed intent; /tmp/jj-bound-local-spool-test.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test local_agent_binding --test local_agent --test local_agent_pages --test local_collab --test local_review --test local_work --locked
```

**Notes**

33 complete relevant local service regressions; /tmp/jj-bound-local-service-consumers.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test local_agent_recovery --test agent_capture_spool --test agent_capture_publication --locked
```

**Notes**

Twelve direct native/local spool and drain regressions; /tmp/jj-bound-local-cli-consumers.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test local_metadata batch_ --locked
```

**Notes**

Two atomic metadata batch regressions preserve receipts and reject stale partial selections; /tmp/jj-bound-local-metadata.log.
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
CARGO_INCREMENTAL=0 cargo clippy -p jj-store-service -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Strict affected provider/CLI lint passes; /tmp/jj-bound-local-clippy.log.
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
pnpm spec:first
```

**Notes**

Eight protected files; six canonical chapters.
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
pnpm spec:validate
```

**Notes**

Four validators; 21 chapters; 140 requirements.
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
pnpm check:progress
```

**Notes**

111 tasks; evidence bookkeeping only.
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

16 package boundaries.
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
cargo fmt -p jj-store-service -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting passes.
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
pnpm exec prettier --write spec/src/milestones.md && pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Canonical chapters and ledger formatted; focused formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_agent_binding.rs#bound_local_capture_selects_one_ref_retains_complete_session_and_replays_after_gc
- crates/jj-store-service/tests/local_agent_binding.rs#bound_local_capture_rejects_stale_and_multi_session_intent_without_selecting_agents
- crates/jj-store-service/tests/local_agent_binding.rs#bound_local_capture_reassigns_one_session_and_retains_inactive_history
- crates/jj-store-cli/tests/local_agent_recovery.rs#bound_local_spool_keeps_original_observation_and_request_across_unknown_restart
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Shared local Agent command and Thread external retention plus shared spool intent validation affect local Agent, Thread and direct Review/Work readers and both capture worker providers. Before widening, select complete relevant local service binaries, all spool/drain/recovery tests, exact metadata batch atomicity regressions and strict service/CLI lint. Native transport/wire, unrelated UI, root aggregates and full Rust workspace remain out of scope; cold bound process/runner and real-RustFS acceptance remain separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Binding provider/spool APIs remain explicit; ordinary hook/watch paths enqueue
  unlinked captures. CLI runner entry points and jj-acp launch/continuation are
  pending.
- Reducer 3, canonical cross-source timeline/search, released Community channel
  UI template, real-RustFS and cold full-flow qualification remain pending.
  Local provider reopen/Git GC and journal reconstruction are not a bound CLI
  process/runner or browser full-flow gate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Before any UI implementation, rebase the complete isolated Thread/Reaction stack onto the clean parent latest committed change, resolve conflicts while retaining both evidence ledgers, then qualify the rebased impact surface.
<!-- /evidence-field -->


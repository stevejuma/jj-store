---
schema: 1
id: E-M13-025
---

# E-M13-025

<!-- evidence-field:id markdown -->
## Id

E-M13-025
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sqpxrrsm
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Portable agent model stage wmwwmqsp/7f89c54d committed. Local provider lacks exact source inspection against committed metadata. New regression cannot compile with ten missing-method diagnostics and one test helper shadowing diagnostic. Physical Save yyknxszk retained separately, awaiting the inherited native Storybook decision.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ0.45.1; Git SHA-1; Rust1.98.1; existing locked dependencies. Service promotes existing jj-review dependency from dev to production without lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Non-publishing owned Rust source-inspection interface, accepting one explicit committed metadata pin and V2 retained-agent anchor. Reads use existing canonical AgentStore envelopes and audited selected closure. No stored format, intent/receipt encoding, HTTP schema, native persistence or publication change. Optional roots must match; native publication counters remain unsupported.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/review_agent_evidence.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-service/tests/local_agent/portable_review.rs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_agent/portable_review.rs#portable_agent_reviews_read_exact_committed_records_after_restart_and_gc
- crates/jj-store-service/tests/local_agent/portable_review.rs#portable_agent_review_rejects_staged_records_wrong_pins_and_identity
- crates/jj-store-service/tests/local_agent/portable_review.rs#portable_agent_review_pin_cannot_read_future_selected_sessions
- crates/jj-store-service/tests/local_agent/portable_review.rs#portable_agent_review_reads_never_initialize_metadata_or_actor
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_agent portable_review (before)
```

**Notes**

Ten missing inspection methods plus one shadowed test helper. Helper renamed, then implementation added. /tmp/local-completion-agent-review-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_agent portable_review (first implementation)
```

**Notes**

jj-review had been a dev-only service dependency; promoted the existing owned model dependency to production. /tmp/local-completion-agent-review-after.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_agent portable_review (final)
```

**Notes**

Five exact provider cases pass: selected canonical records, true staged-but-unselected envelope, future selection rejection, identity and root checks, exact sanitized payload bytes, no actor/metadata initialization, restart/real Git GC, unchanged metadata and JJ operation-head files. /tmp/local-completion-agent-review-after2.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_agent
```

**Notes**

All13 local AgentStore cases pass, including native record identity carrier, cumulative pinned transcripts, disjoint captures, receipt replay and WorkStore preservation. No broader service-suite claim. /tmp/local-completion-agent-review-suite.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-store-service
```

**Notes**

Service and direct dependencies compile with no diagnostics. /tmp/local-completion-agent-review-check.log.
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
cargo fmt -p jj-store-service -- --check
```

**Notes**

Owning crate format check passes. /tmp/local-completion-agent-review-fmt.log.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md
```

**Notes**

Changed canonical prose formatted. /tmp/local-completion-agent-review-doc-format.log.
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
pnpm check:progress
```

**Notes**

120 tasks; evidence bookkeeping only. /tmp/local-completion-agent-review-progress.log.
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
pnpm spec:first
```

**Notes**

Five protected source/test files and five owning chapters. /tmp/local-completion-agent-review-spec-first.log.
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
pnpm spec:validate
```

**Notes**

Five validators,21 chapters,146 requirements. /tmp/local-completion-agent-review-validate.log.
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

<!-- evidence-field:scope markdown -->
## Scope

Exact local portable agent source validation only. Complete local_agent target qualifies the shared reader and real recorded inputs. HTTP/UI/root aggregates/RustFS/optional inherited Clippy diagnostics outside this read-only leaf; no route, generated contract, startup or UI behavior changed. This proof does not qualify portable review writes, descriptor retention, native capsules or M13 completion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement additive stored portable anchors and typed creation through the shared ReviewStore engine, with complete external selected-record retention, preserved native V1 bytes and receipt/restart proofs before UI enablement. Continue pending code-anchor/native-carrier, search, Snapshot/recorded commands, conversion and M13.6 gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



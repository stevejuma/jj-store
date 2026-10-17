---
schema: 1
id: E-INJECTABLE-STORES-004
---

# E-INJECTABLE-STORES-004 details 3

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
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

Final native slice:162tasks/854records,29protected files/4canonical chapters,7validators/4114chapters/206requirements. Changed Rust and Markdown/JavaScript sources formatted.
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

Rust1.99, JJ0.45.1, Git2.55.0;0.1.0workspace libraries. Deterministic MemoryStore evidence is distinct from real RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged native manifests, WAL, Agent delta/result and Thread binding encodings, original request/publication IDs, receipts and canonical domain records. Host clock and factory interfaces add no on-disk markers.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native library five-store factories over Coordinator; original selected transaction/domain lookup; explicit clock variants preserving legacy defaults; durable capture conversion; native source loading limited to required live domains; neutral review-state errors. New native crate has a concise root README. All161affected tests plus2new public review-state regressions passed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Migrate service/CLI/capture command entry points through factories, add independent external consumer and real-RustFS/cold qualification, and restore retained Vite dev stack. UI/root aggregate/release lanes remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-006 gains native factory and preparation/error contracts,
  documented in the architecture chapter. JST-ARCH-001/002 and
  JST-PUB-001/004/005/007/009 retain independent authority, exact identities,
  selected original receipts and unknown-outcome recovery. Host-clock overloads
  preserve existing entry-point defaults. JST-ARCH-005 adds the coordinated
  package inventory without publication.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M32.3 begins In progress for the native adapter. M32 and earlier
  provider composition stages retain their pending end-to-end qualification;
  M32.4 application/consumer work remains Planned until migration. No milestone
  completion follows from compile or fake-store evidence.

```
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/injectable-collaboration-stores.md
- Cargo.lock
- Cargo.toml
- crates/jj-store-core/README.md
- crates/jj-store-core/src/lib.rs
- crates/jj-store-core/src/stores/commands.rs
- crates/jj-store-git/src/lib.rs
- crates/jj-store-git/src/repository/errors.rs
- crates/jj-store-git/tests/factories.rs
- crates/jj-store-native/Cargo.toml
- crates/jj-store-native/LICENSE.md
- crates/jj-store-native/NOTICE.md
- crates/jj-store-native/README.md
- crates/jj-store-native/src/agent.rs
- crates/jj-store-native/src/commands.rs
- crates/jj-store-native/src/lib.rs
- crates/jj-store-native/src/snapshots.rs
- crates/jj-store-native/tests/factories.rs
- crates/jj-store-native/tests/fixtures/codex-0.156.1-hooks.json
- crates/jj-store-native/tests/support/mod.rs
- crates/jj-store-wal/src/lib.rs
- crates/jj-store-wal/src/publication/agent_delta.rs
- crates/jj-store-wal/src/publication/application.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/review_commands.rs
- crates/jj-store-wal/src/publication/work_commands.rs
- crates/jj-store-wal/src/publication.rs
- scripts/check-boundaries.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/publication/07-selected-agentstore-reads.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M32/history/M32/index.md
- spec/src/current/progress/M32/history/M32.1/index.md
- spec/src/current/progress/M32/history/M32.2/index.md
- spec/src/current/progress/M32/history/M32.3/index.md
- spec/src/current/progress/M32/index.md
<!-- /evidence-field -->


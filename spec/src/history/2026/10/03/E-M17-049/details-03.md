---
schema: 1
id: E-M17-049
---

# E-M17-049 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-control-state-format.log 2>&1
```

**Notes**

All changed canonical files pass; repeated after ledger records final command.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Real private filesystem/OS-lock tests; no MemoryStore, real RustFS or live harness acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New installation-private capture-control state version 1, outside checkouts. Exact installation and random generation; monotonic revision and actor/request/expected-revision/paused receipts. 4096 receipts/1 MiB bound, no eviction. state.json and pinned control.lock with at most one unpublished temporary candidate. Native AgentStore/source/spool/wire formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ttupqlwm
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

This leaf selects durable device policy only. Every operation checks private directory/lock binding, bounded state and safe inventory; ordinary open never resets state. Old replay returns latest policy with original receipt; changed reuse, stale requests and foreign generation withhold. Post-replacement fault is unknown; exact replay repairs durability. New public Rust journal has no consumers yet, so required target is all journal integration/owning unit entries, server all-target compile/strict lint, Rust/document formatting and boundary/architecture/canonical/progress checks. Existing runtime/factory/CLI executable/UI/root aggregates/full Rust workspace and real storage lanes remain out of scope for this leaf. Previous 303 entries remain unchanged. M17/M17.2 remain In progress; foreground durable-pause composition, authenticated management/rescan/import/coverage/degraded-health UI, other readers, remaining faults and real JJ/storage/live harness/cold full flow remain pending. M17.3-M17.7 remain Planned; whole goal active. Automatic enabled startup remains guarded. No installation, migration, publication, deployment, unrelated daemon restart or credentials. Final ten direct journal entries, server all-target compile/strict lint and document/Rust/boundary/architecture/canonical/progress gates pass. Original lock-rebinding functional failure count is one to zero; compile scaffold failures repaired without weakened assertions. Production modules are 34-215 lines. Foreground and authenticated management remain the next slice, not qualified by this leaf.
<!-- /evidence-field -->


---
schema: 1
id: E-M17-048
---

# E-M17-048 details 2

<!-- evidence-field:formats markdown -->
## Formats

Existing runtime constructor and public/durable formats unchanged. Initial memory-only command/applied state derives from settled supervisor Paused phase. No new startup mode, configuration field or durable pause journal.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change powskoqr
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

The existing constructor always chose Running, so a settled paused supervisor refused its first dispatch and the driver stopped. Seed both watch command and driver applied state from Paused when the handed-off supervisor is already settled paused; otherwise preserve Running initialization. The exact regression requires a usable paused runtime, unchanged source allocation/empty queue, explicit resume with historical arming, and a later genuinely new start before sanitized native backfill. Shared constructor/driver initialization directly affects the foreground factory and runtime consumers: exact regression, complete owning library with 16-case cold factory/foreground locks, all runtime entries, server/direct CLI all-target compile/strict lint and Rust/document/boundary/architecture/canonical/progress gates are required. Unchanged explicit supervisor-only/CLI executable/UI/root aggregate/full Rust workspace and real storage lanes are out of scope; the 27 supervisor-only entries just passed E-M17-047 and do not use this constructor. Previous 302 entries stay unchanged. A wrapped canonical context patch miss changed no files before reapplying against formatted text. M17/M17.2 stay In progress. Durable pause persistence and authenticated status/pause/resume/rescan/import/coverage/degraded-health management, other readers, remaining faults and real JJ/storage/live harness/cold full flow remain pending; automatic enabled startup stays guarded. M17.3-M17.7 remain planned; full goal active. No installation, migration, daemon restart, deployment or publication. Final original failure count is one to zero. Exact strengthened regression, 37 direct library/runtime entries, all-target compile/strict lint and Rust/document/boundary/architecture/canonical/progress gates pass. Runtime production module remains below 300 lines. Previous 302 entries are unchanged. Narrow paused-handoff fix complete; durable pause state and authenticated management remain the next composition work, and the full M17 goal remains active.
<!-- /evidence-field -->


---
schema: 1
id: E-M17-035
---

# E-M17-035 details 3

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wpuyupll
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reclaims only settled compact source slots after a known negative physical/root/header binding; unknown metadata or malformed-header errors retain the owner for retry. Retiring removes memory-only deduplication and capacity-refusal hints, invalidates stale queue status and retains exact durable catalog/cursors/padded queued bytes. New replacement sessions arm independently and require fresh observed starts; historical starts cannot enroll. Noncompact raw batches and unfinished proofs, busy/draining/lost owners cannot be evicted. Native A snapshot/content remain unchanged while independently admitted B delivers. Before implementation the exact compact replacement regression timed out after 30 seconds: tracked 1, unavailable 9141 and source_limit_reached true (/tmp/jj-store-agent-capture-m172-rotation-before.log); afterward it passes. Expanded fixture compile failures are retained: matrix.log had five unavailable jj_agent references, matrix-corrected.log five assumed IntegrationId re-exports, matrix-final.log two missing inferred integration references. Corrected using existing typed batch integration and inferred serde codex without dependency additions or weakened assertions; matrix-qualified.log passes three cases and matrix-complete.log all four. Earlier runtime 44/server 53/check/lint/format passes preceded the strict classifier/fourth test; final lanes above qualify the final implementation. Complete shared qualification runtime and native supervisor/publication consumers are justified by the changed binding and lifecycle boundary; direct CLI consumers compile/lint. Production modules remain below 300 lines. All previous 289 entries are preserved. M17/M17.2 remain In progress. Remaining gates include probe rewrite/truncation/rebinding and generation-switching matrices, task-loss/deadline faults, compact spool diagnostics, configured startup/ordered storage shutdown, Cursor/OpenCode readers, authenticated controls/coverage UI and live harness/cold acceptance; M17.3-M17.7 remain pending. Automatic startup stays guarded and AgentStore remains current-only. Root aggregates, unrelated UI/model/crates and real-storage lanes are out of scope. No installed binary/service, daemon restart, credential, checkpoint, live harness, deployment or publication changed. Initial changed-document check found ledger-only formatting drift caused by adding run entries after formatting; final sealed formatting/checks resolve it.
<!-- /evidence-field -->


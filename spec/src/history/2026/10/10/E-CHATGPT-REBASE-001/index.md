---
schema: 1
id: E-CHATGPT-REBASE-001
---

# E-CHATGPT-REBASE-001

<!-- evidence-field:id markdown -->
## Id

E-CHATGPT-REBASE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-012
- JST-API-001
- JST-GOV-002
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ChatGPT rebase qualification and main-workspace landing plan
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change nqtxppsx in chatgpt-plan; rebased implementation szzqmzpx is based on committed main klrylvzw (08b34cc51c64).
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both workspaces were clean. default@ was empty mtupxlqt (813a1409), above klrylvzw (08b34cc5); chatgpt-plan@ was empty above szzqmzpx (8812038e), based on 2cd2903b. Main advanced by one committed inbox/read-lifecycle slice, with eight overlapping paths. The post-rebase narrow lane found one assertion expecting three initial reads instead of the four required by the added ChatGPT account endpoint.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Rebased the implementation and its empty workspace descendant onto committed main. Four ledger/history/navigation conflicts were resolved by retaining both parent additions. Source overlap merged additively: retained reads/history and teardown remain main-owned; ChatGPT account freshness and disposal remain present. Tightened the remount assertion to require the exact four endpoints once across repeated ensures. Both original historical evidence payloads remain identical. Rust/runtime/public contract and generated-client files remain byte-identical to the previously qualified ChatGPT implementation. Main workspace and running application were not moved or restarted. Regenerated M24/M25 history corrects inherited E-INBOX-FRESHNESS-001 summary rows from pass to its actual original pending result; no historical evidence field or milestone status changes. The original rebase table resolution was also normalized within szzqmzpx to keep both historical rows in one valid table.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rebase plus landing planning, not execution of the landing plan. The combined installation reader serves independent inbox and account Settings consumers; the shared freshness test and seven directly affected story families justify full catalog-unit qualification after the exact regression. Root aggregates, full Rust/runtime or contract-fixture reruns, storage/RustFS lanes, user credentials, service restarts, installed CLI replacement, signing, publication and visual baselines are outside this task. Prior Rust/runtime/public-contract evidence is carried forward only after source/artifact identity checks; it is not a fresh live-provider result.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/tests/vite/agent-freshness.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-freshness.test.ts#inbox remounts reuse initial status,
  settings and targets
- web/tests/vite/agent-freshness.test.ts#unchanged renewal and feed outages
  never poll data
- web/tests/vite/agent-chatgpt.test.ts#lost Connect replies retain the exact
  request identity and never send credentials
- web/tests/vite/agent-chatgpt.test.ts#proven command refusal releases the
  draft; polling recovers after an outage
<!-- /evidence-field -->


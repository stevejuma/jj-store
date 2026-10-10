---
schema: 1
id: E-CHATGPT-LAND-001
---

# E-CHATGPT-LAND-001

<!-- evidence-field:id markdown -->
## Id

E-CHATGPT-LAND-001
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
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Verified main source landing and recoverable temporary-workspace retirement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change mtupxlqt in default; source implementation szzqmzpx (1cefbee179d7) and rebase qualification nqtxppsx (2dc41995b55c) remain separate ancestors above klrylvzw (08b34cc51c64).
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both workspaces were clean. Main had empty mtupxlqt (813a1409) above klrylvzw (08b34cc5). chatgpt-plan had empty lnntkxxq (37b5d950) above nqtxppsx (2dc41995), whose implementation parent szzqmzpx was already rebased onto committed main. The verified stack had not yet been selected by the default workspace.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Moved only the empty main working-copy change above nqtxppsx. Main was clean, its tree diff against the verified tip was empty, and current/source tips had no conflicts. Both nonempty source changes remain reachable from main. The fresh five-file account/freshness/chat/execution/inbox regression passes 39 tests; both type checks have zero errors and warnings. Retirement inspection found no open files or process CWD beneath the temporary path. Its tracked tree was clean, with no unique nonempty ancestry. Local fixture and staged desktop resources were absent; ignored dependency, build, index and generated documentation artifacts are retained recoverably in Trash. Forgotten chatgpt-plan and moved only its exact directory to /Users/stevejuma/.Trash/jj-store-chatgpt-plan. Only default remains registered. Main Vite PID 67562 and backend PID 92799 retain ports 7460/7462; browser root remains HTTP 200. Historical source and qualification evidence payloads are retained. After final frozen qualification, separate concurrent UI event/freshness edits appeared in main (application-event-hub.ts, application-events.worker.ts, shared-application-events.ts and application-freshness.ts). These are outside this landing slice and are retained uncommitted by selecting only the nine verified spec/evidence paths.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Source landing and cleanup only; no product code, wire contract, credentials, installed CLI, running service or fixture mutation. Existing rebase/source qualification is carried forward through tree equality rather than claimed as a new full-suite result. Root aggregates, full catalog/browser/Rust/runtime/storage/RustFS suites and architecture audit are out of scope under the targeted policy because this slice changes only landing/evidence bookkeeping. Six inherited architecture size diagnostics remain unchanged as recorded in E-CHATGPT-REBASE-001. Concurrent UI event work is separately owned and is excluded from this record and its path-scoped commit.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/10/E-CHATGPT-LAND-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-CHATGPT-LAND-001/details-02.md
- spec/src/history/2026/10/10/E-CHATGPT-LAND-001/details-03.md
- spec/src/history/2026/10/10/E-CHATGPT-LAND-001/index.md
- spec/src/history/2026/10/10/E-CHATGPT-REBASE-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-chatgpt.test.ts#lost Connect replies retain the exact
  request identity and never send credentials
- web/tests/vite/agent-freshness.test.ts#inbox remounts reuse initial status,
  settings and targets
- web/tests/vite/agent-chat.test.ts#opening any scoped chat is draft-only, Send
  resolves exact tags and retains ambiguous identity
- web/tests/vite/agent-execution-controller.test.ts#stopping an agent targets
  its session without changing another chat or replaying a prompt
- web/tests/vite/inbox-request-lifecycle.test.ts#returning to a chat reuses
  history and feedback, opens one stream and retains drafts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHATGPT-LAND-001 details 2](details-02.md)
- [E-CHATGPT-LAND-001 details 3](details-03.md)

# M33 reusable agent workflows and Notes integration

**Current execution:** NOTES-07 is the only Active task. NOTES-06C/06C4 wait for
its backend integration. Stage-level In progress means started with an open
acceptance gate; parent and child milestones do not represent separate workers.
The delivery checkpoints below record earlier task assignments chronologically.

| ID    | Status      | Depends     | Requirements                | Gate                                                                          | Next                                          | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ----- | ----------- | ----------- | --------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M33   | In progress | M32         | JST-ARCH-007,JST-COLLAB-031 | Reusable capture and conversation libraries consumed by jj-store and Notes    | Complete conversation and local read adapters | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003,E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006,E-REUSABLE-AGENTS-007,E-REUSABLE-AGENTS-008,E-REUSABLE-AGENTS-009,E-REUSABLE-AGENTS-010,E-REUSABLE-AGENTS-011,E-REUSABLE-AGENTS-012,E-REUSABLE-PROGRESS-001,E-REUSABLE-PROGRESS-002,E-REUSABLE-PROGRESS-003,E-REUSABLE-PROGRESS-004,E-REUSABLE-PROGRESS-005,E-REUSABLE-PROGRESS-006,E-REUSABLE-PROGRESS-007,E-REUSABLE-PROGRESS-008,E-REUSABLE-PROGRESS-009,E-REUSABLE-PROGRESS-010,E-REUSABLE-PROGRESS-011,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-012,E-REUSABLE-PROGRESS-013,E-REUSABLE-AGENTS-014,E-REUSABLE-PROGRESS-014,E-REUSABLE-AGENTS-015,E-REUSABLE-AGENTS-016,E-REUSABLE-PROGRESS-015,E-REUSABLE-PROGRESS-016,E-REUSABLE-PROGRESS-017,E-REUSABLE-PROGRESS-018,E-REUSABLE-PROGRESS-019,E-REUSABLE-AGENTS-017 |
| M33.1 | In progress | M32         | JST-ARCH-007                | Public boundaries and preserved-format compatibility fixtures                 | Review shared workflow contracts              | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| M33.2 | In progress | M33.1       | JST-ARCH-007,JST-COLLAB-031 | Standalone native redaction and source readers with original capture recovery | Complete remaining source portability gates   | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003,E-REUSABLE-AGENTS-014,E-REUSABLE-PROGRESS-014                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| M33.3 | In progress | M33.2       | JST-ARCH-007                | Injectable conversation engine and typed client consumed by jj-store          | Add store/workspace adapters and shared UI    | E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-013,E-REUSABLE-AGENTS-014,E-REUSABLE-PROGRESS-014,E-REUSABLE-AGENTS-015,E-REUSABLE-AGENTS-016,E-REUSABLE-PROGRESS-015,E-REUSABLE-PROGRESS-016,E-REUSABLE-PROGRESS-017,E-REUSABLE-PROGRESS-018,E-REUSABLE-PROGRESS-019                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| M33.4 | In progress | M33.3       | JST-ARCH-007,JST-LOCAL-001  | Focused local-history feature and admitted Tauri bridge                       | Add durable native conversation adapters      | E-REUSABLE-AGENTS-007,E-REUSABLE-AGENTS-008,E-REUSABLE-AGENTS-009,E-REUSABLE-AGENTS-010,E-REUSABLE-AGENTS-011,E-REUSABLE-AGENTS-012,E-REUSABLE-PROGRESS-001,E-REUSABLE-PROGRESS-002,E-REUSABLE-PROGRESS-003,E-REUSABLE-PROGRESS-004,E-REUSABLE-PROGRESS-005,E-REUSABLE-PROGRESS-006,E-REUSABLE-PROGRESS-008,E-REUSABLE-PROGRESS-009,E-REUSABLE-PROGRESS-010,E-REUSABLE-PROGRESS-011,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-012,E-REUSABLE-PROGRESS-013,E-REUSABLE-AGENTS-014,E-REUSABLE-PROGRESS-014,E-REUSABLE-AGENTS-015,E-REUSABLE-AGENTS-016,E-REUSABLE-PROGRESS-015,E-REUSABLE-PROGRESS-016,E-REUSABLE-PROGRESS-017,E-REUSABLE-PROGRESS-018,E-REUSABLE-PROGRESS-019,E-REUSABLE-AGENTS-017                                                                                                                                                             |
| M33.5 | Planned     | M33.4       | JST-ARCH-007                | Cold Notes history and one recoverable JJ-backed AI conversation              | Integrate existing AI plugin and Design Core  | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| M33.6 | Planned     | M33.5       | JST-ARCH-007                | Addressed participants, queues and exact anchored discussion recovery         | Integrate shared agent addressing             | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| M33.7 | Planned     | M33.5       | JST-ARCH-007                | Explicit deduplicated imports and opt-in local automatic capture              | Qualify local source lifecycle                | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| M33.8 | Planned     | M33.6,M33.7 | JST-ARCH-007                | Independent Rust/Tauri consumers and public package archive acceptance        | Complete downstream qualification             | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

The first usable milestone is M33.5. Existing M24/M25 and desktop AI gates
retain their original requirements. Code mutations, automatic repository
conversion, metadata synchronization, distribution and publication remain
separate work.

The
[delivery checklist](../../../../../docs/maintenance/reusable-agent-workflows-progress.md)
tracks completed slices, current work, pending tasks, previews and qualification
gaps across jj-store and Notes. Update its stable task IDs after each verified
slice; this ledger retains milestone authority. E-REUSABLE-PROGRESS-001 records
the initial checklist and unchanged milestone review.

## Delivery checkpoint history

E-REUSABLE-PROGRESS-006/008 retain source and complete current-app UI evidence;
E-REUSABLE-PROGRESS-007 clarifies execution status. E-REUSABLE-PROGRESS-009
records cold native history and restart acceptance in desktop `fb238ae2`, with
physical foreground assistance. NOTES-04/04B are complete in that profile;
NOTES-06C is the sole Active task. Unattended activation remains GAP-10. Native
raw-event ownership, durable conversations and downstream gates retain their
original requirements and milestone statuses.
E-REUSABLE-PROGRESS-010,E-REUSABLE-PROGRESS-011 accepts the shared native frame
processor in Lapis AI `6ce65ebb` and refreshed default ACP compatibility in
desktop `242e1fe5`. NOTES-06A is complete within that library scope; the
NOTES-06 parent waits for its native Tauri factory, Notes privacy/durable
capture and complete managed acceptance. Original conversation, downstream and
release gates remain open.

E-REUSABLE-PROGRESS-011 qualifies NOTES-06B in Lapis AI `87958267`, with full
source and six cold default ACP compatibility cases recorded in desktop
`0535bb1e` (E-AI-027). Its native factory preserves current scope and drained
processing; it does not supply Notes privacy or durable storage. NOTES-06C is
Active; NOTES-06D and NOTES-07/08/09 remain pending. M33's original statuses and
first usable milestone requirements are unchanged.

E-REUSABLE-AGENTS-013 adds a headless native capture port in
`jj-store-conversations`: injected privacy/durable sink, bounded canonical
batches, exact receipts and fenced uncertain owners. NOTES-06C1 completes only
this library prerequisite. NOTES-06C remains the sole Active task; Notes-native
privacy/storage and ordered transport of all batch frames remain pending.

E-REUSABLE-PROGRESS-012 qualifies NOTES-06C2: atomic native event groups in
Lapis AI `8a8f9149` and the Notes adapter in desktop `2c6cacb3` (E-AI-028).
Seven cold ACP cases pass with foreground assistance, including deferred text
and withheld receipts. Synthetic privacy and private synced candidates do not
qualify production privacy, JJ publication or restart recovery. NOTES-06C is the
sole Active task; NOTES-06C3 is its pending production group-consumption step.
Stage-level In progress means started with an open acceptance gate, not
simultaneous execution. M32 stays Done; M33/M33.1-M33.4 In progress and
M33.5-M33.8 Planned. Desktop M13.3 is In progress with its original gate open.

E-REUSABLE-PROGRESS-013 completes NOTES-06C3 in desktop `6e8ebd17` (E-AI-029):
production complete-group consumption, optional frame provenance and replay
deduplication in both legacy transcript stores. Eleven focused/hash, 198
renderer/governance, 389 AI policy, all 696 catalog checks and seven cold ACP
cases pass. Native tests require foreground assistance; prior setup and
transient WebView failures remain in consumer evidence. NOTES-06C is the sole
Active task; NOTES-06C4 is next for admitted privacy and exact backend
ownership. Actual JJ sink selection depends on NOTES-07. M32 stays Done;
M33/M33.1-M33.4 In progress and M33.5-M33.8 Planned. No production privacy,
AgentStore/ThreadStore conversation, restart or release gate is completed.
E-REUSABLE-AGENTS-014 qualifies the opt-in native privacy bridge in
`jj-store-conversations`. It selects the existing Agent Redaction policy and
retains a scanner-free default dependency closure. Five real-scanner cases, 20
default/25 opt-in crate cases and boundary/CI/manual checks qualify the source
prerequisite NOTES-06C4A. NOTES-06C is the sole Active task; Notes adoption,
current native scope, actual durable backend and restart remain pending. The
optional profile's automatic CI execution remains part of standalone
qualification. Existing M32/M33 statuses and gates are unchanged.
E-REUSABLE-PROGRESS-014 completes NOTES-06C4B in Notes `51bed526` (E-AI-030):
real shared native privacy with native-owned scope/cache and explicit ordinary
backend refusal. Both native profiles pass 112 tests (two inherited ignored
each), strict lint and normal/acceptance builds pass; all eight cold ACP cases
pass with foreground assistance. The candidate sink remains synthetic. NOTES-07
is now the sole Active task for receipt-aware AgentStore/ThreadStore adapters
and per-vault selection. NOTES-06C/06C4 wait for that backend; their
publication/drain and NOTES-06D/08/09 restart gates remain open. M32/M33
statuses and original gates are unchanged. E-REUSABLE-AGENTS-015 qualifies
NOTES-07A: an additive native-capture normalizer retains exact sanitized
envelopes and native progress while reusing existing controller entity
projection. Five model cases and real Git-provider atomic Agent/Thread
publication, fresh opening and receipt replay pass; all 70 affected crate tests
and strict lint pass. NOTES-07 remains the sole Active task for its reusable
receipt-aware adapter and Notes backend selection. NOTES-06C/06C4 wait for that
backend. No Notes journal, managed JJ conversation, native writer drain or
restart gate is completed; M32/M33 statuses are unchanged.

E-REUSABLE-AGENTS-016 qualifies NOTES-07B: optional selected Agent/Thread
capture retains sanitized candidates and exact preparation before publication
and confirms original receipts before output. Real Git/provider and fsynced
test-host journal proofs include lost replies, abandoned waits, stale/foreign
Threads and reopen without re-execution. Native manifest/WAL preparation and
receipt recovery use memory object storage, distinct from real RustFS. NOTES-07
remains the sole Active task for Notes' production journal, explicit per-vault
backend selection and native lifetime integration. NOTES-06C/06C4 wait for that
backend. M32 stays Done; M33/M33.1-M33.4 stay In progress and M33.5-M33.8
Planned with original gates unchanged. No actual Notes JJ conversation, cold
restart, native drain, UI/visual or release gate is promoted.

E-REUSABLE-PROGRESS-015 completes NOTES-07C in Notes `e88f1487` (E-AI-031): an
exclusive private journal retains checked atomic synced records and current
admission. Cold Rust-process lease/reopen and real-JJ native privacy/store
composition prove source preservation and original-receipt-only recovery. Both
native Rust profiles pass 120 cases with two inherited ignored each; strict
Clippy, builds, types and 198 renderer/governance cases pass. M33's statuses and
original gates remain unchanged. NOTES-07 is the sole Active task for explicit
per-vault backend selection and native lifetime wiring. NOTES-06C/06C4 wait for
that integration; actual managed chat, native writer drain, packaged-app cold
restart and UI/visual/release gates remain open.

E-REUSABLE-PROGRESS-016 completes NOTES-07D in Notes `a48fce96` (E-AI-032):
native private backend profiles retain exact choices, original receipts and an
exclusive admitted owner. Preparation precedes selection; missing, malformed or
uncertain state refuses another writer. Immediate close fences unpolled waits
and outstanding calls retain the lease. Cold Rust-process ownership and real JJ
witness/source preservation pass. Both native profiles pass 127 cases with two
inherited ignored each, strict lint/builds, types and 198 renderer/governance
cases pass. NOTES-07 remains the sole Active task for ordinary host/IPC,
renderer backend selection and managed callback ownership. NOTES-06C/06C4 wait
for that integration. M32/M33 statuses and original gates are unchanged; no
actual app chat/restart, native runtime drain or release gate is promoted.

E-REUSABLE-PROGRESS-017 completes NOTES-07E in Notes `81765a34` (E-AI-033):
ordinary native backend ownership and bounded text IPC retain exact choices,
receipts and scope handles. Admission, backend operations and closure share
native transitions. Cancellation retains IO ownership; reload refreshes handles
and delayed old closes cannot revoke a replacement. Destruction fences delayed
admission. Both Rust profiles pass 136 cases (two inherited ignored each), all
16 AI-host cases, strict lint/builds, types and 198 renderer/governance cases
pass. Three cold hidden-WebView cases qualify actual IPC, read-only JJ source
preservation, saved-choice/receipt reopening and existing admission/history.
Native API acceptance is separate from visible UI and managed conversation
restart. NOTES-07 remains the sole Active task for conversation/session
preparation, store/processor construction and renderer persistence.
NOTES-06C/06C4 wait for that integration. M32/M33 statuses and original gates
are unchanged; actual managed capture, app chat/restart, visual/provider and
release gates remain open.

E-REUSABLE-PROGRESS-018 completes NOTES-07F1 in Lapis AI `e12d303c`: a Rust-only
retained processing grant keeps exact native capture authority during renderer
suspension. Commands, factories and startup completion still require current
admission; native grant and registry ownership fence every callback. Five
focused cases, all 34 host cases, 1,018 Rust passes (two inherited ignored), 39
renderer and 45 governance cases, strict checks/builds, package inventories and
all ten specification lanes pass. Notes adoption, real store/processor
construction and managed conversation/restart remain pending. NOTES-07 stays the
sole Active task; NOTES-06C/06C4 stay Waiting. M32/M33 statuses and original
gates remain unchanged. Authorized maintenance preserves the development fixture
and restarts the stack; no release, provider or visual gate is promoted.
E-REUSABLE-PROGRESS-019 qualifies NOTES-07F2 in Notes change `pquxnosp`
(E-AI-034): exact retained permits, physical source fences and logical lease
release pass both 147-case native profiles (two inherited ignored each), strict
lint/builds and three cold hidden WebView compatibility cases. Actual managed
conversation/processor and restart remain open. The requested desktop Column
Canvas commit log and separate Op Log view are NOTES-05F, the only Active task.
NOTES-07 waits during this UI slice; NOTES-06C/06C4 still await its backend.
M32/M33 statuses and original gates remain unchanged. E-REUSABLE-AGENTS-017
qualifies basic operation graphs and exact parent deltas from standard prepared
history handles, plus faithful referenced native status tags in the typed
client. Real source/index preservation, exact parent ancestry and malformed tag
regressions precede package/direct-consumer checks. Notes' Column
Canvas/AppShell body, ID controls, expanded file lists and matching diffs remain
NOTES-05F, the sole Active task. Managed conversation/restart and M33's original
gates/statuses are unchanged.
<!-- generated-navigation -->

## Pages

- [Evidence history](history/index.md)

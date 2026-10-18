# M33 reusable agent workflows and Notes integration

| ID    | Status      | Depends     | Requirements                | Gate                                                                          | Next                                          | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----- | ----------- | ----------- | --------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M33   | In progress | M32         | JST-ARCH-007,JST-COLLAB-031 | Reusable capture and conversation libraries consumed by jj-store and Notes    | Complete conversation and local read adapters | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003,E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006,E-REUSABLE-AGENTS-007,E-REUSABLE-AGENTS-008,E-REUSABLE-AGENTS-009,E-REUSABLE-AGENTS-010,E-REUSABLE-AGENTS-011,E-REUSABLE-AGENTS-012,E-REUSABLE-PROGRESS-001,E-REUSABLE-PROGRESS-002,E-REUSABLE-PROGRESS-003,E-REUSABLE-PROGRESS-004,E-REUSABLE-PROGRESS-005,E-REUSABLE-PROGRESS-006,E-REUSABLE-PROGRESS-007,E-REUSABLE-PROGRESS-008,E-REUSABLE-PROGRESS-009,E-REUSABLE-PROGRESS-010,E-REUSABLE-PROGRESS-011,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-012,E-REUSABLE-PROGRESS-013 |
| M33.1 | In progress | M32         | JST-ARCH-007                | Public boundaries and preserved-format compatibility fixtures                 | Review shared workflow contracts              | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| M33.2 | In progress | M33.1       | JST-ARCH-007,JST-COLLAB-031 | Standalone native redaction and source readers with original capture recovery | Complete remaining source portability gates   | E-REUSABLE-AGENTS-001,E-REUSABLE-AGENTS-002,E-REUSABLE-AGENTS-003                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| M33.3 | In progress | M33.2       | JST-ARCH-007                | Injectable conversation engine and typed client consumed by jj-store          | Add store/workspace adapters and shared UI    | E-REUSABLE-AGENTS-004,E-REUSABLE-AGENTS-005,E-REUSABLE-AGENTS-006,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-013                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| M33.4 | In progress | M33.3       | JST-ARCH-007,JST-LOCAL-001  | Focused local-history feature and admitted Tauri bridge                       | Integrate native raw-event capture            | E-REUSABLE-AGENTS-007,E-REUSABLE-AGENTS-008,E-REUSABLE-AGENTS-009,E-REUSABLE-AGENTS-010,E-REUSABLE-AGENTS-011,E-REUSABLE-AGENTS-012,E-REUSABLE-PROGRESS-001,E-REUSABLE-PROGRESS-002,E-REUSABLE-PROGRESS-003,E-REUSABLE-PROGRESS-004,E-REUSABLE-PROGRESS-005,E-REUSABLE-PROGRESS-006,E-REUSABLE-PROGRESS-008,E-REUSABLE-PROGRESS-009,E-REUSABLE-PROGRESS-010,E-REUSABLE-PROGRESS-011,E-REUSABLE-AGENTS-013,E-REUSABLE-PROGRESS-012,E-REUSABLE-PROGRESS-013                                                                                                                                                             |
| M33.5 | Planned     | M33.4       | JST-ARCH-007                | Cold Notes history and one recoverable JJ-backed AI conversation              | Integrate existing AI plugin and Design Core  | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| M33.6 | Planned     | M33.5       | JST-ARCH-007                | Addressed participants, queues and exact anchored discussion recovery         | Integrate shared agent addressing             | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| M33.7 | Planned     | M33.5       | JST-ARCH-007                | Explicit deduplicated imports and opt-in local automatic capture              | Qualify local source lifecycle                | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| M33.8 | Planned     | M33.6,M33.7 | JST-ARCH-007                | Independent Rust/Tauri consumers and public package archive acceptance        | Complete downstream qualification             | -                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

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
<!-- generated-navigation -->

## Pages

- [Evidence history](history/index.md)

# Verification

The Native activity story verifies a 120-row feed with bounded mounted rows,
deferred association/session reads, measured expanding context, memoized reads
after virtual remount and automatic continuation to 160 rows in shadcn
ScrollArea. Controller regressions cover the two-reader budget and captured
AgentStore root. Cold browser acceptance uses the app's Back and Forward
controls to return to the exact Activity route, and confirms the repository-wide
Reviews heading replaces the workspace picker row. Existing sample sessions have
no recorded code associations; read-only live acceptance keeps that absence
explicit while fixture acceptance exercises genuine relation contracts.

Native review detail stories also exercise full-width Community list rows,
list-to-detail navigation, header return behavior, hidden tabs in detail,
responsive body inset and vertically centered Back control. Route regressions
qualify retired column links and reject invalid legacy mode fields. The reviews
page exposes no column styling or sidebar layout controls.

E-UI-046 qualifies far-end mobile details controls, Activity-first landing tabs,
complete native identifier copy menus and Community hover/focus reply/edit
composition. Production controller regressions pin edit-start revisions, retain
unknown request identity, block replacement commands and reject late receipt
completion after access invalidation. Native message stories prove own-author
editing, read-only copy, exact parent replies and retained edit context. Agent
menu stories prove full native clipboard values and unchanged selection.
Captured transcripts stay immutable. Visual baselines remain pending.

E-UI-047 covers Activity-first project and hub navigation, the Community-style
recorded-agent filter in the right rail, root-pinned metadata-only
classification, two-reader bounds, unavailable context and cancellation.
Repository Projects regressions retain repository tabs, authorization, code
operation and separate linked-project cache across metadata homes. Five document
tabs and existing file and edit-preview consumers use Community's public safe
Markdown renderer; source retains original bytes. Cold owned-demo checks
preserve native state, prove document formatting and three-pixel indicators,
project filters across Back/reload, virtual Activity and exact review anchors.
Visual approval remains pending and no native service contract or persistence
format changed.

Configured Codex whole-server startup for JST-COLLAB-017/JST-COLLAB-030 and
JST-CLI-005 is qualified by
`crates/jj-store-cli/tests/server_executable/configured_capture.rs#cold_configured_codex_capture_initializes_publishes_and_preserves_pause`
and
`crates/jj-store-server/src/process/tests/capture_factory/startup.rs#server_capture_initializes_only_absent_state_and_preserves_existing_policy`.
The cold executable fixture uses the pinned embedded RustFS runtime, an actual
native workspace and protocol-protected local HTTP. It checks disabled and
configuration-check non-initialization, private fresh journals, inactive
discovery, new activity publication, unchanged pause/generation/source
identities across restart, resume, damaged-state degradation without repair and
ordered shutdown. Synthetic transcript input qualifies configured lifecycle and
real storage; it does not qualify a real agent edit or all harness surfaces.
Other readers, browser management and later M18 gates remain separate. E-M17-069
retains exact commands and failures.

Installation browser management for JST-UI-016, JST-COLLAB-017 and
JST-COLLAB-030 is qualified separately by E-M17-070. The exact
`web/tests/vite/capture-management.test.ts` regressions exercise local protocol
transport, exact uint64 policy revisions, unknown/malformed/mismatched outcomes,
same-intent retries, missing or mismatched receipts, stale/duplicate source
continuations, single-flight teardown and evidence-qualified history completion.
`web/tests/vite/production-capture-management.test.ts` proves LocalTrusted
bootstrap capability admission, operation without a selected repository,
installation intent preservation on repository disconnect and transport closure
at application teardown. Existing bootstrap consumers remain passing.

`catalog/components/store-application/StoreCaptureSettings.stories.svelte`
exercises the owned production panel with in-memory fixtures and blocking
interaction/accessibility checks. A fresh built browser in
`web/tests/browser/local-capture-settings.spec.ts` verifies protected token-free
installation requests, exact large revisions, settings-close/reopen retries,
policy versus worker acceptance, explicit rescan and history import, missing
receipt versus published boundary versus live-tail progress, unchanged
repository hash/snapshot, capability absence and desktop/mobile accessibility.
These are browser presentation/transport checks using controlled API responses,
not live harness or real-storage ingestion qualification. Existing configured
real-storage startup evidence remains E-M17-069; remaining readers and later M18
gates stay open. Stories remain visual-pending and no visual baseline is
approved.

API snapshot publication proof for JST-COLLAB-017/JST-COLLAB-030 has separate
E-M17-077 evidence. The exact native-server regressions in
`crates/jj-store-server/tests/opencode_publication.rs` publish empty-session
busy/idle snapshots through current AgentStore on MemoryStore, reopen the spool
and distinguish acknowledgement from committed receipts. A revoked repository's
earlier status remains a hole even after a later repository's message publishes;
restoring access permits the original receipt chain to advance. This is native
publication acceptance with controlled API input, separate from real harness,
RustFS and configured process startup qualification.

`crates/jj-store-capture/tests/snapshot_publication.rs` qualifies identity-only
completion, exact unknown/mismatched retry, numeric generation replacement with
smaller logical counts, missing earlier channels and unknown history, accepted
boundary versus unavailable later tail, body-free inspection, capacity refusal
and selected-file failures without repair. The catalog unit regressions preserve
all 4096 outstanding snapshot requirements under pressure and unchanged JSONL
encoding. Existing JSONL publication and management inspection are direct
consumers of the shared catalog/inspection lock and retain their required lanes.
Snapshot proof supplies no import consent or source authority. Public API
snapshot management/import/export, identity joins and Cursor remain open before
M18.3-M17.7.

E-M17-079 qualifies shared native OpenCode identity adoption for JST-COLLAB-020.
The exact
`opencode_hook_api_and_export_adopt_one_selected_session_message_and_tool`
regression first observes two sessions for one native identity, then passes with
one selected session/message/tool across three independent channels. All source
cursors and sanitized evidence remain separate, and earlier selected roots
preserve their original snapshots. Additional regressions retain exact
case-sensitive IDs, exclude generic labels from common joins, select the common
OpenCode projection precondition and withhold ambiguous prior aliases while
retaining diagnostic source receipts.

The local companion regression qualifies explicitly supplied OpenCode
hook/API/export records across physical repository reopen, original receipt
replay and pinned earlier roots through the same projector. This adds no local
automatic reader, OpenCode history-import consent or filesystem export driver.
The complete native AgentStore, bound-capture and directly consuming local
agent/collaboration binaries, native API source/publication tests and bounded
lookup units form the affected surface. Strict affected-package/all-target
checks preserve native and local publication authority. Existing Codex joins
retain their prior key and UUID spelling rules. Public schemas, UI and unrelated
repository suites are unchanged and out of scope. Real harness/RustFS/cold
release and the remaining M18 gates remain separate.

E-M17-079's complete selected consumer run passes 69 tests across eight native
and local binaries, plus the bounded alias-lookup unit. The current-descriptor
tests separately preserve the rejection of the obsolete local descriptor; the
publication chapter's stale readability sentence is corrected to match this
already-qualified behavior. Both modified production modules and new test leaf
modules remain within their existing budgets. Shared public-contract generation
is out of scope because no public shape changed in this identity slice.

E-M17-078 exposes receipt-backed API source publication through the existing
installation management page. The actual HTTP fixture
`api_snapshot::api_snapshot_progress_is_native_receipt_backed_and_body_free`
publishes an empty busy session through current AgentStore on MemoryStore, then
qualifies offline, paused and stopped inspection, corrupt selected evidence,
Admin revocation and disabled responses. Pending-body corruption cannot affect
inspection; neither policy nor import intent changes. One bounded batch reads
selected publication/reader metadata for at most 100 associations under existing
coordination. File-only rows retain their prior encoding; missing or unavailable
evidence omits the optional closed response record.

The browser transport preserves exact uint64 sequence positions while accepting
safe JSON numbers. Client regressions reject unsafe/negative/overflowing counts,
an impossible committed frontier, a message count without qualified receipts,
unknown private fields and a snapshot record on another harness profile. A
replacement generation may have fewer captured messages than an earlier
published frontier; these counts are deliberately not compared. Primitive and
snapshot validation have separate leaf modules under the existing line budget.
The complete registered HTTP/SSE inventory, generated contracts and their tool
tests are required because the shared response schema and fixture runner
changed. Selected Rust consumers, strict affected-package checks and client
checks remain distinct from real OpenCode/RustFS and full UI/cold release
qualification. No renderer or source/import authority is added. OpenCode history
import/export, common identities, Cursor and M18.3-M17.7 remain pending.

E-M17-078's complete 50-test management module passes sequentially without a
concurrent build, alongside two API ownership units, one runtime observation
unit, two HTTP contract units, 61 selected reader/management consumers and 12
client tests. Its complete generated-contract lane validates 199 actual HTTP/SSE
responses plus seven contract-tool tests and generated-client TypeScript. An
earlier sequential module run overlapped that lane's CLI rebuild and hit the
same existing ten-second import deadline: 49 passed and one timed out. The exact
unchanged case passes alone in 8.28 seconds, followed by the complete unchanged
module pass. Both observations remain in the ledger; concurrent-build timing
remains unqualified. No timeout, assertion or module budget was relaxed.

The E-M17-077 management module qualifies all 50 original tests sequentially.
Its earlier parallel run exceeded the existing ten-second multi-step import
publication deadline in one test; that exact test and the full module pass with
unchanged assertions and deadlines. This preserves the failed observation and
limits parallel-load qualification without substituting the isolated pass for
the complete selected module. The 186 passing top-level count excludes duplicate
verification and separately invoked child fixtures. Strict affected-package
checks pass; the native test linker reports its compact-unwind size diagnostic.

M18 integration adds cached-reader append-witness retention and capture UI
lifecycle regressions. The real HTTP import capture accepts only documented
unknown acknowledgement during cold sanitizer preparation, then independently
observes completion of the original request and verifies exact replay with one
journal entry. It still requires final 200, the same command and immutable
receipt, original completed boundary, pause and removed-source replay, actor
refusal and policy revocation. This strengthens cold unknown-outcome coverage
without increasing the production acknowledgement budget.

Cold import completion observation allows a bounded 240 seconds for real native
sanitizer preparation and publication; observed cold preparation exceeded 120
seconds even with the live preview paused. Ordinary settled observations remain
bounded to ten seconds. Production acknowledgement still uses five seconds. This
cold gate requires the same original receipt and exactly one accepted journal
entry.

The cold API snapshot HTTP contract allows a bounded 120 seconds for initial
native sanitizer preparation. Its receipt-backed, body-free inspection,
stopped-harness, policy-revocation and stopped-worker assertions remain
required; production deadlines are unchanged.

The CLI transcript profile regression qualifies `exec` and the inherited Desktop
originator against the pinned Codex 0.156.1 recorder and originator override
contract. It retains rejection of Desktop `vscode`, foreign versions and
unqualified originators. This profile proof does not bypass the separately armed
fresh-activity and current native-directory admission gates.

Real-location Capture UI qualification includes a cumulative unavailable count
above 4096 with usable controls. The regression retains refusal of negative
counters and tracked counts beyond the source bound. Cold unknown import receipt
recovery retries only the exact original body through documented transient busy
responses, with bounded observation and the same final receipt/journal
assertions.

The E-M18-001 real-service preview uses Codex CLI 0.156.1 in the normal session
location and isolated native embedded RustFS. It qualifies two linked physical
workspaces, a real authorized file edit, automatic Agents updates, retained
selection/draft, pause with withheld appended content, paused restart, resume
and a separate explicit inactive-history import. A final resumed restart retains
all three native session/snapshot/transcript identities and record counts 13, 5
and 3. Distinct response markers occur once; deliberately unlinked trial bodies
are absent. The accepted history receipt retains its original command, complete
boundary 79,990 bytes/15 source records and one publication receipt. An earlier
import selected the deliberately unlinked source and remained withheld with an
unknown acknowledgement and no accepted receipt. Its exact request remains
recorded; it is not classified as rejected from receipt absence. Body-free
source-selection clarity and deterministic pre-acceptance refusal, cold debug
preparation performance, registry-only Design Core release qualification and
visual baselines remain explicit limitations. This qualifies the local Codex
preview only; Desktop and the broader M18 roadmap stay deferred.

[E-M20-005](../../../current/progress/index.md) qualifies the Design Core compact Panel Toggle
defaults under JST-UI-020. The workspace app-shell story measures header/rail
geometry and collapse/expand; stack workspace and repository log layouts retain
focused interaction/accessibility acceptance. Source qualification and registry
release qualification remain distinct.

[E-M20-006](../../../current/progress/index.md) covers the selectable workspace target card above the
Branches column's filters and its separate read-only history stack. The
240-commit example loads 40-commit pages, renders only the visible revision
rows, and expands selected commits' changed files inline with adjacent read-only
diffs. Controller regressions cover pinning, concurrent request deduplication,
exhaustion, retries, cursor bounds, refresh invalidation, teardown and retained
ordinary stack selections. Browser regressions cover keyboard selection of a
different target branch, the card's placement, removal of the redundant header
above the page tabs, and measured header/icon/content alignment using the
displayed graph's inset. This remains a mocked UI slice; physical checkout,
persistence, backend and registry qualification remain separately gated, and
visual baselines are unchanged.

[E-M20-007](../../../current/progress/index.md) covers the public fixed-column Unstaged integration:
expanded and collapsed viewport-edge geometry within one CSS pixel, retained
widths, file controls, adjacent working diffs and staging into a later stack.
Design Core source commit `f19b990e` owns capacity fallback and restoration,
adaptive pair allocation, logical direction, lifecycle and original-DOM pinning;
the consumer uses its existing temporary source-validation dependency with
portable manifests. Registry release qualification and backend integration
remain separate gates.

[E-M21-028](../../../current/progress/index.md) qualifies only exact tree-changing checkout
preparation. Its real unpublished native-operation fixtures bind retained
source/candidate trees, binary/executable/addition/deletion effects and native
checkout inputs. Adversarial cases reject dirty/missing/wrong-mode source files,
matching-content untracked/ignored targets, unsafe files/parents, aliases,
file/directory transitions, unsupported symlink candidates, oversized payloads,
replaced identities and tampered canonical plans. Cancellation retains the
actual foreground lock until worker exit; killed-worker reopening revalidates
the original saved plan without checkout/publication. Provider tests use typed
test guards; complete real scanner ownership remains separately qualified by the
existing service and registered-folder evidence. Durable plan/journal selection,
interrupted installation, native tree-state completion, preserved dirty affected
WIP, wider file/sparse profiles and production command/UI flows remain required.
No mutation capability or milestone completion follows from these preparation
passes.

[E-M21-029](../../../current/progress/index.md) covers separately bounded durable tree-plan input
storage and original-request journal references. Real unpublished JJ candidate
fixtures verify immutable reopen/replay, provider revalidation and interruption
between plan and reference selection without publishing code or modifying
physical/checkout state. Synthetic metadata pressure and replacement faults
qualify capacity and unknown acknowledgement behavior separately from engine
evidence. Legacy journal bytes remain unchanged and new references cannot be
attached to terminal outcomes. Actual interrupted physical installation, native
tree-state completion, combination after-state and production command/UI flows
remain required; production capabilities stay read-only.

[E-M21-030](../../../current/progress/index.md) covers complete original physical-inventory capture.
Scanner fixtures verify canonical roundtrip and parity with the unchanged
physical digest, including empty directories, ignored/binary/executable files,
symlinks and Unicode paths. Adversarial data tests reject malformed, incomplete,
conflicting, unsafe, noncanonical, oversized and wrong-scope input. Registered
real-native guards verify authorization, editor-state rejection, optional
capability refusal and shared admission through cancellation; explicit
real-folder observer fixtures retain ordinary scan behavior. These passes
qualify original input only. Durable linkage, per-path installation progress,
interrupted checkout and native tree-state completion remain required.

[E-M21-031](../../../current/progress/index.md) covers durable original-inventory linkage. Real
native candidate/scanner fixtures verify body-before-reference selection, exact
reopen/replay, complete binding checks, journal-only references and changed-tree
publication admission. Abrupt subprocess exits on both sides of reference
selection retain the exact original input and leave code/checkout/files
unchanged. Missing bodies, malformed references, wrong actors and terminal
upgrade attempts fail without repair or rewritten history. Metadata-only
replacement/unknown-acknowledgement tests remain distinct from engine evidence.
Per-path installation progress and actual interrupted checkout/native tree-state
completion remain required; production Stacks stays read-only.

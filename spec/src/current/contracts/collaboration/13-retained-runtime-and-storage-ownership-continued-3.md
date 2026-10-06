# Retained runtime and storage ownership (continued 3)

Native Codex observation envelopes retain `capture_format=codex-jsonl/v1`, the
exact product version, validated native session identity and sanitized native
record. Only transcript/local-transcript evidence enters this parser; the new
Desktop 0.159.2 normalizer profile does not qualify Desktop hooks. Known task
start/completion/abort events project active/idle state. Native user/assistant
text items retain message identities, with source-sequence identities when the
native item has no ID. Missing IDs use a separate identity namespace and never
claim a native message alias. Tool-call requests project Pending and never imply
tool execution or success from a response-item status. Function/custom-tool
outputs first become typed sanitized observations without activity or alias
publication. Resolve them only against a matching same-batch call or the exact
source/session/tool aliases, or the qualified common-native alias proofs below,
and tool snapshot in the selected repository root. Retain known names, kinds,
turn IDs, input and terminal evidence; receiving output alone projects Unknown,
never success based on output text or arbitrary status fields. Result-only
batches update the matched selected session and retain evidence without changing
session lifecycle. Missing/mismatched aliases, invalid outputs and unsupported
records retain safe coverage diagnostics without inventing sessions or tools. A
different native session is not a match; another source requires the qualified
common-native proof. Rich blocks and full conversation semantics retain separate
gates.

Transcript sanitization uses the same pinned in-process Kingfisher scanner with
an explicit transcript path policy. Workspace, home and temporary roots are
substituted before scanning; substitutions include object keys and values and
ambiguous key collisions omit the record. The scanner is initialized before
source conversion and reused within the batch. Raw invalid/oversized records,
scan failures and unsupported payload sizes become constant safe omissions. No
source body or privacy-root path reaches a durable pending entry before this
boundary. Reader intent retains deterministic observation/batch IDs and the
original cursor; retries do not change native IDs or prepared request meaning.
Hook sanitation policy and bytes remain unchanged. This conversion alone grants
no repository matching, active-session admission, publication or checkpoint
authority; source-context partitioning and worker policy remain M18.2 gates.

One reusable capture runtime serves hooks and the server; jj-agent owns domain
models, the existing JJ boundary owns client access and authorized service owns
publication. The runtime lives in jj-store-capture and preserves existing spool,
prepared request, receipt and sanitization bytes through CLI reexports. Neither
the models nor publication authority depend on this runtime. Extraction alone
does not enable discovery, source readers, checkpoints or publication workers.
One reader leases a source; four readers may run concurrently. Watch
notifications are hints, with bounded 30-second rescans. Partial UTF-8/JSON,
rewritten snapshots, replacement, rotation, compaction and delayed final flush
have explicit generation/cursor semantics. Advance only after sanitized spool
durability. Existing payload/spool limits apply; full queues stop reads without
evicting acknowledged evidence. Private state lives outside checkouts.

Reader enqueue durably retains the sanitized pending batch before acknowledging
its device cursor. An optional reader metadata field records only a versioned
source-generation digest; absent fields preserve legacy pending bytes. A private
per-repository/source/generation cursor journal retains the accepted parser,
position, native alias, last sequence and batch identity after publication
removes the pending entry. Journal updates are atomic and owner-only, under the
existing mutation lease. Restart reconstructs missing cursor progress from
pending sanitized evidence, and native/local completion reconciles that progress
before removing the prepared request. Cursor-write failure retains pending
intent. Contradictory generation identities, parser changes within a generation
and regressive offsets/sequences fail before new queue admission. Journal
capacity is 4096 bounded entries of at most 8 KiB each; a full journal stops new
generation admission and never evicts acknowledged progress. These cursors are
private reader state, not repository authority, code associations or model
activity.

Repository partitions share one physical source cursor per integration, source
ID and generation. Admission validates parser, generation witness, native alias,
sequence and source position against all retained partitions and sanitized
pending entries; a changed repository cannot reset progress or replay completed
offsets. Latest-source lookup selects the validated latest generation/sequence
across partitions. Ambiguous equal offsets, contradictory witnesses and
regressive positions fail closed. The existing per-repository V1 journals remain
retained with unchanged encodings, exact prepared bytes and completion repair;
new partitions consume the same 4096-entry budget without eviction. Restart
recovers every repository journal before selecting the physical cursor. Legacy
generation interpretation remains unchanged; qualified Codex readers continue to
preserve monotonic source sequence numbers across new generations.

Automatic readers MUST select retained progress by their exact physical/header
generation digest across all repository partitions, rather than consuming a
different file's latest cursor. One bounded snapshot under the spool mutation
lock supplies that selection and independent channel-wide generation/sequence
floors. Unchanged archived and current files cannot alternate new generations or
replay acknowledged bytes. A delayed append to an older physical generation
continues from its verified prefix; if another generation has advanced, its next
batch allocates above the channel generation floor without rereading that
prefix. Rewrite/truncation still starts above that floor and resets the physical
offset.

Fresh lease acquisition and every automatic ingress step refresh these bounds.
Only unallocated reading state may adopt newer sequence floors. An exact pending
raw batch that conflicts with independent channel advancement is withheld with
its original identities and bytes retained, never renumbered or discarded.
Automatic sanitized enqueue checks channel-wide generation and first-sequence
bounds atomically under the mutation lock; exact already-durable retries retain
their existing interpretation. Snapshot lookup retains exact typed capacity
refusals if another spool handle exceeds the current handle's byte limit;
refused lookup cannot advance the reader or mutate accepted queue evidence.
Generic reader APIs and all existing cursor, pending, prepared-request and
receipt encodings remain unchanged. Automatic parked raw-channel reservations
exclude competing physical readers until that allocation settles. Independent
uncoordinated advancement still withholds conflicting exact raw batches. Broader
allocation, rotation and task-loss race qualification remains an explicit gate.

A bounded native drain step selects one explicit repository/integration/source
partition from the installation's shared sanitized spool. It publishes at most
one through 256 queued batches per call, in retained generation/source order,
and leaves all other partitions untouched, including local prepared requests.
The supervisor schedules partitions fairly; this selector itself grants no
permission or fairness. Current repository admission and publication
authorization must precede its use. Existing whole-spool CLI drains retain their
strict single-repository/native-only check and legacy behavior.

A native publication scheduler selects one exact pending head per
repository/integration/source partition in retained generation/source order. Its
memory-only cursor advances at selection, so withheld, failed, cancelled or
unknown delivery does not reserve the next turn. Successive selections rotate
through currently pending partitions, including newly added ones, and wrap after
the last partition. Empty selections retain the cursor. Restart may reset the
turn cursor but never the durable queue, prepared requests or receipts. Each
inspection is bounded by the existing 4096-entry spool limit. Invalid or
duplicate batch identities fail without advancing the cursor. Any local-prepared
entry excludes its entire partition from native scheduling; foreign local intent
remains untouched. The selection is a scheduling hint, not proof of activity,
directory admission, sanitization, authorization or publication.

Exact-entry delivery MUST recheck the live queue under the drain lease and
refuse a selected batch if an earlier entry remains in that same partition. This
includes an earlier entry enqueued after selection and an unresolved prepared
request. It must call no publisher and change no durable intent when order is
unavailable. All entries in a selected partition must be native; local-prepared
intent withholds that partition. A different partition's earlier entry cannot
block exact delivery. Each selected head retains its independent exact device
proof and current Write requirement; scheduling cannot lend another batch's
proof. This metadata scheduler does not start workers or qualify blocking-I/O
ownership, deadlines, independent ingress or full fair supervision.

Selected local prepared requests fail before publication. Unknown outcomes stop
only the selected step with the exact prepared request retained; another
partition can be handled in a later call. Cancellation, mismatched receipts and
preparation failures preserve original intent. Reports retain the existing shape
and count the complete remaining shared queue, never implying that unrelated
work committed. Selecting no matching entries calls no publisher. Invalid batch
bounds fail before acquiring the drain lease. No request, receipt, pending entry
or cursor format changes; native/local completion still repairs durable reader
progress before removing evidence.

Source-channel identity is distinct from logical entity identity. Verified
native aliases join hooks/files/APIs/exports, adopting existing IDs when
unambiguous and reporting conflicts rather than silently merging. Preserve prior
V1 records, prepared requests and receipt interpretation. Parser/generation
changes retain prior evidence; replay builds a separately selected projection
instead of resubmitting accepted source offsets.

Common-native joins qualify Codex UUID sessions and OpenCode native session IDs.
Codex UUID spelling is case-insensitive; scoped tool, turn and message IDs
retain exact native suffixes. OpenCode session IDs begin with `ses`, contain at
least one suffix character, use only ASCII letters, digits and underscores, and
are at most 128 bytes. OpenCode session and scoped entity IDs are
case-sensitive. Hooks, API snapshots and exports with these exact IDs may enrich
one selected logical session, message or tool while retaining independent source
channels. A source-scoped alias from the pinned selected root, or an unambiguous
common alias with the same integration/native kind/native identity, may supply
the existing target ID. The selected session must exist with the exact
integration; an adopted tool must also exist in that session's selected tool
map. New source aliases retain their original channel and native spelling while
pointing to the accepted target. All transient activity/result references use
the adopted IDs before staging. Source positions, event IDs and transcript
streams remain source-scoped. Messages lacking native IDs retain their separate
source-message identities. Non-UUID Codex labels, unqualified OpenCode session
labels, other integration profiles, content/path similarity and timestamps grant
no cross-source proof. Cursor common identity remains a separate gate.

Common lookup scans only the audited pinned alias catalog, with at most 4096
alias alternatives and 8192 index pages. It requires a complete scan before
using a match. Incomplete coverage, conflicting native targets, unavailable
selected sessions/tools or inconsistent parent-session targets withhold the
affected session's activity and aliases, retain sanitized evidence plus a
constant diagnostic, and accept source progress. Staged objects cannot supply
proof. Legacy duplicate identities remain retained without merging or rewriting
prior records. Later unrelated valid records can still publish. This bounded
lookup is transient; larger-history indexing/replay remains an explicit coverage
gate rather than permission to choose the first partial match.

New captures publishing qualified common aliases also select the existing V1
projection catalog under `codex-native-alias-join/v1` for Codex or
`opencode-native-alias-join/v1` for OpenCode. Each integration's expected-object
edit is shared across its capture channels, preventing independently prepared
requests from creating duplicate native sessions. A changed precondition is
proven stale; unknown outcomes retain the exact original prepared request and
receipt reconciliation. The normal product projection still records its own
parser identity. This additional selected projection preserves V1 object/wire
shapes, IDs, prepared bytes and receipt interpretation; legacy prepared deltas
remain unchanged. It is neither a source registration nor replay authority.

Omitted/unknown records retain safe gaps and receipts even without session
aliases; never create fictional sessions/tools. Across-batch updates preserve
earlier tool content and terminal outcomes, with diagnostics for conflicting
terminal observations. Append updates bounded tails and index paths while
preserving independent manifest reconstruction.

The server supervises startup, fair scheduling, backpressure and shutdown. A
stable installation capture principal uses existing repository authorization and
manifest/WAL publication. Capture failure degrades capture health, not ordinary
service. Pause/disable stops new reads/publication, retains pending evidence and
reconciles in-flight unknown outcomes through receipts; it does not purge
history. Typed status/pause/resume/rescan/import live in the local API and jj
agent capture. Status distinguishes unsupported, unbound, unavailable, pending,
degraded and caught-up sources without raw bodies/secrets. Existing owned
agent/settings UI shows coverage/backlog/progress. Active views automatically
refresh committed data through the application refresh queue while retaining
exact pending commands, drafts and historical code selections.

The installation browser controller requires advertised LocalTrusted capture
management and survives repository/settings navigation. Control/import timeouts,
malformed responses and mismatched receipts retain the complete original intent
for exact retry. A known refusal after an uncertain attempt does not prove the
original failed. Explicit inspection resolves an import only with its matching
original command receipt; absent receipts do not establish rejection or success.
Sources are bounded runtime-generation snapshots, not path or activity proofs.
Browser pause/rescan receipt presentation separates policy acceptance from
worker completion; history-boundary publication is separate from live-tail
coverage.

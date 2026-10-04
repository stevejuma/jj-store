# Agent activity, repository reviews, Threads and Reactions

Thread directory and selected conversation reads start concurrently through the
repository/session/profile query cache. Navigation cancels obsolete reads;
confirmed reactions invalidate and reload only their exact target, retaining
other reactions, messages, reply context, emoji definitions and raster URLs.
Unknown outcomes keep the original intent and receipt identity. Explicit refresh
invalidates displayed collections; SSE still only offers refresh. Reaction reads
are deduplicated across main/reply views and limited to four concurrent
requests. Reply counts and descendants use indexed passes without changing
loaded-history, exact revision, draft or scroll semantics. Unneeded JJ
log/reference reads do not run when restoring Threads. Cached reads retain
current repository-session isolation and cursor-reset behavior; no batch API or
durable read index is added.

Collaboration reads share validated immutable materializations keyed by
repository, authority epoch and exact domain root references. Each request
independently captures its complete manifest envelope and rechecks current
viewer admission before disclosure. Thread directory/history/detail reads
materialize Thread state only; emoji/assets materialize Reaction state only;
reaction queries also load the owning target domain. Retained external
dependencies still undergo the provider's complete closure validation. Directory
metadata and exact-target slot indexes are derived once per root and never
become authority. Shared entries contain no request session, credentials or
viewer-filtered results, coalesce concurrent loads and obey fixed expiry, count,
object and encoded-byte bounds. When all retention slots are initializing, extra
query roots are validated request-locally under the existing per-pin and
concurrency budgets without entering the cache. Small retention capacity cannot
disable a valid query. Failed validation is never admitted;
expiry/eviction/restart rematerialize and validate from selected immutable
records. Signed cursor pins preserve their original complete snapshots
independently of cache eviction.

Human messages optionally retain an exact same-Thread parent revision for
replies. The parent is an immutable closure dependency; self-links, foreign
Thread parents and changing reply identity during edits are rejected. An absent
parent omits the field and preserves legacy canonical bytes.

Explicit local BoundCapture carries the exact observed Thread version and
primary assignment with the original sanitized batch/profile. It retains one
real session replacement or a validated diagnostic-only capture gap, stages the
shared link without changing lifecycle and selects Agent and Thread descriptors
in one companion ref CAS. The Thread descriptor audits and retains the complete
newly staged session closure before selection. Matching receipt replay precedes
entity/source preconditions; stale bindings select neither root. The private
spool retains this original bound request across unknown outcomes. Legacy
unlinked capture tags, field order and receipts remain unchanged.

Native preparation accepts an optional exact Thread binding. The private capture
spool journals it before acknowledging source progress, compares the returned
binding without substitution and retains the complete prepared delta/time for
publication retries. Missing, injected or changed bindings fail before delivery.
Native and local capture-provider qualification remain distinct from automatic
runner selection.

An optional native prepared capture binding pins an observed Thread version and
primary assignment. It retains one real selected session edit or a validated
diagnostic-only source/projection pair and derives a stable link identity from
the authenticated actor, request and selected evidence. Agent evidence and the
Thread link publish atomically; gap links leave primary assignments untouched.
Receipt replay precedes observed preconditions; stale Thread/assignment or a
failed authorization selects neither root. Late bound capture retains the Thread
lifecycle without reopening it. Unlinked capture keeps its original canonical
request and receipt bytes. Provider atomicity and native spool/RPC behavior
retain separate evidence from the local provider and future runner integration.

HTTP keeps Thread and Reaction intent namespaces distinct. Binary custom emoji
definition and typed retirement assemble the same service commands. Empty reads
do not initialize domains; resumed pages retain signed captured selections.
Exact message revision and image reads validate retained membership. HTTP
transport reconstruction is independent of persistent restart and runner gates.

AgentStore and ReviewStore are first-party repository capabilities. They share
the existing object envelopes, WAL, receipts and manifest publication service,
but retain independent models, authorization and state versions. They are not
upstream JJ traits, Tantivy indexes, Nostr projections or agent executors.

The separately governed local Git profile uses the M13 companion publication
store and the same domain models. New versioned portable anchors distinguish
source native and Git identities and name immutable evidence capsules. Existing
native V1 anchor bytes and their interpretation remain unchanged. An evidence
reference is not a device binding or permission to open a local path.

Agent normalization, cumulative transcript construction and catalog path-copy
edits use one owned projection engine behind a repository-bound immutable object
interface. Persistence providers validate canonical records, kinds and complete
dependencies; the shared engine has no publication capability. Native adapters
retain manifest authorization and their existing record framing, while the local
provider selects its complete closure through companion ref CAS.

The complete capture release targets LocalTrusted Codex, Cursor and OpenCode
through qualified integrations, stores sanitized evidence before normalized
views and supports exact discussions on immutable code, diff and agent
artifacts. The separately scoped usable preview below ships the qualified Codex
workflow first; additional harnesses and extended features are deferred. Review
task context is explicit and version-bound; replaying or reading a comment never
dispatches an agent.

Agent session list metrics align at the trailing end of the second row in
transcript-count, tool-count, duration order. Tool action toolbars remain inside
their row bounds and accessible by pointer, keyboard and touch; open menus
retain visibility.

## Thread presentation alignment

Thread settings and replies occupy the shell's far-right sidebar, with the same
compact channel toolbar and forum-row geometry as the locally copied Community
reference. Settings use exact-version rename and allowed lifecycle commands;
repository admission governs access, and no per-Thread member ACL is implied.
Directory metadata derives only from current selected messages, supplies visible
contributors, excerpts, counts and activity, and excludes tombstoned Threads.
Native links encode repository, Thread/project/task and code-operation
identities. New Thread links use `thread=` on Threads; legacy `channel=` stays
readable and Review routes retain their own `thread=` interpretation.

## Requirements

| ID             | Requirement                                                                                                                                                                                                                                                                                                                                                                    |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-COLLAB-001 | The repository facade MUST expose independent root-pinned AgentStore and ReviewStore services backed only by the existing immutable object, WAL, receipt and manifest publication authority.                                                                                                                                                                                   |
| JST-COLLAB-002 | Capture MUST preserve stable session/turn/tool/checkpoint identities, sanitized native evidence, deterministic normalized activity and explicit evidence-backed JJ associations for Codex, Cursor and OpenCode.                                                                                                                                                                |
| JST-COLLAB-003 | Review threads MUST target exact immutable artifact versions and validated UTF-8 byte ranges, with stale-safe replies, edits, tombstones, resolve/reopen and conservative explicit re-anchoring.                                                                                                                                                                               |
| JST-COLLAB-004 | LocalTrusted CLI and HTTP APIs MUST provide bounded root-pinned reads, typed writes, pagination and live hints without bearer credentials, public listeners or implicit controller dispatch.                                                                                                                                                                                   |
| JST-COLLAB-005 | The application MUST reuse repository-owned Community-derived thread/comment/agent presentation and Design Core diff extension points, with accessible interactions, virtualization and pinned-code versus live-activity state.                                                                                                                                                |
| JST-COLLAB-006 | Sanitization MUST occur before every durable capture copy through the qualified in-process redaction library; credentials, unrelated sessions and unsafe rendered content MUST fail closed.                                                                                                                                                                                    |
| JST-COLLAB-007 | Cold recovery and native copy MUST verify code, agent and review closures directly; search and other projections remain rebuildable, and unsupported remote/export/backup behavior stays deferred.                                                                                                                                                                             |
| JST-COLLAB-008 | Threads MUST have stable distinct identities, immutable versions and auditable lifecycle transitions; optional primary session assignments and reference links preserve historical attribution without backfill or process control.                                                                                                                                            |
| JST-COLLAB-009 | Human Thread messages MUST retain exact immutable revisions; timelines reference canonical AgentStore, ReviewStore and code evidence instead of duplicating captured transcripts or inferring causality from timestamps.                                                                                                                                                       |
| JST-COLLAB-010 | Reactions MUST target typed persisted identities and exact revisions or snapshots, resolve membership and visibility before disclosure, and use server-derived authenticated actors rather than capture AgentIdentity.                                                                                                                                                         |
| JST-COLLAB-011 | Each actor/target reaction slot MUST use exact observed preconditions and immutable additions/retractions; concurrent different values remain explicit conflicts and unseen additions survive retraction.                                                                                                                                                                      |
| JST-COLLAB-012 | Rich reaction values MUST support semantic likes/dislikes, Unicode emoji and repository custom emoji with retained versioned raster assets, bounded decoding and a static reduced-motion preview.                                                                                                                                                                              |
| JST-COLLAB-013 | Independent ThreadStore and ReactionStore roots MUST publish through additive native manifest format 5 and local companion version 2, preserving legacy bytes, complete closures, durable receipts and atomic multi-domain transitions.                                                                                                                                        |
| JST-COLLAB-014 | Typed bounded service, HTTP, native RPC and local CLI commands MUST share authorization, root-pinned reads, idempotency and unknown-outcome semantics; capture and runner binding remain explicit and optional.                                                                                                                                                                |
| JST-COLLAB-015 | Thread/Reaction origin facts MUST converge under semantic reducer 3 without executing agents, tools, hooks, network image fetches or notifications; WorkStore native homes retain their replication freeze.                                                                                                                                                                    |
| JST-COLLAB-016 | The application MUST present persistent Threads and rich reactions through repository-owned Community-derived presentation and Design Core contracts, explicit refresh, authorized actor inspection and reversible optimistic receipt-aware state.                                                                                                                             |
| JST-COLLAB-017 | Automatic transcript capture MUST be explicitly enabled by LocalTrusted server configuration, with selected harness profiles, approved roots, bounded supervised workers and durable pause/restart behavior.                                                                                                                                                                   |
| JST-COLLAB-018 | Automatic capture MUST admit only verified native repository/workspace bindings to this installation, backfill matching active sessions, withhold ambiguous cross-repository content and require explicit inactive-history import.                                                                                                                                             |
| JST-COLLAB-019 | Version-qualified transcript, snapshot and read-only API/database/export readers MUST retain source generations/cursors, recover missed notifications/delayed flushes and advance only after sanitized spool durability.                                                                                                                                                       |
| JST-COLLAB-020 | Hook, transcript, API and export evidence MUST enrich verified common native aliases without duplicating logical sessions/tools or rewriting existing identities, prepared requests or receipts.                                                                                                                                                                               |
| JST-COLLAB-021 | Unsupported or omitted observations MUST retain safe evidence, diagnostics and source progress without inventing activity or permanently blocking valid later captures.                                                                                                                                                                                                        |
| JST-COLLAB-022 | Tool updates MUST cumulatively merge across accepted batches, preserve absent input/output fields and terminal results, and expose contradictory terminal observations as diagnostics.                                                                                                                                                                                         |
| JST-COLLAB-023 | AgentStore MUST support only current transcript tail pages; append MUST update bounded tail/index paths and retained supported manifests MUST remain independently reconstructible.                                                                                                                                                                                            |
| JST-COLLAB-024 | Rich conversations MUST retain stable message/block updates, plans, children and compaction, exact message/plan review targets and identity-preserving versioned projection replay without execution.                                                                                                                                                                          |
| JST-COLLAB-025 | Separately enabled workspace checkpoints and canonical owned producers MUST provide durable baselines, scoped execution/failure boundaries, provenance intents and evidence-qualified code associations.                                                                                                                                                                       |
| JST-COLLAB-026 | Each supported harness surface MUST pass a real authorized edit, declared reader/fallback coverage, exact JJ evidence, shared UI and cold restart acceptance; probes/synthetic fixtures alone MUST NOT qualify it.                                                                                                                                                             |
| JST-COLLAB-027 | Collaboration inspection/search and review work requests MUST retain exact selections, independent freshness/current visibility, explicit feedback addressing and bounded safe stdin/body-file input without implicit dispatch.                                                                                                                                                |
| JST-COLLAB-028 | Collaboration archives/portable carriers MUST verify complete retained closures and explicit source/code identity mappings; ordinary Git transfer MUST NOT claim complete collaboration portability.                                                                                                                                                                           |
| JST-COLLAB-029 | Expanded capture MUST preserve sanitization before durability, workspace/home/temporary path privacy, target permissions, current revocation and explicit retention/purge/export rules before broader deployment.                                                                                                                                                              |
| JST-COLLAB-030 | Capture management MUST live in dedicated installation-scoped Agents settings using the shared divided rows and action columns, and expose bounded typed status, pause/resume, rescan and qualified import controls with coverage/backlog diagnostics; active views MUST refresh automatically while preserving drafts, historical code selections and exact pending commands. |

## Agent capture completion profile (M18)

The original jj-agent v4 plan is non-normative reference material. Relevant
missing behaviors are now governed above. M9's narrow LocalTrusted acceptance
does not qualify automatic readers, complete conversations, automatic provenance
or a real model edit. M16 is reserved for Appearance; M18 owns this roadmap.
Only [milestones.md](milestones.md) carries status and actual command evidence.

### Usable local preview priority — 3 October 2026

The current delivery target is a runnable Codex preview in the current main
application. It MUST reuse configured Codex CLI/Desktop capture, verified native
repository/workspace matching, durable publication/recovery, existing Codex
history import and current AgentStore. Only actually qualified harness versions
may be enabled. A separately verified real-service/browser/restart journey MUST
precede a preview-ready claim. Existing local privacy, current Write admission,
bounded resources, exact retries and manifest publication remain mandatory.

UI integration and automatic freshness take priority after rebasing onto
verified committed main. Active capture management and Agents views MUST update
without required manual refresh, retaining drafts, stable selections, historical
code pins and exact pending command identities. One-time server configuration
and an isolated development setup are sufficient; a configuration editor is not
a preview gate. The main installation and in-progress main workspace edits MUST
be preserved.

OpenCode configured startup/history/export, Cursor, advanced
conversation/replay, automatic checkpoint/causality work, full multi-harness
release qualification, advanced reviews/search/feedback, archives and broader
deployment policy are Deferred for this preview. Their canonical contracts
remain backlog requirements; completed foundations remain preserved. Legacy
AgentStore and migration stay excluded. The ledger's new preview gate is
independent of these deferred gates; preview acceptance MUST NOT mark the full
roadmap Done. The execution order is in
[the preview plan](../../docs/implementation-plan.md#agent-capture-usable-local-preview).

### Local server admission and activity

The first automatic worker serves LocalTrusted native repositories. Capture is
disabled unless agent_capture.enabled and an explicit harness list are set.
Selecting a harness authorizes its version-qualified locations for the server's
user account; explicit roots override conventions and supported home/data-root
overrides are honored. Configuration applies on restart; runtime resume cannot
override disabled configuration. Enablement requires write authority and a
selected current AgentStore root. Capture does not initialize missing legacy
domains or migrate old AgentStore encodings. Unsupported roots fail closed.

The format-2 agent_capture table defaults to disabled, four readers, a 30-second
rescan, at most 1024 tracked sources and a 64 MiB pending spool. Configured
bounds may reduce concurrency to one through four, rescan to one through 30
seconds, sources to one through 4096 and pending capacity to one through 64 MiB.
Each harness entry names a versioned product-surface profile and exact product
version; unknown profiles/versions, duplicate profiles/roots and unknown fields
fail configuration checks. Relative explicit transcript roots resolve against
the configuration file; absent roots use qualified conventions at worker
preparation. A read-only OpenCode API profile requires an explicit
credential-free loopback endpoint and cannot also name filesystem roots. Native
automatic capture is unavailable in filesystem-only serving. Configured Codex
CLI/Desktop startup uses the ordinary foreground server lifecycle; unsupported
reader profiles fail before state creation. After native resources and the
process state lock are owned, first-run startup may exclusively create an absent
private capture root, installation-bound control journal and generation-bound
import journal. It MUST sync that initialization before preparing discovery. An
existing root is opened without repair, permission changes, journal replacement
or renewed consent. Incomplete or corrupt existing policy withholds capture
while ordinary service remains available. Configuration checks and management
inspection MUST NOT initialize state. Missing optional import metadata continues
to withhold import management only. Restart MUST retain pause, generation,
source identities, accepted imports and exact pending publication requests.

Admit a registered directory or another physical JJ workspace only after bounded
discovery proves the native repository/profile and this installation's exact
Unix socket. Nested directories resolve to their actual workspace. Names,
filename patterns, path-prefix similarity and clock proximity are not identity.
Revalidate before publication; moved, missing, rebound or foreign-server
bindings are unavailable. Multiple roots of one repository are supported.
Mixed-repository content needs verified per-record partitioning; otherwise
withhold it with a gap.

Codex file batches retain one device-memory directory context. Only the exact
first qualified header and subsequent `turn_context.payload.cwd` records provide
context; message/tool content, arbitrary `cwd` fields and path similarity do
not. Directory values must be bounded absolute non-root paths without traversal
or control characters. A changed context cuts the batch before the changed row,
which begins the next batch with its own exact ending offset and prefix witness.
No batch is assigned wholesale to the original header directory after a turn
context change. The server's batch admission adapter passes this reconstructed
directory through the existing current native workspace/socket/catalog verifier;
registration membership and publication write authority remain separate checks.

Malformed complete JSON/UTF-8, oversized records and invalid/missing turn cwd
make context unavailable until another explicit valid turn context. A repeated
session header withholds the remainder of the generation, preventing foreign
session content from inheriting the original native session identity. Partial
records never change acknowledged context. Context state advances only with the
exact sanitized durable batch acknowledgement. Replaced/truncated/rewritten
generations reset context to the newly qualified header.

Restart reconstructs context from the same verified acknowledged JSONL prefix,
in the existing at-most-4-MiB verification steps with a 1-MiB partial-record
bound. This reconstruction neither emits old records nor trusts a cached
directory at the old offset. A changed prefix restarts verification or starts
the existing new-generation recovery. Raw directory histories have no
persistence or debug encoding and are not added to reader journal or AgentStore
shapes.

Native body sanitization refuses an unavailable directory context. A separate
safe-gap conversion retains only constant omissions, deterministic observation
identities and source progress, without inspecting or copying source bodies. The
worker may publish those gaps only to a separately authorized bound anchor; an
unavailable/revoked anchor withholds publication. Valid later contexts can still
be ingested after the gap. This capability is not active-session admission and
does not authorize historical import or automatic worker startup.

Capture directory discovery is a separate read-only verifier capability. It
resolves at most 64 physical ancestors to the nearest repository boundary,
rejects nested ordinary Git/unqualified JJ repositories and metadata-directory
paths, and never falls back through an invalid inner boundary. Its proof binds
the canonical workspace and repository metadata paths, physical directory
identities, validated connection bytes and native profile markers. Ordinary
working-copy edits do not alter this device binding. Admission also requires the
native repository in this installation's current published discovery catalog.
Revalidation repeats both proofs and rejects changed bindings or unregistered
repositories. It repeats discovery from the original session directory,
retaining aliases and new inner repository boundaries rather than inspecting
only the prior workspace root. Source directory paths stay in device admission
state and never become an AgentStore body. This directory proof is not write
authorization or a code checkpoint; publication must separately apply the
current service policy.

Qualified live status/hooks or new transcript records while enabled admit an
active session. Source growth alone does not assert model execution. Backfill an
admitted session from its beginning, then follow its tail. Restart resumes
admitted sources and pending tails even if they ended while offline. Existing
inactive sessions require explicit import; absent activity metadata waits for
new activity rather than bulk historical import. Transcript ingestion does not
authorize snapshots or change JJ operation heads. Checkpoints require separate
per-workspace client enablement. Ordinary Git-backed folders, remote hosts and
authenticated capture remain separately gated even where M13 serves folders.

The initial Codex activity probe arms at the exact observed file length. It
reconstructs directory context with bounded reads but ignores all records that
began before that boundary, including an already partial record completed later.
Arming completes through bounded stable-prefix steps. Changes before the initial
boundary has been read reset that unproven baseline to the current file length;
records missed during arming remain historical and wait for a later new start.
Only a new supported `event_msg.payload.type=task_started` with a verified
directory context establishes eligibility. Arbitrary growth, user/tool text,
completion records and historical unfinished starts do not. This proof means
activity was observed while monitoring; it does not assert continuing model
execution. CLI and Desktop headers qualify independently. Live status/hooks
remain separate qualification; a session whose start was missed waits for a new
start or explicit import.

Activity probing has no source IDs, registry writes, reader leases, cursor
acknowledgements or publication capability. Each step scans at most 1 MiB and
256 complete records, retaining at most a 1 MiB partial record. Metadata changes
discard partial scanning and verify the previously scanned complete prefix in
at-most-4-MiB steps. Changed prefixes, truncation, replacement and rebound roots
withhold eligibility and require fresh qualification/arming. A returned activity
proof binds the exact physical/header and file snapshot plus the event's
device-memory directory; append races invalidate that snapshot until another
verified probe step. Repository admission is checked before acquisition and
repeated before publication; write authorization remains separate.

The private V1 source catalog adds an optional, bounded set of domain-separated
native-session admission digests. Empty sets omit the new field and preserve
legacy catalog bytes; generic registration cannot create admission markers.
Active acquisition selects registration and admission in the same atomic catalog
replacement only after current activity proof and both reader leases succeed.
Previously admitted sessions may reacquire after restart without another start,
including offline completion and same-session rotation, through exact current
header qualification and unambiguous retained source aliases. New native
sessions at known locations need fresh activity. Read-only existing-registry
lookup creates no directory, entry or lock. A legacy registration without an
admission marker does not authorize automatic historical capture. At most 4096
admission digests are retained without eviction, within the existing catalog
byte bound. Admission is neither repository authority nor permission to publish
or checkpoint.

### Readers, durability, identity and supervision

Automatic capture derives its authenticated actor from the installation identity
under the fixed `agent-capture-worker` purpose. Source native identities and
captured AgentIdentity never select that principal. Preparing this authority is
read-only, requires validated enabled LocalTrusted native Write/Admin
configuration and reads no installation state when disabled. Changing harness
roots or the effective Write/Admin permission preserves the actor across
restart; another installation and browser/search/replication purposes have
distinct principals.

Binding an in-process native capture publisher revalidates current directory,
socket and repository-catalog admission before and after native Write
authorization. It uses the existing service preparation/publication boundary and
the same sanitized spool/prepared request/receipt rules as RPC capture. No
bearer token, extra credential file, alternate storage writer or harness process
is created. Current admission must still be rechecked by the worker before each
publication; a bound native request alone does not retain directory permission.
Authority preparation does not initialize domains, allocate sources or start the
automatic worker. New native repository creation already selects an empty
AgentStore. Repositories without that selected root are unsupported for native
capture; no legacy initialization or migration gate remains.

The guarded in-process publisher retains only an in-memory directory proof and
binds a fresh native Write request for every preparation and publication,
including already-prepared retries. Both current workspace/socket/catalog
admission and native authorization precede each action. Rebound directory proofs
or removed registrations withhold work without changing prepared intent; capture
requests with another authenticated actor are refused. Preparation additionally
rechecks admission before returning its candidate. A committed publication keeps
its receipt even if device admission changes afterward; a later retry requires
current admission again. This publisher grants no active-source eligibility,
source lease, checkpoint, spool selection or automatic startup authority.

The admitted Codex source driver connects these capabilities without enabling
server startup. Active acquisition verifies the event directory and current
native Write authority before source enrollment; admitted restart requires the
retained exact native-session admission and a current verified directory
binding. The header-only convenience acquisition remains explicit; supervised
restart uses the bounded preparation below when the original directory is
unavailable. The driver retains a memory-only binding to the exact canonical
private spool root and physical directory identity. Each step checks that
binding before source reading or pending resolution; another or rebound spool
cannot clear an old intent. A fresh handle and changed capacity for the same
unchanged root remain valid. Binding inspection is read-only and creates no file
or durable format. It opens the leased reader at the latest source-wide
sanitized cursor, and holds new reading while unresolved durable pending entries
recover their exact directory proofs. Inactive generic registration remains
ineligible.

A staged admitted-source resume preparation MUST perform read-only retained
native-session/channel lookup without allocating identities, aliases or leases.
It reconstructs the current directory from the exact qualified header and all
complete native context records through one stable current file snapshot. Each
step reads at most 4 MiB, retains at most 1 MiB of a partial record and never
emits or durably copies source bodies. A partial final record cannot alter
context. Metadata changes restart bounded reconstruction; changed header,
physical source or approved root requires renewed qualification. This
preparation does not acknowledge a cursor or validate any queued publication
range, whose independent exact proof remains mandatory.

Current context, or a separately verified prior/header anchor, must pass current
native directory/socket/catalog admission and native Write authorization before
reader leases or new alias enrollment. A revoked original header directory MUST
NOT prevent an eligible same-session tail whose verified current directory is
still linked and writable. Retained native eligibility may resume even if the
original process exited before its first sanitized enqueue. Generic registration
or source growth cannot supply eligibility. Recheck the unambiguous retained
channel, file snapshot and directory binding around acquisition; a changed spool
cannot lend its state to the preparation. Missing/rebound/revoked sources
withhold without discarding durable pending intent. Acquiring through an anchor
cannot authorize body evidence: every read batch and queued entry still needs
its own admission or exact safe-omission conversion. Returning a source
completes this one-use preparation; retry cannot create a second reader under
its proof. Blocking-I/O ownership, scheduling, deadlines and automatic startup
remain supervisor gates.

The same bounded current-context preparation may accept a fresh, unforgeable
Codex activity snapshot in place of retained native admission. It MUST verify a
genuinely new start observed by the already armed activity probe; generic
registration, source growth or rearming over a historical start is insufficient.
Qualification extracted from that snapshot grants no activity or directory
authority by itself. Preparation allocates no source identity, alias or lease
and cannot acknowledge a cursor or copy a transcript body.

The fresh proof MUST retain its exact physical/header and file snapshot
throughout preparation and acquisition. A changed snapshot requires renewed
activity proof from the original bounded probe, with its existing prefix
verification; a saved proof cannot authorize newly appended bytes. The complete
current directory, or a separately verified prior/header anchor, must pass
current native directory/socket/catalog and Write admission before fresh source
enrollment and reader leases. Thus a start observed in A followed by a valid
current B context may admit B even when A is now revoked. Partial context
records cannot supply B, and the fresh proof cannot authorize historical A
bodies through B. Each ingress batch and queued range retains independent
admission, safe-omission and publication rules. Recheck the activity snapshot,
reconstructed context, admitted channel and directory proof around acquisition;
source changes withhold. A successful preparation is one-use. Owned preparation
I/O, probe renewal, resident-memory scheduling and automatic startup remain
supervisor gates.

Each driver step reads at most one existing bounded native batch or drains one
queued native batch. Known contexts use current per-batch directory admission;
unknown or unbound contexts become constant safe omissions only under a
separately revalidated authorized prior anchor. Transient admission failure
withholds the batch. Workspace-specific sanitizer instances retain the same
home/temporary/cache privacy roots and are rebuilt when the workspace changes.
Current admission is checked after sanitation and before durable enqueue. The
sanitized spool must accept the exact batch before reader acknowledgement; spool
refusal retains the batch and cannot advance source progress. One uncommitted
batch per conservative `step` driver prevents further source reading until
guarded publication resolves it. Publication selects only the retained exact
batch identity; additional entries require independent proof recovery instead of
inheriting its directory admission. Cancellation and denied/unknown publication
retain the queued intent and its in-memory proof for later exact retry;
committed completion preserves reader journals and code heads. Scheduling,
blocking-I/O ownership, deadlines and typed pause/status remain supervisor
gates. This explicit driver is not a standalone watcher or full server lifecycle
proof.

The independently scheduled Codex pipeline consumes this driver and exposes
separate bounded ingress and exact queued-publication steps. Ingress MUST
continue at the source-wide sanitized durable cursor while old queued entries
remain withheld, prepared or unknown. Every new batch retains its own current
directory/socket/catalog and native Write admission, sanitizer and exact spool
acknowledgement; no queued entry or receipt supplies that admission. Spool
capacity still stops acknowledgement and further reads without dropping the
reader's exact pending batch. The pipeline does not retain an accumulating map
of per-batch raw directory proofs; delivery reconstructs each selected exact
range through the qualified bounded recovery probe.

Publication accepts an explicit head selected for this leased source and
integration. It recovers only that entry's exact proof, and same-partition order
is rechecked under the existing drain lease. Revoked, unavailable or unknown
repository-A intent cannot prevent separately admitted repository-B ingress or
delivery. Earlier A intent still prevents later A delivery. Publishing old
entries MUST NOT rewind the ingress directory anchor or reader to an older
context; only ingestion advances that anchor. Restart opens at the latest
sanitized cursor, without requiring old queued publication first. The consumed
driver's serial convenience is unavailable through the pipeline, preventing
implicit lane changes. One retained bounded recovery probe supplies no other
entry's proof; switching the selected entry restarts its independent recovery.
The supervisor must still own temporal fairness during multi-step proof
verification, blocking I/O, deadlines, backpressure, pause and restart. Pipeline
composition does not qualify automatic startup or full harness/cold acceptance.

A generic pipeline may park its opaque device-memory reader frame to release
both physical and source-channel leases. Automatic supervisor parking MUST
release the physical lease but retain the channel lease while an exact
unacknowledged raw batch owns its generation/sequence allocation. Idle readers,
partial scans and unfinished prefix proofs without a raw batch retain ordinary
lease-free parking. Retained channel reservations count within the existing
bounded noncompact frames, at most twice the reader limit; they grant no body or
publication authority. Pause, cancellation and failed admission retain that
reservation and exact raw state; successful enqueue/acknowledgement followed by
idle parking, completed shutdown or owner death releases it. Parking MUST
preserve partial scans, multi-step prefix/context verification and the exact
unacknowledged raw batch; it cannot acknowledge bytes, serialize raw state or
grant publication authority. The supervisor MUST separately bound resident
frames and raw buffers, including parked frames; a reader-lease limit alone is
insufficient. This handoff itself does not allocate supervisor frames or enable
process startup.

Resumption MUST independently recheck retained native eligibility, the exact
private spool binding, qualified profile, physical source and header witness,
current directory/socket/catalog admission and native Write before acquiring
fresh physical leases and either fresh or retained channel ownership. Retained
channel reacquisition MUST validate the exact private registry directory and
lock-file identity, admitted channel, profile and physical/header qualification
under registry/FIFO coordination. It acquires only a fresh physical lease and
keeps the channel in the parked owner across admission awaits and failed
handoffs. No lock descriptor is cloned; only a fully checked successful transfer
consumes the reservation once. Foreign registry directories, replaced locks and
concurrently held physical leases fail closed. The retained prior ingress anchor
requires its own current authorization and cannot authorize later body batches.
A revoked anchor with a separately eligible current directory needs staged fresh
admission; parking itself cannot manufacture that proof. Changed or unavailable
admission, qualification or spool preserves the frame without reacquiring a
reader. A successful handoff consumes it once; failed handoffs retain it for a
later exact retry, and a concurrently leased source cannot be replaced.

A parked pipeline may separately prepare a current-directory handoff when its
prior ingress anchor is no longer admitted. This retained-session preparation
MUST validate the same qualified physical/header channel and exact private
spool, then reconstruct only complete current context records through a stable
snapshot, at most 4 MiB per step. It allocates no source identity or lease and
cannot acknowledge or discard the parked reader's raw batch, partial scan or
unfinished prefix proof. A partial EOF record supplies no directory authority.
At stable EOF, preparation releases its partial allocation while retaining the
current-context/snapshot proof. The supervisor reserves a noncompact source
frame before this body reconstruction; one such frame may contain one parked
reader plus one bounded current-context preparation.

The current directory, or a separately revalidated prior/header candidate, must
pass current native directory/socket/catalog admission and Write before reader
lease reacquisition. Recheck the preparation snapshot and retained native
channel around acquisition, together with ordinary spool/progress checks. Only
successful transfer changes the ingress anchor and consumes the frame; failures
retain both exact reading state and bounded preparation. An independently
advanced sanitized cursor still requires the ordinary verified reopen. The new
anchor grants no body or queued-publication permission: revoked A bodies may
become constant omissions under authorized B, while each B body/range requires
its own current admission. Fresh generic registration or a changed physical
source cannot use this retained-session handoff. Configured foreground startup
independently owns the server resources.

Only an unchanged durable source cursor permits reuse of retained raw state. An
independently advanced sanitized cursor requires a newly opened reader at that
progress, with ordinary bounded prefix verification; it MUST NOT re-enqueue the
old frame. Missing, changed-at-the-same-position or regressive durable progress
fails closed rather than resetting to the header. Check progress and admission
again after lease acquisition. Appends and rewrites still follow the reader's
existing snapshot/prefix checks; parking cannot suppress those checks. Source
replacement, even with byte-identical headers, cannot reuse the old physical
frame. No durable source/cursor/pending/prepared-request or receipt format
changes.

An explicit device-memory lifecycle driver may own the prepared Codex supervisor
and schedule its independent lanes without caller-driven quanta. It MUST retain
one terminal stop state, bounded last-observed body-free status and its exact
supervisor across pause/resume. Stop prohibits new dispatch. Shutdown timeout
MUST retain the join handle and all draining ownership for a later wait; it
cannot abort or replace blocking owners or permit storage teardown. Dropping the
driver requests stop while its task retains draining ownership until settlement.
Only completed drain permits ordered storage shutdown. This explicit lifecycle
capability grants no configuration enablement and does not independently qualify
authenticated controls, other readers or real harness acceptance.

The explicit runtime may issue cloneable device-memory status/pause/resume
handles for later management composition. Those handles contain only the
body-free cached observation and command channel, never source state, storage
leases or a drain task. Dropping a handle cannot stop the runtime; dropping the
sole runtime owner requests terminal stop even if control clones remain. A
control request acknowledges a desired transition, not completed pause or drain.
Observed draining remains distinct from completed stop. Unexpected loss of a
nonterminal observation producer MUST report unavailable rather than expose its
stale caught-up snapshot as current. A known completed Stopped observation
remains readable. All handles reject controls after terminal stop or producer
loss; resume cannot recreate an owner or grant enablement/admission.
Authenticated transport, durable pause state, rescan/import and management
presentation remain separate gates.

Runtime handoff of a settled paused supervisor MUST preserve that pause in its
initial command and observed phase. It cannot attempt dispatch, report failure
or replace the owner merely because no resume was requested. Initial pause
performs no source discovery/body reading, allocation or publication. Explicit
resume arms new activity boundaries normally; starts already present before
arming remain historical and grant no eligibility. Only independently qualified
new activity may then admit sanitized backfill. This preserves the existing
runtime constructor; it adds no startup mode or durable control record.

Durable capture pause policy is installation-private device state, separate from
AgentStore and native publication authority. Its versioned record MUST bind an
exact installation and fresh control generation, preserve monotonic revisions
and retain exact authenticated actor/request/expected-revision/action receipts.
Replay returns the original receipt alongside the latest selected policy; it
cannot reverse a newer transition. Changed reuse conflicts, stale new requests
withhold, and another generation cannot inherit old requests. The bounded
initial profile retains at most 4096 receipts within 1 MiB; exhaustion refuses
new changes without evicting accepted receipts. Receipt resolution remains
available at capacity and synchronizes the selected file/directory before
acknowledging durability.

Control state lives outside checkouts in an owner-only directory. Explicit
initialization requires a new directory; ordinary open MUST NOT initialize,
repair permissions or reset missing/corrupt/foreign state. Every operation pins
the directory and exact coordination lock, rejects symlinks, unknown files and
unsafe owners/permissions, and verifies the selected regular file around bounded
reads. Under the short exclusive filesystem gate, synchronized temporary-file
replacement and directory sync select a change. Only one recognized unpublished
candidate may be cleaned before another mutation; it grants no selected policy.
Failure after replacement is an unknown outcome resolved with the same request,
not a proven conflict or authority to issue another request. These device-state
receipts authorize no source enrollment, native publication, history import or
configuration enablement. Authenticated management must revalidate current
authority and apply the latest selected policy to the owned runtime separately.

Foreground capture composition MUST consume a selected control journal. Only
whole-server preparation may initialize an absent capture root under the owned
state lock as specified above; an existing root and internal resource-only
preparation MUST NOT initialize missing policy. Validate the selected journal
before creating source or spool state. The ready slot retains its exact control
generation, physical directory/lock binding and installation identity.
Immediately before runtime handoff, re-read the latest selected policy and
installation identity. A pause or resume selected after preparation MUST
determine the initial owned runtime state. Missing, corrupt, foreign,
regenerated or rebound state withholds that handoff without resetting policy or
starting discovery. Initial pause reads no transcript body, enrolls no source
and publishes nothing. Disabled preparation still performs no capture
environment/state I/O. This startup handoff does not watch device-policy files
or replace authenticated management's independent current-authority and
runtime-application requirements.

Installation-local status and pause/resume management MUST require current
LocalTrusted Admin authority and derive its actor from the admitted transport.
Commands contain only a version, request identity, control generation, expected
revision and action; clients cannot select the actor or initialize missing
state. Management binds the selected installation, capture configuration,
private journal generation and one runtime control cell. Current configuration
and identity are revalidated before device-state access and again before runtime
application. Disabling or changing the effective configuration cannot borrow the
prepared owner's authority. Disabled status performs no capture-state I/O.

The complete bounded journal operation and runtime application MUST have one
owned blocking task with retained coordination across caller cancellation or
deadline. Serialize record/replay and application under the same journal gate;
apply the latest selected policy, never the original replayed action. The
selection MUST recheck its exact private directory/lock and selected policy
under that retained gate immediately before runtime application. Physical
rebinding withholds application as unknown, even when replacement bytes are
identical; restoring the original binding permits exact receipt replay without
reauthoring. Durable policy receipts and runtime transition acceptance are
distinct. An absent, stopped or lost runtime may retain the selected policy for
restart but cannot be recreated by management. Current authorization loss after
selection withholds runtime application and reports an unknown outcome requiring
the original request. Controls contain no source bodies, directory proofs or
storage lifetime and cannot enable capture, enroll a source, import history or
publish native state.

Installation rescan MUST use a strict actor-free version-1 command binding the
control generation and expected policy revision. It is an idempotent transient
discovery hint, with no policy mutation or durable receipt. Unknown callers may
retry the same conditional hint; a changed policy requires explicit refresh.
Current Admin/configuration/installation and exact private journal binding are
revalidated under one retained bounded operation and journal gate before runtime
acceptance. Disabled, paused, missing, stopped or lost discovery owners refuse
rescan. The hint cannot resume, initialize or replace an owner, admit historical
activity, advance a source cursor, import history or grant publication
authority.

Rescan hints MUST coalesce in bounded device memory. They preserve active scan
progress and request a fresh pass after that bounded scan finishes; idle root
cooldowns may be brought forward. Source fingerprints, eligibility, directory
and Write checks, fair scheduling and existing reader/frame budgets remain.
Pause and terminal stop discard unconsumed hints and dispatch no new scan.
Cached status exposes discovery phase,
configured/unchecked/available/unavailable root counts, pending rescan and
last-settled degraded-source counts without transcript files, bodies or source
proofs. Availability describes the last observation, not current coverage or
authorization; loss retains observations with a stopped lane.

Cached runtime coverage MUST also group selected qualified harness profiles in
deterministic profile order, retaining declared product versions, root
availability and tracked/observed/awaiting/caught-up/running/degraded source
counts. The current Codex owner has at most two profile groups. These fields
contain no transcript path, native session key or source proof and require no
new filesystem work. Under current LocalTrusted Admin admission, harness
coverage also reports the exact approved directory roots resolved at worker
startup, including default Codex home selection and explicit overrides. At most
sixteen bounded absolute UTF-8 paths are reported per harness; these are
configuration metadata, never discovered transcript files, native session keys
or source proofs. The resolved directory list is retained independently from the
Copy runtime observation and survives pause and terminal stop. Status reads
perform no directory inspection. Root/source totals agree with aggregate
observations. They describe prepared reader coverage, not real product-surface
qualification or import authority. Source-level coverage, import and browser
controls remain separate gates. The installation CLI uses the existing local
Admin HTTP manager with an explicit literal-loopback URL, conditional policy
inputs and exact request identity. It performs no repository/device
initialization or implicit policy refresh and cannot enable or replace capture.
Unknown acknowledgement retains the original command; native publication and
historical import remain separate. Completion notifications may hold the larger
body-free aggregate observation behind bounded indirection; that notification
grants no resource ownership and does not change the cached Copy runtime
snapshot, scheduling or settlement.

Installation source coverage MUST be a bounded cached device observation, served
only through current LocalTrusted Admin/configuration/installation and private
journal checks. Disabled reads return an empty disabled page without
capture-state I/O. Enabled reads require an existing runtime observation;
unexpected producer loss withholds stale nonterminal data. A completed stopped
observation remains readable and grants no ability to restart an owner.

Each runtime issues a fresh opaque generation and monotonically increasing
source selector ordinals. Selectors bind retained physical/header slots in that
runtime; retirement never reuses an ordinal and restart invalidates the old
generation. They expose no path, native session key, source-instance identity or
physical/header digest, and authorize no enrollment, cursor advancement, import,
publication or checkpoint. Import must independently revalidate the retained
source and current repository/Write admission; it remains a separate gate.

Source pages use a default limit of 50 and maximum 100. Typed cursors bind the
runtime generation and last selector ordinal; foreign generations withhold as
stale, invalid bounds/encodings fail closed, and guessed ordinals beyond the
runtime high-water mark cannot enumerate a fabricated continuation. Current
cached membership may grow or retire between pages; monotonic ordering prevents
reuse or duplicate enumeration. These pages do not freeze evolving diagnostics
or select a native repository snapshot. No continuation token retains storage,
raw frames, directory proofs or a task owner.

Per-source diagnostics include the qualified profile/version, owned reader
phase, observed/awaiting-activity/caught-up/running/degraded flags, last
verified bound repository when known and exact cached ingress capacity refusal.
Missing binding is an unknown observation, not proof of an unbound repository.
Observed and running describe reader work, not continuing model execution or
historical eligibility. Caught-up uses the same conservative settled queue
condition as aggregate status. Sources and aggregates retain existing
reader/frame/source bounds. A separate bounded body-free observation cache
leaves the Copy aggregate snapshot and all native/source/spool/control formats
unchanged; reading pages performs no transcript filesystem work.

### Explicit inactive-history import

The private version-1 import journal, native source enrollment and retained
library scheduling support the typed public import/inspection flow. Browser
management remains pending. Configured Codex startup uses the foreground
composition specified above. Optional retained accepted-import recovery is
qualified separately below. Its explicit command MUST bind a nonzero
caller-retained request identity, control generation, expected policy revision
and one current source selector. Current LocalTrusted Admin supplies the actor;
clients cannot supply paths, roots, native session keys, source proofs or
storage lifetime. New requests require an existing enabled, unpaused runtime and
a retained qualified physical/header slot. Cached coverage, rescan and a stopped
observation cannot enroll history or create an owner.

An import admission policy guard MUST retain the installation manager gate
through the owned operation. It accepts only the journal at the configured
`state_dir/agent-capture/imports`, bound to the exact installation and control
generation. Its caller/command are validated before private state I/O; disabled
admission reads no capture state. Resolve an exact selected request before
revision or runtime-selector rules for new work. The guard then pins the exact
control directory, lock, selected file and policy throughout settlement,
rechecking current Admin/configuration/installation and live runtime
command/observation. New work requires the selected revision and retained
runtime selector; replay retains the original receipt and bypasses only these
new-selection checks. Both modes withhold work while paused, stopped, lost or
unavailable. A post-application loss is an unknown outcome, preserving accepted
intent. The guard itself grants no source, native Write, frame or storage
ownership; owned source admission must independently provide them.

An owned import settlement guard MUST retain the same manager permit and exact
selected control directory/lock/file across native admission awaits. After
durable selection it also owns the import coordination descriptor and exact
selected receipt/file stamp. Current checks use those held descriptors, never
recursively reopening their locks. Receipt resolution still precedes new
selector/revision rules, and replay never substitutes offered source metadata.
Postselection policy, runtime or physical-binding loss reports an unknown
outcome while retaining the original receipt for reconciliation. Cancellation of
a caller cannot release gates retained by the owned operation; completed
settlement releases them once. This guard supplies no source frame, task owner,
storage lifetime, native Write or publication authority. Those remain required
independent worker capabilities before source enrollment or body ingestion.

Before enrollment, reconstruct bounded current context from the exact source and
obtain fresh native directory/socket/catalog admission and Write authority. The
cached repository is not this proof. Preparation may precede durable acceptance
but stays in an owned bounded reader frame, without source allocation, offset
acknowledgement, raw persistence or publication. Unsupported, unbound, revoked
or changed bindings withhold enrollment. Existing source/frame/record,
partial/prefix, cancellation and storage-lifetime bounds apply.

Select a separate installation-private versioned import intent before historical
source admission, progress or publication. Its receipt retains the authenticated
actor, exact original command, installation/control generation and opaque
durable binding to the selected physical/header/native session and qualified
profile. Persist domain-separated digests, never raw paths, headers, keys,
bodies or directory proofs. The import journal is independent of pause
revisions, AgentStore and native publication receipts. Initially retain at most
4096 requests within 2 MiB; exhaustion refuses new intent without evicting
accepted identity. Ordinary open/lookup cannot initialize, repair or reset
missing, corrupt, foreign or rebound state. Exact private directory/lock/file
checks, synchronized selection and retained gates preserve the existing
unknown-outcome policy.

The accepted intent also pins a stable complete-record backfill boundary and its
opaque prefix witness. Partial EOF records grant no offset or completion
authority. Appends are continuing tail coverage; changed/truncated prefixes
withhold the original selected work rather than moving its boundary or
reauthoring its receipt.

Read-only import preparation MUST scan no more than the existing 4-MiB prefix
verification budget per step, retaining only bounded record/context state and
opaque digest accumulators. Initial preparation selects the last complete record
of a stable snapshot. Reconciliation retains the accepted boundary while
reconstructing current complete-record context, independently verifying
physical/header/session/profile binding and the entire selected prefix. An
append may update current context but never extend accepted history. A changed
snapshot restarts only read-only preparation; a changed accepted prefix fails
closed. Neither preparation mode allocates source identity, aliases, leases,
progress, intent or native state, and EOF releases partial record allocation.
Fixed bounded header checks remain independent of that prefix budget. The Codex
version-1 binding uses domain-separated SHA-256 under `jj-store/codex-import/`:
`physical/v1` binds device/inode big-endian u64 values, `header/v1` binds the
qualified header witness, and `session/v1` binds the exact product version and
lowercase native session key. Each input part has a big-endian u64 byte length.
The complete-prefix witness hashes `prefix/v1` plus a NUL after that common
domain, then exact transcript bytes through the selected newline. Only these
digests and complete offset/count leave device preparation; neither raw context
nor keys are journal data.

OpenCode API import preparation uses the qualified native message chain, with
one bounded API page per step and no retained raw page after that step.
Selection scans the observed history, then independently verifies its selected
prefix in a second pass. Reconciliation verifies the original accepted prefix
and allows a later tail without changing the receipt. Every page repeats exact
health, session, version and directory checks. Endpoint or native session
rebinding, rewrites, truncation and exhausted page/record limits withhold the
boundary. Neither preparation mode enrolls a source, acknowledges a cursor,
selects intent or publishes native state.

The private version-1 API import binding retains the existing closed fields:
`physical_digest` is a domain-separated endpoint digest, `header_digest` is the
exact endpoint/session/directory/version source witness, and `session_digest`
binds the case-sensitive native session ID. Endpoint and session digests use
`jj-store/opencode-import/endpoint/v1` and `session/v1`, followed by NUL and
big-endian u64 input lengths. `prefix_digest` is the unchanged native snapshot
message chain. API `complete_byte_offset` MUST equal zero; `complete_records` is
the logical message count, bounded to 131072. Empty history requires the exact
initial prefix for its source witness. Existing Codex byte/count encodings stay
unchanged; unqualified Cursor/export bindings cannot enter this journal. This
private preparation qualification does not enable public API history import,
source enrollment or configured OpenCode startup.

Resolve a selected request before interpreting its original selector as a new
runtime selection. Exact replay returns the original receipt and current bounded
progress; changed reuse conflicts. New retired/foreign selectors fail as stale.
Restart reconciles accepted work only against the exact durable source binding
and fresh admission; it cannot retarget a replacement file, session or selector.
Receipt lookup grants no read/publication authority. Pause, disable, revocation,
unavailable resources and owner loss retain intent and withhold new work.
Disabled management performs no capture-state I/O. Timeout or caller
cancellation requires the original request; neither can replace identity or
restart an owner.

Explicit consent admits only the selected qualified logical session. Native
aliases, leases, cursors, sanitizer and exact publication receipts remain the
common path; import cannot duplicate sessions/tools, reset progress or bypass
unknown outcomes. Each historical batch independently verifies its directory and
native Write authority; ambiguous/unauthorized cross-repository bodies remain
withheld or constant omissions under existing rules. A new
physical/header/native session cannot inherit historical consent. Acceptance
acknowledges durable intent, not completed ingestion or a committed root.
Progress distinguishes selected, pending, running, withheld, completed and
unavailable. Completion requires the selected backfill boundary and
corresponding publication receipts; continuing tail coverage is separate. Import
grants no replay execution, checkpoints, configuration enablement, filesystem
mirroring, authenticated capture or another harness profile.

Automatic JSONL and API snapshot reader enqueue MUST retain separate body-free
publication requirements before acknowledging its cursor. Reader progress, an
empty pending queue and identity-only spool completion are not committed
publication evidence. The device publication journal folds only exact validated
native receipts, in source order across repository partitions. A later confirmed
partition cannot advance the contiguous frontier past an earlier unconfirmed
range. Receipt selection precedes pending-body removal and survives exact retry
after restart. Its version-1 catalog retains at most 4096 channels and 4096
outstanding ranges within 2 MiB; capacity refuses new requirements without
evicting a hole or confirmed frontier. Ingress capacity is checked before
requirement selection. Shared reader enqueue into an already tracked channel
MUST append the same requirements before acknowledgement; it cannot create a new
publication channel or turn an acknowledgement into receipt evidence. This
preserves mixed reader recovery without admitting unqualified history. The
private `reader-publication/catalog.json` lives under the bound spool and uses
its existing mutation coordination. Channels bind integration, source, numeric
generation and opaque reader generation digest; ordered ranges bind repository,
batch, first/last sequence and a typed position. JSONL ranges retain their
complete byte position/prefix digest and unchanged version-1 encoding. API
snapshot ranges add an optional typed snapshot digest and logical message count;
byte fields are zero and cannot serve as snapshot evidence. A zero message count
is valid for an approved lifecycle checkpoint. Snapshot publication advances by
source sequence, including status-only transitions. A new numeric generation may
reduce its logical message count after replacement; it cannot jump a prior
unconfirmed range. Body-free snapshot inspection reports the selected sequence
boundary separately from tail sequence/count, with an absent committed count
until qualified receipts exist. It uses the same existing inspection lock and
never reads pending bodies, initializes or repairs state. Unknown earlier
history stays unqualified; a proven boundary survives unavailable later tail
metadata. Byte and snapshot coverage must reject the other position type. Folded
receipts retain a domain-separated digest chain and the last exact body-free
receipt identity. No paths, native session keys or transcript bodies enter this
catalog. Existing history without this evidence remains explicitly unqualified;
neither reopen nor a later receipt promotes it. Read-only coverage is bounded
device metadata, grants no publication authority and reports continuing tail
separately from a selected import boundary. Local companion completion and
nonautomatic enqueue cannot grant native coverage. Receipt digests hash
`jj-store/reader-publication-receipt/v1` plus a NUL followed by the exact
version-1 receipt JSON. The chain hashes `jj-store/reader-publication-chain/v1`
plus a NUL, previous 32-byte chain and the version-1 JSON pair of channel key
and confirmed range/evidence. Zero is the initial chain; only validated
committed receipts advance it. Missing, foreign, corrupt, oversized, symlinked
or permissively accessible selected evidence fails closed, and cannot be
repaired into successful coverage. A missing whole publication history cannot
qualify already acknowledged reader history.

Read-only publication coverage MUST combine every numeric channel of the exact
integration/source/physical-header digest, in generation order. A later numeric
generation cannot hide an earlier withheld range or replace unknown history.
Unrelated physical digests remain separate. A pending continuation retains the
previous committed frontier; missing publication channels for retained reader
history leave coverage explicitly unqualified. Inspection validates each
channel's latest range against retained reader progress and sums folded receipts
without counting unrelated physical files. This is bounded metadata inspection,
not transcript rereading or a new journal format.

Before exposing completed import progress, bind the original accepted receipt to
its exact reader channel and physical generation witness. Retain or reconstruct
that association under the original intent/source admission across restart. An
ephemeral selector, the latest repository cursor or an unrelated source
generation cannot stand in for it. The publication frontier alone is source
coverage, not per-request import completion. Management inspection must use
bounded body-free evidence under current installation admission, without
transcript filesystem work or creating a source owner.

Public progress MUST project the accepted byte boundary independently of later
tail coverage. It validates the exact associated physical/header digest across
numeric channels in order, refusing earlier holes, missing reader/publication
metadata and unqualified history. Already-qualified earlier numeric generations
of that physical file may prove the accepted boundary even when the initial
import association has a larger numeric generation. Stop the accepted-boundary
proof once contiguous committed receipts cover it; unknown or lost later tail
history cannot erase this proof. Tail coverage remains independently qualified.
No pending body is opened, recovered or removed during inspection. Use persisted
bounded reader metadata and publication catalog under the existing spool FIFO
and existing file lock, checking the retained spool binding before and after. A
missing lock/catalog or rebound/private-state failure withholds coverage;
inspection never creates or repairs those files. Structurally corrupt evidence
cannot become completion. No new persisted publication format is introduced.

The version-1 HTTP/CLI import outcome MUST contain its exact original public
command, accepted ordinal/profile/version and complete-record byte/count
boundary, plus bounded progress. It exposes no actor, path, native key or
binding digest. Local Admin inspection selects one nonzero request ID and
performs no source I/O. Selected/pending/running/withheld/unavailable describe
metadata and cached exact-request observations; completed requires associated
receipt coverage. Accepted history completion and continuing tail phase/coverage
remain separate. No-acceptance status cannot report completion. Existing
receipts are readable while paused/stopped. New command dispatch is one-shot to
retained workers; volatile scheduling alone cannot grant public acceptance. A
five-second bounded wait for durable intent returns unknown on timeout or
post-dispatch authority loss; original command identity remains mandatory.
Caller cancellation neither releases owned work nor restarts an owner.
Foreground preparation may open an existing import journal and fork the selected
spool handle; missing import state withholds management without initialization.
Browser qualification remains pending. Retained runtime recovery follows the
separately qualified lane below.

Automatic accepted-import recovery MUST use one bounded body-free supervised
metadata lane on the existing retained runtime. It reads only the exact existing
installation journal after current LocalTrusted Admin/configuration and private
binding checks, and offers at most one original accepted command per step to a
uniquely matching retained source. It never mints intent, repairs missing state,
creates source owners or starts an inactive session lacking accepted consent.
Current pause, terminal/lost owners, changed source identity and revoked
installation/native Write continue to withhold work. Recovered commands renew
the original accepted prefix and native admission through the common import
driver; selector changes cannot retarget them. Metadata polling is bounded and
coalesces exact already-retained requests without starving source work. Its
owned policy acquisition MUST queue fairly behind management inspection, retain
that permit through original-command resolution and obey the worker deadline;
frequent progress reads cannot indefinitely withhold recovery. Imported source
preparation and renewal MUST use the same fair owned policy acquisition so
inspection cannot starve ingestion, publication or parked lease transfer.
Synchronous policy I/O remains on retained blocking work; cancellation cannot
release a permit while that work is still running. The lane shares pause, stop,
cancellation, deadline and storage-lifetime settlement with the existing
supervisor. Completed history may still recover a continuing tail; completion is
not permission to skip renewed admission.

The private version-1 import catalog MAY retain an additive `readers` array,
bounded by its 4096 receipt and 2-MiB catalog limits. Each entry binds one
original receipt ordinal to the admitted source ID, initial positive numeric
reader generation and exact physical/header generation digest. The owned capture
format MUST verify that digest against the accepted header digest. Select the
association only after original-prefix/native Write admission and opening the
common leased reader, while retaining the exact policy/import selection gates. A
`prefix_verified` boolean defaults to false for older reader associations; those
links cannot establish public completion until renewed native/source admission
verifies any retained raw reader checkpoint against the current accepted
physical bytes. The probe shares its existing four-MiB step allowance with a
separate bounded standard SHA-256 checkpoint accumulator. Mismatched or
truncated retained prefixes fail closed before linking old publication history;
they cannot be silently reset into completion for a fresh request. No-progress
readers grant no old receipts. Retain the accumulator only for the exact
unchanged physical snapshot; advancing reader cursors extend its hash rather
than repeatedly rescanning previously verified bytes and starving publication. A
shorter cursor restarts verification and every snapshot change clears the
accumulator. Pin the accepted import boundary before later checkpoint steps so
appends cannot extend consent. On upgrade retain the original
ordinal/source/initial numeric generation and immutable receipt, selecting only
the verified flag under the existing import gate and refreshed file stamp.
Common imported preparation checks the same checkpoint before link
reconstruction and body work. Inspection does no prefix scan and cannot upgrade
an unverified association. Synchronize selection before returning an imported
pipeline; refresh the held file stamp after the atomic catalog replacement.
Exact retry returns the original association and receipt. A later positive
numeric continuation may reuse that association only for the same source and
physical digest; it cannot replace the original numeric witness. An unknown
outcome retains original intent for reconciliation. Missing associations remain
unqualified until renewed original-source admission reconstructs them. Invalid,
duplicate, foreign, oversized or changed associations fail closed; lookup never
opens transcripts, leases or owners and never grants publication authority.
Existing receipt-only version-1 catalogs remain readable. Accepted receipts are
immutable; association selection cannot change their input, boundary or order.

Registry import admission MUST require a completed opaque preparation selecting
the exact accepted binding and a synchronous current-consent check retained by
the caller's durable intent, installation policy and native Write gates. Recheck
consent and the stable source snapshot before source/alias allocation,
immediately before registry selection and after selection under registry/FIFO
coordination. Offered digest metadata or ordinary registration alone cannot
admit history. Preselection loss leaves the catalog unchanged; postselection
loss retains the marker and requires exact original-request reconciliation.

Imported admission uses the existing version-1 bounded admission-digest catalog,
with a separate `codex-import-admitted/v1` key under
`jj-store/source-registry/`. Its length-prefixed parts are the qualified product
version, existing physical-lease digest, exact physical/header witness and
existing logical-session admission digest. Existing active-session markers
remain unchanged. Ordinary active lookup/resume MUST NOT consume import-only
markers. Import lookup, lease acquisition and reserved-channel reacquisition
require renewed exact accepted-prefix preparation and retained consent/current
policy/Write gates. This holds even after import selection but before the first
sanitized cursor: a rewritten prefix cannot become ordinary offline eligibility.
Each bounded prefix preparation remains independently scheduled; registry lookup
does not scan transcript bodies. A hardlink or overlapping approved root may
retain the same channel; replacement physical files, changed
headers/sessions/profiles cannot inherit import-only consent. Enrollment neither
resets durable reader progress nor grants prefix/body or publication authority.
Completed preparation is independent of accepted intent; this registry primitive
does not implement executable server import or scheduling.

The server source-import driver MUST obtain the actual retained qualified slot
from a separate runtime-private target view after current installation
admission; public observation pages remain body-free and carry no qualified
binding. New requests use the exact current selector. Replay searches retained
slots by the original opaque physical/header/session/profile binding before
considering an old selector, and fails closed when no unique current slot
matches. The target view is bounded by the existing source limit and grants no
lease or frame.

Native import preparation MUST retain the owned settlement through each native
admission await. It verifies bounded accepted-prefix/current context, fresh
native directory admission and Write, selects durable intent, then acquires the
shared import-only registry lease and opens the common reader at existing spool
progress. No source allocation or progress precedes intent selection. Rechecks
surround lease enrollment and reader opening. Imported pipelines MUST retain an
opaque renewal capability for the original receipt, reacquire current
policy/intent gates before body inspection, and renew accepted-prefix
preparation before ingestion, publication recovery and parked lease transfer.
They expose no conversion to an ordinary active pipeline. Parking preserves the
renewal capability and raw-channel reservation; failed or cancelled native
awaits cannot consume the parked frame. Each step releases policy gates after
settlement so pause remains actionable. This source driver does not allocate
supervisor budgets, own storage or dispatch work: those independent
retained-worker capabilities remain scheduling gates.

Explicit library scheduling MUST use the existing retained source slot and
reader/raw-frame/storage budgets. A single bounded runtime mailbox carries only
opaque body-free original-command metadata. Installation admission selects the
actual current target before dispatch; queuing releases its policy descriptors
and grants no acceptance, enrollment or body work. A busy/noncompact source
keeps its original reader until it can safely install the request. Each slot
retains at most one original command; exact retry cannot replace it, and
changed/conflicting work is refused without resetting the source. Stopped/lost
owners cannot be recreated by a scheduling request.

Every import preparation or prefix/body step MUST reserve a frame before opening
its probe and renew current installation/control/intent authority. The original
command and prefix state remain on the retained owner through cancellation and
deadline; only after that step's future settles or is dropped may policy gates
be released for pause. Retry reacquires the original accepted boundary before
enrollment. Imported parking carries incomplete and completed prefix state
without forcing a new history selection or monopolizing a reader lease. Raw
state is compact only when both common reader/recovery and import prefix state
are compact. A completed prefix witness remains cached only while its exact
snapshot is stable. Changed completed snapshots reconstruct the accepted prefix
before granting body work; incomplete scans remain retained within their budget.
Guarded publication runs on the same retained source owner, with fresh
original-receipt consent and accepted-prefix checks inside native publisher
preparation and publication awaits. Generic delivery cannot consume import-only
eligibility. Volatile scheduling acknowledgement is distinct from durable
selection and completion. Public progress observes accepted metadata separately;
accepted-request recovery uses the separately qualified supervised lane.

Implementation proceeds through intent/replay, owned admission/scheduling, typed
local API/CLI, then browser management and cold qualification. Regressions MUST
cover exact retry across timeout/restart, changed reuse, stale selectors,
byte-identical physical replacement, current Admin/Write/configuration/identity
loss, missing/rebound state, paused/stopped/lost owners, capacity and
cancellation retaining gates/frames/storage. Prove inactive headers remain
unadmitted until explicit acceptance and preserve aliases, progress, omissions
and unknown receipts. Configured Codex startup does not qualify other readers,
browser management, broader faults or real agent edit acceptance.

### Retained runtime and storage ownership

Explicit capture composed with authoritative storage MUST acquire an opaque
storage lifetime before its first dispatch. Every discovery, source and delivery
owner, including its queued/running blocking task and unconsumed completion,
retains that lifetime. The lease is resource ownership only; it grants no
repository admission, Write permission or runtime enablement. Late attachment
and replacement of an existing lifetime fail closed. Authoritative storage
shutdown waits for the last owner before stopping embedded storage or releasing
its installation lock. Dropping storage or cancelling its shutdown wait MUST
retain those resources through the exact outstanding capture owners; driver loss
or a detached blocking task cannot release them early. Runtime shutdown still
has its existing five-second bound and never converts a timeout to proof of
completed drain. Whole-server startup retains this ownership; real harness
qualification remains separate from storage fixtures.

Foreground native serving MUST transfer its external-storage process state lock
into the same authoritative-storage owner before any capture lifetime is issued.
The transfer retains the exact file descriptor and rejects late attachment or an
already-owned lock. Dropping a prepared server or cancelling its run future MUST
NOT release that lock while capture still retains storage. Ordered storage
shutdown retains the lock through the last capture lease and storage stop.
Filesystem-only serving keeps its process lock without acquiring native capture
ownership; automatic capture remains unavailable there. This ownership transfer
qualifies neither configured capture enablement nor the remaining reader,
management and real harness/cold full-flow gates.

Guarded foreground composition MUST use one prepared capture slot. Disabled
selection reads no environment or installation state and creates no capture
directory. Enabled preparation validates the selected reader profiles, native
host, directory verifier, retained storage and exact registry installation
identity before opening private capture state. Codex locations follow explicit
roots, then CODEX_HOME, then HOME; privacy requires bounded absolute home and
temporary paths. The source registry, sanitized spool and scanner cache live
under the private server-state agent-capture directory outside checkouts.
Preparation opens no transcript, admits no historical session and initializes no
body scanner. Unsupported profiles, invalid privacy state or unavailable
resources retain a body-free unavailable slot rather than disabling ordinary
service or falling back to another reader. The prepared slot attaches storage
ownership before dispatch. Foreground execution starts this exact slot and
drains it before storage shutdown; cancellation drops its control handle while
retained owners finish draining. Whole-server Codex startup uses this same
composition and independently selected configuration; other reader, browser,
pressure/fault and real agent edit qualification remain separate gates.

An explicit Codex supervisor composes configured root discovery, activity
probes, staged admission, parked ingress and exact delivery without enabling
process startup. Disabled preparation performs no source I/O. Discovery
qualifies at most one candidate per owned step, deduplicates an opaque
physical/header fingerprint and tracks no more than the configured source bound.
Missing roots are retried on the configured rescan interval; discovery grants no
historical eligibility.

Source steps, discovery and delivery have independent retained blocking owners.
Forked in-process spool handles share a FIFO gate for synchronous journal
operations so repeated local lock collisions cannot starve a reader. Each turn
still acquires the authoritative filesystem mutation lock; independently opened
handles and processes retain their existing fail-closed lock behavior. The
compatibility lease stays alive until the last fork closes. The mutation gate
and mutation lease are released before native publication awaits. Acquisitions
through one shared source registry also take FIFO local turns before the
existing registry/physical/channel filesystem leases. Compact parked readers
probe exact metadata, header, durable progress and current admission without
reacquiring reader leases or reading source bodies. Changed snapshots/progress
need a reserved body frame before resumption; idle metadata grants no ingress or
publication proof. At most the configured one through four source owners may
hold a reader or run a source step. At most twice that bound may retain
noncompact source frames; compact activity/reader frames hold no partial
transcript allocation. Only a reserved frame may start body reconstruction or
ingress. A cancelled owner keeps its reservations until a completed step proves
them released. Exact pending batches and unfinished prefix proofs cannot be
compacted or evicted to make room. Delivery separately retains at most twice the
reader bound of independent partition recovery frames, preserving multi-step
proof progress across turns. Queue snapshots cannot report caught-up while
source or delivery work is in flight or a later enqueue has made that snapshot
stale. These bounds and the connected native-service regression are distinct
from full pressure, fault, rotation, typed-control and cold server/harness
qualification; configured Codex startup retains these bounds.

A compact source owner whose qualified physical/header binding is no longer
current may retire its memory-only tracking slot after its owned step settles.
Retirement MUST retain the source catalog, sanitized cursor and exact durable
queue and invalidate stale caught-up queue status. It MUST NOT evict an
unfinished prefix/context proof, raw batch or noncompact source frame, or
replace a busy, draining or lost owner. Task loss still requires explicit
replacement. Removing a compact slot removes its opaque deduplication
fingerprint and clears the prior tracking-capacity refusal; subsequent discovery
must independently qualify and arm or resume a source within the same configured
bounds. A new native session requires genuinely new observed activity after
arming; historical starts in a replacement file supply no eligibility. Retained
native-session resume keeps its existing current admission and exact-progress
checks. Read-only current-binding inspection supplies no identity, lease, cursor
or publication capability. Metadata and header inspection errors withhold the
compact owner for retry; negative binding results retire only body-free state.

An unadmitted activity probe whose bounded scan definitively detects truncation
or a changed verified prefix MUST discard that invalid activity/preparation
state and arm a fresh boundary on its next reserved source step. The typed
terminal probe state distinguishes these outcomes from retryable metadata,
header and I/O errors, which retain the original probe. Rearming MUST recheck
the exact qualified physical/header binding and treat every record already
present at the new boundary as historical, including starts appended with the
rewrite. Later body growth alone supplies no eligibility. Only a new observed
start after arming may admit sanitized backfill. Rearming allocates no source
identity, catalog entry, cursor, lease or native publication; existing admitted
reader frames and exact durable intent retain their independent recovery rules.
The same tracking slot and resident bounds apply across pause/resume. This
recovery does not qualify configured process startup or the full rotation
matrix.

When sanitized spool capacity refuses all new batches, exact unacknowledged
source frames still occupy their configured reservations. The supervisor MUST
NOT evict them, acknowledge bytes or enroll additional sources by exceeding the
resident-frame bound. Pausing retains those reservations; completed shutdown
releases owned memory and leases without persisting raw transcript state.
Stopped source owners MUST report zero occupied/resident reservations even when
shutdown completed directly from an idle paused state. Draining owners retain
their reported reservations until their synchronous work settles. Explicitly
restarting with a larger configured spool reconstructs unacknowledged bytes only
through retained native-session eligibility and fresh current directory/Write
admission. Sources whose new starts were never admitted require fresh observed
activity after restart; historical starts cannot fill that gap. This
capacity-refusal/restart qualification is separate from a saturated durable
queue, source rotation/retirement, faults and cold whole-server startup.

A durable queue filled by exact prepared native intent MUST refuse additional
ingress without acknowledging source bytes or replacing the original request,
actor or receipt identity. A committed native publication whose acknowledgement
was lost remains unknown to the caller until the original request resolves its
receipt under current admission. Revoking the prepared entry's repository also
withholds receipt resolution; another current directory cannot lend authority.
Pause and explicit supervisor restart preserve the selected prepared bytes and
latest sanitized source cursor. Once original admission is restored, the worker
may resolve that exact receipt and release queue capacity, then independently
admit later repository batches. A receipt retry cannot reauthor AgentStore
roots. This prepared-queue pressure qualification is separate from
captured-entry preparation growth/headroom, source rotation, fault matrices and
process startup.

Captured-entry admission MUST retain bounded preparation headroom so a permitted
queue head cannot become permanently undrainable solely because
Captured-to-Prepared growth fills the configured byte budget. Headroom must
preserve the existing exact-intent and hard byte-bound contracts without
assuming a native delta's contents, preauthoring dependent same-partition
captures or evicting accepted evidence. Legacy queues without enough preparation
capacity MUST retain captured bytes/cursors, withhold native publication and
expose the need for explicit configured capacity relief. The strict byte-limit
profile retains that legacy fail-closed behavior. New automatic reader captures
reserve the larger of the native and local prepared encodings as trailing JSON
whitespace in their atomically synchronized pending file. Every handle,
including existing hook writers, counts these physical bytes. Native sizing
covers all 4096 permitted delta edits, full references, the exact runtime
acknowledgement and any explicit binding with a maximum timestamp; it MUST NOT
assume which edits the current projector produces. Local sizing covers the exact
duplicated profile, sanitized batch and binding. Neither reservation authors a
request or reads a repository root. Preparation replaces only the selected
partition head and releases unused reserved bytes. Retry and restart preserve
accepted intent; no cursor is acknowledged when reservation capacity is
unavailable. Explicit hook admission and legacy compact records retain their
existing byte-bound profile. Configured startup preserves these contracts;
management diagnostics and full pressure/fault qualification remain separate
gates.

Admission and preparation byte-capacity refusal MUST retain a body-free typed
diagnostic with the configured limit and exact required total physical bytes for
the attempted replacement or admission. Existing reserved whitespace counts
toward that total; replacing one pending entry excludes its prior physical
length. These numbers authorize no capacity increase, queue eviction or
publication and contain no path, body, repository, principal or request
identity. Non-capacity errors MUST NOT masquerade as insufficient space. A known
refusal preserves exact pending bytes and reader cursors until successful retry
or explicit relief. Internal supervisor status retains the last settled ingress
refusal per source and the last settled delivery refusal, aggregating ingress
requirements by their maximum. Pause preserves these observations; successful
later steps clear them, and stopped/lost source owners cannot report active
ingress pressure. Status performs no filesystem work and cannot assert that a
cached refusal remains current after independent writers change the queue.
Authenticated controls and coverage presentation remain separate management
gates.

Loss of a queued discovery or delivery task MUST close that exact owner and
increment the body-free unavailable diagnostic once on settlement. Delivery loss
also invalidates queue freshness; it cannot report caught-up from a prior queue
snapshot. Neither lost global lane may be revived by pause/resume or replaced
implicitly. Accepted sanitized batch identities, exact pending bytes and source
cursors remain unchanged. An explicit new supervisor independently reacquires
current retained-session eligibility, directory and Write admission before
resolving the old queue; losing a task supplies no publication or history-import
authority. Queued task abortion qualifies this loss boundary separately from
panic after dispatch, OS-I/O loss, real storage or full cold harness faults.

Panic after discovery has qualified a candidate, or delivery has recovered an
exact queued range, MUST retain authoritative storage and its process lock
through task-state destruction. Cancelling a shutdown wait or dropping the
controller while that synchronous step is blocked cannot release storage early.
Settled task loss closes the lane and reports unavailable once. An already
committed native publication with a lost acknowledgement retains its exact
Prepared request, actor, receipt identity, queue bytes and sanitized cursor.
Explicit replacement still requires current admission before resolving that
receipt; resolution cannot duplicate the publication or rewrite native evidence.
Running-task panic qualification is distinct from OS faults, real RustFS and
live harness acceptance, and supplies no automatic startup authority.

An exact queued-range proof MUST be revalidated after current-directory
admission and before publication. Rebinding its approved transcript root,
installation-private source registry or sanitized spool while admission is
blocked cannot lend the old proof to replacement state, even with identical
transcript bytes or a symlink to the retained state. Withholding preserves exact
pending bytes, source catalog and sanitized cursor. Restoring the original
physical state permits independently qualified recovery under current admission;
neither the binding failure nor the restoration supplies history-import
authority. Loss of an initialized source catalog during that handoff remains
unavailable; the worker cannot reset source allocation or substitute an empty
catalog.

An owned pipeline-step executor MUST keep the pipeline, both reader leases,
private spool handle and current authority/admission context in exactly one
owner. It runs bounded synchronous reader/sanitizer/spool work on an owned
blocking task, while native asynchronous admission/publication uses the existing
runtime. At most one step is in flight for this owner; no second step or
replacement may race a busy or undrained owner. Deadlines are positive and at
most 300 seconds, begin at dispatch and prevent a queued step from starting
after expiry. A deadline or cancellation interrupts asynchronous waits while
retaining original cursor and durable/prepared intent for exact later work.

Synchronous OS work may outlive a deadline. Its owner MUST remain retained and
report overdue/draining until that work settles, rather than claiming that the
I/O stopped or releasing its leases for a replacement. Ordinary runtime work and
status/pause calls remain responsive. A known completed result stays known even
if it returns late; an interrupted publication is not a proven conflict. Pause
requests in-flight cancellation and stops new dispatch; resume cannot start
another step while the old owner drains or restore missing native Write.
Explicit shutdown stops dispatch, requests cancellation and waits at most five
seconds; timeout retains the owner for later settlement. Successful shutdown
releases its resources. Unexpected task loss closes that owner and requires
renewed source qualification; it cannot fabricate a reader or discard pending
evidence. Supervisor pause/resume may resume healthy lanes while the lost source
owner remains closed. It MUST NOT restart that source or acknowledge its unread
tail through the old owner. An explicit supervisor replacement reconstructs
retained admitted native-session progress and renews current physical/channel,
directory/catalog and native Write admission before resuming ingress. Revoked
queued partitions retain exact bytes and cursors across loss and control turns.
Drop requests cancellation; any running blocking task retains its OS leases
until it settles. Server composition must drain these owners before
authoritative storage shutdown. This executor does not schedule discovery,
allocate additional readers, authorize runtime enablement or qualify automatic
startup/cold full flow.

Queued Codex recovery MUST verify the admitted channel, qualified physical file,
exact header/generation witness, native session, configured parser/profile,
complete ending offset, prefix SHA-256 and deterministic batch identity before
using a reconstructed directory. It scans at most 4 MiB per step, retains at
most 1 MiB of a partial record and at most 256 trailing record contexts, and
rejects noncontiguous observation sequences, incomplete endings and mixed
contexts. The exact queued range is the retained observation count of complete
records ending at that verified offset. Raw history and reconstructed paths
remain in memory without Debug or persistence encodings. Source changes restart
bounded verification; replacement, rewritten prefixes, missing sources and
unavailable bindings preserve pending intent and withhold publication.

Recover queued entries in generation/sequence order, one exact batch per step;
other repository/source partitions remain untouched. New source reads wait for
all of that channel's unresolved entries, then reopen at the latest sanitized
durable source cursor without relinquishing either reader lease. Reopening
failure preserves the next-step resume requirement and cannot discard a
committed receipt. Current directory/socket/catalog and native Write
authorization precede every exact preparation or prepared retry. A known context
must resolve to the queued repository before body publication. Only a batch of
exact constant unverified-directory omissions may use a separately authorized
reconstructed prior directory or header anchor matching that queued repository.
This fallback cannot authorize body evidence. If the conservative prior anchor
cannot be rebound, retain the gap for later recovery. Prepared bytes,
authenticated actor, request/publication identities and receipt meaning are
never regenerated during recovery. Fresh-process proof recovery is qualified
separately from full cold server startup and real-harness acceptance.

Codex uses a versioned bound transcript reader. Cursor uses a validated
transcript when available and reports reduced coverage when absent/disabled.
OpenCode uses an explicitly bound read-only session/message API or qualified
consistent local storage, plus explicit exported JSON import. The server never
launches another harness, runs export commands or tails a database as JSONL.
Document each product surface/version, locations, overrides, format, activity
detection and coverage with sanitized fixtures. Desktop and CLI products qualify
independently.

The OpenCode 1.18.30 acquisition contract is pinned to source revision
`3104c1428ec91f809e5ab86631300de41eb6952e`, its V2 generated SDK types, instance
session/message routes and `export` implementation. The read-only adapter
requests only global health, session metadata/status and bounded message pages;
it never follows a supplied Link URL, sends credentials, follows a redirect,
prompts, launches the harness or runs an export command. A health version
mismatch withholds acquisition. Explicit busy/retry status may prove current
activity; session existence, title, updated time and exported messages alone
cannot. Message paging uses the opaque X-Next-Cursor value only against the same
configured endpoint/session/directory, with bounded responses and repeated
cursor refusal. API pages are observations, not a claim of one globally pinned
OpenCode database snapshot.

Acquisition responses are at most 1 MiB, API pages contain at most 32 messages
and 256 total parts, status maps have at most 4096 entries, and message
acquisition traverses at most 4096 pages. Oversized/malformed pages retain the
prior acquisition boundary; no source acknowledgement follows a refusal. Each
assistant snapshot retains its own declared native `path.cwd` as a memory-only
directory context. Missing/unsafe context cannot borrow the session header
directory to publish body evidence. Header version, message/part owners,
duplicate identities and changed session directory are checked independently.
These acquisition bounds are not complete filesystem export-reader or large
history qualification; those readers still require bounded streaming, safe
omissions, durable progress and exact restart coverage.

Native API and explicit exported JSON message snapshots share a distinct
`opencode-snapshot/v1` sanitized envelope, while existing plugin/hook semantics
remain unchanged. Exact session, message and part identities and product version
must agree before reduction. API status may project lifecycle only under
NativeApi/ApprovedHarnessApi evidence; local exported snapshots use
SessionExport/LocalTranscript and never imply running activity. User/assistant
messages preserve their complete sanitized native parts and stable message
aliases; tool parts enrich their native call with declared pending/running/
completed/error state. Unknown part kinds retain safe evidence and diagnostics
without invented tool or lifecycle transitions. Cross-source alias enrichment,
reader durability, supervision and real-edit coverage still require their own
complete acceptance; acquisition/normalization tests alone cannot enable a
runtime profile.

OpenCode API source channels use the installation-private source catalog and
existing exclusive source/physical lease classes. Their location/lease keys bind
the canonical approved endpoint and case-sensitive native session identity;
generation/admission witnesses additionally bind the exact session directory and
qualified version. Endpoint, directory and witness bindings enter the catalog
only as digests. An idle/unqualified session cannot allocate automatic
admission. Restart may reacquire a previously admitted exact witness without
requiring renewed execution; a changed directory cannot inherit that admission.
Enrollment is separate from current directory/socket, catalog membership, native
Write and current AgentStore checks.

The same atomic catalog admission MUST retain a bounded private recovery index
of validated case-sensitive native session IDs, endpoint/witness digests,
qualified version and owned source channel. This metadata contains no native
directory, endpoint URL, transcript body or acquisition cursor. It is committed
before admission returns, including sessions that have not enqueued a message.
At most 4096 index entries fit within the existing 2 MiB catalog and alias
budgets; capacity refusal MUST NOT evict prior admission or grant a new lease.
Duplicate, malformed, foreign-version or unbound index entries withhold registry
opening. Read-only endpoint-scoped enumeration allocates no identity, lease or
alias, and supplies discovery candidates only. Every candidate still requires
current API witness, directory/socket/catalog and native Write checks. Ended
sessions may be rediscovered without a fresh busy status; changed witnesses do
not inherit admission. Catalog version 1 accepts an absent optional index as
empty for older qualified state; no ID is inferred from an opaque digest.
Reacquiring an exact admitted binding may atomically populate its missing index.

Snapshot ingress partitions at each message. Sanitization requires the exact
per-record directory used for admission; missing/unbound context may retain only
a constant omission at a separately authorized anchor. The existing sanitized
pending spool is durable before its reader acknowledgement. Stable observation,
batch and snapshot cursor identities survive exact retry and reopen. API and
export trust remain distinct. No native body or path gets a durability/debug
encoding from this bridge, and no publication or runtime profile is enabled by
source registration or successful enqueue alone.

The admitted API source driver acquires an exclusive installation source after
current activity or exact prior admission, verifies its session binding again,
then reads bounded pages. It holds at most one page of raw records in memory and
spools one context-partitioned observation per step. The snapshot cursor is a
domain-separated cumulative native-record prefix digest and logical record
count; the session witness remains distinct from that mutable prefix. An opaque
API paging cursor is never a durable acknowledgement or source authority.

Reopen and explicit refresh reconstruct the selected acknowledged prefix from
new context-bound GETs before accepting another record. Unchanged reconstruction
does not enqueue duplicate observations. A rewritten/shortened prefix starts a
new monotonic generation/sequence allocation, retaining earlier exact intents;
those intents cannot borrow the new prefix. Each pending body requires matching
native prefix, sanitized observation, repository partition and current directory
proof before restart publication. Prepared requests retain their original actor,
bytes and request/publication identities. Each native preparation/publication
rechecks the API session binding, exclusive source/spool ownership and current
directory/catalog/Write policy. Spool refusal does not acknowledge a record.
Publication uses the current native AgentStore and never advances code heads.

Approved OpenCode lifecycle observations use a typed optional private checkpoint
alongside the existing reader generation witness. The checkpoint contains only
message count/prefix and the observed busy/retry/idle class; it contains no
native path, body, opaque paging cursor or execution identity. Its
domain-separated composite digest is the acknowledged snapshot position. Status
may advance that digest and source sequence without advancing message count or
replaying messages. An empty admitted session may acknowledge logical message
count zero under this explicit checkpoint, with its initial prefix bound to the
exact session witness. Message-only snapshot positions and other harness
metadata keep their existing encodings. Optional typed state and ready batches
use memory indirection to keep the shared polling enum bounded; their durable
JSON is unchanged. A pass shares one sanitizer for its current verified
workspace and releases it at idle. Dropping a lifecycle checkpoint, changing its
witness/parser/native session, decreasing its message count or changing a
fixed-count message prefix fails closed. Duplicate state cannot acknowledge
another sequence.

The source records explicit API status at its independently authorized declared
session directory. Messages keep their individual directory partitions. A status
poll is an observation of the approved API class, never a reconstructed
execution from messages, timestamps or exported history. Polling does not claim
an event stream or capture intermediate states missed between polls. Sanitized
constant status evidence and its exact typed checkpoint become durable together
before acknowledgement. Restart may reconstruct that already acknowledged
historical status from its matching private checkpoint and exact constant
evidence, with current source/session/directory/Write proof; current status
cannot replace its body or prepared request. New observations use a later
ordered source sequence. Retained message prefixes are reconstructed
independently before tail reading or new status allocation. Pending statuses do
not impersonate message records at the same logical count. Repeated status and
restart cannot enqueue duplicate messages. Rewritten message prefixes retain the
last explicit status while starting a new monotonic generation; unavailable
evidence withholds original intent. Lifecycle checkpoints supply neither source
eligibility nor directory, thread or publication authority.

Directory-aware discovery uses the pinned read-only project inventory and known
project-directory API. Global/non-Git projects deliberately omit persisted
project-directory entries, so their bounded directory candidates come from the
pinned session metadata list under `directory=/`, `scope=project`, `limit=4096`.
Metadata grants no activity or inactive-history import. Reaching that list limit
reports incomplete coverage; oversized metadata withholds the discovery step.
Project metadata and paths are memory-only candidates. Each rescan traverses at
most 4096 project IDs and 4096 distinct safe absolute non-root directories and
4096 offered native session IDs; each response retains the one MiB acquisition
bound. Invalid paths are withheld with safe unsupported coverage;
malformed/oversized inventories withhold their step. One owned discovery step
processes at most one directory or project-directory response. Candidate
directories require current native workspace/socket/catalog and Write proof
before directory-scoped status GETs, and that proof is rechecked before offering
session IDs. Duplicate directories and session IDs are offered once per cycle.
Unbound or denied directories cannot grant source admission. The default
instance status map is insufficient cross-workspace discovery. Previously
admitted session recovery remains independent of new directory enumeration
failures. No project/strategy metadata authorizes a source, executes a program,
mutates a harness workspace, or enters durable capture state.

Explicit retained supervisor preparation may select the read-only OpenCode API
alongside the qualified Codex file profiles. API discovery merges current
busy/retry candidates with the endpoint-scoped admitted-session recovery index;
all candidate acquisition still rechecks exact native witness and current
repository admission. Discovery, source reading and publication use separate
retained owned workers, with shared reader/source/raw-frame bounds and
alternating harness priority. Pause, resume, rescan, deadlines, drain and native
storage ownership apply to every API lane. API source observations join the same
generation/ordinal catalog in selector order. They expose body-free coverage
only; API discovery has no filesystem root counts. API sources are not eligible
filesystem import targets until their import semantics are qualified.

A settled API source MUST release message pages, pagination scanners/cursor sets
and sanitizers. Body-free session/witness/lease and exact publication proof
metadata may remain. Periodic following reconstructs durable prefixes without
duplicate enqueue. Publication holds independent existing lease references and
an immutable digest of the exact sanitized repository/profile/cursor/body/
metadata/Thread binding. It MUST verify that digest, API witness, spool binding
and current directory/catalog/Write before preparing or publishing. Prepared
requests keep their exact identities and bytes. Unavailable repository
partitions MUST yield to other partitions while preserving earliest intent
within each partition; a blocked publication await cannot block source ingress.

This retained library supervisor qualifies observed API lifecycle classes and
bounded directory-aware discovery; hook/API alias joins, complete streaming
export, API history/import/publication management and real harness/storage
release gates remain unqualified. Ordinary configured process startup continues
to withhold OpenCode until these required integration gates are qualified. The
driver preserves unpinned API snapshot semantics; no consistency claim spans all
OpenCode database pages.

The Codex file profiles distinguish CLI 0.156.1 (`source=cli` or `source=exec`)
and Desktop 0.159.2 (`source=vscode`, `originator=Codex Desktop`). The CLI
accepts the qualified `codex_cli_rs`, `codex_exec` and inherited `Codex Desktop`
originators; an inherited originator does not change the product surface. Other
originators remain unqualified. Codex's pinned rollout recorder writes the
session source independently of the environment-overridable originator. A
complete bounded first `session_meta` record MUST match the configured
surface/version, contain a native UUID session identity and an absolute working
directory before source admission. Header validation alone does not prove
repository binding or session activity. Product surfaces sharing one sessions
directory MUST NOT admit each other's files by filename or version similarity.
Selecting Desktop 0.156.1 is unsupported; existing CLI hook/version
qualification remains unchanged.

Codex location resolution uses explicit configured roots first, then
`CODEX_HOME/sessions`, then the server user's `~/.codex/sessions`. CLI and
Desktop share this convention but retain independent header/profile
qualification. Missing roots are unavailable and never created. Explicit roots
override ambient home settings; invalid relative/default home overrides fail
closed. Other harness conventions require their own qualification rather than
reusing Codex locations.

An approved Codex directory binds its logical selection to a canonical physical
directory. Metadata-only discovery resumes directory iterators across bounded
steps instead of restarting an incomplete scan at the first entries. Each step
examines at most 4096 entries and returns at most 256 regular JSONL candidates;
depth is at most eight nested directories and at most nine directory handles
remain open. No transcript body is read, no filename proves a product/session,
and no discovery candidate is serializable or debug-renderable. Nested symlinks
and special files are skipped; changed/unavailable entries are coverage gaps.
Root rebinding and candidate replacement are unavailable. Explicit cycle restart
revalidates the same approved root and can recover additions missed by directory
notifications. These candidates grant no session/repository/activity admission,
source lease, sanitization, checkpoint or publication authority.

A bound Codex reader consumes a discovery candidate rather than accepting an
arbitrary path. Preparation revalidates the approved root and physical file
before and after exact surface/header qualification. Inspection compares the
opened file's device/inode with the candidate before parsing the header; each
later file read retains that expected identity. Poll revalidates the root/file
before and after reading, including before returning an already pending exact
batch. Replaced files, symlinks and rebound roots are unavailable rather than
permission to read a new source through the old candidate. The device-only
wrapper and its raw batches have no persistence/debug encoding.

An exact sanitized durable acknowledgement may still repair progress after the
source becomes unavailable. Explicit rediscovery with the same caller-supplied
source identity and retained cursor can start the replacement file's new
generation without resetting source sequence numbers. In-place growth/rewrite
and acknowledged-prefix verification retain the bounded reader semantics below;
the existing unbound reader's replacement semantics are unchanged. Bound
preparation does not allocate stable source identities, lease readers, match a
repository, admit activity or authorize sanitization/publication. Source
registration and worker composition are separate capabilities. Callers must
separately revalidate current native directory/catalog bindings and service
authorization before publication.

Header-only qualification consumes an approved candidate and retains its exact
header witness without allocating a source or reading transcript batches. It
exposes only device-memory session/directory context for independent admission.
The worker MUST finish repository and activity admission before acquiring the
installation-private source registry; unsupported, unbound and inactive
historical discovery MUST NOT allocate registry entries or reader locks.
Qualification and registration alone do not implement those worker checks.

The version-1 source registry binds the existing installation identity and
allocates opaque nonzero source IDs from a persisted monotonic ordinal. It
retains only source IDs, ordinals and domain-separated canonical-location,
physical-file and exact-header digests, never raw paths, headers or bodies.
Known canonical locations retain their channel ID across replacement/rotation;
exact physical/header witnesses join hard-linked locations and overlapping
approved roots. Product profiles do not create separate channels for the same
physical source. Logical native-session joins remain the independent alias
publication contract below.

Conflicting location/witness/physical mappings withhold registration without
changing the catalog. A known physical file with a new header at an unknown
location waits for rediscovery of a known location; bare device/inode reuse
never grants channel adoption. Retained physical aliases may conservatively
withhold a recycled inode, rather than silently merge source channels. Header
changes require renewed qualification and registration before a leased reader
returns new-generation batches. Prior source progress is supplied separately;
registration does not reset sequences, rewrite journals or evict history.

The owner-only registry lives outside checkouts and publishes its bounded JSON
catalog by synchronized temporary-file replacement and directory sync under a
short exclusive registry gate. Capacity is one through 4096 retained sources, at
most 4096 location aliases, 4096 header witnesses and 4096 physical aliases,
with a 2 MiB catalog limit and 8194 durable coordination/catalog files. One
additional recognized temporary file is permitted for crash recovery; exhausted
bounds stop new acquisition without evicting acknowledged identity evidence.
Unknown files, unsafe permissions, foreign owners, symlinks, state rebinding,
invalid installation/identity records and ambiguous aliases fail closed. An
initialized registry with a missing catalog is unavailable and MUST NOT restart
allocation at ordinal one. Complete private-state loss requires separately
qualified recovery; registration does not claim backup/restore authority.

Each registered reader retains both a source-channel lease and a physical-file
lease. Nonblocking OS locks exclude simultaneous readers across overlapping
roots, profiles and processes sharing this installation registry, while
independent sources can be read concurrently. Distinct installation registries
remain independent. Leases explicitly unlock on drop and are released by the OS
after process death. Reader preparation and each poll revalidate the private
directory and exact lease file identities before returning source batches;
rebound locks or source/root replacement withhold results. Durable exact
acknowledgements can still repair already-sanitized progress when unavailable.
Source registration does not authorize a repository, assert activity, publish,
checkpoint or enable automatic server startup. Configuration and whole-server
resource ownership remain independent prerequisites.

The read-only Codex source reads at most 1 MiB of new bytes and 256 complete
records per poll, keeps at most 1 MiB for a partial record and represents an
oversized complete record as an omission for subsequent sanitization. Invalid
complete UTF-8/JSON also proceeds to safe omission rather than blocking the
tail. Partial records never acknowledge offsets. Header inspection is bounded by
1 MiB. Raw records and directories remain in memory in types without persistence
serialization or debug rendering; only the sanitization stage can construct
durable observations. A batch and its acknowledgement position remain stable
until the sanitized spool acknowledges that exact identity.

Source generations bind the physical file identity and complete first record.
Replacement, truncation or contradictory acknowledged prefix bytes start a new
generation, retaining monotonically increasing source sequence numbers and prior
cursor journals. Resume and changed files verify the acknowledged SHA-256 prefix
in at most 4 MiB steps before reading new records. A metadata change between
verification steps restarts verification, and a change during a read withholds
the candidate. This is bounded work per poll, not constant total verification
cost; long prefixes can take multiple scheduling steps. Same-handle partial
bytes are discarded on file changes to prevent splicing old and new records. The
source reader has no sanitization, activity-admission, publication or checkpoint
authority. Native transcript projection and leased reader preparation are
independent capabilities; active admission/backfill and supervised worker
composition remain separate M18.2 gates.

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

### Provenance, remaining capabilities and traceability

Opted-in client scopes separate preexisting dirty baselines, checkpoint in
failure and finally paths and retain exact JJ operations/commits. Immutable
intents precede operations; later evidence never changes published hashes.
File-only recovery reports interval/reported evidence rather than exclusive
authorship. Owned producers share canonical registration/binding/append/scope
contracts and explicit optional Thread binding. Full runner dispatch is
separately qualified.

| Original v4 behavior or review finding                                                                           | Requirements                                                               | Gate                                      |
| ---------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------- |
| Sections 5/12: discovery, active backfill, reader recovery and hook/file enrichment                              | JST-COLLAB-017,JST-COLLAB-018,JST-COLLAB-019,JST-COLLAB-020,JST-COLLAB-030 | M18.2                                     |
| Sections 10.6/18: omitted/unsupported records block drain                                                        | JST-COLLAB-021                                                             | M18.1                                     |
| Section 7.3: cross-batch tool field loss/terminal regression                                                     | JST-COLLAB-022                                                             | M18.1                                     |
| Section 10.2: full-prefix transcript append                                                                      | JST-COLLAB-023                                                             | M18.1                                     |
| Sections 7/25: blocks/plans/children/compaction, exact message/plan reviews and renormalization/replay           | JST-COLLAB-024                                                             | M18.3                                     |
| Sections 8/9/13: automatic provenance and owned producer scopes                                                  | JST-COLLAB-025                                                             | M18.4                                     |
| Section 12.7: real harness edit/UI/restart qualification                                                         | JST-COLLAB-026                                                             | M18.5                                     |
| Sections 15/16/25: exact inspection, search, immutable work requests, feedback addressing and safe comment input | JST-COLLAB-027,JST-COLLAB-030                                              | M18.6                                     |
| Sections 17/25: collaboration archive and portable carriers                                                      | JST-COLLAB-028                                                             | M18.7 and existing M13 portable gates     |
| Sections 11/19/25: home/temp path privacy, target permissions, current revocation and purge                      | JST-COLLAB-029                                                             | Local privacy M18.2; broader policy M18.7 |

M18.5 remains the complete multi-harness capture release boundary, now Deferred.
M18.8 owns the narrower usable Codex preview and does not depend on M18.3–M18.7.
The aggregate cannot be Done while deferred requirements remain open. Existing
native copy, M13 and M15 evidence does not qualify broader gates. Official ACP
Rust SDK adoption is not a required outcome; common producer contracts and
execution qualification are.

## Thread presentation template

Forum directory summaries derive their preview from the latest visible human
message. A later rename or lifecycle event advances the activity timestamp
without clearing that message preview. Tombstoned messages contribute neither
preview nor visible message counts.

A persistent Thread is presented as a Community channel. M15.5 MUST use
Community channel navigation and room-selection semantics, the channel header,
conversation timeline, shared composer, message actions, reply affordances,
reactions, history loading and responsive behavior as its template. The locally
owned conversation, composer, message and reaction components are the first
composition owners; inspect the current Community channel application and
contract stories before copying the remaining presentation parts. jj-store owns
the authorized adapter and explicit captured-root refresh behavior. It MUST
preserve the Community visual and interaction template while owning all copied
presentation locally. Community package/checkout dependencies are prohibited by
JST-GOV-008. Native adapters replace Community identity and policy assumptions;
Nostr policy never becomes repository authority. Native Thread
identity/lifecycle and receipt semantics remain the persistence contract;
matching channel presentation does not imply running an agent or importing Nostr
membership.

## Persistent Threads and Reactions (M15)

`jj-collab` owns the Thread, human-message, reaction and custom-emoji domain.
ThreadStore and ReactionStore have separate root catalogs and state versions.
AgentStore remains the owner of sessions/transcripts/tools, ReviewStore of
reviews, and JJ of code topology. No existing session is migrated or
automatically linked. A session has zero or one primary Thread; additional
Threads may reference it. Reassignment records the old relationship and retains
historical attribution. Link session is an explicit Thread action. No lifecycle
action controls a runner.

Thread lifecycle is Created to Active, Active/Idle to each other, Active/Idle to
Archived, Archived restore to Idle, and Active/Idle/Archived to Tombstoned. An
authorized tombstone restore returns to Idle. Late linked capture remains
retained without reopening archived or tombstoned Threads. Optional baselines
pin exact native operations/views/commits or portable evidence capsules.
Multiple heads remain explicit. Code associations retain evidence methods;
manual links cannot claim stronger provenance. Review parenting is a ThreadStore
relationship, so standalone reviews remain valid and jj-review has no reverse
dependency. Human messages outside sessions have immutable versioned first-party
records.

Reaction targets distinguish logical Thread/review identities from exact human
message, agent message, tool invocation/output, review comment/reply, WorkStore
comment and checkpoint versions. WorkStore comment targets are scoped to the
metadata home and name the task, stable comment identity and exact nonzero
comment version. The resolver admits only a visible comment at that version in
the captured WorkStore root, with matching task membership. Edits do not
transfer reactions to the new version; tombstones, missing membership and stale
versions are unavailable. Reaction commands keep the existing actor admission,
observed records, durable receipts and independent Reaction root. They do not
change task versions or grant access to linked code. Native and local providers
capture Work and Reaction selections together; existing target variants retain
their canonical bytes. Agent message targets include validated event/batch and
content witnesses; outputs use the existing content object identity, never an
invented ToolResultId. A central resolver validates membership and visibility
against captured roots. Aggregation does not silently transfer feedback across
edited revisions. Earlier revision reactions are separately disclosed only when
authorized. Hidden targets or actors produce no count, search or identity
disclosure.

The shared target resolver consumes one authorized, repository-bound selection
of Thread, Agent, Review and Work roots. Its reader never exposes staged
objects. Its provider verifies existing repository-bound Agent/Review
identities; the domain resolver verifies collaboration content IDs and typed
entity membership without replacing the owning stores' identity codecs. Human
and review revisions must be reachable from the selected logical
message/comment; checkpoint versions must be in the selected checkpoint's
supersession chain. Tool snapshots require the selected session/tool and
retained snapshot membership; output content must match that exact snapshot.
Message event witnesses must belong to the selected session's event batches and
match the exact message and content IDs. Resolution returns only an exact key
and a current/earlier distinction, never a target body or reaction count.

Current tombstones and hidden or conflicted review comment selections suppress
all earlier-revision reaction disclosure. Absent and inaccessible targets have
the same unavailable result. Revision traversal is bounded to 4096 objects;
malformed identities or exhausted bounds fail closed. Authorization of reacting
actors remains a provider responsibility before aggregation or identity
disclosure. These checks do not authorize publication or establish runner
control.

Values are Like, Dislike, Emoji(sequence), and Custom(emoji_id, exact_version).
Imported + and - normalize to semantic values while retaining original source
representation. Literal emoji remain distinct. Custom definitions are repository
library records and pin retained GIF/PNG/JPEG/WebP bytes and a static preview.
Limits are 2 MiB encoded, 512 by 512 pixels, 100 frames and 64 MiB aggregate
decoded pixels. SVG/executable content and external URL dependencies are
rejected.

Custom library commands use an exact observed definition version. A new reaction
selects the current non-retired definition; editing or retiring a definition
does not rewrite historical reactions or discard their source/preview bytes.
Shortcodes are unique among current non-retired definitions. Thread and reaction
projectors derive immutable event identities from the authenticated actor and
stable request ID, and retain exact command results in the provider receipt.

The slot key is repository, authenticated actor and exact target. Set, Clear and
ResolveConflict name exactly observed active records. Replacement atomically
retracts observed records and adds the new immutable value. Duplicate same-value
additions count once; different concurrent values contribute no selected
ordinary value until explicitly resolved. Clock time and WAL delivery order
never choose a winner. Retractions only remove named observed additions. Stable
request identities and durable receipts distinguish accepted, rejected and
unknown outcomes. Capture never generates reactions as the user.

Native format 5 adds optional Thread/Reaction roots and independent versions on
first explicit feature use. Formats 1 through 4 retain their original encoding
and interpretation. Native inventories, WAL, checkpoints, receipts, recovery and
manifest-last copy retain complete histories and assets. The ordinary Git
companion uses the existing owned refs/lapis/state/v1 with a versioned envelope
and real Git reachability. Typed multi-domain transitions select all changed
roots atomically. Projectors stage and validate; providers retain authorization
and publication CAS.

Native typed commands capture Thread, Reaction, Agent, Review and code
selections from one authorized manifest. Durable receipt replay precedes target
and entity preconditions. A manifest CAS race invalidates the captured source
selection and reapplies the original typed intent only after all sources are
reloaded and validated; a changed source cannot publish feedback against a now
hidden target. Native code baselines and links require accepted operations and
commits in the selected inventory, never merely staged bytes. Manual evidence
must belong to the selected AgentStore; stronger attribution and the portable
native evidence carrier stay separately gated. Native and local providers share
identical typed command/result domain tags without changing local receipt
encodings.

Native service commands derive the actor and publication identity from the
currently authenticated caller. Status decodes canonical typed receipts and
never discloses another actor's request. Credential-free read pins capture all
source roots together, enforce object/byte/time budgets, and reauthorize the
same repository, viewer and epoch before every response. Repository actor policy
filters slots before counts, conflicts, viewer state or actor pages are
produced. Conflicted actors contribute no ordinary selected value. Earlier
revisions remain separate exact queries. A Thread history page follows only the
selected Thread's event chain, validates continuation membership, and carries
canonical message/link references without copying captured transcript bodies.
Absent Thread/Reaction domains remain absent during reads. Native continuation
tokens are process-key signed and bind the viewer, repository, complete captured
snapshot, query kind, exact target or Thread, page size and traversal position.
Their bounded registry stores credential-free pins with object and encoded
collaboration-byte budgets, fixed expiry and fail-fast concurrency. Resumed
pages renew only the request deadline and reauthorize every query and response;
expired or restarted pins return cursor-reset, never a latest-state fallback.
Public pages expose opaque tokens rather than raw traversal positions. Native
service command acceptance currently covers WorkStore homes with the frozen
origin ledger and explicit code-repository rejection; replicated code writes
require the separately qualified reducer 3.

The public namespace is /v1/repos/{repo}/threads with list/detail/timeline and
commands plus command status, /reactions/query and commands/status,
/reaction-emojis and /reaction-assets. CLI lives under jj agent thread and jj
agent reaction set/clear with typed target JSON. Bounded timeline pages pin all
relevant roots, contain source references and never copy transcript bodies. WAL
order is local publication order, not JJ ancestry or global causality. Runner
integration begins with an explicit Thread binding contract; qualified jj-acp
launch/continuation is a dependent gate. Search is rebuildable and indexes
canonical source documents with authorized Thread joins and source explanations.
Counts remain ReactionStore-derived. SSE announces freshness; refresh changes
captured displayed roots. UI release/baseline gates remain separate.

The local Git CLI uses the same typed provider without probing or initializing
agent integrations. Separate executable processes preserve receipt replay, exact
human-message revision feedback, observed-set preconditions, retained custom
definition versions, bounded input/output and explicit metadata-commit
continuations. These commands leave code operation heads and unsnapshotted files
unchanged. Native executable CLI acceptance and jj-acp process execution remain
independent gates; local command acceptance does not establish either. Native
CLI commands share input admission with the local profile, use signed --cursor
pages, retain original identities on uncertain transport responses, and never
initialize an agent registry or invoke a runner.

Reducer 3 transports Thread/Reaction facts and complete retained closures.
Equivalent accepted origin sets converge. Replay/import only reconstructs data.
Native WorkStore homes retain their origin freeze. Normal Git push/fetch remains
code-only; complete collaboration exports and verified Nostr mappings are later
adapters. Notifications are optional committed-event consumers and are never a
publication prerequisite. No backfill, controller dispatch or adapter projection
is implicit in Thread/Reaction publication.

## Initial delivery boundaries

Native RPC is a thin typed service boundary for commands/status, all four signed
page kinds, Thread detail, exact human-message revisions and retained source or
preview images. Binary image fields preserve the domain's encoded-byte limit.
Inline/binary ambiguity, unknown request fields, kind or content-ID substitution
fail closed. It invokes no runner and retains the native code reducer gate.

Review summary presentation is derived from the same selected ReviewStore root
as the page, including resumed pages. It records the immutable creation actor
and time, last accepted event time and a bounded excerpt of the selected initial
comment. Hidden, tombstoned or conflicted comment bodies produce no excerpt.
This adds a rebuildable read projection and changes no native persistence
format.

The active sequence is publication foundation, AgentStore model and capture,
ReviewStore exact anchors, LocalTrusted API/CLI integrations, shared UI, then
cold recovery and real-harness qualification. Codex CLI 0.156.1, Cursor 3.22.7
and OpenCode 1.18.30 are the initial compatibility surfaces. Deterministic
sanitized contract fixtures exist for all three. The executed local capability
matrix accepts exact installed Codex and OpenCode versions and records the
installed Cursor drift as unsupported without invoking a paid or network model.

Authenticated/shared serving, remote replication, backup/restore, Git or Nostr
export, controller job dispatch, arbitrary cross-repository targets, CRDT/edit
identity, advanced semantic re-anchoring, other harnesses and visual baselines
are deferred. Tantivy may index committed collaboration records later but never
becomes their authority.

## AgentStore object foundation

`jj-agent` owns distinct fixed-width stable IDs for sessions, attempts, turns,
messages, blocks, tool calls, terminals, checkpoints, associations,
observations, events, devices and capture sources. Content-addressed
`AgentObjectId` values remain separate from those durable entity IDs. Native JJ
operation, commit and change IDs retain their existing owned types and are not
reinterpreted as agent identities.

`AgentRootV1` selects independent persistent radix catalogs for sessions,
aliases, checkpoints, associations, relations, sources and projections. Empty
catalogs are represented by absent page roots in one canonical empty root.
Catalog pages are bounded immutable objects with canonical digest keys and typed
strong references. Every staged root and page validates its complete
repository-bound dependency closure before it can be selected; missing objects,
semantic kind mismatches, unknown fields and noncanonical bytes fail closed.
Only the publication coordinator can turn staged objects into visible state.

Sanitized observation batches and normalized event batches are distinct
immutable objects. Observation batches record integration, source instance,
source order, source kind/trust, representation and sanitizer version; their
payload is validated canonical JSON or UTF-8 text rather than an arbitrary
untyped JSON value. Event batches reference their exact evidence batch and use
stable event/entity IDs. Normalization therefore cannot replace or rename the
accepted source evidence.

Session snapshots select exact evidence, event, transcript and tool object
versions. Tool snapshots keep execution state separate from capture evidence and
reference typed sanitized input/output content. Transcript manifests keep each
source generation separate and root an ordered immutable chunk-page closure
sufficient for direct reconstruction; they have no indispensable
previous-manifest link. Checkpoint discovery remains in its root-level catalog,
so a session snapshot has no mutable or circular checkpoint reference.

Checkpoint versions pin exact session, transcript, event, tool and association
objects plus complete before/after native operation and commit anchor sets. They
distinguish changed, verified no-change and not-observed code relations; a
failed tool can still have changed code. Completeness, capture status and
execution outcome remain independent. Superseded checkpoints are immutable
strong edges and cannot point back from a session snapshot.

Code associations are many-to-many assertions with stable association, session,
optional checkpoint/turn/tool/scope identities, full native IDs, optional JJ
change IDs, repository-relative paths, evidence method, trust, coverage and
exclusivity. Correction versions must name a prior immutable association and a
reason. File claims reject absolute or traversing paths, and manual assertions
cannot masquerade as stronger evidence. Stable IDs and JJ change IDs are lookup
facts; exact operation/commit IDs and immutable evidence objects are retention
edges.

Portable `CodeAssociationV2` and `AgentCheckpointVersionV2` use explicit source
repository, profile and capsule references for operations and commits. A commit
must match its evidence profile. Their shared non-code facts preserve the native
path, evidence, correction, coverage and dependency rules. Canonical bounded
decoders reject unknown versions, fields and noncanonical bytes. Native V1 field
order and serialization remain frozen; these additive models do not by
themselves enable a persistence provider or CLI command.

Portable object variants retain the existing repository-bound AgentObject record
framing and semantic catalog kinds. Shared provenance projection validates the
selected session and pinned checkpoint dependencies, exact correction selection
and stable identities, then adds typed session/tool/checkpoint/code relation
pages. Git relation subjects carry source-profile evidence rather than native
commit IDs. Native storage rejects portable evidence until its own capsule
carrier is implemented; it cannot silently discard those retention edges.

An AgentStore read is pinned to one selected `AgentRootV1`. Stable session,
checkpoint and association keys resolve through bounded persistent radix pages;
immutable object reads succeed only for references in that root's audited
transitive closure. A staged but unselected object is not readable through the
pinned facade. Catalog leaves contain at most 32 selections, branch updates copy
only the affected path, and prior roots remain readable.

Typed agent deltas carry expected and replacement selections for all seven
catalogs. They reject duplicate edits, semantic kind substitution and no-op
edits. A disjoint edit can reapply to a newer agent root after a CAS race, while
a changed expected selection is stale. The stable request intent covers the
original delta rather than generated page/root IDs, so receipt replay remains
valid after later agent publications.

Alias objects bind an integration, source instance, native entity kind and
native ID to a matching stable session, turn, message, tool, checkpoint or
association identity. These lookup facts are not strong object references.
Relation subjects are typed session, tool, operation, revision or checkpoint
identities. Their catalog selections are immutable linked pages containing only
checkpoint and association object references; each page repeats its subject so a
cursor from another relation fails closed.

Source-state objects retain the integration/source identity, generation,
versioned parser, optional native session key, source-specific accepted
position, strictly ordered non-overlapping capture-batch receipts and coverage.
Projection catalogs retain one normalizer version and activation time plus exact
selected session, normalized-event and source-state object versions. Replaying a
parser therefore publishes new selected objects rather than mutating prior
facts.

Catalog enumeration is radix ordered and root pinned, returns at most 256
selections and resumes after a stable catalog key. Relation traversal uses a
root-local page/offset cursor and rejects mismatched subjects. Transcript reads
select one source generation from an immutable manifest, verify strictly
increasing chunk sequences across linked pages and enforce both a record count
and an eight MiB caller-selected byte ceiling. Session/checkpoint enumeration,
relation traversal and transcript streaming do not depend on Tantivy.

## LocalTrusted collaboration reads

The M9.4 service surfaces list AgentStore sessions from one exact selected root,
resolve one stable session against the current selected root, page one selected
transcript stream and traverse typed relations for session, tool, operation,
revision or checkpoint subjects. They also page the stable checkpoint catalog,
resolve its current selected version and read a requested immutable version
through that selection's supersession chain. Their owned responses identify the
repository, epoch, publication position, manifest revision, AgentStore root
digest and AgentStore state version. Each session row retains the complete
stable session ID and immutable session-snapshot object ID, then exposes only
bounded integration, lifecycle, parent, coverage and referenced object counts.
Session summaries derive optional first and last activity times only by matching
selected normalized event IDs to timestamps in their exact sanitized observation
batches; missing or over-bound evidence leaves timing absent rather than
inferred. For qualified Codex CLI/Desktop JSONL records whose observation
timestamp field is absent, timing may use the preserved RFC 3339 timestamp in
the matched sanitized native record. The exact profile/envelope and local
transcript source/trust remain required; explicit observation timestamps take
precedence. Invalid, missing and unsupported native timestamps remain unknown.
This applies to both native and companion-pinned list/detail reads, including
already published records, without changing immutable evidence, receipts,
normalization or capture recovery identities. Duration remains the captured
activity span, not elapsed time since ingestion or a guessed process lifetime.
Session detail also returns the exact immutable observation-batch, event-batch
and tool-snapshot IDs selected by that session; tool links retain their full
stable tool-call IDs. Selected transcript manifests expand only into stream
descriptors carrying the exact manifest/source identity, generation,
format/decoder/sanitizer versions, representation, byte and record counts and
gaps. The same detail resolves a presentation-ready, sequence-ordered activity
stream from the selected event batches. It preserves session-state, message,
tool, permission, observed-file and capture-gap variants; message and tool
payloads remain sanitized native content. Final tool state resolves the selected
tool snapshot, optional input/output, observed paths and exact code
associations. One response admits at most 4,096 source events, eight MiB of
resolved content and 64 code associations, and reports activity or association
truncation explicitly. Relation pages repeat the exact typed subject and return
ordered immutable checkpoint-version or association links; association links
also expose their verified stable association/session/tool identities, relation,
native before/after operation and commit anchors, change IDs, paths and creation
time. An unlinked subject returns an empty page. Transport DTOs do not expose
WAL objects or reinterpret stable entity identities as immutable object
identities.

Checkpoint summaries keep their complete stable checkpoint and immutable version
IDs, session, optional turn/scope, native before/after anchors, code relation,
evidence/trust state, coverage, capture status, execution outcome and bounded
reference counts. Detail adds exact selected session, transcript, event, tool
and association object links. The immutable version route walks at most 4,096
unique checkpoint versions from the current catalog selection and rejects a
cycle, wrong stable checkpoint or unrelated object. It does not expose arbitrary
reads from the selected AgentStore closure.

ReviewStore thread collection likewise enumerates the stable thread catalog from
one exact selected root. Each summary retains the stable thread and current
immutable snapshot IDs, exact anchor and event-page object IDs, workflow state,
three monotonic versions and bounded comment/event counts. Collection reads
resolve and return the validated typed anchor so a browser can select an exact
revision or path without issuing an unbounded detail request; the anchor's
repository must equal the pinned repository. Stable-thread detail resolves that
current catalog selection, its validated anchor, ordered selected comment
revisions and safe bodies, plus the complete bounded immutable event stream. The
combined visible comment bodies in one detail response are limited to eight MiB.
Comment and event DTOs retain exact object IDs and authenticated actors;
tombstone and moderation states remain explicit. The service never serializes a
raw `ReviewObjectV1` envelope or permits arbitrary closure-object reads. No
historical thread-version route exists because v1 thread snapshots do not carry
a validated predecessor chain.

Initial and resumed pages require current repository read permission. Page sizes
are limited to 1 through 256. An opaque signed continuation binds repository,
epoch, publication position, manifest revision, AgentStore root, AgentStore
state version, page size and the radix key after the last row. Transcript
continuations bind the same root pin plus stable session, immutable manifest,
source instance, source generation, byte ceiling and last returned sequence. The
relation continuation instead binds the complete typed subject plus the
root-local relation page and offset. The process-local cursor registry retains
the exact audited immutable AgentStore snapshot across later publications,
bounds retained snapshots and closure objects, reauthorizes every page and
resets after expiry, server restart or epoch replacement. Tampering, repository
reuse and query changes fail explicitly. Reads neither dispatch an agent nor
stage or publish repository state.

Review collection continuations use the separate signed `jsr1` domain and bind
repository, epoch, publication position, manifest revision, ReviewStore root,
ReviewStore state version, page size and the radix key after the last row. They
retain the exact audited ReviewStore closure within the same snapshot, object,
concurrency, lifetime and authorization bounds.

ReviewStore commands accept a versioned request ID plus one validated typed
create, reply, edit, tombstone, moderation, resolve or reopen command. The
service derives the authenticated principal and a repository-bound publication
ID; neither is caller-authored. Successful responses decode the
manifest-selected WAL receipt into exact accepted snapshot and ReviewStore
object identities. Identical retries and status after service reconstruction
return that durable receipt. An unavailable or timed-out apply remains
`outcome_unknown`; a missing status remains `not_known`, which never proves
rejection. Stale workflow and thread-version preconditions fail without
publication.

The native RPC exposes the same bounded ReviewStore page, detail, typed command
and durable status models used by HTTP. `jj review list/show` preserve full
stable identities and cursors, while create, reply, edit, tombstone, hide,
resolve and reopen require an explicit request ID and exact applicable workflow,
revision or thread-snapshot precondition. Thread creation accepts one complete
typed anchor document for the bound repository rather than inferring an anchor
from display text. Text and lossless JSON presentation share this contract.

A committed ReviewStore command sends only the existing bounded process-local
repository wake hint. An already idle subscription replays the selected WAL
record and reports `repository-changed` at the receipt's accepted position; the
hint contains neither collaboration content nor credentials. Polling and WAL
replay remain authoritative across lost, coalesced or lagged hints. This closes
the LocalTrusted CLI and live-hint portion of JST-COLLAB-004 without adding a
public listener or controller dispatch.

## Application presentation

The repository Reviews page opens the Community review detail composition with
Description, Files changed, Commits and Activity. Its detail replaces the list
in the main page from either repository or Projects review navigation,
independently of the thread discussion layout preference. The fixed header and
comment composer surround one scrolling body; comments remain under Activity,
and page comments select no parent until an explicit reply action.
Draft/edit/reply state and existing native commands are retained. Its shared
native activity timeline is also used by Projects and Tasks: native commit rows
contain inline Agent context disclosures using recorded associations, sanitized
messages and tool-call primitives. Code and agent roots remain independent. An
anchored agent snapshot that differs from the available selected session is
explicitly unavailable; the UI does not represent the latest session as the
original artifact. Review comments, resolve/reopen, durable unknown requests and
inline diff/tool threads retain their existing service contracts.

The shared Activity timeline and sanitized transcript use Community's measured
virtual rows in shadcn ScrollArea viewports. Agent association metadata is read
for mounted commit rows only, under the captured AgentStore root and a bounded
reader budget. Session detail reads and transcript rendering are deferred until
the disclosure opens. A changed root is reported as unavailable until explicit
refresh, rather than substituting newer context. Samples without recorded code
associations cannot imply that a session produced a commit.

The LocalTrusted application exposes Agent activity and Reviews as first-class
repository pages. Their stable hash routes retain the selected operation plus
either the complete stable session or thread identity. Reload and browser
Back/Forward restore that exact selection; collaboration identities never occupy
native operation, commit or change fields.

Agent session, transcript and review-thread collections use cursor-paged
TanStack Query reads scoped to the active access session and repository.
Measured virtual rows bound rendered session, transcript, thread and comment
collections. Agent session rows pair the integration's agent avatar and name
with state, captured start time and captured duration when exact evidence timing
is available. The selected session displays its independently pinned AgentStore
root alongside the selected native code snapshot. Review detail displays its
independently pinned ReviewStore root and exact anchor, retaining the durable
request ID whenever a command outcome is unknown. Reply, resolve and reopen use
the same optimistic-concurrency inputs and status route as the LocalTrusted
service contract.

The presentation consumes Community's public `CommunityMessage` and
`CommunityChatComposer` components plus Design Core chat tool-call and tool
detail primitives, then adapts native agent and review records into their public
presentation models. Agent activity renders as a normal Community conversation;
tool rows retain their native kind and state, expose copyable stable identities
and use a kind-specific detail without escaping the transcript's inline inset.
Edit, delete and move details embed the exact changed-file diff when the loaded
revision patch or sanitized old/new tool input can supply it. The agent header
exposes the associated Git/JJ identities and a route link back to the related
change. Review comments use Community's separator-free compact timeline and
bounded reply-tree indentation, guides and connectors, retain copyable
author/comment identities and keep the composer at the bottom of an open
discussion.

Each rendered text-diff row exposes a keyboard and pointer reachable comment
action through Design Core's line-accessory boundary. Activating it opens an
inline composer after that row. A committed first comment creates one
`code_text` ReviewStore thread whose immutable file object, UTF-8 byte
selection, digest and explicit before/after comparison operations are resolved
from the selected native comparison; an ambiguous, mismatched, conflicted or
unselectable row fails closed. The resulting discussion renders through the
line-annotation boundary immediately after the anchored row. Existing code-text
threads use the same exact row placement, while whole-file anchors retain their
below-diff placement.

Native diff hunk starts remain byte offsets in the transport. Before exposing a
line comment action, presentation MUST derive one-based unified-diff positions
by accumulating the ordered hunk contents on each side; a zero-length side uses
the preceding position required by unified diff syntax. A native byte offset
MUST NOT be presented or resolved as a source line number.

Each agent tool-call row likewise exposes a keyboard and pointer reachable reply
action. Its first committed comment creates one `agent_artifact` thread bound to
the selected session snapshot and exact tool snapshot; subsequent comments
append through the thread's normal workflow precondition. A captured V1 tool
call without finer attempt evidence uses its stable tool-call identity as the
single coalesced attempt identity. Tool discussions render directly after the
call with the same bounded reply indentation as Community review threads, and
the Agents route retains the selected review-thread identity.

Design Core supplies the column canvas, diff, Empty, Alert, Button and Skeleton
primitives. Revision and operation selection follow bounded association and
review pages to the exact commit or operation match, open sibling Agent context
and Review discussion columns when present and record both stable selections in
the route. Operation review matching includes direct operation anchors,
revision/code anchor operation IDs and both sides of an explicit code-text diff
context. Selecting a changed path follows review pages to the exact code-text or
code-artifact anchor for the selected revision or operation and keeps that
discussion in the review and inline diff surfaces. Repeated cursors or more than
4,096 inspected context rows fail closed. Agent and review list rows use the
same separator, metadata and tag hierarchy as Branches and Op Log, and
SHA-shaped identities are presented through the existing copy controls instead
of bare shortened text. The Files header resolves the selected commit
independently of graph history and displays its author avatar, message, Git/JJ
identities and relative time. Collaboration columns have explicit close actions
that clear their route selections. All canvases are keyboard and pointer
resizable and persist their versioned layout through the application's public
storage boundary. Catalog stories inject memory storage and cover ready
interaction, exact revision/path context, pagination-compatible virtualization,
loading, empty, failed and durable command states with automatic accessibility
checks. Stories remain `visual-pending`; interaction and accessibility evidence
does not approve a visual baseline.

Repository SSE events announce that newer collaboration history exists without
mutating displayed roots or records. Explicit Refresh invalidates the complete
repository-session query scope, captures the current operation heads and then
restores the selected collaboration route against the new snapshot. Disconnect,
credential replacement and repository switching cancel requests and clear the
protected query cache; late detail responses are ignored before assigning
session or thread state.

## Capture sanitization boundary

The capture boundary uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431` under Apache-2.0. Scanner default
features are disabled, so validation and network calls are not compiled. Base64
decoding is also disabled. The fixed policy identity is
`mongodb-kingfisher@2.7.0/builtin-medium/v1`.

Each hook submits one bounded JSON object. The boundary canonicalizes the object
with recursively sorted keys independently of workspace dependency feature
unification, normalizes the canonical workspace path to `$WORKSPACE`, scans
those in-memory bytes with the built-in medium-confidence rules and replaces
merged finding byte ranges with `[REDACTED]`. It reparses the result before
constructing the bounded sanitized payload. Invalid JSON, oversized input or
output, scanner/cache errors and invalid finding ranges produce a fixed omitted
state without returning raw values or secret-bearing diagnostics. The raw hook
payload is never copied to a temporary file or durable state. A user-restricted
compiled-rule cache is warmed during real integration setup and reused by hook
processes.

The packaged hook applies this boundary before capture-state creation. It writes
only a validated sanitized payload or fixed `{"status":"omitted"}` object into
the durable spool. Publication recovery remains required before M9.2 capture
support can be claimed.

## Version-bound normalization

Normalization validates the complete sanitized observation batch before
dispatch, then accepts only the exact integration/version pairs Codex 0.156.1,
Cursor 3.22.7 and OpenCode 1.18.30. An unknown profile returns one safe
unsupported-version diagnostic without native record dispatch. Cursor records
also carry and verify their embedded product version; OpenCode session events
verify the version when the native callback supplies it. Diagnostics contain a
stable code and optional source sequence, never rejected source content.

Stable session, turn, message and tool IDs are SHA-256 domain-separated over the
integration, capture-source identity, entity kind and scoped native ID. Native
aliases remain explicit lookup facts. Replaying the same sanitized batch
therefore returns byte-equivalent normalized records and aliases. Native tool
names remain intact while the common kind is conservative; unknown names map to
`other`, and a file observation without a native tool-call ID cannot create one.

Lifecycle records remain source ordered. Capture completeness, permission state
and execution state are separate fields. The reducer enriches one stable tool
from input and output observations, does not regress a terminal state when a
late running event arrives, and reports conflicting terminal observations
without selecting one by wall-clock time.

The Codex fixture follows the locally installed 0.156.1 hook fields. The
OpenCode fixture follows the exact `@opencode-ai/plugin` and SDK 1.18.30
callbacks; the published plugin archive observed during qualification had npm
integrity
`sha512-bXWcA2VD478PXXke1qAYANTq58fCcbgY0GAUqZIFZqPpo48FoG8RH37nNZ78eHtz+ddiKIhFTBZJRaHF3IgFqw==`.
Both remain sanitized synthetic contract fixtures rather than captured model
runs. M9.6 additionally probes the actual local executables through the bounded
device registry and previews their owned project integration in disposable
workspace/state directories. Codex 0.156.1 and OpenCode 1.18.30 match their
qualified contracts. Cursor 3.22.7 remains synthetic contract acceptance only:
the installed agent reports `2026.09.26-dd393fe`, is recorded as
`unsupported_version`, and cannot be enabled. Qualification never invokes a
model, sends captured content or modifies the active workspace hooks.

## Exact review anchors

`jj-review` owns fixed-width stable identifiers for review threads, comments,
actions and work requests, plus separate SHA-256 identifiers for anchors,
comment revisions, thread events, snapshots, work-request versions and roots.
Stable workflow identities never substitute for immutable version identities.
All identifiers use canonical lowercase hexadecimal encoding.

`ArtifactAnchorV1` binds one repository and exactly one code text, whole code
artifact, agent text, whole agent artifact, whole agent session, revision,
operation or repository target. Code anchors carry full operation and commit
IDs, optional change IDs, repository-relative paths and the exact native
file/object identity. Diff text anchors additionally carry an explicit
before/after operation, commit and path pair; the selected side must equal the
anchor's primary identity. Whole native artifacts accept matching file or
symlink object kinds. External-width gitlink identities have no owned native
object key and therefore fail closed in v1.

Text selections use canonical decimal-string UTF-8 byte offsets and a SHA-256
digest of the selected bytes. Empty, reversed, out-of-bounds, non-UTF-8-boundary
or digest-mismatched selections are invalid. No line number, display text or
search result can replace this byte-level check.

The additive `agent_session` target binds the stable session identity and one
exact immutable session-snapshot reference. It retains that snapshot and its
complete AgentStore closure through existing review dependency traversal. Native
staging and cold reads verify that the referenced snapshot belongs to that
session; wrong kinds, missing snapshots and mismatched identities fail closed.
Existing V1 anchor encodings and hashes remain unchanged. Filesystem-local agent
targets remain unsupported. Session comments are human ReviewStore comments,
never agent prompts, capture edits, tool replies or controller dispatch.

Other agent anchors pin an immutable session snapshot plus the exact tool
attempt, message or plan owner snapshot. Text targets also pin the block
version, content object, content part, source coverage and representation,
sanitizer and normalizer versions. Kind mismatches fail closed. Anchor
dependency traversal enumerates every native or AgentStore object needed for
original-context reads; stable entity IDs and optional change IDs remain lookup
facts rather than strong object edges.

The additive portable V2 anchor represents retained AgentStore sessions and tool
invocations directly. It never invents a tool attempt or message/block identity
when capture did not retain one. A tool target binds the session ID and
snapshot, tool-call ID and exact tool snapshot. A text target additionally binds
the stored input/output content reference and a validated UTF-8 selection over
the original sanitized payload bytes, without reformatting them. A session
target binds its exact session snapshot. Existing V1 anchor bytes and decoding
remain unchanged.

Portable agent structural validation requires the tool to occur under that exact
ID and snapshot in the named session and requires both stable IDs to match the
tool record. Text validation requires the content reference to equal the
selected tool input/output and verifies the selected byte digest. The read port
must supply canonical repository-bound records from a qualified retained
selection; structural validation alone cannot establish publication, access or
retention. Portable code evidence must bind the anchor repository, including
both sides of a diff. Providers continue rejecting portable publication until
their immutable retention, shared command/audit and receipt gates pass.

The local provider backs portable agent structural inspection with its
explicitly companion-pinned canonical reader. Only records in that selected
AgentStore closure are admitted; a staged Git blob or future session version
does not become visible through an earlier pin. Inspection preserves operation
heads and selected metadata across restart and ordinary Git GC, and remains
separate from review publication and complete historical retention.

Canonical JSON encoding rejects unknown fields, alternate byte-offset forms and
noncanonical round trips. The anchor ID hashes those validated canonical bytes.
The v1 resolver maps an immutable anchor only against an explicitly identified
target supplied through a narrow read port. Callers provide retained original
bytes, bounded target bytes and typed validated predecessor, rename, copy or
agent-block lineage. A same `ChangeId` never substitutes for an exact revision
or for lineage evidence.

## Immutable ReviewStore objects

Catalog path copying and bounded root-pinned enumeration share one owned engine
across persistence adapters. Its object interface requires canonical identity,
semantic kind and dependency validation; its snapshot interface admits only the
audited selected closure. These interfaces do not carry publication authority.
Native staging repeats existing catalog, relation, comment ownership and event
reconstruction audits before selecting the returned root. Typed command
reduction and complete cross-object review auditing MUST use the same owned
projector across adapters. It returns a root value, delta and typed result only;
authorization, external code/agent retention, receipt replay and publication
remain adapter responsibilities. Repository identity is checked before immutable
reads or staging, and no projector result alone selects state.

`ReviewRootV1` selects independent COW catalog roots for threads, comments,
thread events, original-anchor relations, artifact relations, statuses, work
requests, external aliases and accepted source positions. The canonical empty
root omits all page references. Every populated root entry must select a typed
review index page; leaves contain at most 32 stable-key selections and branches
at most sixteen lowercase-hex children.

The registered v1 object kinds are index page, anchor, comment body, comment
revision, thread event, event page and thread snapshot. Review objects use
bounded canonical encoding, typed review-object references and explicit native
or AgentStore dependencies. Anchors forward their exact external retention
edges; other review objects can only point through declared ReviewStore kinds.
Root, revision, event and snapshot content IDs hash their validated canonical
forms and remain separate from stable workflow identities.

Comment bodies are bounded safe Markdown or plain UTF-8 text with an explicit
sanitizer version. Empty bodies, disallowed control characters and oversized
content fail closed. A comment revision retains stable thread/comment IDs,
original author, acting principal and accepted time. Visible edits select a body
object; tombstones and moderated hides require a prior revision, preserving
auditable history rather than deleting prior bytes.

Thread events have repository-accepted, strictly positive per-thread sequence
numbers and authenticated actors. Creation binds the original anchor and first
comment revision. Replies and revisions bind exact selected comment objects;
resolve/reopen actions retain their stable action identity and optional bounded
reason. Event pages contain at most 256 distinct, contiguous sequence entries
and a typed continuation, avoiding an unbounded reverse-linked event chain.

A thread snapshot selects one immutable original anchor, ordered unique comment
revisions, one event-page root, the last accepted sequence, open/resolved state
and selected workflow action. Its overall thread version advances for every
accepted event, while content and workflow versions remain distinct monotonic
preconditions. Parent comments must already occur in the selected order and a
resolved snapshot must identify its workflow action, while an open snapshot has
none. Cross-object loading reconstructs the ordered comments, selected
revisions, open/resolved state and content/workflow counters from the immutable
event stream. It rejects comment revisions from another thread, invalid reply
parents or revision predecessors, repeated workflow transitions, event-page or
event thread/sequence mismatches, missing creation identity and any
reconstructed state that disagrees with the snapshot.

## Root-pinned review store

The WAL coordinator stages each validated review object in the existing
repository-bound envelope and traverses every typed ReviewStore, native code and
AgentStore strong edge before staging a root. A review root independently audits
the semantic value kind of its thread, comment, thread-event, relation and
status catalogs. Work-request, alias and source catalogs still fail closed until
their typed object schemas are implemented.

Original-anchor and exact-artifact relations are bounded immutable linked pages
of stable thread IDs and current thread-snapshot references. Status pages use
the same bounded entry shape for open and resolved threads. Catalog keys are
derived from the typed anchor, artifact or status subject. Root audit verifies
the key, page-chain subject, unique threads, current thread-catalog selection,
original anchor or artifact identity and selected workflow state. A stale
relation snapshot or an open thread in the resolved catalog fails closed.

`ReviewDeltaV1` carries expected and replacement selections for thread
snapshots, comment revisions, thread-event pages, original-anchor relations,
artifact relations and statuses. It rejects no-op edits, duplicate catalog keys
and semantic kind substitution. Applying a delta checks every expected selection
against one pinned root, rewrites only affected persistent radix paths and
stages a fully re-audited replacement root. Thread commands rebuild the affected
direct-index pages in the same review-domain publication, including moving a
thread between open and resolved status pages. Staged objects and roots remain
invisible until the shared domain publication selects them.

A `ReviewStoreSnapshot` binds exact lookup, bounded radix-ordered enumeration
and object reads to the selected root closure. Reading an unselected staged
object returns stale. Catalog enumeration accepts at most 256 selections per
call and uses the last returned stable catalog key for continuation. Old
snapshots retain their original roots and selections after later COW updates.

Local repository-wide ReviewStore threads use the same command projector and
root-pinned reader. Their companion descriptor and receipt select the canonical
root and complete retained record inventory in one review-domain publication.
Local code/agent anchors remain unsupported until their external evidence and
ownership have a qualified provider; repository threads do not qualify them.

## Conservative anchor placement

`AnchorResolverV1` is a bounded read-only service over an original immutable
anchor and one explicit target. It returns `exact`, `mapped_unchanged`,
`candidate_modified`, `deleted`, `ambiguous`, `unresolved` or `unavailable` with
categorical evidence and at most the caller's bounded candidate count. A
root-pinned `ReviewStoreSnapshot` can load a selected anchor and invoke the same
pure resolver; the call has no publication, WAL or manifest capability.

Code placement accepts exact object identity or lineage whose from/to operation,
commit, path and file identities match both endpoints. It maps whole-file
identity and byte ranges proven unchanged by common prefix or suffix, returns a
modified candidate when the selected range intersects changed bytes, and keeps
multiple paths or repeated candidate ranges ambiguous. A complete supported
lineage with no target path is deleted. Missing lineage, unsupported targets and
exhausted byte budgets are unresolved; missing or redacted retained content is
unavailable.

Agent text maps to a newer block only when the session/block identity,
representation, sanitizer and normalizer versions match, retained block/content
lineage names both immutable versions, and the newer bytes retain the complete
old content as an unchanged prefix. Exact content versions remain exact. The
resolver never uses a stable tool or change ID alone, mutates the original
anchor, publishes a JJ operation, changes workflow state or records a permanent
placement decision.

## Stale-safe review commands

`ReviewCommandV1` admits only create thread, append reply, edit comment, resolve
comment conflict, tombstone comment, moderated hide, resolve thread and reopen
thread. A create stages its validated anchor, safe body, initial revision,
creation event, bounded event pages and first snapshot atomically. No staged
subset is visible until the shared manifest CAS selects the replacement review
root. The generated HTTP contract enumerates all eight commands and requires the
observed alternative revision IDs for conflict resolution.

The authenticated principal is the author and actor for new comments and the
actor for later revisions or workflow actions. The command has no client author
or accepted-time field. Stable thread, comment and action IDs are derived from
the repository plus authenticated request/publication identity before retry, and
server acceptance time is frozen for the command attempt. Receipt results return
those stable IDs and the exact committed event, comment revision and thread
snapshot references.

Concurrency is scoped to the entity being changed. Replies require the observed
workflow version and an open thread, so two replies can reapply to a newer
content version and both survive while workflow state is unchanged. Edits,
tombstones and hides require the exact selected comment revision. Resolve and
reopen require the complete observed thread snapshot ID, including all visible
comment activity. Resolution therefore rejects an unseen reply; a reply that
loses to resolution returns `thread_resolved` until an exact-snapshot reopen
commits.

Each accepted action appends one immutable event. The coordinator reads the
bounded selected event stream and rebuilds pages with at most 256 entries rather
than appending one reverse-linked page per action. Comment revisions retain the
prior revision edge and original author. Tombstone and moderated-hide records
remain auditable selected history rather than erasing prior bytes.

## Replicated collaboration facts

Authored AgentStore and ReviewStore publications retain their validated typed
deltas in the origin envelope. Import reads the referenced authored root to
prove every replacement exists in the transferred closure, then applies those
accepted facts to the destination's current pinned root. Import never retries a
capture parser, review command or other user action.

AgentStore uses deterministic catalog-specific reduction. Independent stable
keys union. Session snapshots union disjoint evidence, events, transcripts and
tools. A terminal tool state dominates a nonterminal observation; incompatible
terminal results remain an explicit `AgentCatalogConflictV1`. Checkpoint and
association versions follow their declared supersession edges, and source states
advance only when one receipt history is a prefix of the other. Conflicting
alias bindings remain explicit. Relation and projection selections are canonical
unions. An ordinary authored delta rejects direct conflict-object selection so
only the internal replication reducer can construct one.

ReviewStore represents concurrent state with `ReviewCommentConflictV1`,
`ReviewEventGraphV1` and `ReviewThreadMergeV1` while preserving the existing
catalog reference kinds and prior canonical object bytes. Event nodes retain
immutable event identities and causal parents. Independent replies survive;
concurrent edits retain every maximal revision. A resolve that did not observe
later activity cannot close the merged thread, and a reopen concurrent with a
resolve keeps it open. A later resolve whose graph parents cover all activity
can close it.

`ResolveCommentConflict` requires the complete strictly sorted set of currently
selected alternative revision IDs. The resulting visible revision has no single
previous-revision winner and instead names every superseded revision. The
conflict-resolution event retains the same set. Stale or incomplete sets fail
without publication. Relation and status pages are rebuilt atomically with the
reduced thread and comment catalogs.

Root-pinned service reads expose comment alternatives separately, include each
event's causal parents and never present one conflicting revision as canonical.
Normal reply and workflow commands can continue from a merged thread graph.
Native copy and cold recovery retain both AgentStore and ReviewStore conflict
objects and all alternatives. The restricted import capability cannot invoke
ordinary publication, domain-command or native-transition authorization and has
no controller, tool, hook, message or external-adapter dispatch interface.

## Root-pinned capture projection

The publication coordinator can stage one validated sanitized batch and its
normalized result against an explicitly pinned AgentStore root. It stages the
evidence batch first, then typed content, normalized event, reduced tool,
session, alias, source-receipt and projection objects. Permission and file
observations have explicit event variants; a file observation still cannot
become a tool invocation. Every dependency is audited by the existing typed
AgentStore object closure before the returned delta can select it.

The returned `AgentDeltaV1` updates session, alias, source and projection
catalogs in one existing manifest-domain publication. Session snapshots retain
prior selected evidence, event batches, transcripts and tool versions. Each
accepted batch replaces a selected transcript manifest with a cumulative,
self-contained immutable closure containing the prior ordered chunks plus the
new normalized message chunks; reconstruction never depends on an unselected
earlier manifest. Source state rejects repeated or overlapping receipts, records
the accepted native sequence and carries forward explicit coverage gaps.
Projection updates replace superseded session/source versions while preserving
prior immutable event batches. The caller must durably retain the resulting
delta and request identity before publication.

## Durable pending capture

`CaptureCursorV1` records the device-local integration and source identity,
nonzero generation, bounded parser version, optional native session key and the
source-specific accepted position. It is distinct from the published source
state. A native-sequence cursor can acknowledge a batch only at that batch's
last accepted observation.

The local capture spool uses short exclusive mutation locks outside repository
checkouts. Its directories are user-restricted on Unix, its entry count and
aggregate bytes are bounded, and every entry is a versioned regular file named
by the capture batch ID. Enqueue validates the supported normalizer profile,
sanitized batch, cursor and repository binding, writes through an atomically
renamed temporary file, synchronizes both file and directory and only then
returns the cursor acknowledgement. Exact retries are idempotent; a changed
repository, profile, cursor or batch under the same ID fails closed.

Preparing publication atomically replaces that entry with the exact validated
`AgentDeltaRequest`, including stable principal, request and publication IDs.
Restart discards only unacknowledged atomic temporary files, then reopens and
validates every pending entry. Corruption, symlinks, unexpected files,
concurrent mutation and byte-limit exhaustion fail closed. Completion removes
only a prepared entry whose request and publication IDs match the committed
outcome, then synchronizes the directory. An unprepared capture cannot be
discarded through the completion path.

Native capture publication is two phase. A bounded canonical prepare request
sends the supported profile, sanitized batch, stable request/publication IDs and
result to the repository service. The server derives the principal and
activation time from the admitted peer, pins the current AgentStore root and
returns the exact validated delta request; the device spool must persist those
bytes before publication. The second call submits only that prepared request.
Committed receipts and unknown outcomes retain the same identities, while a
different peer principal, changed retry or stale same-source edit is rejected.
Preparing again after an unpersisted response may stage a fresh intent, but a
persisted prepared entry is never restaged under its existing receipt identity.
The CLI worker validates every pending entry against the active repository
before its first network call. It orders by integration, source instance,
generation, first source sequence and batch ID, derives domain-separated stable
request and publication IDs from repository plus batch ID, and uses a bounded
batch acknowledgement as the receipt result. Only a matching committed receipt
removes the synchronized prepared entry. Unknown outcomes stop the drain;
transport errors, staleness, rejection and mismatched replies retain the exact
entry and prevent later captures from passing it.

Native and local recovery share a separate exclusive drain lease held from queue
selection through the final outcome. The drain lease is released on
cancellation, error or process exit. Queue reads, enqueue, preparation and
completion each hold the mutation lock only around bounded local I/O, never
across publication or other awaits. New hook processes can durably enqueue while
either native prepare or publication is waiting; they cannot start a competing
drain. Each drain uses its initial bounded queue snapshot and leaves later
arrivals for the next pass. Recovery of orphan temporary files also holds the
mutation lock, so opening another handle cannot remove a writer's live temporary
file. Both locks are device-local coordination only; exact prepared requests and
matching receipts remain the authority for retry and completion.

New handles retain a shared legacy `capture.lock` guard for their lifetime.
Older binaries require that guard exclusively and therefore cannot bypass the
new `mutation.lock` and `drain.lock` protocol. If an older writer already owns
the spool, a new handle fails closed before queue inspection or mutation. The
pending V1 record format is unchanged.

## Installed hook ingestion

`jj agent hook <codex|cursor|opencode>` is the owned, noninteractive ingestion
entrypoint installed by setup. It accepts no caller-selected repository or
source identity. Before sanitization it reads the device registry and verifies
the repository, canonical workspace, explicit enablement, exact qualified
version, recorded installed state and current owned project-file hash. A missing
compiled-rule cache fails closed and does not trigger a cold hook-time build.

Standard input is bounded to four MiB plus one overflow byte. After in-memory
sanitization, a dedicated bounded lock records the local observation time and
allocates a strictly increasing native sequence under one stable random source
instance and generation. The source reservation is atomically synchronized
before spool enqueue; a failed or interrupted enqueue can leave an explicit
sequence gap, while restart and unknown outcomes cannot reuse the sequence.
Batch and observation IDs are domain-separated hashes of source identity,
generation and sequence.

Codex and Cursor observations use local-hook trust; OpenCode observations use
local-plugin trust. Every batch records the exact Kingfisher policy and matching
version-bound normalizer profile. Concurrent hook processes share the allocator
without opening the registry's exclusive administrative lock. Raw input,
findings and secret-bearing errors are never written or logged. Pending batches
remain in the existing capture spool until the hook's bounded best-effort
publication worker prepares and commits their exact AgentStore delta and receipt
identity. A server outage or three-second hook budget expiry preserves the
capture and defers another attempt to a later hook invocation.

## Device-local integration registry

The CLI owns a versioned registry outside the checkout for each native
repository and canonical physical workspace. It persists one nonzero random
device identity and separate detection, desired enablement and setup state for
Codex, Cursor and OpenCode under an exclusive user-restricted lock. Atomic
synced replacement and strict repository/workspace validation preserve the
identity across restart and reject another authority opening the state.

Discovery searches the current PATH for each owned executable and executes a
two-second, 256-byte version probe. Only the exact qualified versions Codex
0.156.1, Cursor 3.22.7 and OpenCode 1.18.30 are supported. Missing executables,
other versions and failed probes are explicit states; stderr is discarded and
never enters the registry or command output. Explicit noninteractive enablement
may select only supported integrations, and dry-run does not modify durable
state. Enablement records desired policy while setup remains not installed.

Setup requires both enabled policy and exact-version support. It merges only the
owned command handlers into Codex or Cursor project hook documents and installs
one namespaced OpenCode project plugin. Before writing, the device registry
atomically stores exact original bytes and the original and owned SHA-256
hashes. Project files use bounded atomic replacement, preserve unrelated JSON
values and reject symlinked target directories. Restart recognizes either the
exact original-to-owned transition or the installed hash; any other bytes are
drift and block replacement. Dry-run writes nothing.

Owned removal is independent of current executable detection so an absent or
upgraded harness can still be disconnected safely. It accepts only the exact
owned target or an exact original already restored before interruption. The
former is atomically replaced by the recorded original, or deleted when setup
created it; the latter resumes ownership cleanup. A synchronized tombstone makes
record retirement restart-safe. Only after target and ownership cleanup does the
registry record disabled/not-installed state. Drift, symlinks and unexpected
ownership files fail closed, while dry-run and completed retries do not modify
project bytes.

The Codex contract follows `openai/codex` tag `rust-v0.156.1` at source revision
`b412ff32c417f855c2b2d1581b77058eed87c84b`; the OpenCode plugin contract follows
`anomalyco/opencode` tag `v1.18.30` at source revision
`3104c1428ec91f809e5ab86631300de41eb6952e`. Cursor uses the documented version-1
project hooks shape, but remains synthetic setup acceptance because the local
agent reports `2026.09.26-dd393fe` rather than qualified `3.22.7`. Drift-safe
removal and source-ordered exact publication are implemented. The M9.6 real-PATH
gate accepts the installed Codex and OpenCode versions, rejects the Cursor
drift, and proves side-effect-free setup previews in a disposable workspace. It
does not claim compatibility for the unqualified Cursor build.

## Cold three-root native copy

The WAL authority can build one bounded copy closure from a manifest selected by
successful cold recovery. It enumerates native inventory, current and retained
catalog pages, WAL/control records, settings, receipts, checkpoints, and the
exact AgentStore and ReviewStore root/object closures without bucket listing.
Review objects retain any referenced AgentStore objects, and every current or
historical domain root is included. The same publication, closure, byte and
timeout budgets that bound recovery also bound copying.

Stored native objects and immutable control dependencies are copied with
conditional creation and exact byte comparison. The canonical selected manifest
is installed last. A lost final acknowledgement succeeds only when the
destination contains those exact manifest bytes; different pre-existing bytes
fail closed. The destination then cold-recovers through its own native
dependency source and must reproduce the exact manifest digest plus code,
AgentStore and ReviewStore roots before success is returned. This is a local
native snapshot copy primitive, not backup, migration, remote export, failover
or promotion; those deployment behaviors remain deferred.

### Native comment presentation and edit context

Review reply actions bind the selected stable parent comment ID. Native comment
editing captures the original text and exact comment revision/version when the
edit begins; an explicit refresh does not silently replace that precondition.
Only visible own-author comments offer the Community edit action, and native
service authorization remains authoritative. A missing edited comment must not
fall back to creating a reply. Task edit context also retains the observed task
version. Agent transcript and tool captures stay immutable; copy menus expose
only the selected sanitized native presentation records.

Browser transcript presentation groups adjacent tool entries without changing
their native IDs, order, snapshot pins or immutable records. Other recorded
events remain group boundaries. Per-tool detail, copy and review actions bind
the original call. A failed call has its error status in the row and preserves
the recorded error in expanded details, without a redundant failure caption.

An unresolved review command retains its durable request identity and blocks
replacement submissions until status proves commitment. Draft and edit/reply
context remain present on failure or unknown outcome. Late command completions
from an invalidated connection cannot repopulate or navigate the active view.

M15's first persistence slice retains canonical content identities separately
from repository-bound envelope digests. Each selected domain root carries a
bounded canonical-object-to-record catalog, so providers can audit exact closure
without bucket listing. Native code anchors reuse full operation/commit IDs;
portable anchors retain explicit evidence profiles. Until the native portable
capsule carrier is qualified, those native portable anchors fail closed. A
native typed multi-domain request authorizes every selected domain, checks all
observed roots, and publishes one WAL position and original durable receipt.
Local companion batches similarly select all named domain blobs under one
expected-old-value CAS.

### Local browser recorded AgentStore

The local Git profile exposes independently companion-pinned recorded sessions,
sanitized activity, transcript streams and qualified portable associations.
Presentation uses the shared immutable reader and sanitizer-preserving
projection; local snapshots identify their real metadata commit and AgentStore
version. Native epochs, publication positions and V1 code IDs are never
fabricated. Association presentation retains source repository and capsule
evidence alongside exact operation and Git commit IDs. Session/transcript reads
validate committed metadata history and selected session membership before
returning content.

Local Agents presentation retains selected companion metadata, qualified source
repository identities, Git commits and immutable evidence. The browser gates
review controls for local repositories and compares freshness announcements with
retained selections, including stream reconnects.

Projects Agents directories adapt the existing bounded repository session reads;
they introduce no cross-repository authority or inferred task association.
Native and qualified local snapshots remain distinct and independently pinned.
Permission changes discard retained sessions, unsupported profiles use explicit
icon Empty states, and opening a session authorizes its repository separately.
Project and repository response panes respect the device Thread layout. Split
uses the same full-body AppShell standard right sidebar as Thread replies, with
shared header, close, resize and mobile lane mechanics. Focus uses the same
structural discussion sheet, including desktop body/panel coverage and mobile
root insets. Close, Escape and scrim dismissal restore the selected row focus.
The independently controlled details rail remains separate. They reuse the
repository Agent response presentation, including separately authorized
reactions and tool discussions. They retain the Projects directory and pinned
continuation rather than navigating to a repository page. Closing, changed scope
and late reads cannot leak or restore an obsolete response; unknown command
identities remain retained for recovery.

### Initial M18.1 correction semantics

Evidence-only V1 event batches may have zero activity events. Their strong
observation reference and selected source receipt/coverage retain
diagnostic-only captures without inventing a session. Known session aliases may
retain partial coverage; unbound omissions select no session. Existing nonempty
batch bytes are unchanged. Across publications, enrichment loads only the
selected session/tool, retains input/output, attempts and evidence, and records
terminal conflicts in source/session coverage. Original event batches retain
contradictory source observations. Exact bound Thread capture retains either one
real session selection or an explicit diagnostic-only capture gap. A gap link
pins the exact SourceState, empty EventBatch and original Thread binding as
strong dependencies. The source receipt and sanitized batch must agree on
integration, source, batch and sequence interval; the empty batch cannot claim
activity or a session alias. Both roots select atomically under the same
authorization and exact observed Thread/assignment preconditions. No session or
primary assignment is invented. Later valid capture uses its own explicit
binding and retains the earlier gap. Receipt replay, unknown outcomes and prior
unlinked bytes remain unchanged.

### Bounded transcript tail format

The supported AgentStore transcript page encoding is `transcript_tail_page`,
page schema 2, under the existing TranscriptPage semantic kind. A tail retains
up to 256 ordered chunk references and an optional strong prefix reference to
another current tail. Schema-1 forward pages are unsupported and rejected by
decoding, retained-closure audit, reads, append and recovery. There is no
automatic conversion, compatibility reader or migration profile. Current
`AgentRootV1`, chunk and manifest encodings retain their identity; version-1
naming alone does not indicate an obsolete format.

Tail append reads at most one selected page and its last chunk, copies the
bounded tail and stages only new tails. Root-pinned sequence witnesses derived
during complete provider closure auditing qualify ordering and record counts.
Missing, corrupt or inconsistent ordering evidence fails closed. Witnesses are
rebuildable snapshot metadata and cannot publish state. Other streams in a
manifest remain retained.

Current reads walk prefix-before-tail with cycle, page, stream identity, byte
and record limits. Every supported selected manifest remains reconstructible
from its own closure. Provider closure audits remain mandatory and separate from
projection append I/O; no publication authority or dependency check is weakened.
Durable capture spools, exact prepared requests and receipts remain independent
retry contracts. Removing obsolete AgentStore encodings does not rewrite or
discard those durable intents.

API source management retains a memory-only association to the last admitted
reader source and exact snapshot generation witness. The association survives
pause, source acquisition failure and terminal cache retention; it grants no new
source eligibility, Write access, import consent or publication authority. The
selected source page may inspect publication coverage for at most 100 such
associations in one existing FIFO/lock turn, reading the bounded publication
catalog and retained reader metadata once. Empty pages and file-only pages
perform no snapshot evidence inspection. No transcript body, pending body,
OpenCode request or new source owner is allowed in this read. Missing/corrupt
history or an unavailable/rebound prepared metadata handle yields absent
coverage, never an invented zero-message committed frontier. Captured sequence
and logical count remain distinct from exact contiguous native receipt sequence
and optional committed count. An empty lifecycle receipt may qualify count zero.
This source coverage is distinct from per-request accepted import completion.
Installation Admin/configuration and selected control-file admission bracket
metadata inspection. API history import/export and normal configured OpenCode
startup remain separately gated.

Capture presentation schedules ordinary status, loaded source coverage and
original import progress observations independently of stable command ownership.
UI closure or visibility cannot cancel, replace or automatically resend a
pending command. Background source discovery never retargets an accepted import.
Only qualified Codex profiles expose history import in the initial preview.

Installation capture feedback follows JST-UI-019: operational failures use
installation-scoped notifications while exact unknown-command recovery remains
beside the retained original request. Notification dismissal or navigation
cannot release, retarget or retry the command.

Agent prompt-element disclosures are presentation derived from recorded text,
independent of harness identity. Their renderer never modifies captured source,
canonical content, media type, request identity or copy values. Markup
attributes are inert text; the shared Markdown safe-HTML and link policy still
applies to element bodies. Malformed, unfinished or excessively nested markup
falls back to visible literal text rather than discarding source.

### Editable local capture configuration

JST-COLLAB-017 now includes a LocalTrusted Admin configuration editor. The
installation owns saved capture enablement, qualified Codex CLI/Desktop
profiles, approved directory overrides and bounded reader, discovery, source and
spool limits. Empty directory overrides use the server process's resolved
harness defaults. The editor MUST show saved versus startup-effective settings
and resolved default roots even when capture is disabled. Saving MUST NOT
initialize capture journals, renew import consent, repair state or change a
running worker. Changes take effect on the next server startup; pending
configuration MUST withhold runtime mutations until that restart. Existing
policy, source identities, prepared requests and receipts remain retained. Other
harness startup editing and arbitrary server TOML/credential editing remain
deferred.

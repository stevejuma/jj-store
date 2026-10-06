# Persistent Threads and Reactions (M15)

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

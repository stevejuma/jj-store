# Explicit inactive-history import

A private journal guard explicitly releases its lock when its owning operation
ends. A duplicate descriptor inherited by a child before exec MUST NOT extend
the completed guard's lock lifetime. The owning guard still excludes another
writer until release, and closing an old duplicate cannot unlock a later guard.
Nonblocking acquisition, private-file binding and unknown-outcome policy remain
unchanged. A deterministic duplicate-descriptor regression and the combined
receipt/cold-process family qualify this lifetime boundary.

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

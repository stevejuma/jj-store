# Historical retained stacks execution contracts

The following implementation evidence describes the previous retained-executor
profile. It is not a dependency of ordinary local stack workspace delivery.

JST-LOCAL-005 governs the production stacks command boundary. Version-1
selection separates repository identity, authorized device binding generation,
physical workspace identity and JJ workspace name from named combination
identity/generation. It pins the exact code operation and view, profile-specific
working-copy commit and physical working-tree fingerprint. Reading or validating
these records cannot initialize state, snapshot files or publish an operation.

Prepared intent includes this complete selection and a typed command. A
domain-separated digest identifies that exact intent independently of the stable
request ID. Apply names the original request and server-owned preparation; it
cannot submit replacement command input. Status binds the original identity and
digest before any new preparation or current-state validation. A preparation
identity additionally binds the exact unpublished operation/view, checkout
before/after trees and affected paths, proposed combination selection, stable
change rewrite map, ordered bookmark terms and conflict fingerprints. Preview
output tampering invalidates that identity; receipt publication must match the
retained operation/view. Receipts separate unpublished preparation, unknown
publication, published code with incomplete checkout, verified completion,
rejection and retired undone/abandoned operations. Neither completed nor retired
receipts authorize command execution. Discovery and durable journal storage
remain separate provider qualification.

Public command recovery MUST identify the original repository, physical
workspace, JJ workspace and code profile independently of current binding,
operation and combination generations. Exact status additionally binds the
original request ID and intent digest; cancellation input also names the exact
preparation. Authentication and registration remain separate runtime admission.
Versioned public results MUST contain only the validated original preview,
historical receipt and replay flag, never a private execution archive, physical
inventory, retention key, principal or installation path. A result validates
receipt-to-preview linkage before projection. Pending discovery MUST be
actor/scoped, complete within the journal's 128-record bound, unique by request
ID and exclude completed, rejected and retired receipts. Public results retain
an eight-MiB response bound. These records and journal projection methods do not
initialize missing state, revalidate against a newer code snapshot, execute a
command or authorize HTTP/UI capabilities. Cancellation contract validation does
not yet qualify its coordinated runtime implementation.

Public current-workspace discovery MUST remain separate from historical command
status. Its versioned input binds repository, current registration generation
and exact operation without client paths. Its result retains the full observed
selection and distinguishes absent configuration, ready, stale selected code and
pending recovery. An unreadable store is an error, never absence. A missing
store or an initialized empty catalog can represent absent configuration;
neither read initializes setup. Pending command coordination may exist before
any combination catalog is selected.

The public combination catalog MUST bind stable physical/profile scope and
contain only selected combination identities/generations, names, targets,
merge/code identities and applied-stack membership. It excludes staged proposed
after-state, retention evidence, private principal and installation paths. Its
explicit pending flag reports unresolved combination coordination independently
of the selected state. Bound discovery to 64 combinations, 4096 stack records
per combination and eight MiB. Validate unique IDs/names, active selection,
profile agreement and complete response linkage. A ready result requires the
active combination's code to match observed current code; stale and pending
results grant no command authority. The registered runtime must join current
observation, catalog and original journal discovery before transport/UI use.

Historical runtime admission MUST compare the requested stable physical
workspace against a provider-owned identity read from the registered canonical
root. Reuse the current observer's path/root/`.jj` device-and-inode digest;
server code MUST NOT duplicate that identity algorithm. This bounded read opens
directories without following links and rechecks identity across authorization.
It reads no operation, head, checkout/tree-state, ignore policy or working file,
and performs no initialization or repair. Current code loss or changed editor
bytes cannot prevent this identity read. Root or `.jj` replacement, unsafe paths
and revoked access fail or yield a distinct identity for the runtime to reject.
This proves physical admission only: original actor/scope/ID/digest,
registration and journal validation remain mandatory and runtime composition is
still a separate gate.

The installation-private stack command journal MUST explicitly create its
bounded state outside repository checkouts. Open, status and pending discovery
MUST NOT initialize missing state. Its scope binds installation, repository,
physical workspace, JJ workspace name and code profile; binding and combination
generations remain in each original request so historical recovery does not
depend on the current selection. The authenticated principal, complete preview
and original receipt MUST survive reopen. Reusing a request with different
intent, preparation or principal MUST fail; exact retry returns the retained
historical outcome without preparing again or resetting its state.

Journal selection MUST use private physical identity checks, exclusive
coordination, bounded reads, synced atomic replacement and directory sync.
Acknowledgement loss after replacement is an unknown outcome. Exact replay MUST
sync the selected record before acknowledging it. Records are never evicted to
admit new commands: capacity exhaustion fails before selection. Admission
reserves bounded receipt growth so later recovery transitions remain available
even when no further preparation fits. Only one unresolved publication or
checkout may occupy a physical scope. Prepared commands first become
publication-unknown before upstream publication. Recovery may select only the
matching prepared operation/view, with checkout required before verified
completion. Completed checkout cannot regress; rejected and undone/abandoned
records are terminal. Status matches the original request, digest and principal
before any live-state validation. The journal alone does not prove provider
object retention, publication or checkout and MUST NOT enable a runtime command
capability.

Prepared local code retention MUST use a separate reserved
`refs/lapis/stacks/prepared/v1/<physical-workspace>/<request>` namespace, never
the portable companion ref, user branches, tags or upstream JJ refs. A slot
binds repository, physical workspace, authenticated principal, request, intent
and prepared identity to the exact archived operation/view and code capsule.
Existing slots are immutable: conflicting reuse or an unexpected object at the
reserved name fails closed, without overwriting it. Read lookup cannot create a
slot or initialize companion state. These refs are not user branch discovery or
selected-push inputs and remain retained without automatic deletion.

Retention MUST link real Git commits through parent edges and raw trees/blobs
through tree entries, including JJ's nonstandard conflict-tree dependencies.
Complete bounded closure audit and object sync precede create-only ref
selection; exact retry verifies and syncs the selected loose or packed ref.
Selection uncertainty remains unknown. Archival operation/view/extra data MUST
remain readable after source operation/index loss and Git GC. This retention
boundary alone does not recreate the full source operation ancestry, authorize
head publication or establish guarded checkout; those execution gates remain
required before a writable capability.

Foreground Stacks coordination MUST acquire the existing JJ local working-copy
lock before an execution boundary, without waiting indefinitely or snapshotting
files. The lease retains worker admission until release. It MUST revalidate the
authorized registration and complete current checkout witness after acquiring
the lock, including physical workspace and metadata directory identity, exact
operation/view, named workspace, tree terms/labels, sparse rules, ignore inputs
and unchanged operation heads. Stale, busy, canceled or missing inputs cannot
initialize state or repair a checkout. Ordinary CLI commands must contend on the
same lock, and independent physical workspaces keep independent locks.

The lock MUST use descriptor-relative, no-follow file access and reject
symlinks, hardlinks and replaced/unlinked lock identities. Release MUST NOT
unlink a replacement file. A metadata lease is not a lock against external
editors and does not prove physical bytes: the bounded physical observation and
fresh checkout guards remain required. This coordination primitive alone cannot
publish code, modify checkout bookkeeping or enable a runtime write capability.

Local reword preparation MUST pin the foreground lease's exact source operation,
view and named workspace, authorize before and after work, and write only an
unpublished JJ transaction. Source membership follows visible parent ancestry,
not hidden predecessor records or an arbitrary client commit. Root, hidden,
oversized, signed or unsupported tree inputs fail without publication. The
bounded source closure and rewrite report MUST preserve stable change identity,
author, exact trees, descendant rewrites and ordered bookmark effects. The named
workspace's checkout tree must remain unchanged for this preparation.

Preparation MUST retain worker and foreground admission until a canceled worker
actually exits. It returns exact owned source/prepared capsules, without
selecting a retention ref, journal record, operation head or checkout metadata.
The caller's command policy, exact intent/retention/journal composition and
publication/checkout recovery remain separate required gates. An unpublished
reword primitive MUST NOT enable a runtime mutation capability.

Recoverable Stacks preparation MUST additionally retain the complete bounded
source operation ancestry, every exact referenced view and their complete code
and evolution closures, including historical views no longer selected by current
heads. Version-1 operation archives preserve ordered parents, metadata, optional
predecessor maps and profile identities. Missing ancestry, unrelated records,
identity mismatch, unsupported formats or resource exhaustion fail without a
partial archive. Capture MUST use bounded no-follow source reads, never a latest
load, index repair, snapshot or reconciliation. Historical code reads MUST
require the original extra record for each physical commit; missing records
cannot trigger implicit import or substitute synthetic change identities.

The reserved preparation slot supports an explicit version-2 retention record
with the operation archive alongside the unchanged selected capsule. Legacy
version-1 records remain readable and immutable; adding history to an existing
legacy slot is conflicting reuse. Complete historical code closure uses real Git
reachability before ref selection. Lookup and exact replay verify the entire
archive after source operation/index loss and Git GC. Private upstream operation
store materialization MUST reproduce every archived operation/view identity and
field, without implicitly restoring source metadata or publishing any head.
Runtime source restoration, command policy, journal coordination, publication
and checkout recovery remain separately required.

Real reword preparation MUST use this complete strict source archive before any
upstream candidate work and return the complete candidate archive with its
effects. Every previously captured operation, view and code record MUST remain
identical in the candidate ancestry. Lost original extras or missing operation
history fail before implicit import or candidate writes; cancellation still
retains the foreground lease until the worker exits. Returning recovery input
does not select a ref or journal, restore source state or enable execution.

Explicit source-history recovery MUST bind a version-1 installation-private
recovery witness to the original retained request before any execution. Its
canonical bounded encoding and domain-separated digest preserve authorized
repository/binding generation, physical workspace, original operation/view and
WC identity, checkout metadata/policy/head fingerprints and source Git/directory
binding. A client cannot supply recovery paths, archives or replacement intent.
Composition MUST look up the original journal principal/request before invoking
recovery; this provider primitive alone does not perform that policy lookup.

Recovery MUST work after operation-store and derived-index loss without loading
a latest operation or reconciling heads. It acquires the same nonblocking
foreground lock, rechecks the saved witness and reads only the matching
immutable reserved retention slot. Missing original source ancestry or
mismatched workspace/tree/view inputs fail. Private upstream materialization
supplies exact native operation/view records. The complete existing destination
is checked for conflicts and unsafe links before any missing source directory or
record is created. Existing identical records stay byte-identical; conflicting
records are never overwritten. Restoring missing operation-store directories and
their original fixed profile marker is an explicit recovery action, not
initialization on a read.

Missing immutable records MUST be selected with synced create-only rename and
directory sync, through held no-follow directory descriptors. Replaced
directories, linked entries, stale metadata/policy/heads, revoked authority or
cancellation fail explicitly. An interrupted recovery may leave exact
unpublished records; the same original identity verifies and syncs those records
on retry. Admission and the foreground lock remain held until the recovery
worker exits. Recovery MUST NOT select a head, recreate a derived index, restore
Git extras, modify companion selection, alter checkout bookkeeping or write
physical working files. Head publication, extra/index recovery, journal/status
composition and guarded checkout still require separate qualification before
runtime writable capabilities.

The server's private local preparation composition MUST authorize the original
principal and typed request, validate physical journal scope and look up the
original request before obtaining live context or preparing a candidate. Exact
historical replay returns the original preview and receipt without re-preparing
against current heads. Legacy metadata-only records cannot be upgraded to local
execution evidence. Production registration, command policy and combination
authority remain separate prerequisites; fixture-supplied context is not their
qualification.

Before reserved-ref selection, the server MUST durably select the complete exact
preview, bounded recovery witness, original retention key and candidate archive
as pending retention. This archive counts toward the journal's existing
128-record/8-MiB limits, including reserved receipt-growth allowance. Capacity
exhaustion cannot select a partial record or evict original requests. Once
selected, retry MUST resume this archive through the saved witness and never
prepare a replacement operation, even after operation/index loss. Reserved-ref
acknowledgement selects only the identical archive/key and removes the temporary
archive body from the journal. Acknowledged evidence retains its original
witness, archive digest and exact selected retention commit.

Catalog version 2 distinguishes records containing version-1 local execution
evidence; legacy version-1 records retain their byte-compatible encoding and
historical semantics. Malformed witness/key/archive linkage, conflicting intent
or principal, and unsafe private state fail without repair. Pending retention
cannot transition to publication-unknown; explicit rejection remains terminal.
Cancellation or authority loss after pending selection leaves discoverable
original recovery input and releases admission only when the worker exits.
Saving a preview and retaining code are separate durable boundaries, not an
atomic transaction. This preparation composition publishes no head and changes
no source checkout, companion state or physical file. It cannot enable writable
capabilities before real authority, source-extra/index recovery, publication and
guarded checkout are qualified.

The local registered preparation authority MUST accept only the configured
LocalTrusted Admin principal. It resolves the current folder exclusively through
device registration, rechecking exact canonical folder and generation throughout
provider work. A request cannot grant access through a folder path, physical
identity, combination claim or code profile. Missing/replaced/unregistered
bindings and unsupported caller modes fail before preparing or selecting a
journal record. Current physical selection MUST come from the existing bounded
observer, including ignored/untracked obstacles, rather than client
fingerprints.

Active combination and immutable heads MUST come from the trusted combination
owner at the actual observed physical workspace and JJ workspace, never the
request. Missing durable state remains unavailable, without fabricated setup.
Reword preparation MUST reject rewriting any configured immutable head or its
ancestors at the exact captured source, before candidate writes. Protected heads
require visible source membership, bounded distinct inputs and complete parent
closure. Runtime combination-state ownership, publication and checkout remain
separate gates; a tested registration/scanner adapter does not establish them.
Historical request replay precedes live combination/physical validation while
still requiring current principal and registration authorization. Resuming
pending retention uses the registered root and saved witness without reading a
missing operation store or re-preparing; saved generation/binding changes fail
at the provider boundary.

Server-retained content selections bind source and parent tree identities,
complete code/device/combination selection, safe repository path, exact
file-state fingerprint and whole-file or verified hunk scope. Commands carry
opaque issued tokens; client patch strings and hunk indexes are never executable
inputs. Hunk claims preserve the diff version and exact hunk fingerprints.
Profile-mismatched identities, duplicated hunks and stale physical or logical
selections fail closed. Tokens require authenticated provider lookup; DTO
validation is not authorization or proof of durability.

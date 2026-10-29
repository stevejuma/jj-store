# Thread files and static attachments

JST-COLLAB-037 through JST-COLLAB-039 specify the planned M35 extension to
ThreadStore. **This capability is not implemented or qualified.** The
[implementation plan](../../../../../docs/maintenance/thread-files-implementation-plan.md)
records the agreed product choices and delivery slices; the
[M35 ledger](../../progress/M35/index.md) owns status. Existing ThreadStore,
AgentStore and native/local publication contracts remain prerequisites.

## Ownership and durable content

Standalone agent chats and intentional Threads share one flat Files library in
their immutable metadata home. Standalone chats use internal ThreadStore owners,
which remain excluded from intentional Thread directories. Files are independent
of attached code repositories, client working files and agent scratch
directories. Reads do not initialize a home or file catalog.

A stable file identity selects immutable revisions retaining name, file kind,
media type, byte length, content digest, author and predecessor. Static
attachments preserve exact uploaded bytes. Editable files contain Markdown or
other UTF-8 text, initially bounded to the existing 256 KiB editing limit.
Static uploads allow 20 MiB per file, at most 10 files and 100 MiB per message.
Eligible static text can become a separate editable copy.

The catalog and revisions belong to ThreadStore. Bounded immutable content
chunks remain outside eagerly materialized metadata graphs and read pages.
Native WAL, manifest inventories, checkpoints and copy retain complete selected
closures; local companion publication retains actual Git blobs through selected
tree entries under `refs/lapis/state/v1`. All historical versions remain
reachable. Hashes alone do not retain bytes. Staging is invisible until selected
by the existing publication authority, and existing global metadata limits are
not increased to fit uploaded bodies.

Optional catalogs and references preserve legacy canonical encodings when
absent. New capabilities are advertised explicitly and unsupported profiles
refuse file writes. Native WorkStore origin freeze and code-repository reducer
refusals remain in force. This plan does not qualify new replicated origins,
ordinary Git/JJ metadata transfer, export/import or destructive committed GC.

## Uploads, saves and exact message references

Composer uploads stay private until accepted Send publishes file records and
exact message references atomically within one metadata home. Cancellation
leaves no visible attachment. Direct library uploads and explicit New file/Save
publish independently. Such an explicit first file write may provision the
personal metadata home and internal conversation owner; it never starts an agent
or creates an intentional user Thread.

Publication acceptance is distinct from recipient dispatch or acknowledgement.
If publication commits and dispatch fails, retain the files and message links
and expose delivery recovery. Unknown outcomes retain original request/body
identities and resolve through receipts; they never trigger blind writes or
prompt replay. Interrupted and cancelled staging has its own bounded lifecycle.

Explicit Save creates a new revision with an exact observed-revision
precondition. Concurrent human/agent edits fail stale and retain the draft.
Renames preserve old revision names. Remove hides the current library entry
while historical messages keep their original authorized file references;
restore creates a new revision. Earlier messages always open the version sent,
with explicit navigation to the latest version. Refresh preserves unsaved
drafts, historical selections and unresolved commands.

## Public access and agent context

Typed native/local service, HTTP and CLI boundaries share list, exact read,
history, create/upload, update, rename, remove, restore and receipt/status
semantics. Binary staging/download is bounded separately from metadata pages.
Readers validate selected file/thread membership and current home access;
possession of a file ID or content hash grants no access.

Message submissions and projections add optional typed home/thread/file/revision
references, distinct from repository `attachments`. Attachment-only messages are
valid. AgentStore retains canonical authored prompt text; ThreadStore retains
exact file associations without duplicating transcript bodies.

An explicit agent Send freezes the file catalog and selected revisions. Retained
Sent context exposes a bounded index with pagination over that same selection,
highlights selected attachments and supplies exact read commands. Queued sends
and retries reuse frozen snapshots. Content is read on demand with current
authorization rather than injecting all text automatically.

The paired custom `jj agent thread file` commands operate through the verified
coordination home. Agents may create or edit files within the user's requested
task through the same version-checked publication commands. Downloaded binary
scratch copies are disposable, and their interpretation depends on separately
qualified adapter tools. File reads, writes and freshness events never dispatch
an agent turn automatically.

## Presentation and acceptance

The shared Files area supports upload/new, open/download, rename/remove,
history/restore and explicit Markdown/text Save. Composer picking, drop and
pasted-image flows retain pending chips, progress, cancel and retry. Static
attachments keep their original bytes; bounded text and raster previews use safe
renderers. Uploaded HTML/SVG cannot execute or cause automatic external resource
fetching. Other formats remain downloadable.

Owned Community-derived presentation, released Design Core/Mira primitives,
provenance, keyboard/accessibility, application toasts and draft/pending-request
retention follow the existing UI rules. Storybook review precedes live adoption;
visual baselines require separate approval. Folder hierarchy, CRDT editing,
office-format editing and automatic code-file linking are deferred.

M35 acceptance requires exact bytes and historical links after cold restart,
local Git GC retention, native real-RustFS recovery/copy, stale-write and access
refusals, atomic attachment submission, unknown-outcome recovery, frozen queued
context, installed-adapter CLI reads/writes and cold browser coverage in both
storage profiles. Code working files and operation heads remain unaffected.
Documentation and fixture passes cannot close these implementation gates.

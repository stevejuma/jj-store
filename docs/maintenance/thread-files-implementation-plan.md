# Persistent attachments and editable thread files

Agreed 2026-10-23; saved for later implementation on 2026-10-24. Status:
**Planned; implementation has not started**. Saving this plan authorizes
documentation only. The
[M35 ledger](../../spec/src/current/progress/M35/index.md) owns delivery status;
the
[owning contract](../../spec/src/current/contracts/collaboration/45-thread-files-and-attachments.md)
and JST-COLLAB-037 through JST-COLLAB-039 govern implementation.

## Intended result and agreed choices

Standalone agent chats and intentional Threads get the same flat Files library.
Files belong to the conversation's metadata home, independently of any attached
code repository or working copy. A standalone chat uses its existing internal
ThreadStore owner and does not become an intentional user Thread.

| Choice               | Agreed behavior                                                                                                                                      |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Static attachments   | Preserve exact uploaded bytes; up to 20 MiB per file, 10 files and 100 MiB per message.                                                              |
| Editable files       | Markdown and other UTF-8 text, with explicit Save, immutable versions, history and restore. Initially retain the existing 256 KiB text-editor limit. |
| Library ownership    | Both standalone chats and intentional Threads use their metadata home.                                                                               |
| Agent discovery      | Supply a bounded file index; agents read exact saved versions on demand. Highlight files explicitly attached to a message.                           |
| Agent writes         | Allow creation and editing within the user's requested task through the same version-checked commands as people.                                     |
| Composer publication | Stage uploads privately; publish file records and message references together on Send acceptance. Cancelling a draft leaves no visible files.        |
| Persistence          | Existing native manifest/WAL authority or local Git companion publication under `refs/lapis/state/v1`.                                               |
| Transfer             | New metadata push/fetch and portable export/import are deferred. Ordinary Git/JJ push/fetch does not transfer complete collaboration state.          |

## Storage and publication

Extend `jj-collab` ThreadStore with stable file identities, immutable file
revisions and a file catalog. A revision retains name, file kind, media type,
byte length, content digest, author and predecessor. Static bytes cannot be
replaced; eligible text offers Make editable copy. Renames and removals retain
historical names, versions and message associations. Removal hides the current
library entry; restore creates a new selected revision.

Keep file content outside the eagerly materialized collaboration metadata graph.
Use bounded, immutable content-addressed chunks with explicit complete retention
inventories and streaming readers. Existing metadata/object/page limits must not
be globally raised to accommodate uploads. Native inventories, WAL, checkpoints
and copy retain the complete selected closure. The local provider retains actual
Git blobs through selected tree entries, including old versions; a digest or
JSON object ID alone does not retain content.

Composer uploads are private staging, not published repository state. The
accepted message submission selects staged files and exact references together
under one metadata-home receipt and publication boundary. A direct library
upload, New file or document Save is an explicit independent publication. Its
first use in a new standalone chat may provision the personal home and internal
owner, without starting an agent or creating an intentional Thread. Opening or
reading a chat still creates no durable state.

Message publication precedes agent delivery. A committed submission followed by
failed or unknown dispatch retains its files and references; show delivery
recovery rather than rolling them back or replaying a prompt. Interrupted
uploads and unknown writes preserve stable request identities and use status
lookup before retry. Unselected cancelled staging may be cleaned through its own
bounded lifecycle; this does not enable destructive GC of committed data.

Add optional fields and capability advertisement while preserving legacy bytes
when absent. Unsupported profiles refuse writes. Native WorkStore origin freeze,
code ThreadStore reducer refusals and separately qualified replication profiles
remain authoritative. This feature does not enable another storage authority.

## Public interfaces and agent access

Add typed commands for create/upload, update, rename, remove and restore, and
bounded list, revision-history and exact-content queries. Mutation preconditions
include the observed file revision and stable request ID. HTTP binary staging
and status routes, exact-version downloads, and the local/native CLI share
domain validation, current authorization and publication receipts. Content reads
verify selected thread/file membership; knowing a hash grants no access.

Use `jj agent thread file` for list, show/read/download, create/upload, edit,
rename, remove, history and restore. Agents invoke the verified paired custom
`jj` through the existing coordination checkout. Bounded input/body-file
handling and explicit output paths avoid treating a filename as authority.
Downloaded scratch files are disposable copies; editing one does not silently
publish it.

Extend conversation submissions, human-message revisions, typed agent-message
associations and read projections with optional exact file references containing
metadata home, thread, file and immutable revision. Keep these distinct from the
existing repository `attachments` field. Support attachment-only submissions.
AgentStore retains authored prompt text; ThreadStore links file associations to
the exact capture without duplicating transcript bodies or fabricating replies.
Update generated contracts and direct consumers when the executable interfaces
are implemented; this documentation slice generates no new wire contract.

Each explicit Send freezes its file-catalog selection and chosen revisions.
Retained Sent context carries a bounded index with pagination, selected
attachments and exact read commands. Index paging continues the same frozen
selection. Queued sends and exact retries keep their original snapshots; newer
edits require a later explicit selection. Every file read reauthorizes the
current caller. Do not inject all text into each prompt automatically.

Agents may read saved text or download binaries on demand. Interpretation of
PDFs/images depends on available adapter tools and is qualified separately from
durable storage. Agent saves retain actor/request provenance and use the same
stale-version checks as human saves. Neither saves nor file events dispatch a
turn. A failed save retains the draft and never overwrites concurrent work.

## Browser experience

Provide one flat Files area for chats and Threads, distinct from repository
Files/Changes. Include Upload, New file, open, download, rename, remove, history
and restore. An earlier message opens the original sent revision and can link
explicitly to the latest saved version. Changes to a file cannot rewrite old
message cards or Sent context.

The composer supports file picking, drag-and-drop, pasted images, pending chips,
upload progress, cancel and retry. Existing library files attach by exact saved
version. Keep unsent files private until submission acceptance. A cancelled or
failed draft preserves the user's remaining text and pending identities.

Use Mira for Markdown and the existing text/code editor for other UTF-8 files.
Explicit Save creates a revision. Preserve unsaved drafts across navigation and
refresh; warn before discarding them. On a stale save, retain the draft and
offer comparison with current content, followed by an explicit revised save. No
automatic last-write-wins or CRDT merge is introduced.

Preview bounded text and supported raster images safely; other formats remain
downloadable. Uploaded HTML/SVG is not executed, external resources are not
fetched automatically, and original bytes remain intact. Static text above the
editing limit remains downloadable as an attachment. Full office-document
editing is outside this first version.

Inspect Community first and Design Core second. Maintain adopted Community
presentation under `web/src/lib` with original paths, revisions and license in
`docs/ui-provenance.json`; use released foundational components. Operational
feedback uses application-owned toasts with stable identities and explicit
lifetime. Failure handling preserves drafts and pending requests. Review focused
Storybook stories before live adoption; visual baseline approval is separate.

## Delivery slices and acceptance

1. **M35.1 — domain and retention.** Add canonical metadata/content identities,
   file catalog, revision chains and provider retention. Prove legacy-byte
   compatibility, empty files, limits, content verification, exact historical
   membership, duplicate-content handling and rejection of incomplete closures.
2. **M35.2 — service, HTTP and CLI.** Add bounded staging/read/write/status
   interfaces and generated contracts. Prove unauthorized and foreign-thread
   refusals, unknown receipt recovery, interrupted staging and concurrent saves.
3. **M35.3 — submissions and agent access.** Publish files with message
   references atomically, then prepare/deliver frozen context. Prove
   attachment-only sends, dispatch failure after publication, queued/retried
   snapshot stability, and real CLI read/create/edit without implicit dispatch.
4. **M35.4 — Files UI and editor.** Review and qualify picker/drop/paste,
   cancel/retry, safe previews, Markdown/text editing, rename/remove,
   history/restore, stale-save recovery, draft retention, keyboard/accessibility
   behavior and toast assertions for each new failure path. Adopt the reviewed
   stories in the live application after their focused checks pass.
5. **M35.5 — cold qualification.** Upload, send, edit, reopen the earlier
   message, restart and download its original bytes in both local Git-backed and
   native profiles. Prove ordinary Git GC retention in disposable repositories,
   native real-RustFS restart/copy retention and actual installed-adapter
   access. Confirm thread-file mutations preserve code operation heads and
   working files.

Start every slice with its exact failing regression or named acceptance target
and record the before state. Run affected Rust packages and direct consumer
suites, generated-contract checks, relevant web/catalog type checks, focused
interaction/accessibility and UI ownership checks. Explain cross-layer
escalations before wider commands. Cold lifecycle checks are mandatory; fixtures
cannot replace real storage or installed-adapter evidence. Root aggregate lanes,
unrelated suites and visual baseline updates are not default qualification.

Update the owning topic before or with executable changes; add actual command,
failure, format/version and remaining-gap evidence under `spec/src/history` in
each Jujutsu slice. M35 remains Planned until implementation begins. M15/M24
retain their original statuses and incomplete gates; this plan closes none of
them. Remote metadata transfer, archive/export, rich office editing, folders,
CRDT collaboration and automatic code-repository file linking remain deferred.

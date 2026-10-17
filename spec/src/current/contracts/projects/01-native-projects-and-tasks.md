# Native projects and tasks

The jj-work overview identifies storage-independent commands and pure state
application. E-CRATE-READMES-001 preserves JST-WORK-001/004/005 identities,
versioned metadata, selected roots and original receipt behavior.

WorkStore is a repository-owned application domain alongside AgentStore and
ReviewStore. One project home owns metadata, tasks and discussion history. A
home may contain several projects and may also contain ordinary code.

Repository directory rows and cards use Community's name/description,
contributor avatar, commit/review/task activity bar and relative date anatomy.
Cards place link-state chips at the right of the name row, use the full content
width for description/access, bound descriptions to two visible lines, and
stretch to equal grid heights with bottom-aligned metadata. Retired and active
links share this layout; full description text remains accessible. The native
profile uses the latest selected change summary and contributor when there is no
independently stored repository description or owner. Code counts and activity
come from bounded, separately authorized snapshots; partial pages and
unavailable homes remain explicit. Mine continues to mean administer access.
Review rows and cards use the current root comment excerpt, full bound thread
identity, exact artifact label, comment count, creation actor and last accepted
event date. Tombstoned, hidden and conflicted root comments expose no old body.
Selecting a review from the directory or a project's Reviews tab opens the same
full main-page detail as the repository Reviews tab, with a fixed header and
composer and scrolling content between them. Thread Focus/Split settings do not
alter this top-level review navigation.

Repository discovery MUST expose names and links before card metadata reads.
Only visible rows and a small viewport overscan may request directory metadata;
scrolling does not drain the complete repository inventory. Retain at most two
actual reads across viewport changes and refresh, including canceled requests
whose transports have not settled. Cancel obsolete queued/in-flight reads and
discard late results. Refresh the visible loaded window; mark retained hidden
rows stale and reload them when shown. Independent unavailable metadata homes
remain partial, and existing errors survive retries until successful recovery.

Independent conversion stages complete validated WorkStore snapshots, retaining
stable entities, actor attribution, versions, reply history and original project
repository links. Staging cannot select a root; source receipts remain
provenance and do not grant destination authority. Historical source snapshots
must remain archived and map explicitly to independent destination selections.

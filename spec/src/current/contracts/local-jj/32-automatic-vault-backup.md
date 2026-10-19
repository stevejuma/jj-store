# Automatic vault backup

JST-LOCAL-007 adds an opt-in host capability for automatic local vault backup.
The Notes plugin enables it by default for open vaults. This capability is
separate from immutable history reads and manifest-backed server mutations.

The native host MUST bind a durable vault identity to its admitted physical root
and external repository/working-copy paths. Default storage leaves no `.jj` or
`.git` pointer in the vault. An explicit alternative puts `.jj` at the root;
existing repositories are preserved and require explicit adoption. Root relink
and storage migration MUST retain identity and recoverable history.

Saved editor and external file changes schedule capture after two seconds of
quiet, with a thirty-second maximum delay and sixty-second reconciliation.
Opening/resuming a vault reconciles before reporting current backup status. Each
changed tree becomes a retained Git-compatible checkpoint with a fresh
working-copy child. Identical trees create no checkpoint. Completed checkpoints
remain reachable without relying on the operation log. Independent backup
exclusions override Git ignore rules; metadata/runtime/cache/credential state is
excluded. Unsupported or incomplete capture MUST NOT report success.

Native workers own bounded scans, JJ locks and serialized per-vault mutations.
Read-only history queries never trigger capture. A host close drains capture
after editor persistence; remote availability cannot block saved-file closure.
Pending work and unknown command outcomes survive restart without duplicate
checkpoints or blind replay. All active work reports through host progress.

Explicit restore preserves a safety checkpoint and uses guarded file writes.
File restore previews pin retained checkpoint bytes and current disk hashes.
Durable request receipts recover lost outcomes without overwriting later edits.
Symlink restoration fails explicitly until its guarded profile is qualified.
Whole-vault preview pins the included tree and reports added/changed/deleted
files. Apply retains one original safety checkpoint and a durable progress
receipt, guards each replacement and verifies the complete resulting tree.
Partial work never reports success; a retry keeps the original request and
refuses to overwrite later edits. Runtime/excluded files remain host-owned.
Automatic remote sync fetches and merges before pushing the selected branch,
preserves published ancestry, and retains conflicts without overwriting current
files. Ordinary Git restores files and checkpoint versions on a fresh device;
native JJ metadata is device-local. Accounts use injected native credential
custody. GitHub precedes Forgejo; no credential enters a repository or log.

Historical search is a rebuildable projection of checkpoint text, paths,
descriptions, authors and dates. Hits retain exact revision/file identities.
Indexes never authorize source access or become backup authority.

Implementation starts with external initialization and checkpoint primitives in
`jj-store-jj`. `jj-vault` composes these through owned interfaces without
upstream JJ imports. The Tauri bridge keeps writer permissions distinct from its
existing read capabilities. The delivery ledger records implementation and
verification separately; deferred checks do not satisfy original gates.

Vault forge accounts use an installation-owned native manager and OS credential
namespace, outside vaults and separate from jj-store accounts. The desktop may
reuse jj-store's configured public GitHub OAuth client ID. Device codes and
tokens never enter renderer persistence. Account work is bounded and runs off
the UI thread; cancellation can revoke an in-flight authorization before it
persists.

Remote transfer uses a native-approved HTTPS endpoint and selected branch.
Credentials stay in process memory and per-child environment; neither Git config
nor receipts store them. Network transfer runs without the capture command
lease. Fetched Git heads are imported through the native backend and pinned
explicitly before any guarded restore. A fresh-device restore retains imported
ancestry. Publication records its exact expected remote head and immutable
target before network IO. An unknown push is observed before any further
publication; automatic replay cannot create another intent. Exact-head guards
reject remote drift.

Hosts queue fetched-head import and guarded remote restore through the injected
vault worker. Transfer remains outside its queue; only the resulting native
restore updates the saved local checkpoint status.

Restoration MUST reject symbolic links in every directory ancestor, including
when an alias targets otherwise excluded runtime state inside the vault. Preview
and apply both enforce this boundary; a previously valid preview cannot
authorize a later alias replacement.

Automatic synchronization captures saved local files before constructing a
retained native merge proposal against the exact imported remote head. A clean
proposal is applied only while its local checkpoint and complete disk-tree
preview remain current. Conflicted proposals never overwrite current files;
explicit per-path term/text/delete resolution creates retained native commits.
Proposal IDs and guarded apply receipts survive restart. Unresolved conflicts
cannot be published. Native JJ merge ancestry and conflict terms are preserved.

Publication recovery exposes a path-free original-request summary. Observation
never sends a push, including for a prepared request or read-only account. An
explicit user stop retains a terminal stopped receipt, clears only its own
active pointer and never cancels or undoes remote state. Replays of a stopped
request cannot publish or clear a newer intent. Root relink preserves the
original request through the stable vault/private-repository binding. Native
identities, immutable reads and network/capture separation stay unchanged.

Historical search walks a fixed retained checkpoint ancestry, with bounded
revision and changed-file pages. Text projection includes UTF-8 regular files up
to one MiB; other files remain path-searchable and their omission from text
search is reported. Current backup exclusions also apply to past file results.
Projection loss or malformed cache rebuilds from immutable native objects.
Selection revalidates the exact native revision/path before preview or restore.
Search never snapshots disk, merges heads, transfers or publishes. Immutable
search takes no working-copy writer lease; pausing capture retains read access.

Forgejo setup is explicit and initially uses personal tokens against an admitted
HTTPS `/api/v1` instance root. Native custody, token headers and Git basic-auth
login follow the selected provider. Git clone endpoints must remain on that
account's origin/project. GitHub device OAuth and endpoint admission do not
change; Forgejo OAuth, HTTP endpoints and hosted subpath profiles remain outside
this initial provider profile.

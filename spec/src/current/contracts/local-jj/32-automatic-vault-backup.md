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

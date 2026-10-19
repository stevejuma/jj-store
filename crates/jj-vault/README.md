# jj-vault

Automatic local history for an admitted notes vault. The coordinator batches
filesystem changes, reconciles periodically, and creates retained native JJ
checkpoints on its own worker thread. Repository metadata can live entirely
outside the vault. Checkpoints preserve ordinary Git ancestry for backup.

Construct a native `Binding` from host-owned vault identity and paths, create a
`NativeRepository`, and pass it with `Timing::default()` to `Backup::start`.
Forward internal and external change notifications to `changed()`, observe
`subscribe()` for progress, and await `try_close()` after editor saves drain.
Hosts supply watching, credentials, UI and admission; the library does not infer
authority from renderer paths. `Repository` is injectable for other providers.

Keep the same `Backup` after a failed `try_close`: repair the saved-file failure
and retry before releasing the vault. Failure status and the original worker
remain owned. Successful close is idempotent. The consuming `close()` remains
available for terminal cleanup, returning any capture error before retiring its
worker. Dropping the coordinator retires its worker without a final-capture
success claim; retained change handles cannot keep it alive.

Detailed contracts and delivery progress live in
`spec/src/current/contracts/local-jj/32-automatic-vault-backup.md` and M34. Use
`preview_restore(checkpoint, path)` before `restore(RestoreRequest)` for a
guarded file restore. Keep the same request ID when recovering a lost response;
the native provider retains a safety checkpoint and a durable receipt. Hosts
must protect and flush editor writers around preview and apply. Whole-vault
previews and restore use the corresponding `preview_vault_restore` and
`restore_vault` methods.

Hosts perform remote transfer separately, then pass its opaque fetched head to
`import_remote`. `restore_remote` uses the same serialized worker, safety and
status path while preserving imported ancestry. A network outage cannot hold the
capture worker. Remote setup and synchronization policy/UI belong to the host.

Automatic sync hosts fetch/import first, then call `prepare_sync` to retain an
upstream-native merge proposal. Conflict inspection/resolution writes immutable
candidates. `preview_sync` and `apply_sync` materialize only a current clean
proposal through guarded safety/receipt semantics. Keep the editor writer guard
across preview/apply and preserve the original request during recovery. Transfer
and credentials stay host-owned, outside this serialized worker.

Inspect `Binding::pending_publication` for a safe original-request summary.
`RemoteTransfer::reconcile` observes acceptance without sending a push,
including with read-only credentials. After explicit user confirmation,
`Binding::stop_publication` retains a terminal receipt and stops tracking that
request without contacting the remote. It cannot cancel or undo an accepted
push; subsequent synchronization must fetch and merge before a new publication.

For historical search, construct `search::NativeHistory` from the admitted
binding and current exclusions, then call `search(&search::Request)`. Continue
with the returned checkpoint-anchored cursor until it is absent, even when an
intermediate page has no hits. Text, path, author and date filters run over
retained changed-file versions and checkpoint metadata. Eight files are read per
page; non-UTF-8, binary, conflicted, symlink and over-one-MiB contents remain
path-only and are counted explicitly. Search has no capture, network or writer
lease. Its disposable private cache can be removed without losing history.
Caching stops at 256 MiB or 4,096 entries and reads continue from native
objects.

Custom readers implement `search::Source` and can use `search::Index` with a
host-owned cache directory and a namespace that includes their exclusion policy.
A result is a selection hint: revalidate its exact native checkpoint and path
through restore preview before applying anything.

For Forgejo, the native host supplies `GitCredentials::basic` with the approved
account login and PAT. GitHub retains `GitCredentials::token`. Neither type is
printable or serializable; selected endpoint/branch guards and original push
recovery are unchanged.

Hosts can offer external or root `.jj` storage using `Storage::new` with a
host-owned external binding and separate private control directory. Call
`recover()` before starting capture, and use revision-guarded `change()` for an
explicit layout choice. The move archives the original native repository and
retains receipt lineage. A failed move can be resumed with its original request;
`rollback()` keeps the original layout only before installation. Stop owned
capture and drain readers before changing layout, then bind fresh readers and
restart capture using `binding()`. All migration work belongs on a native
worker.

For a vault that already has JJ history, `Binding::existing_history()` observes
its recorded operation and working revision without snapshotting the original.
Pass that exact source and a stable request ID to `Backup::adopt_history` to
retain its Git ancestry in the independent backup. Saved vault files stay in
place. Native operation/evolution metadata remains in the original repository;
hosts should provide a separate immutable reader for it. Reuse the original
request after a lost response; `stop_adoption` fences even a request that has
not arrived yet, without undoing accepted ancestry or enabling remote
publication.

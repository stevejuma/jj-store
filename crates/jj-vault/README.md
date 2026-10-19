# jj-vault

Automatic local history for an admitted notes vault. The coordinator batches
filesystem changes, reconciles periodically, and creates retained native JJ
checkpoints on its own worker thread. Repository metadata can live entirely
outside the vault. Checkpoints preserve ordinary Git ancestry for backup.

Construct a native `Binding` from host-owned vault identity and paths, create a
`NativeRepository`, and pass it with `Timing::default()` to `Backup::start`.
Forward internal and external change notifications to `changed()`, observe
`subscribe()` for progress, and await `close()` after editor saves drain. Hosts
supply watching, credentials, UI and admission; the library does not infer
authority from renderer paths. `Repository` is injectable for other providers.

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
capture worker. Remote setup, automatic synchronization and historical search
remain tracked delivery stages; policy and UI belong to the host.

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

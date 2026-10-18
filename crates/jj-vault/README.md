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
must protect and flush editor writers around preview and apply. Remote
synchronization, whole-vault restore and search remain tracked stages.

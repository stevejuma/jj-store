# Native commands

- `jj store init`: create a new repository and its first physical workspace.
- `jj store attach`: create a physical workspace for an existing repository.
- `jj store migrate-local`: hydrate and switch the current remote-backed
  workspace to LocalTrusted storage.
- `jj store status`: inspect authoritative operation heads.
- `jj sync status`: inspect local readiness, worker state, lag and search
  freshness without contacting the upstream.
- `jj sync now`, `pause`, and `resume`: wake or control background transfer.
- `jj sync wait --timeout 30s`: wait for the finite local and upstream frontiers
  captured by the command.
- `jj sync reconcile`: explicitly merge the exact current operation heads.
- `jj replica inspect`: inspect issuer-bound local, peer and checkpoint
  frontiers.
- `jj store doctor`: check binding and current access without publishing.
- `jj store login` and `logout`: manage one OS credential account.
- `jj store capabilities`: inspect verified and unqualified capabilities.
- `jj store version`: inspect build information.
- `jj store help -k credentials`: credential and TLS guidance.
- `jj util completion`: upstream completions including the store command family.
- `jj util markdown-help`: raw authored command help in Markdown.

- `jj store serve-local --server-config PATH`: run the paired foreground server.
- `jj-store run --config PATH`: run the standalone server.
- `jj-store config`, `doctor`, and `operator`: configure, inspect and administer
  it.
- `jj-store registry init`: create private authenticated installation discovery
  state.
- `jj-store token generate`: create one manual bearer credential without
  persisting it.

- `jj store token` and `jj-store token`: list grants and apply or recover typed
  grant/revoke requests independently per repository.

The milestone ledger records remaining command parity and release gates.

Native Git bridge supports approved remote add/list/set-url/rename/remove and
scoped fetch. Bookmark publication supports patterns, revisions, generated
change and named bookmarks, all/tracked/deleted selection and default tracked
selection. Each update uses an exact destination lease; multiple updates require
remote atomic capability. `--dry-run` plans without publication. Fetched commit
tags are retained; tag publication and push options remain unsupported.

After interrupted publication, use `jj store git status [attempt]` (also
`--format=json`) and `jj store git recover <attempt>`. Recovery inspects the
original approved destination and repairs satisfied local bookkeeping. It does
not push again or prove who authored a currently observed desired ref.

Native Git code repositories carry immutable Git facts through reducer-2 origin
replication. Peers receive logical aliases and exact evidence; endpoint bindings
and credentials remain device-local. WorkStore homes retain local Git support
and their frozen origin ledger. GB6 qualification covers macOS ARM64 with JJ
0.45.1 and Git 2.55.0 under `jj-git-sha1-0.45.1-v1`, including generated
HTTPS/SSH credentials and the cold real-RustFS/browser external merge cycle.
Other platforms and external forge deployments remain unqualified.

For explicit device credential approval, use an absolute `GIT_ASKPASS` or
`GIT_SSH` executable path and, where required, an absolute `GIT_SSL_CAINFO`
file. `JJ_STORE_GIT_CREDENTIAL_HELPER` accepts `none`, `osxkeychain`, `manager`
and `libsecret`. Native/server credentials are withheld from transport helpers.
Dry-run prints the source, destination approval and every exact proposed lease
and target; it retains no attempt and sends no ref update.

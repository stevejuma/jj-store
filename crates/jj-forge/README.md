# jj-forge

Forge adapters, guarded stack submission and recoverable external-effect
journals for GitHub, GitLab and Forgejo. Repository code and operation
identities remain owned by JJ; forge actions retain their own request recovery.

Start with the `client`, `planner`, `engine` and `journal` modules when
composing a forge integration. Plan an explicit stack operation, execute it
under host admission and retain the original journal identity across unknown
outcomes. Saved account credentials use native credential storage.

See the [Git and forge guide](../../docs/site/guides/git.md) and
[transport semantics](../../docs/site/reference/transport.md).

For another native host, inject `accounts::Manager::new` with its private
account root and `accounts::store::ScopedNativeSecrets::new` using the host
application namespace. The default `NativeSecrets` namespace remains jj-store.
Only the public OAuth client ID may be reused; account secrets remain separate.

Native hosts may call `accounts::Manager::add_forgejo_token` with an explicitly
selected HTTPS `/api/v1` instance and an owned PAT. The shared injected secret
store retains credentials; inventory returns public identity only. The built-in
HTTP adapter uses provider token headers. This does not extend the default
GitHub OAuth/Enterprise commands; hosts own caller admission and setup UI.

An embedding host may replace a selected account token with
`accounts::Manager::replace_token`. The manager verifies the token against the
account's approved API and provider identity, then rechecks its metadata
revision and saved credential under the account-store lock. Changed or forgotten
accounts and concurrently refreshed credentials refuse without overwriting
custody. Successful replacement keeps the opaque reference and scope, refreshes
public identity fields and discards previous OAuth refresh material. Host
admission and pending-publication credential holds remain the embedding
application's policy.

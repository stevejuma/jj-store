Administer repository-scoped tokens through the authenticated native API.

```sh
jj store token --endpoint https://store.example --repo-id REPOSITORY_ID list
jj-store token --endpoint https://store.example --repo-id REPOSITORY_ID list
```

Both command families use the same repository admin credential from the OS
facility or `JJ_STORE_TOKEN`. An operator credential grants no repository
access. Connection flags precede the subcommand; `--format json` returns
structured results without a pager.

`grant --principal ID --permission read|write|admin --request PATH` adds a
client-generated token. Supply its complete bearer credential through a hidden
prompt, `--token-stdin`, or `JJ_STORE_GRANT_TOKEN`. The grant credential is
separate from the admin credential. Optional `--expires-at UNIX_SECONDS` sets
its expiry. No secret is accepted in arguments, printed, or saved in the
journal.

`revoke --token-id ID --request PATH` removes a grant. Both mutations retain the
exact snapshot, principal, request ID and intent in a bounded local request file
outside checkouts before publication. Existing request files must match the
original intent; a stale request never silently refreshes its snapshot.

`status --request PATH` checks the durable receipt without publishing. An
`unknown` result means no receipt was observed; it does not prove rejection.
`retry --request PATH` reapplies the exact saved request. Grant status/retry
needs the original grant credential through the same hidden/stdin/environment
input; revoke requires none. Keep that credential in its OS facility or secret
manager. Current admin authorization and the original principal are required for
every receipt read. Rotation to another admin token for the same principal is
allowed.

Keep request records for recovery. If a request is stale, inspect current token
state before deliberately creating a new request file. Never overwrite or delete
a journal to make an uncertain publication look like a new operation.

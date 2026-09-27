Attach a new physical workspace to an existing native repository.

```sh
jj store attach ./work --endpoint https://store.example --repo-id REPOSITORY_ID
```

The destination must be empty. A unique logical workspace name is generated
unless `--workspace` is supplied. Local JJ indexes and store bindings go in a
separate client data directory; `--cache` selects an explicit empty directory
outside the checkout. The workspace initially has an empty working change based
on the root. Use ordinary `jj edit` or `jj new` to select existing history.

Credentials come from `JJ_STORE_TOKEN` or the OS credential facility. TLS
verifies the server hostname and trust chain. `--ca-certificate` adds a
deployment CA. Plain HTTP requires both a literal loopback address and
`--insecure-loopback`. The command never converts a Git repository or prints a
token.

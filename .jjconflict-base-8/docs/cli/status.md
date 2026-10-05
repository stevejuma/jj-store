Inspect the current native repository heads without changing the checkout.

`jj store status --format json` reports the repository, endpoint, native
profile, and full operation head IDs. Multiple heads remain explicit. This
command never snapshots physical edits or reconciles operations. Use ordinary
`jj log` and `jj op log` for native history; JJ may snapshot when those commands
normally do.

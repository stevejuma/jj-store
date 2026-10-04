# Git interoperability

The native Git bridge provides explicit fetch and selected bookmark push. It
uses a private disposable CLI cache and retains manifest-selected Git facts;
that cache is not repository authority. This is different from browsing an
already Git-backed local JJ repository.

## Approve a destination

Remote aliases are logical names. A destination URL and credential helper are
approved on the device performing transport. Replication can carry logical
aliases and retained evidence, but does not carry device endpoint approval or
credentials. Changing a remote URL requires the applicable fresh approval.

The supported native profile covers approved remote add/list/set-url/rename/
remove and scoped fetch. Use the custom CLI's `jj git` interface and current
help for its admitted selectors. Ordinary Git-backed commands retain upstream
handling rather than being routed through native authority.

## Fetch and inspect

Fetch imports supported Git code through the bridge's exact identity conversion
and committed native facts. It does not initialize an arbitrary server-side
working copy. Inspect imported bookmarks and code from a pinned native view.
Fetched commit tags may be retained, but native tag publication is unsupported.

## Push selected bookmarks

Supported selection includes patterns, revisions, generated change bookmarks,
named bookmarks and all/tracked/deleted/default tracked selection. Every update
has an exact destination lease. Multiple updates require remote atomic support.
`--dry-run` reports the proposed leases and targets without publishing refs or
retaining a push attempt.

After interruption, inspect `jj store git status` and recover the original
attempt with `jj store git recover`. Recovery checks the original approved
destination and repairs satisfied local bookkeeping; it does not push again or
prove who authored an observed desired remote ref.

## Credentials and limits

Approved helpers use absolute `GIT_ASKPASS`, `GIT_SSH` and applicable
`GIT_SSL_CAINFO` paths. `JJ_STORE_GIT_CREDENTIAL_HELPER` accepts `none`,
`osxkeychain`, `manager` and `libsecret`. Native server credentials are not
passed to Git transport helpers.

Qualification covers macOS ARM64, JJ 0.45.1 and Git 2.55.0 with the SHA-1
bookmark profile. Push options, native import/export, signing/conflict-export
profiles, other platforms and arbitrary forge deployments are not implied by
this gate.

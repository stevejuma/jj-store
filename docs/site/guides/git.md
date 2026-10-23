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

## Submit an ordinary-local stack for review

Git-backed JJ repositories can use **Submit stack** in the Stack or Workspace
view, or the `jj-store stack` CLI. For GitHub, first add an account in
**Settings → Integrations**. Verified credentials stay in the OS credential
store; the browser receives account metadata only. A single verified account and
unambiguous remote can connect automatically, using the observed default branch.
Otherwise select the account, source/push remote, target repository and target
branch in Repository access. GitHub PR controls appear after connection.

GitLab and Forgejo retain explicit repository connection settings: API URL,
upstream project, source project/fork and push/target remotes. Their global
account screens are not yet available. CLI automation can supply
`JJ_STORE_FORGE_TOKEN` explicitly. Tokens never belong in the checkout or device
receipt. See [Integrations](settings.md#integrations) for saved accounts.

```text
jj-store stack pr configure --provider github --api-url https://api.github.com --repository owner/project
jj-store stack submit feature/api --dry-run
jj-store stack submit feature/api
jj-store stack pr checks 123
jj-store stack pr merge 123 --dry-run
jj-store stack pr merge 123 --yes
jj-store stack restack feature/api --dry-run
jj-store stack restack feature/api --yes
```

Preview shows each required ancestor bookmark, exact head and publication step.
Submission pushes those branches, creates missing draft PRs and maintains links
to the related PRs. Edit titles and descriptions before submission or in the PR
section beside its file diffs afterward; the PR section can also float. Checks
form a collapsed expandable footer beneath the description, with status groups,
bounded scrolling and reported durations where available. Row menus provide
check details, jobs, bounded logs and supported rerun/cancel actions. The bottom
action bar offers provider-supported merge methods. A queued merge stays
distinct from a completed merge.

Explicit status refresh updates managed links after landing. **Restack**
explicitly rebases and publishes remaining branches; polling never rewrites
local commits. Fork-only parent branches use labelled cumulative upstream PRs
and bottom-first merging. GitHub native stack membership is optional for
eligible same-repository chains. Shared-root/cross-combination Restack is
unavailable in this profile.

If an effect's outcome is unknown, inspect the retained original request with
`stack pr recover` or `stack pr operation-status`; do not create a replacement
submission. Forge reads load on first use and explicit Refresh. Session caches
reuse PR, checks, jobs and logs through navigation and reopening; Refresh,
connection/account changes and mutations invalidate them. There is no periodic
forge polling or always-on backend maintenance service. These controls are
implemented for the ordinary-local profile. Live forge/platform and packaged
end-to-end qualification remain open; native-storage submission is separate.

## Forgejo credentials in embedding hosts

An embedding native host may explicitly admit a Forgejo PAT for an HTTPS
`/api/v1` endpoint through the forge account manager. The endpoint is verified
against its user API before the account is retained. Provider requests use
Forgejo's token header, and a host-supplied credential namespace keeps separate
installations independent. The jj-store browser continues to use explicit
repository settings for Forgejo; native account support does not imply a global
Forgejo account screen or permission to push.

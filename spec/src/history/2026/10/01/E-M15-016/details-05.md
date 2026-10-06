---
schema: 1
id: E-M15-016
---

# E-M15-016 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

Final production bundle and manifest pass after the hidden Review reaction control condition. Existing large-chunk advisory remains. /tmp/jj-thread-web-build-final.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Formatted the evidence update; all previous entries are retained.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm view @lapismd/design-core@0.2.0 version
```

**Notes**

Registry reports 0.2.0; installed development snapshot is 0.1.2, so registry consumer parity remains unqualified.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm view @lapismd/lapis-community@0.1.0 version
```

**Notes**

Registry returns E404. Corrected the earlier released-package provenance claim; all UI evidence uses existing local file snapshots. No release or dependency-source patch was performed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 (compare every parent evidence object and inspect installed package provenance)
```

**Notes**

All 227 previous evidence objects are unchanged; E-M15-016 is the sole new entry. Installed package paths are local file snapshots, as corrected in versions and remaining.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "M15.5 remains In progress: actual cold backend/browser full flow, retained
  reference/link/lifecycle presentation and visual approval remain separate
  qualification."
- Reducer 3, Thread search, runner/JJ ACP dispatch and real RustFS full-flow
  remain deferred; code repository writes still reject unsupported replication.
- "Released Community dependency qualification remains pending: npm registry
  lookup for @lapismd/lapis-community@0.1.0 returned E404. UI type, unit,
  Storybook and build evidence used existing local file snapshots, not released
  registry packages. The parent lockfile already contains these development
  resolutions and is unchanged by this slice. No package publication is
  authorized."
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the locally verified channel UI slice in the original isolated workspace. Main workspace and its dev server remain unchanged. Qualify released Community consumption, cold full flow and remaining M15 stages separately.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Production composes CommunityWorkspaceSurface, CommunityThreadPanel, CommunityDetails, CommunityMemberAvatarGroup, CommunityMessage and CommunityReactions. Sidebar channel selection, deep-linked replies, authorized actors, explicit refresh, observed exact reactions, shared custom loading, stale response rejection and identity-scoped stable pending requests are jj-store-owned adapters. The existing Agent Sessions screen is retained; no JJ ACP process integration exists. Pending intents are scoped to repository and authenticated principal, retained only across repository switches within one credential session, and erased with protected caches on disconnect/revocation. Late completions cannot restore cleared drafts. Hidden Review comments have no reaction controls.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Public contract and UI/service crossing require selected domain, WAL, provider, RPC/HTTP/CLI and UI consumer evidence. No root aggregates, full Rust workspace, visual baseline changes, publication, installation, main restart or existing-repository migration.
<!-- /evidence-field -->

<!-- evidence-field:stories yaml -->
## Stories

```yaml
- file: catalog/components/store-threads/StoreThreads.stories.svelte
  name: Channel navigation creation and members
  result: pass
  scope: cold interaction/accessibility; visual-pending
- file: catalog/components/store-threads/StoreThreads.stories.svelte
  name: Shared composer replies and exact reactions
  result: pass
  scope: cold interaction/accessibility; visual-pending
- file: catalog/components/store-threads/StoreThreads.stories.svelte
  name: Unknown action keeps its receipt identity
  result: pass
  scope: cold interaction/accessibility; visual-pending
- file: catalog/components/store-threads/StoreCapturedReactions.stories.svelte
  name: Captured agent messages
  result: pass
  scope: cold interaction/accessibility; visual-pending
- file: catalog/components/store-threads/StoreCapturedReactions.stories.svelte
  name: Exact review comments
  result: pass
  scope: cold interaction/accessibility; visual-pending

```
<!-- /evidence-field -->


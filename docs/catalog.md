# Storybook catalog

Run `pnpm storybook` and open http://127.0.0.1:6046/. Override the port with
`pnpm storybook --port <port>` when occupied; never stop an unrelated listener.
`pnpm catalog:test`, `pnpm test:storybook` and `pnpm build-storybook` provide
unit, Chromium interaction/accessibility and static-build acceptance.
`pnpm catalog:test:browser` builds and checks a cold static catalog. Components,
presentation models and styles under `web/src/lib` are the production-owned
implementation shared by the served browser and these stories. Fixtures,
harnesses and the in-memory data source remain under `catalog`. All stories
remain visual-pending.

## Native macOS visual capture

The catalog uses released `@lapismd/storybook-addon-visual-delta` 0.1.0. Enable
the native adapter on your Mac with the ignored local configuration:

```sh
cp .visual-delta/config.local.example.json .visual-delta/config.local.json
mkdir -p "tests/visual/storybook.spec.ts-snapshots/profiles/native/$(node -p 'process.platform + "-" + process.arch')"
pnpm exec playwright install chromium
pnpm visual-delta doctor --runner
pnpm storybook
```

Restart Storybook after changing the local file. Native capture uses the
project's installed Playwright and browsers; it needs no Docker image. Shared
defaults retain Docker, and CI ignores the local file. Use
`--capture-profile native` to select native explicitly on macOS or Linux. You
can override browsers and comparison settings in the local file; the default
comparison thresholds remain unchanged.

The panel and CLI use the same native PNGs. On macOS ARM64 their paths are:

```text
tests/visual/storybook.spec.ts-snapshots/profiles/native/darwin-arm64/<story-id>-chromium.png
.visual-delta/artifacts/profiles/native/darwin-arm64/<story-id>-chromium.actual.png
.visual-delta/artifacts/profiles/native/darwin-arm64/<story-id>-chromium.diff.png
.visual-delta/artifacts/profiles/native/darwin-arm64/<story-id>-chromium.result.json
```

Intel Macs use `darwin-x64`. PNG baselines persist independently of the ignored
actual/diff/result artifacts. Create an exact story's baseline explicitly,
inspect it, then run compare-only verification against it:

```sh
pnpm visual-delta update --story-id jj-store-components-chip--bookmark --approved --create-only
pnpm visual-delta test --story-id jj-store-components-chip--bookmark --failure-mode strict --fresh
```

`pnpm test:visual` compares all eligible stories; `pnpm test:visual:affected`
selects affected stories. These commands preserve baseline files. Committing or
approving baseline PNGs remains a separate review. The static capture server
uses port 6047; set `VISUAL_SERVER_PORT` to override it consistently for the
CLI, Playwright suite and Storybook addon. Static builds retain their read-only
behavior and emit dependency statistics for affected selection.

## Provenance

Community source revision: `d859f33a484a9238bb11f974f4ead8e9be729ec6`. Source
repository: https://github.com/lapismd/lapis-community. Directly adapted source
is copyright its original contributors and licensed Apache-2.0, as is jj-store.
Design Core primitives are consumed through released
`@lapismd/design-core@^0.5.1`; their source is not copied. Community-local
tarballs are not runtime dependencies.

Repository loading compositions were additionally inspected at Community
revision `a2e4b21b`. They adapt the repository log, repository pane and shared
skeleton geometry while continuing to consume Design Core's released Skeleton
primitive.

Repository command search was inspected at Community revision `9d82df59` and
Design Core revision `94a1b70a`. The jj-store adaptation keeps Community's
sidebar trigger, keyboard shortcut, dialog/Command View composition and grouped
results. It replaces rooms, messages and members with snapshot-bound native
operations, commits and files, and consumes the released Design Core Command
View and Dialog exports without copying them.

Community-derived production presentation, models, helpers, styles and assets
are maintained entirely under `web/src/lib`. `docs/ui-provenance.json` records
source paths, revisions and Apache-2.0 attribution for the complete copied
closure, plus explicitly native helpers. The Threads, forum directory, settings,
search and composer adaptation inspected source revision
`741b8285e2081f1c9af58afe5a9b72f538fad781`. No runtime, type, build, test or
local checkout dependency on Community is permitted. `pnpm check:ui-ownership`
enforces dependencies, local file ownership and declared provenance coverage.
Design Core and Mira remain foundational packages. Native stable identities,
root pins, pagination, durable review commands, route state and layout storage
remain jj-store-owned. Design Core's public Column Canvas, Empty, Alert, Button
and Skeleton exports provide the shared interaction primitives; no upstream
component was copied for this slice.

| Community source                                                                                                                                                                                                    | jj-store destination                                                                         | Adaptation                                                                                                                                               |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/components/community-chip/CommunityChip.svelte`                                                                                                                                                                | `web/src/lib/ui/community-chip/CommunityChip.svelte`                                         | Direct copy; catalog-local imports where needed.                                                                                                         |
| `src/components/community-chip/props.ts`                                                                                                                                                                            | `web/src/lib/ui/community-chip/props.ts`                                                     | Direct copy; catalog-local imports where needed.                                                                                                         |
| `src/components/community-typography.css`                                                                                                                                                                           | `web/src/lib/ui/community-typography.css`                                                    | Direct copy; catalog-local imports where needed.                                                                                                         |
| `.storybook/portable-canvas.ts`                                                                                                                                                                                     | `.storybook/portable-canvas.ts`                                                              | Direct copy; catalog-local imports where needed.                                                                                                         |
| `.storybook/svelte-runtime-diagnostics.ts`                                                                                                                                                                          | `.storybook/svelte-runtime-diagnostics.ts`                                                   | Direct copy; catalog-local imports where needed.                                                                                                         |
| `.storybook/vitest.setup.ts`                                                                                                                                                                                        | `.storybook/vitest.setup.ts`                                                                 | Direct copy; catalog-local imports where needed.                                                                                                         |
| `src/components/community-chip/model.ts`                                                                                                                                                                            | `web/src/lib/ui/community-chip/model.ts`                                                     | Direct copy.                                                                                                                                             |
| `src/components/community-app-shell/CommunityAppShell.svelte`                                                                                                                                                       | `web/src/lib/ui/community-app-shell/CommunityAppShell.svelte`                                | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-app-shell/CommunitySidebarToggleIcon.svelte`                                                                                                                                              | `web/src/lib/ui/community-app-shell/CommunitySidebarToggleIcon.svelte`                       | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-app-shell/right-sidebar.css`                                                                                                                                                              | `web/src/lib/ui/community-app-shell/right-sidebar.css`                                       | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-overlays.css`                                                                                                                                                                             | `web/src/lib/ui/community-overlays.css`                                                      | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-navigation/CommunityNavigation.css`                                                                                                                                                       | `web/src/lib/ui/community-navigation/CommunityNavigation.css`                                | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-application/CommunityApplicationMobileNavigation.svelte`                                                                                                                                  | `web/src/lib/ui/community-application/CommunityApplicationMobileNavigation.svelte`           | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-navigation/CommunityNavigation.svelte`                                                                                                                                                    | `web/src/lib/ui/community-navigation/CommunityNavigation.svelte`                             | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-navigation/model.ts`                                                                                                                                                                      | `web/src/lib/ui/community-navigation/model.ts`                                               | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-application/CommunityApplication.svelte (shell composition)`                                                                                                                              | `web/src/lib/ui/store-application/StoreApplication.svelte`                                   | Shell geometry/styles retained; domain actions replaced with presentation inputs.                                                                        |
| `src/components/community-projects/CommunityProjectsRevisionGraph.svelte`                                                                                                                                           | `web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte`                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRevisionGraphCopy.svelte`                                                                                                                                       | `web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRevisionGraphIds.svelte`                                                                                                                                        | `web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte`                 | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRevisionGraphMessage.svelte`                                                                                                                                    | `web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphMessage.svelte`             | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsVirtualRows.svelte`                                                                                                                                             | `web/src/lib/ui/community-projects/CommunityProjectsVirtualRows.svelte`                      | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/virtual-project-row-estimates.ts`                                                                                                                                                | `web/src/lib/ui/community-projects/virtual-project-row-estimates.ts`                         | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLogFiles.svelte`                                                                                                                                      | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte`               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLogDiff.svelte`                                                                                                                                       | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLogHeader.svelte`                                                                                                                                     | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogHeader.svelte`              | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLogEmpty.svelte`                                                                                                                                      | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogEmpty.svelte`               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsGitAuthorAvatar.svelte`                                                                                                                                         | `web/src/lib/ui/community-projects/CommunityProjectsGitAuthorAvatar.svelte`                  | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log-graph.ts`                                                                                                                                                                              | `web/src/lib/projects/repository-log-graph.ts`                                               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log-graph.test.ts`                                                                                                                                                                         | `catalog/projects/repository-log-graph.test.ts`                                              | Catalog regression exercises the production-owned graph helper.                                                                                          |
| `src/projects/repository-log-message.ts`                                                                                                                                                                            | `web/src/lib/projects/repository-log-message.ts`                                             | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log-collapse.ts`                                                                                                                                                                           | `web/src/lib/projects/repository-log-collapse.ts`                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-relative-time/CommunityRelativeTime.svelte`                                                                                                                                               | `web/src/lib/ui/community-relative-time/CommunityRelativeTime.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-relative-time/model.ts`                                                                                                                                                                   | `web/src/lib/ui/community-relative-time/model.ts`                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-diff-controls/CommunityDiffControls.svelte`                                                                                                                                               | `web/src/lib/ui/community-diff-controls/CommunityDiffControls.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-member-avatar/CommunityMemberAvatar.svelte`                                                                                                                                               | `web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-member-avatar/community-member-avatar.css`                                                                                                                                                | `web/src/lib/ui/community-member-avatar/community-member-avatar.css`                         | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-member-avatar/model.ts`                                                                                                                                                                   | `web/src/lib/ui/community-member-avatar/model.ts`                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/community/avatar-content.ts`                                                                                                                                                                                   | `web/src/lib/community/avatar-content.ts`                                                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log.ts`                                                                                                                                                                                    | `web/src/lib/projects/repository-log.ts`                                                     | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-commit-diff/CommunityCommitDiff.svelte`                                                                                                                                                   | `web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte`                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log-files.ts`                                                                                                                                                                              | `web/src/lib/projects/repository-log-files.ts`                                               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/projects/repository-log-canvas.ts`                                                                                                                                                                             | `web/src/lib/projects/repository-log-canvas.ts`                                              | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte`                                                                                                                        | `web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLogRail.svelte`                                                                                                                                       | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogRail.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-projects/CommunityProjectsRepositoryLog.svelte`                                                                                                                                           | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte`                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies.                                                            |
| `src/components/community-skeleton/CommunitySkeleton.svelte`, `src/components/community-projects/CommunityProjectsRepositoryLog.svelte`, `src/components/community-projects/CommunityProjectsRepositoryPane.svelte` | `web/src/lib/ui/community-projects/CommunityProjectsRepositorySkeleton.svelte`               | Repository-specific bookmark, graph, file-tree, directory, source and diff loading geometry; native controller states replace Community transport state. |

## Native adaptations

IDs are native 128-character commit/operation identities and 32-character change
identities; copy actions return the full ID, never Community links. Graph
messages use keyboard-selectable text controls when no route is supplied. Author
avatars use local initials; fixtures provide fixed time anchors. The graph
retains Community layout and convergence handling. Native collapse deduplicates
exact commit IDs only: different commits sharing a change ID remain divergent
rows. Branch rows are copied from the branch picker and embedded in Column
Canvas. Operation detail uses the contextual rail composition without review
actions. Agent discussions, evolution navigation and all transport decoding are
omitted.

## Full application and shared styling

Open **JJ Store → Application → Full app** at
http://127.0.0.1:6046/?path=/story/jj-store-application--full-app. This story
creates its own in-memory controller and does not run scripted interactions. The
simple **Shell** stories remain available for inspecting shell mechanics.

`StoreController` is production-owned and follows Community's public
state/actions organization. Its asynchronous contract distinguishes connection,
collection and detail states as well as snapshot operation, inspected operation,
commit, change and comparison identities. `createInMemoryStoreController`
supplies local repository snapshots, per-instance history and fixture identity.
Repository navigation opens Files; the snapshot picker accepts bookmarks or
changes, preserving divergent targets explicitly. The copied profile footer has
local Sign in/Sign out actions. No credentials, repository publications or real
authentication are involved.

The copied Community Tailwind Vite setup compiles Design Core's public
stylesheet, including DM Sans Variable and Source Code Pro Variable from bundled
font assets. It also enables the shared 16px header icon rules. No font CDN is
needed.

Additional source mapping at the same Community revision:

| Community source                                                                             | jj-store destination                                                                         | Adaptation                                                                                                                                            |
| -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vite.config.ts`                                                                             | `.storybook/vite.config.ts`, `catalog/vitest.config.ts`                                      | Tailwind Vite plugin retained for shared styles; backend plugins omitted.                                                                             |
| `src/community/contracts.ts`, `src/community/community-controller.svelte.ts`                 | `web/src/lib/controller/` and `catalog/controller/`                                          | State/actions interface pattern only; new in-memory JJ domain implementation, no relay code.                                                          |
| `src/components/community-profile-menu/CommunityProfileMenu.svelte`                          | `web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte`                          | Avatar, trigger, tooltip, menu identity and styles retained; actions reduced to fixture sign-in/sign-out.                                             |
| `src/components/community-projects/CommunityProjectsRepositoryFileTree.svelte`               | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte`               | Presentation references replace forge types.                                                                                                          |
| `src/components/community-projects/CommunityProjectsRepositoryFilePanel.svelte`              | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte`              | Read-only CodeBlock and binary/loading/empty states; Markdown uses safe source, no Community mentions or OWNERS policy.                               |
| `src/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte`         | Same popover, filters and command rows, with bookmarks/changes in place of Git branches/sync refs.                                                    |
| `src/components/community-projects/repository-browser.test.ts`                               | `catalog/components/community-projects/repository-browser.test.ts`                           | Copied regression exercises the production-owned helper with presentation type substitution.                                                          |
| `src/components/community-projects/repository-browser.ts`                                    | `web/src/lib/ui/community-projects/repository-browser.ts`                                    | Pure path/breadcrumb helpers; owned presentation types.                                                                                               |
| `src/components/community-projects/CommunityProjects.css`                                    | `web/src/lib/ui/community-projects/repository-files.css`                                     | File-tree, source-panel and mobile layout rules only.                                                                                                 |
| `src/components/community-projects/CommunityProjectsCommitList.svelte`                       | `web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | Reuses author/time composition with the existing branch command rows; screenshot-directed card arrangement is a requested JJ presentation adaptation. |
| `src/components/community-workspace/model.ts`                                                | `web/src/lib/ui/community-workspace/model.ts`                                                | Minimal author/profile display type; no runtime workspace state.                                                                                      |

The Branches cards use All/Workspace/Conflicted filters instead of implying live
PR integration. Graph paths are unchanged; row endpoints follow measured native
bookmark content height and IDs wrap beneath text in narrow columns. Copied
graph and header-geometry regressions remain catalog unit tests. New native
identity, divergence, controller and cold-browser tests qualify the adaptations.

## Review refinements

The same source revision also supplies:

| Community source                                                                               | jj-store destination                                                            | Adaptation                                                                                                  |
| ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `src/components/community-projects/CommunityProjectsRepositoryDirectory.svelte`                | `web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte` | Native revision fields, deterministic dates and local controller callbacks replace forge operations.        |
| `src/components/community-projects/CommunityProjects.css` (directory table and commit summary) | `web/src/lib/ui/community-projects/repository-directory.css`                    | Copied directory table and summary styles.                                                                  |
| `src/components/community-application/CommunityApplication.svelte` (project breadcrumb styles) | `web/src/lib/ui/store-application/repository-breadcrumb.css`                    | Same geometry and muted colors. Compose the public breadcrumb list inside the existing navigation landmark. |
| `src/components/community-application/CommunityApplication.svelte` (project overview sidebar)  | `web/src/lib/ui/store-application/StoreApplication.svelte`                      | Public rail variant, transparent border and resize indicator copied from Community.                         |

The in-memory route records directory navigation as well as file selection.
Opening Files or returning from a file displays the copied directory table.
Branch card dividers apply to populated command wrappers so filtered final rows
have no trailing divider. All Column Canvas headers share the theme secondary
surface and bottom border. The three history pages omit duplicate body titles;
mobile sidebar/history controls remain available. Graph content reserves the
widest graph lane to keep author avatars aligned. These are requested native
presentation adaptations; no shared package implementation is patched.

**Review refinements** stories exercise the requested presentation and directory
navigation; branch-card stories cover separators before and after filtering. All
changed stories remain visual-pending.

## AppShell layout adoption

This workspace adopts the Design Core shell extensions from change `kulovuqu`
(`e6320c0d`). It keeps jj-store's existing shell controllers, panel IDs, layout
storage, history callbacks, route selection, pending commands and lazy view
owners. The application uses one mounted topbar, body-adjacent attached panels,
an independent outer details rail, shared rail sections and overlay-page chrome.

The browser topbar moves into a target before Main's scrolling content. Native
macOS chrome retains top placement, 40px height, traffic-light clearance and its
existing drag region. The details action still opens/closes its existing rail;
mobile entry selects that same panel through the existing mobile controller.

Discussion pages use contextual Close, fixed Footer and responsive inset tokens.
Nested thread bodies, document readers and the conflict editor explicitly own
their scrolling. Other discussion bodies retain the default shell Scroll Area.
The existing single-active-layer, coverage and return-focus rules remain intact.

### Temporary dependency validation

The new shell APIs are not included in the current registry release. This is a
local adoption workspace, using an explicitly packed archive rather than a
source alias. The tracked manifest and lockfile retain registry dependencies.
Registry-only deployment requires a released Design Core version containing the
new APIs, a normal dependency update and focused acceptance again.

The qualified archive is
`../design-core/.release/tarballs/lapismd-design-core-0.6.0.tgz`, with SHA-256:

```text
272678190a387b017f73b58c46417093340d9ccc069d31240e4e857fa629817a
```

Its public prepared shell components were compared with the owning source. The
package provides compiled rune modules, declarations, consumer-compiled Svelte
components and CSS at its public subpaths. Its `0.6.0` development manifest does
not mean the registry's `0.6.0` release includes these additions.

To reproduce from this checkout, first install the normal dependencies, then
prepare the ignored local archive and pnpm hook:

```sh
pnpm install --frozen-lockfile --ignore-pnpmfile
mkdir -p .local/shell-layout
cp ../design-core/.release/tarballs/lapismd-design-core-0.6.0.tgz \
  .local/shell-layout/lapismd-design-core-shell-layout.tgz
```

Save this as `.local/shell-layout/pnpmfile.cjs`:

```js
const archive =
  "file:" +
  require("node:path").join(__dirname, "lapismd-design-core-shell-layout.tgz");
module.exports = {
  hooks: {
    readPackage(manifest) {
      for (const field of [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
      ]) {
        if (manifest[field]?.["@lapismd/design-core"])
          manifest[field]["@lapismd/design-core"] = archive;
      }
      return manifest;
    },
  },
};
```

Install it without rewriting the tracked registry lockfile:

```sh
pnpm --config.pnpmfile=.local/shell-layout/pnpmfile.cjs \
  --config.lockfile=false install
```

Do not run an ordinary install over this setup before testing these unpublished
APIs. Restore registry-only dependencies with
`pnpm install --frozen-lockfile --ignore-pnpmfile` when retiring the local
review. No package publication, production restart or state migration is part of
this slice. Functional evidence and the remaining release/visual gates are
recorded in
[E-SHELL-LAYOUT-001](../spec/src/history/2026/10/22/E-SHELL-LAYOUT-001/index.md).

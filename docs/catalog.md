# Storybook catalog

Run `pnpm storybook` and open http://127.0.0.1:6046/. Override the port with
`pnpm storybook --port <port>` when occupied; never stop an unrelated listener.
`pnpm catalog:test`, `pnpm test:storybook` and `pnpm build-storybook` provide
unit, Chromium interaction/accessibility and static-build acceptance.
`pnpm catalog:test:browser` builds and checks a cold static catalog. Production
UI integration stays deferred. All stories remain visual-pending.

## Provenance

Community source revision: `d859f33a484a9238bb11f974f4ead8e9be729ec6`. Source
repository: https://github.com/lapismd/lapis-community. Directly adapted source
is copyright its original contributors and licensed Apache-2.0, as is jj-store.
Design Core primitives are consumed through released
`@lapismd/design-core@^0.2.0`; their source is not copied. Community-local
tarballs are not runtime dependencies.

| Community source                                                                             | jj-store destination                                                                             | Adaptation                                                                                    |
| -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------- |
| `src/components/community-chip/CommunityChip.svelte`                                         | `catalog/components/community-chip/CommunityChip.svelte`                                         | Direct copy; catalog-local imports where needed.                                              |
| `src/components/community-chip/props.ts`                                                     | `catalog/components/community-chip/props.ts`                                                     | Direct copy; catalog-local imports where needed.                                              |
| `src/components/community-typography.css`                                                    | `catalog/components/community-typography.css`                                                    | Direct copy; catalog-local imports where needed.                                              |
| `.storybook/portable-canvas.ts`                                                              | `.storybook/portable-canvas.ts`                                                                  | Direct copy; catalog-local imports where needed.                                              |
| `.storybook/svelte-runtime-diagnostics.ts`                                                   | `.storybook/svelte-runtime-diagnostics.ts`                                                       | Direct copy; catalog-local imports where needed.                                              |
| `.storybook/vitest.setup.ts`                                                                 | `.storybook/vitest.setup.ts`                                                                     | Direct copy; catalog-local imports where needed.                                              |
| `src/components/community-chip/model.ts`                                                     | `catalog/components/community-chip/model.ts`                                                     | Direct copy.                                                                                  |
| `src/components/community-app-shell/CommunityAppShell.svelte`                                | `catalog/components/community-app-shell/CommunityAppShell.svelte`                                | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-app-shell/CommunitySidebarToggleIcon.svelte`                       | `catalog/components/community-app-shell/CommunitySidebarToggleIcon.svelte`                       | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-app-shell/shell-header-geometry.ts`                                | `catalog/components/community-app-shell/shell-header-geometry.ts`                                | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-app-shell/shell-header-geometry.test.ts`                           | `catalog/components/community-app-shell/shell-header-geometry.test.ts`                           | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-app-shell/right-sidebar.css`                                       | `catalog/components/community-app-shell/right-sidebar.css`                                       | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-overlays.css`                                                      | `catalog/components/community-overlays.css`                                                      | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-navigation/CommunityNavigation.css`                                | `catalog/components/community-navigation/CommunityNavigation.css`                                | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-application/CommunityApplicationMobileNavigation.svelte`           | `catalog/components/community-application/CommunityApplicationMobileNavigation.svelte`           | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-navigation/CommunityNavigation.svelte`                             | `catalog/components/community-navigation/CommunityNavigation.svelte`                             | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-navigation/model.ts`                                               | `catalog/components/community-navigation/model.ts`                                               | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-application/CommunityApplication.svelte (shell composition)`       | `catalog/components/store-application/StoreApplication.svelte`                                   | Shell geometry/styles retained; domain actions replaced with presentation inputs.             |
| `src/components/community-projects/CommunityProjectsRevisionGraph.svelte`                    | `catalog/components/community-projects/CommunityProjectsRevisionGraph.svelte`                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRevisionGraphCopy.svelte`                | `catalog/components/community-projects/CommunityProjectsRevisionGraphCopy.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRevisionGraphIds.svelte`                 | `catalog/components/community-projects/CommunityProjectsRevisionGraphIds.svelte`                 | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRevisionGraphMessage.svelte`             | `catalog/components/community-projects/CommunityProjectsRevisionGraphMessage.svelte`             | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsVirtualRows.svelte`                      | `catalog/components/community-projects/CommunityProjectsVirtualRows.svelte`                      | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/virtual-project-row-estimates.ts`                         | `catalog/components/community-projects/virtual-project-row-estimates.ts`                         | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLogFiles.svelte`               | `catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.svelte`               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLogDiff.svelte`                | `catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLogHeader.svelte`              | `catalog/components/community-projects/CommunityProjectsRepositoryLogHeader.svelte`              | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLogEmpty.svelte`               | `catalog/components/community-projects/CommunityProjectsRepositoryLogEmpty.svelte`               | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsGitAuthorAvatar.svelte`                  | `catalog/components/community-projects/CommunityProjectsGitAuthorAvatar.svelte`                  | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-graph.ts`                                                       | `catalog/projects/repository-log-graph.ts`                                                       | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-graph.test.ts`                                                  | `catalog/projects/repository-log-graph.test.ts`                                                  | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-message.ts`                                                     | `catalog/projects/repository-log-message.ts`                                                     | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-collapse.ts`                                                    | `catalog/projects/repository-log-collapse.ts`                                                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-relative-time/CommunityRelativeTime.svelte`                        | `catalog/components/community-relative-time/CommunityRelativeTime.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-relative-time/model.ts`                                            | `catalog/components/community-relative-time/model.ts`                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-diff-controls/CommunityDiffControls.svelte`                        | `catalog/components/community-diff-controls/CommunityDiffControls.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-member-avatar/CommunityMemberAvatar.svelte`                        | `catalog/components/community-member-avatar/CommunityMemberAvatar.svelte`                        | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-member-avatar/community-member-avatar.css`                         | `catalog/components/community-member-avatar/community-member-avatar.css`                         | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-member-avatar/model.ts`                                            | `catalog/components/community-member-avatar/model.ts`                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/community/avatar-content.ts`                                                            | `catalog/community/avatar-content.ts`                                                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log.ts`                                                             | `catalog/projects/repository-log.ts`                                                             | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-commit-diff/CommunityCommitDiff.svelte`                            | `catalog/components/community-commit-diff/CommunityCommitDiff.svelte`                            | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-files.ts`                                                       | `catalog/projects/repository-log-files.ts`                                                       | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/projects/repository-log-canvas.ts`                                                      | `catalog/projects/repository-log-canvas.ts`                                                      | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | `catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLogRail.svelte`                | `catalog/components/community-projects/CommunityProjectsRepositoryLogRail.svelte`                | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |
| `src/components/community-projects/CommunityProjectsRepositoryLog.svelte`                    | `catalog/components/community-projects/CommunityProjectsRepositoryLog.svelte`                    | Reused composition/helpers; native presentation inputs replace transport/domain dependencies. |

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

`StoreController` follows Community's public state/actions organization.
`createInMemoryStoreController` supplies local repository snapshots,
per-instance history and fixture identity. Repository navigation opens Files;
the snapshot picker accepts bookmarks or changes, preserving divergent targets
explicitly. The copied profile footer has local Sign in/Sign out actions. No
credentials, repository publications or real authentication are involved.

The copied Community Tailwind Vite setup compiles Design Core's public
stylesheet, including DM Sans Variable and Source Code Pro Variable from bundled
font assets. It also enables the shared 16px header icon rules. No font CDN is
needed.

Additional source mapping at the same Community revision:

| Community source                                                                             | jj-store destination                                                                             | Adaptation                                                                                                                                            |
| -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vite.config.ts`                                                                             | `.storybook/vite.config.ts`, `catalog/vitest.config.ts`                                          | Tailwind Vite plugin retained for shared styles; backend plugins omitted.                                                                             |
| `src/community/contracts.ts`, `src/community/community-controller.svelte.ts`                 | `catalog/controller/`                                                                            | State/actions interface pattern only; new in-memory JJ domain implementation, no relay code.                                                          |
| `src/components/community-profile-menu/CommunityProfileMenu.svelte`                          | `catalog/components/community-profile-menu/CommunityProfileMenu.svelte`                          | Avatar, trigger, tooltip, menu identity and styles retained; actions reduced to fixture sign-in/sign-out.                                             |
| `src/components/community-projects/CommunityProjectsRepositoryFileTree.svelte`               | `catalog/components/community-projects/CommunityProjectsRepositoryFileTree.svelte`               | Presentation references replace forge types.                                                                                                          |
| `src/components/community-projects/CommunityProjectsRepositoryFilePanel.svelte`              | `catalog/components/community-projects/CommunityProjectsRepositoryFilePanel.svelte`              | Read-only CodeBlock and binary/loading/empty states; Markdown uses safe source, no Community mentions or OWNERS policy.                               |
| `src/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | `catalog/components/community-projects/CommunityProjectsRepositoryBranchSelector.svelte`         | Same popover, filters and command rows, with bookmarks/changes in place of Git branches/sync refs.                                                    |
| `src/components/community-projects/repository-browser.test.ts`                               | `catalog/components/community-projects/repository-browser.test.ts`                               | Copied helper regressions with presentation type substitution.                                                                                        |
| `src/components/community-projects/repository-browser.ts`                                    | `catalog/components/community-projects/repository-browser.ts`                                    | Pure path/breadcrumb helpers; owned presentation types.                                                                                               |
| `src/components/community-projects/CommunityProjects.css`                                    | `catalog/components/community-projects/repository-files.css`                                     | File-tree, source-panel and mobile layout rules only.                                                                                                 |
| `src/components/community-projects/CommunityProjectsCommitList.svelte`                       | `catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte` | Reuses author/time composition with the existing branch command rows; screenshot-directed card arrangement is a requested JJ presentation adaptation. |
| `src/components/community-workspace/model.ts`                                                | `catalog/components/community-workspace/model.ts`                                                | Minimal author/profile display type; no runtime workspace state.                                                                                      |

The Branches cards use All/Workspace/Conflicted filters instead of implying live
PR integration. Graph paths are unchanged; row endpoints follow measured native
bookmark content height and IDs wrap beneath text in narrow columns. Copied
graph and header-geometry regressions remain catalog unit tests. New native
identity, divergence, controller and cold-browser tests qualify the adaptations.

## Review refinements

The same source revision also supplies:

| Community source                                                                               | jj-store destination                                                                | Adaptation                                                                                                  |
| ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `src/components/community-projects/CommunityProjectsRepositoryDirectory.svelte`                | `catalog/components/community-projects/CommunityProjectsRepositoryDirectory.svelte` | Native revision fields, deterministic dates and local controller callbacks replace forge operations.        |
| `src/components/community-projects/CommunityProjects.css` (directory table and commit summary) | `catalog/components/community-projects/repository-directory.css`                    | Copied directory table and summary styles.                                                                  |
| `src/components/community-application/CommunityApplication.svelte` (project breadcrumb styles) | `catalog/components/store-application/repository-breadcrumb.css`                    | Same geometry and muted colors. Compose the public breadcrumb list inside the existing navigation landmark. |
| `src/components/community-application/CommunityApplication.svelte` (project overview sidebar)  | `catalog/components/store-application/StoreApplication.svelte`                      | Public rail variant, transparent border and resize indicator copied from Community.                         |

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

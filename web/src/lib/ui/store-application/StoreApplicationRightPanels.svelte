<!-- @community-copy src/components/community-application/CommunityApplication.svelte -->
<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { untrack } from "svelte";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreProjectsRail from "../store-work/StoreProjectsRail.svelte";
  import StoreThreadSidebar from "../store-threads/StoreThreadSidebar.svelte";
  import { useThreadPanel } from "../store-threads/thread-panel.svelte.js";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
  import StoreRepositoryNavigation from "./StoreRepositoryNavigation.svelte";
  import StoreRepositoryContextActions from "./StoreRepositoryContextActions.svelte";
  let {
    controller,
    shell,
    onSelectPage,
  }: {
    controller: StoreController;
    shell: AppShellController;
    onSelectPage: StoreController["actions"]["selectPage"];
  } = $props();
  const current = $derived(controller.state.route);
  const threadPanel = useThreadPanel();
  const detailsRail = untrack(() => shell.getPanel(repositoryDetailsPanelId)!);
  const threadLabel = $derived(
    threadPanel.kind === "settings"
      ? "Thread settings"
      : threadPanel.kind === "members"
        ? "Thread contributors"
        : "Thread replies",
  );
</script>

{#if current.page === "threads" && controller.threads?.detail}
  <AppShell.Sidebar
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label={threadLabel}
    resizeLabel="Resize thread details"
    closeable
  >
    <StoreThreadSidebar {controller} />
  </AppShell.Sidebar>
{/if}
<AppShell.Sidebar
  side="right"
  sidebarController={detailsRail}
  constraintPriority={100}
  variant="rail"
  class="community-application__project-overview-sidebar"
  label="Repository details"
  resizeLabel="Resize repository details"
  closeable
>
  <AppShell.Sidebar.Body>
    {#if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}
      <StoreProjectsRail {controller} />
    {:else}
      <StoreRepositoryDetails {controller} />
      <StoreRepositoryNavigation page={current.page} onSelect={onSelectPage} />
      <StoreRepositoryContextActions {controller} />
    {/if}
  </AppShell.Sidebar.Body>
</AppShell.Sidebar>

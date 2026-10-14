<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import { untrack } from "svelte";
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
  import StoreRepositoryNavigation from "./StoreRepositoryNavigation.svelte";
  import { repositoryRailSections } from "./repository-sections.js";
  import StoreRepositoryContextActions from "./StoreRepositoryContextActions.svelte";
  let {
    controller,
    stackWorkspace,
    shell,
    onSelectPage,
    visible = true,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    shell: AppShellController;
    visible?: boolean;
    onSelectPage: StoreController["actions"]["selectPage"];
  } = $props();
  const current = $derived(controller.state.route);
  const inspection = useConversationInspection();
  const detailsRail = untrack(() => shell.getPanel(repositoryDetailsPanelId)!);
</script>

{#if inspection?.file}<AppShell.Sidebar
    hidden={!visible}
    side="right"
    sidebarController={inspection.workspaceInspector}
    label="Workspace file"
    resizeLabel="Resize workspace file"
    constrainedPresentation="replace-main"
    closeable
  >
    <StoreLazyView load={views.workspaceInspector} label="Workspace file"
      >{#snippet content(View)}<View
          browser={inspection.file!}
          onClose={() => void inspection.close()}
        />{/snippet}</StoreLazyView
    >
  </AppShell.Sidebar>{/if}

{#if inspection?.current}<AppShell.Sidebar
    hidden={!visible}
    side="right"
    label="Read tool details"
    resizeLabel="Resize read tool details"
    constrainedPresentation="replace-main"
    closeable
  >
    <StoreLazyView load={views.toolPanel} label="Tool details"
      >{#snippet content(View)}<View
          tool={inspection.current!.tool}
          state={inspection.current!.state}
          onClose={() => void inspection.close()}
        />{/snippet}</StoreLazyView
    >
  </AppShell.Sidebar>{/if}

{#if inspection?.workspace}
  <StoreLazyView
    load={views.workspaceFiles}
    label="Workspace files"
    active={!inspection.workspaceRail.closed}
    >{#snippet content(View)}<View
        browser={inspection.workspace!}
        {visible}
        sidebarController={inspection.workspaceRail}
      />{/snippet}</StoreLazyView
  >
{:else if !current.agentsHub && current.page !== "threads"}<AppShell.Sidebar
    hidden={!visible}
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
        <StoreLazyView
          load={views.projectsRail}
          label="Project details"
          active={!detailsRail.closed}
          >{#snippet content(View)}<View
              {controller}
            />{/snippet}</StoreLazyView
        >
      {:else if isStacksRoute(current) && stackWorkspace?.snapshot?.repository === current.repository}
        <StoreLazyView
          load={views.stackDetails}
          label="Workspace details"
          active={!detailsRail.closed}
          >{#snippet content(View)}<View
              {controller}
              workspace={stackWorkspace}
            />{/snippet}</StoreLazyView
        >
      {:else}
        <StoreRepositoryDetails {controller} />
        {#if !isStacksRoute(current)}<StoreRepositoryNavigation
            page={current.page}
            sections={repositoryRailSections.filter(
              (item) =>
                item.page !== "actions" ||
                controller.state.repository?.profile !== "git_sha1_jj045",
            )}
            onSelect={onSelectPage}
          />
          <StoreRepositoryContextActions {controller} />{/if}
      {/if}
    </AppShell.Sidebar.Body>
  </AppShell.Sidebar>{/if}

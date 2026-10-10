<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import StoreStackWorkspacePicker from "../store-stack-workspace/StoreStackWorkspacePicker.svelte";
  import StoreStacksApplicationView from "../store-stack-workspace/StoreStacksApplicationView.svelte";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import StoreStackWorkspaceState from "../store-stack-workspace/StoreStackWorkspaceState.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { Snippet } from "svelte";
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  import type { StorePage } from "../community-navigation/model.js";
  import type { StoreSettingsSection } from "../store-settings/model.js";
  import StoreTerminalWorkspace from "../store-terminal/StoreTerminalWorkspace.svelte";
  import StoreSettings from "../store-settings/StoreSettings.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
  import StoreThreads from "../store-threads/StoreThreads.svelte";
  import StoreThreadsBreadcrumb from "../store-threads/StoreThreadsBreadcrumb.svelte";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreRepositoryTabs from "./StoreRepositoryTabs.svelte";
  import StoreWorkspaceSectionHeader from "./StoreWorkspaceSectionHeader.svelte";
  import StoreRepositoryNotices from "./StoreRepositoryNotices.svelte";
  import StoreApplicationPlaceholder from "./StoreApplicationPlaceholder.svelte";
  let {
    controller,
    stackWorkspace,
    shell,
    current,
    settingsOpen,
    settingsSection,
    settingsRepository,
    closeSettings,
    linkFolder,
    conflictOpen,
    workspaceBlocked,
    canNavigateBack,
    canNavigateForward,
    navigate,
    detailsRail,
    selectPage,
    children,
    agents,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    shell: AppShellController;
    current: StoreRoute;
    settingsOpen: boolean;
    settingsSection: StoreSettingsSection;
    settingsRepository: Snippet;
    closeSettings(): void;
    linkFolder(): void;
    conflictOpen: boolean;
    workspaceBlocked: boolean;
    canNavigateBack: boolean;
    canNavigateForward: boolean;
    navigate(delta: -1 | 1): void;
    detailsRail: AppShellController["right"];
    selectPage(page: StorePage): void;
    children?: Snippet<[StorePage, string]>;
    agents?: Snippet;
  } = $props();
  const focusedStacks = $derived(isStacksRoute(current));
  const stacksPreparing = $derived(
    controller.stackWorkspaceLoading ||
      controller.state.accessMode === "bootstrapping" ||
      controller.state.status.connection === "connecting" ||
      controller.state.status.navigation === "loading" ||
      controller.state.status.navigation === "refreshing" ||
      (controller.state.status.connection === "disconnected" &&
        controller.state.status.navigation === "idle"),
  );
  const initialStackFailure = $derived(
    !stackWorkspace?.snapshotLoaded &&
      !stackWorkspace?.loading &&
      controller.state.status.navigation === "failed",
  );
  const stacksAvailable = $derived(
    Boolean(
      stackWorkspace?.snapshot?.folder &&
      stackWorkspace.snapshot.repository === current.repository,
    ),
  );
</script>

<AppShell.Main>
  <StoreTerminalWorkspace
    repository={current.repository}
    profile={controller.state.repository.profile ?? "native"}
    enabled={!current.agentsHub &&
      current.page !== "threads" &&
      !focusedStacks &&
      !settingsOpen &&
      !conflictOpen &&
      !(
        current.page === "projects" &&
        controller.projects &&
        !isRepositoryProjects(current)
      )}
    onLink={() => {
      linkFolder();
    }}
    onMaximize={() => detailsRail.close()}
    onRefresh={() => {
      void controller.actions.refresh();
    }}
  >
    {#if settingsOpen}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <AppShell.Sidebar.Toggle side="left" sidebarName="settings" />
          {@render settingsRepository()}
        </AppShell.Toolbar>
      {/if}
      <StoreSettings
        {controller}
        section={settingsSection}
        onClose={closeSettings}
      />
    {:else if current.agentsHub}{@render agents?.()}
    {:else if current.page === "threads"}<StoreThreads {controller} />
    {:else}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />
          {#if focusedStacks && stackWorkspace?.snapshot?.repository === current.repository}<StoreStackWorkspacePicker
              controller={stackWorkspace}
            />{:else if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
              {controller}
            />{/if}
          <div class="store-application__details-toggle">
            <AppShell.Sidebar.Toggle
              side="right"
              sidebarController={detailsRail}
              sidebarName="details"
            />
          </div>
        </AppShell.Toolbar>
      {/if}
      <AppShell.Body
        layout="regions"
        label={current.page === "files"
          ? "Repository files workspace"
          : undefined}
        class={current.page === "files"
          ? "store-application__files-body"
          : undefined}
      >
        <div class="store-application__body-frame">
          {#if !(current.page === "projects" && controller.projects && !isRepositoryProjects(current)) && !focusedStacks}
            {#if !(current.page === "reviews" && current.reviewThreadId)}<StoreRepositoryTabs
                page={current.page}
                local={controller.state.repository.profile === "git_sha1_jj045"}
                stacks={stacksAvailable}
                onSelect={selectPage}
              />{/if}
            {#if current.page !== "reviews"}<StoreWorkspaceSectionHeader
                {controller}
                disabled={workspaceBlocked}
              />{/if}
            <StoreRepositoryNotices {controller} />{/if}
          {#if focusedStacks}<StoreRepositoryNotices {controller} />{/if}
          <div class="store-application__body-regions" data-page={current.page}>
            {#key current.repository + (focusedStacks ? "stacks" : current.page)}
              {#if focusedStacks}
                {#if stackWorkspace && (!stackWorkspace.snapshot || stackWorkspace.snapshot.repository === current.repository) && !initialStackFailure}
                  <StoreStacksApplicationView
                    {controller}
                    workspace={stackWorkspace}
                  >
                    {#snippet history()}
                      {#if children}{@render children(
                          current.page,
                          current.repository,
                        )}{:else}<StoreApplicationPlaceholder
                          repository={current.repository}
                          page={current.page}
                        />{/if}
                    {/snippet}
                  </StoreStacksApplicationView>
                {:else}
                  <StoreStackWorkspaceState
                    loading={Boolean(stacksPreparing)}
                    onRetry={() => controller.actions.refresh()}
                  />
                {/if}
              {:else if children}{@render children(
                  current.page,
                  current.repository,
                )}{:else}<StoreApplicationPlaceholder
                  repository={current.repository}
                  page={current.page}
                />{/if}
            {/key}
          </div>
        </div>
      </AppShell.Body>
    {/if}
  </StoreTerminalWorkspace>
</AppShell.Main>

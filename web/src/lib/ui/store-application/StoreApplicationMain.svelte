<script lang="ts">
  import StoreRepositorySettingsButton from "../store-settings/StoreRepositorySettingsButton.svelte";
  import StoreLazyView from "./StoreLazyView.svelte";
  import StoreGlobalSettingsLoading from "../store-settings/StoreGlobalSettingsLoading.svelte";
  import StoreRepositorySettingsLoading from "../store-settings/StoreRepositorySettingsLoading.svelte";
  import StorePageSkeleton from "./StorePageSkeleton.svelte";
  import { views } from "./application-views.js";
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import StoreStackWorkspacePicker from "../store-stack-workspace/StoreStackWorkspacePicker.svelte";
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
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
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
    openRepositorySettings,
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
    applicationMenu,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    shell: AppShellController;
    current: StoreRoute;
    settingsOpen: boolean;
    settingsSection: StoreSettingsSection;
    openRepositorySettings(): void;
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
    applicationMenu?: Snippet;
  } = $props();
  const repositorySettingsOpen = $derived(
    current.page === "settings" && !settingsOpen,
  );
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
      current.page !== "workflows" &&
      current.page !== "threads" &&
      !focusedStacks &&
      !settingsOpen &&
      current.page !== "settings" &&
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
    {#if shell.mobile.resolvedMode === "mobile"}
      <AppShell.Toolbar>
        {#if settingsOpen}
          <AppShell.Sidebar.Toggle side="left" sidebarName="settings" />
        {:else}
          <CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />
          {#if focusedStacks && stackWorkspace?.snapshot?.repository === current.repository}<StoreStackWorkspacePicker
              controller={stackWorkspace}
            />{:else if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
              {controller}
            />{/if}
        {/if}
        <div class="store-application__details-toggle">
          {#if !settingsOpen && !repositorySettingsOpen && !current.agentsHub && !["workflows", "threads"].includes(current.page)}<AppShell.Sidebar.Toggle
              side="right"
              sidebarController={detailsRail}
              sidebarName="details"
            />{/if}
          {@render applicationMenu?.()}
        </div>
      </AppShell.Toolbar>
    {/if}
    {#key current.repository}
      <div
        hidden={!repositorySettingsOpen}
        class="store-repository-settings-host"
      >
        <AppShell.Body layout="regions" label="Repository settings"
          ><div class="store-application__body-frame">
            <StoreLazyView
              load={views.repositorySettings}
              label="Repository settings"
              active={repositorySettingsOpen}
            >
              {#snippet loading()}<StoreRepositorySettingsLoading
                  section={current.repositorySettingsSection}
                  tab={current.repositorySettingsTab}
                  local={controller.state.repository.profile ===
                    "git_sha1_jj045"}
                />{/snippet}
              {#snippet content(View)}<View
                  {controller}
                  visible={repositorySettingsOpen}
                />{/snippet}
            </StoreLazyView>
          </div></AppShell.Body
        >
      </div>
    {/key}
    {#if settingsOpen}
      <StoreLazyView load={views.settings} label="Settings"
        >{#snippet loading()}<StoreGlobalSettingsLoading
            section={settingsSection}
            tab={current.globalSettings?.tab}
            onSelect={(tab) =>
              controller.actions.selectGlobalSettings(settingsSection, tab)}
          />{/snippet}{#snippet content(View)}<View
            {controller}
            section={settingsSection}
            onClose={closeSettings}
          />{/snippet}</StoreLazyView
      >
    {:else if repositorySettingsOpen}
      <!-- Repository settings is retained in the sibling body above. -->
    {:else if current.agentsHub}{@render agents?.()}
    {:else if current.page === "workflows"}<AppShell.Body
        layout="regions"
        label="Workflows"
        ><StoreLazyView load={views.workflows} label="Workflows"
          >{#snippet loading()}<StorePageSkeleton
              route={current}
              label="Workflows"
            />{/snippet}{#snippet content(View)}<View
              {controller}
            />{/snippet}</StoreLazyView
        ></AppShell.Body
      >
    {:else if current.page === "threads"}<StoreLazyView
        load={views.threads}
        label="Threads"
        >{#snippet loading()}<StorePageSkeleton
            route={current}
            label="Threads"
            surface="threads"
          />{/snippet}{#snippet content(View)}<View
            {controller}
          />{/snippet}</StoreLazyView
      >
    {:else}
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
                >{#snippet actions()}<div
                    class="store-repository-settings-action"
                  >
                    <StoreRepositorySettingsButton
                      disabled={workspaceBlocked}
                      onOpen={openRepositorySettings}
                    />
                  </div>{/snippet}</StoreRepositoryTabs
              >{/if}
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
                  <StoreLazyView load={views.stacks} label="Workspace"
                    >{#snippet loading()}<StoreStackWorkspaceState
                        loading
                      />{/snippet}{#snippet content(View)}<View
                        {controller}
                        workspace={stackWorkspace}
                        onRepositorySettings={openRepositorySettings}
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
                      </View>{/snippet}</StoreLazyView
                  >
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

<style>
  .store-repository-settings-host[hidden] {
    display: none;
  }
  .store-repository-settings-host {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    flex: 1;
    overflow: hidden;
  }
  .store-repository-settings-action {
    margin-inline-start: auto;
  }
</style>

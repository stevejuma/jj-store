<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import StoreStackWorkspacePicker from "../store-stack-workspace/StoreStackWorkspacePicker.svelte";
  import StoreStackWorkspace from "../store-stack-workspace/StoreStackWorkspace.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
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
  } = $props();
</script>

<AppShell.Main>
  <StoreTerminalWorkspace
    repository={current.repository}
    profile={controller.state.repository.profile ?? "native"}
    enabled={current.page !== "stacks" &&
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
    {:else}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />
          {#if current.page === "stacks" && stackWorkspace?.snapshot?.repository === current.repository}<StoreStackWorkspacePicker
              controller={stackWorkspace}
            />{:else if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
              {controller}
            />{:else if current.page === "threads"}<StoreThreadsBreadcrumb
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
          {#if !(current.page === "projects" && controller.projects && !isRepositoryProjects(current)) && current.page !== "threads" && current.page !== "stacks"}
            {#if !(current.page === "reviews" && current.reviewThreadId)}<StoreRepositoryTabs
                page={current.page}
                local={controller.state.repository.profile === "git_sha1_jj045"}
                onSelect={selectPage}
              />{/if}
            {#if current.page !== "reviews"}<StoreWorkspaceSectionHeader
                {controller}
                disabled={workspaceBlocked}
              />{/if}
            <StoreRepositoryNotices {controller} />{/if}
          {#if current.page === "stacks"}<StoreRepositoryNotices
              {controller}
            />{/if}
          <div class="store-application__body-regions" data-page={current.page}>
            {#key current.repository + current.page}
              {#if current.page === "stacks"}
                {#if stackWorkspace?.snapshot?.repository === current.repository}
                  <StoreStackWorkspace
                    controller={stackWorkspace}
                    showPicker={false}
                  />
                {:else}
                  <CommunityProjectsColumnAlert
                    title="Local stack workspace unavailable"
                    description="Stacks require a linked local repository and a workspace controller."
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

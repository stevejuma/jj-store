<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import { AppShell } from "@lapismd/design-core/shell";
  import StoreApplicationRightPanels from "./StoreApplicationRightPanels.svelte";
  import { createStoreShell } from "./create-store-shell.js";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  import { provideStoreEntityContext } from "./entity-context.js";
  import { observeRepositoryThreadNavigation } from "../store-threads/thread-navigation-binding.svelte.js";
  import {
    selectStoreNavigation,
    activeStoreNavigation,
  } from "./store-navigation.js";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
  import StoreThreadsBreadcrumb from "../store-threads/StoreThreadsBreadcrumb.svelte";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreThreadFocus from "../store-threads/StoreThreadFocus.svelte";
  import { createApplicationThreadPanel as ownThreadPanel } from "../store-threads/thread-panel.svelte.js";
  import StoreRepositoryTabs from "./StoreRepositoryTabs.svelte";
  import { repositorySections } from "./repository-sections.js";
  import { onDestroy, setContext, untrack } from "svelte";
  import type { StoreApplicationProps } from "./application-props.js";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import "./store-application.css";
  import { workspaceNavigationKey } from "./workspace-navigation.js";
  import StoreWorkspaceSectionHeader from "./StoreWorkspaceSectionHeader.svelte";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import StoreApplicationNavigation from "./StoreApplicationNavigation.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
  } from "../community-navigation/model.js";
  import CommunityProfileMenu from "../community-profile-menu/CommunityProfileMenu.svelte";
  import StoreRepositoryNotices from "./StoreRepositoryNotices.svelte";
  import StoreSearchOverlay from "../store-search/StoreSearchOverlay.svelte";
  import StoreSettings from "../store-settings/StoreSettings.svelte";
  import StoreSettingsRepository from "../store-settings/StoreSettingsRepository.svelte";
  import StoreSettingsNavigation from "../store-settings/StoreSettingsNavigation.svelte";
  import type { StoreSettingsSection } from "../store-settings/model.js";
  import StoreApplicationHeaderActions from "./StoreApplicationHeaderActions.svelte";
  import { provideDiscussionFocus } from "../store-discussion/discussion-focus.svelte.js";
  import StoreFocusedDiscussion from "../store-discussion/StoreFocusedDiscussion.svelte";
  import StoreConflictEditor from "../store-conflicts/StoreConflictEditor.svelte";
  import { conflictEditorKey } from "../store-conflicts/conflict-editor.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import {
    loadNavigationLayout,
    saveNavigationLayout,
  } from "../navigation-storage.js";
  let {
    controller,
    initiallyCollapsed = true,
    displayMode = "auto",
    layoutStorage,
    desktopChrome = untrack(isMacDesktopWindow),
    children,
  }: StoreApplicationProps = $props();
  import { ownApplicationAppearance as ownAppearance } from "../../appearance/runtime.svelte.js";
  const appearance = ownAppearance(
    () => controller.state.route.repository,
    untrack(() => layoutStorage),
  );
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const navigationLayout = untrack(() => loadNavigationLayout(storage));
  let sectionOpen = $state(navigationLayout.sections);
  let itemOpen = $state(navigationLayout.items);
  $effect(() =>
    saveNavigationLayout(storage, { sections: sectionOpen, items: itemOpen }),
  );
  const shell = createStoreShell(
    storage,
    untrack(() => initiallyCollapsed),
  );
  onDestroy(() => void shell.dispose());
  const detailsRail = shell.getPanel(repositoryDetailsPanelId)!;
  const threadPanel = ownThreadPanel(() => controller, shell, appearance);
  provideDiscussionFocus();
  const workspaceGuard = $state({ editing: false });
  setContext(workspaceNavigationKey, workspaceGuard);
  let conflictOpen = $state(false);
  $effect(() => {
    controller.actions.setRefreshPaused?.(
      "code-editor",
      conflictOpen || workspaceGuard.editing,
    );
    return () => controller.actions.setRefreshPaused?.("code-editor", false);
  });
  let conflictReturnFocus = $state<HTMLElement | null>(null);
  setContext(conflictEditorKey, () => {
    conflictReturnFocus =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;
    conflictOpen = true;
  });
  const workspaceBlocked = $derived(
    conflictOpen ||
      workspaceGuard.editing ||
      ["previewing", "prepared", "applying", "unknown"].includes(
        controller.state.mutation.status,
      ),
  );
  provideStoreEntityContext(
    () => controller,
    () => workspaceBlocked,
    storage,
  );
  const current = $derived(controller.state.route);
  let searchOpen = $state(false);
  let settingsOpen = $state(false);
  $effect(() => {
    controller.actions.setRefreshPaused?.("settings", settingsOpen);
    return () => controller.actions.setRefreshPaused?.("settings", false);
  });
  let settingsSection = $state<StoreSettingsSection>("service");
  const canNavigateBack = $derived(controller.state.canNavigateBack);
  const canNavigateForward = $derived(controller.state.canNavigateForward);
  const compact = $derived(
    shell.mobile.resolvedMode === "desktop" &&
      shell.left.collapsed &&
      !shell.left.previewed,
  );
  const activeNavigationItem = $derived(activeStoreNavigation(current));
  observeRepositoryThreadNavigation(() => controller);
  const title = $derived(
    repositorySections.find(({ page }) => page === current.page)?.label,
  );
  function selectPage(page: StorePage) {
    if (conflictOpen) return;
    controller.actions.selectPage(page);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function select(item: CommunityNavigationItemModel) {
    if (conflictOpen) return;
    selectStoreNavigation(controller, item);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function navigate(delta: -1 | 1) {
    if (conflictOpen) return;
    controller.actions.navigate(delta);
  }
  function openSettings() {
    if (conflictOpen) return;
    settingsOpen = true;
    void controller.actions.refreshSettings();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function selectSettings(section: StoreSettingsSection) {
    settingsSection = section;
    if (section !== "appearance") void controller.actions.refreshSettings();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function closeSettings() {
    settingsOpen = false;
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
</script>

{#snippet settingsRepository()}
  <StoreSettingsRepository
    {controller}
    section={settingsSection}
    disabled={workspaceBlocked}
  />
{/snippet}

<CommunityAppShell
  controller={shell}
  {displayMode}
  {desktopChrome}
  canNavigateBack={canNavigateBack && !conflictOpen}
  canNavigateForward={canNavigateForward && !conflictOpen}
  onNavigateBack={() => navigate(-1)}
  onNavigateForward={() => navigate(1)}
>
  {#snippet headerCenter()}{#if settingsOpen}{@render settingsRepository()}{:else if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
        {controller}
      />{:else if !settingsOpen && current.page === "threads"}<StoreThreadsBreadcrumb
        {controller}
      />{/if}{/snippet}
  {#snippet headerEnd()}{#if !settingsOpen}<StoreApplicationHeaderActions
        {controller}
        disabled={conflictOpen}
        detailsOpen={!detailsRail.closed}
        toggleDetails={() =>
          detailsRail.closed ? detailsRail.open() : detailsRail.close()}
      />{/if}{/snippet}
  <AppShell.Sidebar
    side="left"
    inert={conflictOpen}
    label={settingsOpen ? "Settings sidebar" : "Repository navigation sidebar"}
    mobileLabel={settingsOpen ? "Settings" : "Repositories"}
    resizeLabel="Resize repository navigation sidebar"
    closeable
    revealOnEdgeHover
    edgeRevealLabel="Preview repository navigation sidebar"
  >
    {#if settingsOpen}<StoreSettingsNavigation
        section={settingsSection}
        {compact}
        onSelect={selectSettings}
        onClose={closeSettings}
      />{:else}<StoreApplicationNavigation
        {controller}
        bind:sectionOpen
        bind:itemOpen
        activeItemId={activeNavigationItem}
        {compact}
        onSelect={select}
        onSearch={() => (searchOpen = true)}
      >
        {#snippet footer()}<CommunityProfileMenu
            member={controller.state.member}
            authenticated={controller.state.authenticated}
            {compact}
            onAction={(action) => {
              if (action === "settings") openSettings();
              else if (action === "sign-in") controller.actions.signIn();
              else controller.actions.signOut();
            }}
          />{/snippet}
      </StoreApplicationNavigation>{/if}
  </AppShell.Sidebar>
  <AppShell.Main>
    {#if settingsOpen}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <AppShell.Sidebar.Toggle side="left" sidebarName="settings" />
          {@render settingsRepository()}
        </AppShell.Toolbar>
      {/if}
      <StoreSettings {controller} section={settingsSection} />
    {:else}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />
          {#if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
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
          {#if !(current.page === "projects" && controller.projects && !isRepositoryProjects(current)) && current.page !== "threads"}
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
          <div class="store-application__body-regions" data-page={current.page}>
            {#key current.repository + current.page}
              {#if children}{@render children(
                  current.page,
                  current.repository,
                )}{:else}<p class="store-application__placeholder">
                  {current.repository} · {title ?? "Repository"}
                </p>{/if}
            {/key}
          </div>
        </div>
      </AppShell.Body>
    {/if}
  </AppShell.Main>
  {#if !settingsOpen}<StoreApplicationRightPanels
      {controller}
      {shell}
      onSelectPage={selectPage}
      threadFocused={threadPanel.focused}
    />{/if}
  <StoreFocusedDiscussion
    open={!settingsOpen && !conflictOpen && !threadPanel.focused}
  />
  <StoreThreadFocus
    open={!settingsOpen && !conflictOpen && threadPanel.focused}
    {controller}
  />
  <StoreConflictEditor
    {controller}
    open={conflictOpen}
    returnFocus={conflictReturnFocus}
    onClose={() => (conflictOpen = false)}
  />
</CommunityAppShell>
{#if !settingsOpen}<StoreSearchOverlay
    {controller}
    bind:open={searchOpen}
  />{/if}

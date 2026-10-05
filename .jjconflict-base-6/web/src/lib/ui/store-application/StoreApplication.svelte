<script lang="ts">
  import { provideTerminalControls } from "../../terminal/controls.svelte.js";
  provideTerminalControls();
  import StoreIndexPreparationNotice from "./StoreIndexPreparationNotice.svelte";
  import StoreApplicationMain from "./StoreApplicationMain.svelte";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import StoreHelp from "../store-help/StoreHelp.svelte";
  import { ownApplicationNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
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
  import StoreStackWorkspacePicker from "../store-stack-workspace/StoreStackWorkspacePicker.svelte";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
  import StoreThreadsBreadcrumb from "../store-threads/StoreThreadsBreadcrumb.svelte";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreThreadFocus from "../store-threads/StoreThreadFocus.svelte";
  import { createApplicationThreadPanel as ownThreadPanel } from "../store-threads/thread-panel.svelte.js";
  import { onDestroy, setContext, untrack } from "svelte";
  import type { StoreApplicationProps } from "./application-props.js";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import "./store-application.css";
  import { workspaceNavigationKey } from "./workspace-navigation.js";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import StoreApplicationNavigation from "./StoreApplicationNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
  } from "../community-navigation/model.js";
  import CommunityProfileMenu from "../community-profile-menu/CommunityProfileMenu.svelte";
  import StoreSearchOverlay from "../store-search/StoreSearchOverlay.svelte";
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
    stackWorkspace,
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
  const layout = $state(untrack(() => loadNavigationLayout(storage)));
  $effect(() => saveNavigationLayout(storage, layout));
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
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function selectSettings(section: StoreSettingsSection) {
    settingsSection = section;
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function closeSettings() {
    settingsOpen = false;
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  const notifications = ownApplicationNotifications(() => controller);
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
  {#snippet headerCenter()}{#if settingsOpen}{@render settingsRepository()}{:else if isStacksRoute(current) && stackWorkspace?.snapshot?.repository === current.repository}<StoreStackWorkspacePicker
        controller={stackWorkspace}
      />{:else if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
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
        bind:sectionOpen={layout.sections}
        bind:itemOpen={layout.items}
        activeItemId={activeNavigationItem}
        {compact}
        onSelect={select}
        onSearch={() => (searchOpen = true)}
      >
        {#snippet footer()}<StoreHelp {compact} sidebar /><CommunityProfileMenu
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
  <StoreApplicationMain
    {controller}
    {stackWorkspace}
    {shell}
    {current}
    {settingsOpen}
    {settingsSection}
    {settingsRepository}
    {closeSettings}
    {conflictOpen}
    {workspaceBlocked}
    {canNavigateBack}
    {canNavigateForward}
    {navigate}
    {detailsRail}
    {selectPage}
    {children}
    linkFolder={() => {
      openSettings();
      selectSettings("repository");
    }}
  />
  {#if !settingsOpen}<StoreApplicationRightPanels
      {controller}
      {stackWorkspace}
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

<StoreIndexPreparationNotice {controller} />
<StoreNotificationHost owner={notifications} />

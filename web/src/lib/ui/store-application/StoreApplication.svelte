<script lang="ts">
  import { ownRepositorySettingsGuard } from "../store-settings/repository-settings-guard.svelte.js";
  import StoreApplicationHeaderCenter from "./StoreApplicationHeaderCenter.svelte";
  import StoreApplicationAgents from "./StoreApplicationAgents.svelte";
  import StoreApplicationOverlays from "./StoreApplicationOverlays.svelte";
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import { provideConversationInspection } from "../../conversations/inspection.svelte.js";
  import StoreIndexPreparationNotice from "./StoreIndexPreparationNotice.svelte";
  import StoreApplicationMain from "./StoreApplicationMain.svelte";
  import StoreConnectionFooter from "../store-connections/StoreConnectionFooter.svelte";
  import {
    ownConnectionNavigation,
    connectionSettingsNavigation,
  } from "../store-connections/navigation.svelte.js";
  import { ownApplicationNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import StoreAgentExecutionNotice from "../store-agent-execution/StoreAgentExecutionNotice.svelte";
  import { provideAgentExecution } from "../store-agent-execution/context.js";
  import { observeAgentChats } from "../../controller/agent-chat-navigation.svelte.js";
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
  import StoreThreadFocus from "../store-threads/StoreThreadFocus.svelte";
  import { createApplicationThreadPanel as ownThreadPanel } from "../store-threads/thread-panel.svelte.js";
  import { onDestroy, setContext, untrack } from "svelte";
  import type { StoreApplicationProps } from "./application-props.js";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import "./store-application.css";
  import "./store-application-settings.css";
  import { workspaceNavigationKey } from "./workspace-navigation.js";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import StoreApplicationNavigation from "./StoreApplicationNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
  } from "../community-navigation/model.js";
  import CommunityProfileMenu from "../community-profile-menu/CommunityProfileMenu.svelte";
  import StoreRuntimeReload from "../store-settings/StoreRuntimeReload.svelte";
  import StoreSettingsNavigation from "../store-settings/StoreSettingsNavigation.svelte";
  import StoreApplicationHeaderActions from "./StoreApplicationHeaderActions.svelte";
  import StoreWorkspaceToggle from "../store-agent-workspace/StoreWorkspaceToggle.svelte";
  import { provideDiscussionFocus } from "../store-discussion/discussion-focus.svelte.js";
  import StoreFocusedDiscussion from "../store-discussion/StoreFocusedDiscussion.svelte";
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
  provideConversationInspection(shell, () => {
    if (controller.execution?.chat.open)
      return `chat:${controller.execution.chat.selected}`;
    const route = controller.state.route;
    if (route.page === "threads" && route.threadId && controller.threads)
      return `${controller.threads.conversationScope}:${route.threadId}:main`;
    return route.agentsHub && route.agentView === "session"
      ? (route.executionSessionId ?? null)
      : null;
  });
  const threadPanel = ownThreadPanel(() => controller, shell, appearance);
  provideDiscussionFocus();
  const workspaceGuard = $state({ editing: false });
  setContext(workspaceNavigationKey, workspaceGuard);
  let conflictOpen = $state(false);
  const editorOpen = $derived(
    conflictOpen || Boolean(stackWorkspace?.conflictOpen),
  );
  $effect(() => {
    controller.actions.setRefreshPaused?.(
      "code-editor",
      editorOpen || workspaceGuard.editing,
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
    editorOpen ||
      workspaceGuard.editing ||
      ["previewing", "prepared", "applying", "unknown"].includes(
        controller.state.mutation.status,
      ),
  );
  const { openSettings, selectSettings, closeSettings } =
    connectionSettingsNavigation(
      () => controller,
      () => editorOpen,
      shell,
    );
  const profileAction = ownConnectionNavigation(
    () => controller,
    () => workspaceBlocked,
    openSettings,
  );
  provideStoreEntityContext(
    () => controller,
    () => workspaceBlocked,
    storage,
  );
  const current = $derived(controller.state.route);
  const settingsGuard = ownRepositorySettingsGuard({
    controller: () => controller,
    blocked: () => workspaceBlocked,
    stacksLocked: () => Boolean(stackWorkspace?.locked),
    showMain: () => {
      if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    },
  });
  observeAgentChats(() => controller);
  let searchOpen = $state(false);
  const settingsOpen = $derived(Boolean(current.globalSettings));
  const repositorySettingsOpen = $derived(current.page === "settings");
  $effect(() => {
    controller.actions.setRefreshPaused?.(
      "settings",
      settingsOpen || repositorySettingsOpen,
    );
    return () => controller.actions.setRefreshPaused?.("settings", false);
  });
  const compact = $derived(
    shell.mobile.resolvedMode === "desktop" &&
      shell.left.collapsed &&
      !shell.left.previewed,
  );
  const activeNavigationItem = $derived(activeStoreNavigation(current));
  observeRepositoryThreadNavigation(() => controller);
  function selectPage(page: StorePage) {
    if (editorOpen) return;
    controller.actions.selectPage(page);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function select(item: CommunityNavigationItemModel) {
    if (editorOpen) return;
    if (!settingsGuard.allow(item)) return;
    selectStoreNavigation(controller, item);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function navigate(delta: -1 | 1) {
    if (editorOpen) return;
    controller.actions.navigate(delta);
  }
  const notifications = ownApplicationNotifications(() => controller);
  provideAgentExecution(() => controller.execution);
</script>

{#snippet agents()}<StoreApplicationAgents {controller} {current} />{/snippet}

<CommunityAppShell
  controller={shell}
  {displayMode}
  {desktopChrome}
  canNavigateBack={controller.state.canNavigateBack && !editorOpen}
  canNavigateForward={controller.state.canNavigateForward && !editorOpen}
  onNavigateBack={() => navigate(-1)}
  onNavigateForward={() => navigate(1)}
>
  {#snippet headerCenter()}<StoreApplicationHeaderCenter
      {controller}
      {stackWorkspace}
      {current}
      {editorOpen}
      hidden={settingsOpen || repositorySettingsOpen}
    />{/snippet}
  {#snippet headerEnd()}{#if !repositorySettingsOpen && !settingsOpen && ((current.agentsHub && (current.agentView === "session" || current.agentView === "sessions")) || (current.page === "threads" && current.threadId))}<StoreWorkspaceToggle
      />{:else if !repositorySettingsOpen && !settingsOpen && (!current.agentsHub || controller.execution?.chat.open)}<StoreApplicationHeaderActions
        {controller}
        disabled={editorOpen}
        detailsOpen={!detailsRail.closed}
        toggleDetails={() =>
          detailsRail.closed ? detailsRail.open() : detailsRail.close()}
      />{/if}{/snippet}
  {#snippet headerMenu()}<StoreRuntimeReload
      manager={controller.reload}
      menu
      disabled={editorOpen}
    />{/snippet}
  <AppShell.Sidebar
    side="left"
    inert={editorOpen}
    label={settingsOpen ? "Settings sidebar" : "Repository navigation sidebar"}
    mobileLabel={settingsOpen ? "Settings" : "Repositories"}
    resizeLabel="Resize repository navigation sidebar"
    closeable
    revealOnEdgeHover
    edgeRevealLabel="Preview repository navigation sidebar"
  >
    {#if settingsOpen}<StoreSettingsNavigation
        installationManagement={controller.state.accessMode === "local_trusted"}
        section={current.globalSettings?.section ?? "service"}
        {compact}
        onSelect={selectSettings}
        onClose={closeSettings}
      /><StoreConnectionFooter
        {compact}
        showVersion
      />{:else}<StoreApplicationNavigation
        {controller}
        bind:sectionOpen={layout.sections}
        bind:itemOpen={layout.items}
        activeItemId={activeNavigationItem}
        {compact}
        onSelect={select}
        onSearch={() => (searchOpen = true)}
      >
        {#snippet footer()}<StoreConnectionFooter
            {compact}
          /><CommunityProfileMenu
            member={controller.state.member}
            authenticated={controller.state.authenticated}
            {compact}
            onAction={profileAction}
          />{/snippet}
      </StoreApplicationNavigation>{/if}
  </AppShell.Sidebar>
  <StoreApplicationMain
    {controller}
    {stackWorkspace}
    {shell}
    {current}
    {settingsOpen}
    settingsSection={current.globalSettings?.section ?? "service"}
    openRepositorySettings={() => settingsGuard.open()}
    {closeSettings}
    conflictOpen={editorOpen}
    {workspaceBlocked}
    canNavigateBack={controller.state.canNavigateBack}
    canNavigateForward={controller.state.canNavigateForward}
    {navigate}
    {detailsRail}
    {selectPage}
    {children}
    {agents}
    linkFolder={() => settingsGuard.open("folders")}
  />
  {#if !settingsOpen && current.agentsHub && controller.execution && !controller.execution.chat.open}<StoreLazyView
      load={views.agentDetails}
      label="Agent details"
      active={Boolean(controller.execution.detailsId)}
      >{#snippet content(View)}<View
          manager={controller.execution!}
          {shell}
          route={current}
        />{/snippet}</StoreLazyView
    >{/if}
  <StoreApplicationRightPanels
    {controller}
    {stackWorkspace}
    {shell}
    onSelectPage={selectPage}
    threadFocused={threadPanel.focused}
    visible={!settingsOpen && !repositorySettingsOpen}
  />
  <StoreFocusedDiscussion
    open={!settingsOpen &&
      !repositorySettingsOpen &&
      !editorOpen &&
      !threadPanel.focused}
  />
  <StoreThreadFocus
    open={!settingsOpen &&
      !repositorySettingsOpen &&
      !editorOpen &&
      threadPanel.focused}
    {controller}
  />
  <StoreApplicationOverlays
    {controller}
    {stackWorkspace}
    {current}
    {settingsOpen}
    bind:conflictOpen
    bind:searchOpen
    {conflictReturnFocus}
    settingsNavigationNotice={settingsGuard.notice}
  />
</CommunityAppShell>

<StoreIndexPreparationNotice {controller} />
<StoreNotificationHost owner={notifications} />
{#if controller.execution}<StoreAgentExecutionNotice
    manager={controller.execution!}
    activeSession={current.agentsHub && current.agentView === "session"
      ? current.executionSessionId
      : undefined}
  />{/if}

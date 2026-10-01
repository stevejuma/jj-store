<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    createLocalStorageAppShellLayoutPersistence,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import {
    projectsSidebarModel,
    projectSidebarId,
  } from "../store-work/projects-sidebar-model.js";
  import { selectStoreNavigation } from "./store-navigation.js";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreProjectsRail from "../store-work/StoreProjectsRail.svelte";
  import StoreRepositoryTabs from "./StoreRepositoryTabs.svelte";
  import StoreRepositoryNavigation from "./StoreRepositoryNavigation.svelte";
  import { repositorySections } from "./repository-sections.js";
  import { onDestroy, setContext, untrack, type Snippet } from "svelte";
  import "./store-application.css";
  import { workspaceNavigationKey } from "./workspace-navigation.js";
  import StoreWorkspaceSectionHeader from "./StoreWorkspaceSectionHeader.svelte";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import CommunityNavigation from "../community-navigation/CommunityNavigation.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
  } from "../community-navigation/model.js";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProfileMenu from "../community-profile-menu/CommunityProfileMenu.svelte";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
  import StoreRepositoryContextActions from "./StoreRepositoryContextActions.svelte";
  import StoreRepositoryNotices from "./StoreRepositoryNotices.svelte";
  import StoreSearchOverlay from "../store-search/StoreSearchOverlay.svelte";
  import StoreSettings from "../store-settings/StoreSettings.svelte";
  import StoreSettingsNavigation from "../store-settings/StoreSettingsNavigation.svelte";
  import type { StoreSettingsSection } from "../store-settings/model.js";
  import StoreApplicationHeaderActions from "./StoreApplicationHeaderActions.svelte";
  import StoreConflictEditor from "../store-conflicts/StoreConflictEditor.svelte";
  import { conflictEditorKey } from "../store-conflicts/conflict-editor.js";
  import {
    loadRepositoryShellLayout,
    repositoryShellStorageKey,
    resolveLayoutStorage,
  } from "../layout-storage.js";
  import {
    loadNavigationLayout,
    saveNavigationLayout,
  } from "../navigation-storage.js";
  let {
    controller,
    initiallyCollapsed = true,
    displayMode = "auto",
    layoutStorage,
    children,
  }: {
    controller: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    layoutStorage?: Storage;
    children?: Snippet<[StorePage, string]>;
  } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const navigationLayout = untrack(() => loadNavigationLayout(storage));
  let sectionOpen = $state(navigationLayout.sections);
  let itemOpen = $state(navigationLayout.items);
  $effect(() =>
    saveNavigationLayout(storage, { sections: sectionOpen, items: itemOpen }),
  );
  const initialLayout = untrack(() => loadRepositoryShellLayout(storage));
  const shell = new AppShellController({
    leftCollapsed:
      initialLayout.left?.collapsed ?? untrack(() => initiallyCollapsed),
    leftClosed: initialLayout.left?.closed ?? false,
    ...(initialLayout.left?.width === undefined
      ? {}
      : { leftWidth: initialLayout.left.width }),
    rightCollapsed: initialLayout.right?.collapsed ?? false,
    rightClosed: initialLayout.right?.closed ?? true,
    rightWidth: initialLayout.right?.width ?? 320,
    sidebarMinWidth: 48,
    sidebarMaxWidth: 440,
    ...(storage
      ? {
          persistence: createLocalStorageAppShellLayoutPersistence(
            repositoryShellStorageKey,
            storage,
          ),
        }
      : {}),
  });
  onDestroy(() => void shell.dispose());
  const workspaceGuard = $state({ editing: false });
  setContext(workspaceNavigationKey, workspaceGuard);
  let conflictOpen = $state(false);
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
  const current = $derived(controller.state.route);
  let searchOpen = $state(false);
  let settingsOpen = $state(false);
  let settingsSection = $state<StoreSettingsSection>("service");
  const canNavigateBack = $derived(controller.state.canNavigateBack);
  const canNavigateForward = $derived(controller.state.canNavigateForward);
  const compact = $derived(
    shell.mobile.resolvedMode === "desktop" &&
      shell.left.collapsed &&
      !shell.left.previewed,
  );
  const navigation = $derived(
    projectsSidebarModel(
      controller.projects?.navigation.homes ?? [],
      controller.state.repositories,
      !!controller.projects,
    ),
  );
  const activeNavigationItem = $derived(
    current.page === "projects" && !isRepositoryProjects(current)
      ? current.projectId
        ? projectSidebarId(current.repository, current.projectId)
        : "projects"
      : current.repository,
  );
  const title = $derived(
    repositorySections.find((section) => section.page === current.page)
      ?.label ?? "Repository",
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
    if (section === "search") void controller.actions.refreshSettings();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function closeSettings() {
    settingsOpen = false;
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
</script>

<CommunityAppShell
  controller={shell}
  {displayMode}
  canNavigateBack={canNavigateBack && !conflictOpen}
  canNavigateForward={canNavigateForward && !conflictOpen}
  onNavigateBack={() => navigate(-1)}
  onNavigateForward={() => navigate(1)}
>
  {#snippet headerCenter()}{#if !settingsOpen && current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
        {controller}
      />{/if}{/snippet}
  {#snippet headerEnd()}{#if !settingsOpen}<StoreApplicationHeaderActions
        {controller}
        disabled={conflictOpen}
        detailsOpen={!shell.right.closed}
        toggleDetails={() =>
          shell.right.closed ? shell.right.open() : shell.right.close()}
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
      />{:else}<CommunityNavigation
        model={navigation}
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
      </CommunityNavigation>{/if}
  </AppShell.Sidebar>
  <AppShell.Main>
    {#if settingsOpen}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          <AppShell.Sidebar.Toggle side="left" sidebarName="settings" />
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
          {#if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
              {controller}
            />{/if}
          <div
            style="margin-inline-start: auto; display: flex; flex: 0 0 auto;"
          >
            <AppShell.Sidebar.Toggle side="right" sidebarName="details" />
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
          {#if !(current.page === "projects" && controller.projects && !isRepositoryProjects(current))}
            {#if !(current.page === "reviews" && current.reviewThreadId)}<StoreRepositoryTabs
                page={current.page}
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
                  {current.repository} · {title}
                </p>{/if}
            {/key}
          </div>
        </div>
      </AppShell.Body>
    {/if}
  </AppShell.Main>
  {#if !settingsOpen}<AppShell.Sidebar
      side="right"
      variant="rail"
      class="community-application__project-overview-sidebar"
      label="Repository details"
      resizeLabel="Resize repository details"
      closeable
    >
      <AppShell.Sidebar.Body>
        {#if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsRail
            {controller}
          />{:else}<StoreRepositoryDetails {controller} />
          <StoreRepositoryNavigation
            page={current.page}
            onSelect={selectPage}
          />
          <StoreRepositoryContextActions {controller} />{/if}
      </AppShell.Sidebar.Body>
    </AppShell.Sidebar>{/if}
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

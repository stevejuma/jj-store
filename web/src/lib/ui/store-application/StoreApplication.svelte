<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    createLocalStorageAppShellLayoutPersistence,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import StoreRepositoryTabs from "./StoreRepositoryTabs.svelte";
  import StoreRepositoryNavigation from "./StoreRepositoryNavigation.svelte";
  import { repositorySections } from "./repository-sections.js";
  import { onDestroy, setContext, untrack, type Snippet } from "svelte";
  import "./store-application.css";
  import { workspaceNavigationKey } from "./workspace-navigation.js";
  import StoreWorkspacePicker from "./StoreWorkspacePicker.svelte";
  import StoreComparisonParentPicker from "./StoreComparisonParentPicker.svelte";
  import PanelsTopLeftIcon from "@lucide/svelte/icons/panels-top-left";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import CommunityNavigation from "../community-navigation/CommunityNavigation.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
    CommunityNavigationModel,
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
  import {
    loadRepositoryShellLayout,
    repositoryShellStorageKey,
    resolveLayoutStorage,
  } from "../layout-storage.js";
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
  const workspaceBlocked = $derived(
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
  const navigation: CommunityNavigationModel = $derived({
    primaryItems: [],
    sections: [
      {
        id: "repositories",
        label: "Repositories",
        items: controller.state.repositories.map((repository) => ({
          id: repository.id,
          label: repository.name,
          kind: "repository" as const,
        })),
      },
    ],
  });
  const title = $derived(
    repositorySections.find((section) => section.page === current.page)
      ?.label ?? "Repository",
  );
  function selectPage(page: StorePage) {
    controller.actions.selectPage(page);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function select(item: CommunityNavigationItemModel) {
    if (item.kind === "repository") controller.actions.openRepository(item.id);
    else controller.actions.selectPage(item.kind);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function navigate(delta: -1 | 1) {
    controller.actions.navigate(delta);
  }
  function openSettings() {
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
  {canNavigateBack}
  {canNavigateForward}
  onNavigateBack={() => navigate(-1)}
  onNavigateForward={() => navigate(1)}
>
  {#snippet headerEnd()}{#if !settingsOpen}<StoreApplicationHeaderActions
        {controller}
        detailsOpen={!shell.right.closed}
        toggleDetails={() =>
          shell.right.closed ? shell.right.open() : shell.right.close()}
      />{/if}{/snippet}
  <AppShell.Sidebar
    side="left"
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
        activeItemId={current.repository}
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
      <StoreSettings {controller} section={settingsSection} />
    {:else}
      {#if shell.mobile.resolvedMode === "mobile"}
        <AppShell.Toolbar>
          {#if shell.mobile.resolvedMode === "mobile"}<CommunityApplicationMobileNavigation
              model={{ canNavigateBack, canNavigateForward, navigate }}
            />{/if}
          {#if shell.mobile.resolvedMode === "mobile"}<AppShell.Sidebar.Toggle
              side="right"
              sidebarName="details"
            />{/if}
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
          <StoreRepositoryTabs page={current.page} onSelect={selectPage} />
          <header
            class="store-application__section-header"
            aria-label="Workspace section header"
          >
            <div>
              <PanelsTopLeftIcon aria-hidden="true" />
              <h1 class="store-application__title">Workspace</h1>
            </div>
            <div class="store-application__section-controls">
              {#if controller.state.comparisonParents.length}
                <StoreComparisonParentPicker
                  parents={controller.state.comparisonParents}
                  revisions={controller.state.graph.revisions}
                  devices={controller.state.graph.devices}
                  now={controller.state.now}
                  disabled={workspaceBlocked}
                  onSelect={(id) =>
                    void controller.actions.selectComparisonParent(id)}
                />
              {/if}
              <StoreWorkspacePicker {controller} disabled={workspaceBlocked} />
            </div>
          </header>
          <StoreRepositoryNotices {controller} />
          <div class="store-application__body-regions">
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
        <StoreRepositoryDetails {controller} />
        <StoreRepositoryNavigation page={current.page} onSelect={selectPage} />
        <StoreRepositoryContextActions {controller} />
      </AppShell.Sidebar.Body>
    </AppShell.Sidebar>{/if}
</CommunityAppShell>
{#if !settingsOpen}<StoreSearchOverlay
    {controller}
    bind:open={searchOpen}
  />{/if}

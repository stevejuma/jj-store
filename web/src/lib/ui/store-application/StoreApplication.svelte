<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    createLocalStorageAppShellLayoutPersistence,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { onDestroy, untrack, type Snippet } from "svelte";
  import "./store-application.css";
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
    details,
  }: {
    controller: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    layoutStorage?: Storage;
    children?: Snippet<[StorePage, string]>;
    details?: Snippet<[StorePage, string]>;
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
  const pages: CommunityNavigationItemModel[] = [
    { id: "branches", label: "Branches", kind: "branches" },
    { id: "log", label: "JJ Log", kind: "log" },
    { id: "oplog", label: "Op Log", kind: "oplog" },
    { id: "agents", label: "Agents", kind: "agents" },
    { id: "reviews", label: "Reviews", kind: "reviews" },
  ];
  const bodyTabs: readonly {
    page: StorePage;
    label: string;
  }[] = [
    { page: "agents", label: "Activity" },
    { page: "files", label: "Files" },
    { page: "branches", label: "Workspaces" },
    { page: "reviews", label: "Review" },
    { page: "log", label: "Commits" },
  ];
  const navigation: CommunityNavigationModel = $derived({
    primaryItems: pages,
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
    current.page === "files"
      ? "Files"
      : (pages.find((p) => p.id === current.page)?.label ?? "Repository"),
  );
  const bodyTitle = $derived(
    bodyTabs.find((tab) => tab.page === current.page)?.label ?? title,
  );
  const activeBodyTab = $derived(
    bodyTabs.some((tab) => tab.page === current.page) ? current.page : "",
  );
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
    mobileLabel={settingsOpen ? "Settings" : "Repositories and history"}
    resizeLabel="Resize repository navigation sidebar"
    closeable
    revealOnEdgeHover
    edgeRevealLabel="Preview repository navigation sidebar"
  >
    {#if settingsOpen}<StoreSettingsNavigation
        section={settingsSection}
        onSelect={selectSettings}
        onClose={closeSettings}
      />{:else}<CommunityNavigation
        model={navigation}
        activeItemId={current.page === "files"
          ? current.repository
          : current.page}
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
          <div class="store-application__tabs">
            <ScrollArea
              type="hover"
              orientation="horizontal"
              class="store-application__tabs-scroll-area"
            >
              <Tabs.Root value={activeBodyTab}>
                <Tabs.List
                  class="store-application__tab-list"
                  aria-label="Repository sections"
                >
                  {#each bodyTabs as tab (tab.page)}
                    <Tabs.Trigger
                      class="store-application__tab-trigger"
                      value={tab.page}
                      onclick={() => controller.actions.selectPage(tab.page)}
                      >{tab.label}</Tabs.Trigger
                    >
                  {/each}
                </Tabs.List>
              </Tabs.Root>
            </ScrollArea>
          </div>
          <h1 class="store-application__title">{bodyTitle}</h1>
          <div class="store-application__body-regions">
            {#key current.repository + current.page}
              {#if children}{@render children(
                  current.page,
                  current.repository,
                )}{:else}<p class="store-application__placeholder">
                  {current.repository} · {bodyTitle}
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
      <AppShell.Sidebar.Body
        >{#if details}{@render details(
            current.page,
            current.repository,
          )}{:else}<StoreRepositoryDetails
            {controller}
            {title}
          />{/if}</AppShell.Sidebar.Body
      >
    </AppShell.Sidebar>{/if}
</CommunityAppShell>
{#if !settingsOpen}<StoreSearchOverlay
    {controller}
    bind:open={searchOpen}
  />{/if}

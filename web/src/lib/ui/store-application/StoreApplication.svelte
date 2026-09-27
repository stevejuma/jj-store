<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    createLocalStorageAppShellLayoutPersistence,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Breadcrumb from "@lapismd/design-core/shadcn/breadcrumb";
  import { onDestroy, untrack, type Snippet } from "svelte";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import "./repository-breadcrumb.css";
  import "./store-application.css";
  import { repositoryBreadcrumbSegments } from "../community-projects/repository-browser.js";
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
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
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
  const canNavigateBack = $derived(controller.state.canNavigateBack);
  const canNavigateForward = $derived(controller.state.canNavigateForward);
  const filesBreadcrumbPath = $derived(current.path ?? current.directory);
  const filesBreadcrumbSegments = $derived(
    repositoryBreadcrumbSegments(filesBreadcrumbPath),
  );
  const compact = $derived(
    shell.mobile.resolvedMode === "desktop" &&
      shell.left.collapsed &&
      !shell.left.previewed,
  );
  const pages: CommunityNavigationItemModel[] = [
    { id: "branches", label: "Branches", kind: "branches" },
    { id: "log", label: "JJ Log", kind: "log" },
    { id: "oplog", label: "Op Log", kind: "oplog" },
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
      : pages.find((p) => p.id === current.page)!.label,
  );
  function select(item: CommunityNavigationItemModel) {
    if (item.kind === "repository") controller.actions.openRepository(item.id);
    else controller.actions.selectPage(item.kind);
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function navigate(delta: -1 | 1) {
    controller.actions.navigate(delta);
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
  {#snippet headerCenter()}
    <div class="community-application__project-breadcrumb">
      <Breadcrumb.Root>
        <Breadcrumb.List aria-label="Repository breadcrumb">
          <Breadcrumb.Item
            ><Breadcrumb.Link
              href="#"
              onclick={(event) => {
                event.preventDefault();
                controller.actions.openRepository(current.repository);
              }}><span>{current.repository}</span></Breadcrumb.Link
            ></Breadcrumb.Item
          >
          <Breadcrumb.Separator />
          <Breadcrumb.Item
            ><Breadcrumb.Page><span>{title}</span></Breadcrumb.Page
            ></Breadcrumb.Item
          >
        </Breadcrumb.List>
      </Breadcrumb.Root>
    </div>
  {/snippet}
  {#snippet headerEnd()}<Button
      variant={controller.state.status.refreshAvailable ? "outline" : "ghost"}
      size="icon-sm"
      aria-label={controller.state.status.refreshAvailable
        ? "Refresh snapshot (new history available)"
        : "Refresh snapshot"}
      title="Refresh snapshot"
      disabled={controller.state.status.connection !== "connected"}
      onclick={() => controller.actions.refresh()}
      ><RefreshCwIcon aria-hidden="true" /></Button
    ><Button
      variant="ghost"
      size="icon-sm"
      aria-label="Toggle details"
      aria-expanded={!shell.right.closed}
      onclick={() =>
        shell.right.closed ? shell.right.open() : shell.right.close()}
      ><CommunitySidebarToggleIcon
        side="right"
        expanded={!shell.right.closed}
      /></Button
    >{/snippet}
  <AppShell.Sidebar
    side="left"
    label="Repository navigation sidebar"
    mobileLabel="Repositories and history"
    resizeLabel="Resize repository navigation sidebar"
    closeable
    revealOnEdgeHover
    edgeRevealLabel="Preview repository navigation sidebar"
  >
    <CommunityNavigation
      model={navigation}
      activeItemId={current.page === "files"
        ? current.repository
        : current.page}
      {compact}
      onSelect={select}
    >
      {#snippet footer()}<CommunityProfileMenu
          member={controller.state.member}
          authenticated={controller.state.authenticated}
          {compact}
          onAction={(action) =>
            action === "sign-in"
              ? controller.actions.signIn()
              : controller.actions.signOut()}
        />{/snippet}
    </CommunityNavigation>
  </AppShell.Sidebar>
  <AppShell.Main>
    {#if current.page === "files" || shell.mobile.resolvedMode === "mobile"}
      <AppShell.Toolbar
        class={current.page === "files"
          ? "store-application__page-nav"
          : undefined}
      >
        {#if shell.mobile.resolvedMode === "mobile"}<CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />{/if}
        {#if current.page === "files"}<div
            class="store-application__files-toolbar"
          >
            <Breadcrumb.List aria-label="Files breadcrumb">
              <Breadcrumb.Item>
                {#if filesBreadcrumbSegments.length === 0}
                  <Breadcrumb.Page><span>Files</span></Breadcrumb.Page>
                {:else}
                  <Breadcrumb.Link
                    href="#"
                    onclick={(event) => {
                      event.preventDefault();
                      controller.actions.selectDirectory("");
                    }}><span>Files</span></Breadcrumb.Link
                  >
                {/if}
              </Breadcrumb.Item>
              {#each filesBreadcrumbSegments as segment, index (segment.path)}
                <Breadcrumb.Separator />
                <Breadcrumb.Item>
                  {#if index === filesBreadcrumbSegments.length - 1}
                    <Breadcrumb.Page
                      ><span>{segment.name}</span></Breadcrumb.Page
                    >
                  {:else}
                    <Breadcrumb.Link
                      href="#"
                      onclick={(event) => {
                        event.preventDefault();
                        controller.actions.selectDirectory(segment.path);
                      }}><span>{segment.name}</span></Breadcrumb.Link
                    >
                  {/if}
                </Breadcrumb.Item>
              {/each}
            </Breadcrumb.List>
          </div>{/if}
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
      <h1 class="store-application__title">{title}</h1>
      {#key current.repository + current.page}
        {#if children}{@render children(
            current.page,
            current.repository,
          )}{:else}<p class="store-application__placeholder">
            {current.repository} · {title}
          </p>{/if}
      {/key}
    </AppShell.Body>
  </AppShell.Main>
  <AppShell.Sidebar
    side="right"
    variant="rail"
    class="community-application__project-overview-sidebar"
    label="Repository details"
    resizeLabel="Resize repository details"
    closeable
  >
    <AppShell.Sidebar.Header
      ><strong>Details</strong><AppShell.Sidebar.Close
        label="Close details"
      /></AppShell.Sidebar.Header
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
  </AppShell.Sidebar>
</CommunityAppShell>

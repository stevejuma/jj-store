<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Breadcrumb from "@lapismd/design-core/shadcn/breadcrumb";
  import { untrack, type Snippet } from "svelte";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import "./repository-breadcrumb.css";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import CommunityNavigation from "../community-navigation/CommunityNavigation.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
    CommunityNavigationModel,
  } from "../community-navigation/model.js";
  import { createInMemoryStoreController } from "../../controller/in-memory-store-controller.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProfileMenu from "../community-profile-menu/CommunityProfileMenu.svelte";
  let {
    initialPage = "branches",
    controller = untrack(() => createInMemoryStoreController(initialPage)),
    initiallyCollapsed = true,
    displayMode = "auto",
    children,
    details,
  }: {
    initialPage?: StorePage;
    controller?: StoreController;
    initiallyCollapsed?: boolean;
    displayMode?: AppShellDisplayMode;
    children?: Snippet<[StorePage, string]>;
    details?: Snippet<[StorePage, string]>;
  } = $props();
  const shell = new AppShellController({
    leftCollapsed: untrack(() => initiallyCollapsed),
    rightClosed: true,
    rightWidth: 320,
    sidebarMinWidth: 48,
    sidebarMaxWidth: 440,
  });
  const current = $derived(controller.state.route);
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
  ];
  const navigation: CommunityNavigationModel = {
    primaryItems: pages,
    sections: [
      {
        id: "repositories",
        label: "Repositories",
        items: [
          { id: "jj-store", label: "jj-store", kind: "repository" },
          { id: "native-demo", label: "native-demo", kind: "repository" },
        ],
      },
    ],
  };
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
    </div>
  {/snippet}
  {#snippet headerEnd()}<Button
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
      <AppShell.Toolbar>
        {#if shell.mobile.resolvedMode === "mobile"}<CommunityApplicationMobileNavigation
            model={{ canNavigateBack, canNavigateForward, navigate }}
          />{/if}
        {#if current.page === "files"}<h1>{title}</h1>{/if}
        {#if shell.mobile.resolvedMode === "mobile"}<AppShell.Sidebar.Toggle
            side="right"
            sidebarName="details"
          />{/if}
      </AppShell.Toolbar>
    {/if}
    <AppShell.Body>
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
        )}{:else}<dl>
          <dt>Repository</dt>
          <dd>{current.repository}</dd>
          <dt>View</dt>
          <dd>{title}</dd>
          <dt>Source</dt>
          <dd>Deterministic fixtures</dd>
        </dl>{/if}</AppShell.Sidebar.Body
    >
  </AppShell.Sidebar>
</CommunityAppShell>

<style>
  :global(.community-application__project-overview-sidebar[data-side="right"]) {
    border-inline-start-color: transparent !important;
  }
  :global(
    .community-application__project-overview-sidebar
      [data-ui-part="sidebar-resize-indicator"]
  ) {
    background: transparent !important;
  }
  h1 {
    margin: 0;
    font-size: var(--community-text-body);
    font-weight: 600;
  }
  .store-application__placeholder {
    padding: 1rem;
  }
  dl {
    padding: 0.75rem;
    font-size: var(--community-text-body);
  }
  dd {
    margin: 0 0 1rem;
    color: var(--muted-foreground);
  }
</style>

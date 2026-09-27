<script lang="ts">
  // Adapted from CommunityApplication's shell composition; all state is local.
  import {
    AppShell,
    AppShellController,
    type AppShellDisplayMode,
  } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { untrack, type Snippet } from "svelte";
  import PanelRightIcon from "@lucide/svelte/icons/panel-right";
  import CommunityAppShell from "../community-app-shell/CommunityAppShell.svelte";
  import CommunityNavigation from "../community-navigation/CommunityNavigation.svelte";
  import CommunityApplicationMobileNavigation from "../community-application/CommunityApplicationMobileNavigation.svelte";
  import type {
    StorePage,
    CommunityNavigationItemModel,
    CommunityNavigationModel,
  } from "../community-navigation/model.js";
  let {
    initialPage = "branches",
    initiallyCollapsed = true,
    displayMode = "auto",
    children,
    details,
  }: {
    initialPage?: StorePage;
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
  let history = $state([
    { page: untrack(() => initialPage), repository: "jj-store" },
  ]);
  let historyIndex = $state(0);
  const current = $derived(history[historyIndex]!);
  const canNavigateBack = $derived(historyIndex > 0);
  const canNavigateForward = $derived(historyIndex < history.length - 1);
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
  const title = $derived(pages.find((p) => p.id === current.page)!.label);
  function select(item: CommunityNavigationItemModel) {
    const next =
      item.kind === "repository"
        ? { ...current, repository: item.id }
        : { ...current, page: item.kind };
    if (next.page !== current.page || next.repository !== current.repository) {
      history = [...history.slice(0, historyIndex + 1), next];
      historyIndex += 1;
    }
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  function navigate(delta: -1 | 1) {
    historyIndex = Math.max(
      0,
      Math.min(history.length - 1, historyIndex + delta),
    );
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
  {#snippet headerCenter()}<span>{current.repository} / {title}</span>{/snippet}
  {#snippet headerEnd()}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Toggle details"
      aria-expanded={!shell.right.closed}
      onclick={() =>
        shell.right.closed ? shell.right.open() : shell.right.close()}
      ><PanelRightIcon aria-hidden="true" /></Button
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
      activeItemId={current.page}
      {compact}
      onSelect={select}
    >
      {#snippet headerStart()}<strong aria-label="jj-store"
          >{compact ? "JJ" : "jj-store"}</strong
        >{/snippet}
      {#snippet footer()}<span title="Fixture catalog"
          >{compact ? "Demo" : "Fixture catalog"}</span
        >{/snippet}
    </CommunityNavigation>
  </AppShell.Sidebar>
  <AppShell.Main>
    <AppShell.Toolbar>
      {#if shell.mobile.resolvedMode === "mobile"}<CommunityApplicationMobileNavigation
          model={{ canNavigateBack, canNavigateForward, navigate }}
        />{/if}
      <h1>{title}</h1>
      {#if shell.mobile.resolvedMode === "mobile"}<AppShell.Sidebar.Toggle
          side="right"
          sidebarName="details"
        />{/if}
    </AppShell.Toolbar>
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

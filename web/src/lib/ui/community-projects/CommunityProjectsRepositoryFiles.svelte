<script lang="ts">
  import {
    workspaceSnapshotChoices,
    selectedWorkspaceSnapshot,
  } from "./workspace-snapshots.js";
  import { guardWorkspaceEditing } from "../store-application/workspace-editing.svelte.js";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRepositoryFileTree from "./CommunityProjectsRepositoryFileTree.svelte";
  import CommunityProjectsColumnAlert from "./CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsRepositoryFilePanel from "./CommunityProjectsRepositoryFilePanel.svelte";
  import CommunityProjectsRepositoryBreadcrumb from "./CommunityProjectsRepositoryBreadcrumb.svelte";
  import CommunityProjectsRepositoryDirectory from "./CommunityProjectsRepositoryDirectory.svelte";
  import CommunityProjectsRepositorySkeleton from "./CommunityProjectsRepositorySkeleton.svelte";
  import {
    isMarkdownRepositoryPath,
    isRepositoryLandingDocumentPath,
    repositoryBrowserEntries,
  } from "./repository-browser.js";
  import {
    clampFileTreeWidth,
    loadRepositoryFileTreeLayout,
    repositoryFileTreeDefaultWidth,
    repositoryFileTreeMaximumWidth,
    repositoryFileTreeMinimumWidth,
    resolveLayoutStorage,
    saveRepositoryFileTreeLayout,
  } from "../layout-storage.js";
  import { untrack } from "svelte";
  import "./repository-files.css";
  let {
    controller,
    now,
    layoutStorage,
  }: { controller: StoreController; now: number; layoutStorage?: Storage } =
    $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const initialFileTreeLayout = untrack(() =>
    loadRepositoryFileTreeLayout(storage),
  );
  let fileTreeOpen = $state(initialFileTreeLayout.open);
  let fileTreeWidth = $state(initialFileTreeLayout.width);
  let editing = $state(false);
  guardWorkspaceEditing(() => editing);
  let editFilename = $state("");
  let selectedFilePath = $state("");
  let requestedEditPath = $state("");
  let resizePointerId = $state<number | null>(null);
  let resizeStartX = 0;
  let resizeStartWidth = 0;
  const fileTreeMinimum = repositoryFileTreeMinimumWidth;
  const fileTreeMaximum = repositoryFileTreeMaximumWidth;
  const view = $derived(controller.state);
  const initialLoading = $derived(
    !view.snapshot &&
      (view.status.navigation === "loading" || view.status.files === "loading"),
  );
  const fileTreeStyle = $derived(
    `--ui-shell-body-sidebar-width: ${fileTreeWidth}px`,
  );
  const entries = $derived(
    repositoryBrowserEntries(
      [
        ...(view.snapshot?.directories ?? []).map((directory) => ({
          ...directory,
          name: directory.path,
          type: "tree" as const,
        })),
        ...(view.snapshot?.files ?? []).map((file) => ({
          ...file,
          name: file.path,
          type: "blob" as const,
        })),
      ],
      view.route.directory,
    ),
  );
  const latestCommit = $derived(view.fileRevision);
  const pathCommits = $derived(
    Object.fromEntries(entries.map((entry) => [entry.path, latestCommit])),
  );

  const snapshotChoices = $derived(workspaceSnapshotChoices(view));
  const selectedSnapshot = $derived(selectedWorkspaceSnapshot(view));
  const previewDocuments = $derived(
    (view.snapshot?.files ?? []).filter(
      (file) =>
        isRepositoryLandingDocumentPath(file.path) &&
        !file.binary &&
        !file.too_large &&
        file.contents !== undefined,
    ),
  );
  const editUnavailable = $derived.by(() => {
    if (!view.authenticated)
      return "Connect with repository write access to edit this file.";
    if (!view.mutation.available.includes("edit_file"))
      return "This repository server does not support file editing.";
    const bookmark = view.bookmarks.find(
      (candidate) => candidate.name === view.route.bookmark,
    );
    if (!view.route.bookmark || !bookmark)
      return "Choose a local bookmark to edit this file.";
    if (
      bookmark.targets.length !== 1 ||
      view.targets.length !== 1 ||
      bookmark.targets[0] !== view.targets[0]
    )
      return "Choose the bookmark's single current target to edit this file.";
    return "";
  });
  function selectSnapshot(value: string | undefined) {
    if (value?.startsWith("working-copy:"))
      controller.actions.selectWorkspace(value.slice("working-copy:".length));
    else if (value) controller.actions.selectRef(value);
  }
  $effect(() =>
    saveRepositoryFileTreeLayout(storage, {
      open: fileTreeOpen,
      width: fileTreeWidth,
    }),
  );
  $effect(() => {
    const path = view.route.path ?? "";
    if (path === selectedFilePath) return;
    selectedFilePath = path;
    editFilename = path;
    editing = requestedEditPath === path;
    if (editing) requestedEditPath = "";
  });
  function editLandingDocument(path: string) {
    requestedEditPath = path;
    controller.actions.selectFile(path);
  }
  function beginFileTreeResize(event: PointerEvent) {
    if (!fileTreeOpen) return;
    event.preventDefault();
    resizePointerId = event.pointerId;
    resizeStartX = event.clientX;
    resizeStartWidth = fileTreeWidth;
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
  }
  function moveFileTreeResize(event: PointerEvent) {
    if (event.pointerId !== resizePointerId) return;
    fileTreeWidth = clampFileTreeWidth(
      resizeStartWidth + event.clientX - resizeStartX,
    );
  }
  function finishFileTreeResize(event: PointerEvent) {
    if (event.pointerId !== resizePointerId) return;
    resizePointerId = null;
    (event.currentTarget as HTMLElement).releasePointerCapture(event.pointerId);
  }
  function resizeFileTreeWithKeyboard(event: KeyboardEvent) {
    const step = event.shiftKey ? 32 : 16;
    if (event.key === "ArrowLeft") {
      event.preventDefault();
      fileTreeWidth = clampFileTreeWidth(fileTreeWidth - step);
    } else if (event.key === "ArrowRight") {
      event.preventDefault();
      fileTreeWidth = clampFileTreeWidth(fileTreeWidth + step);
    } else if (event.key === "Home") {
      event.preventDefault();
      fileTreeWidth = fileTreeMinimum;
    } else if (event.key === "End") {
      event.preventDefault();
      fileTreeWidth = fileTreeMaximum;
    }
  }
</script>

{#if fileTreeOpen}<AppShell.Body.Sidebar
    panelId="repository-file-tree"
    side="left"
    label="Repository file tree"
    mobileLabel="Repository file tree"
    open={fileTreeOpen}
    scrollbarVisibility="hover"
    class="community-projects__file-tree-region"
    style={fileTreeStyle}
  >
    {#if initialLoading}<CommunityProjectsRepositorySkeleton
        layout="file-tree"
        label="Loading repository file tree"
      />{:else}<CommunityProjectsRepositoryFileTree
        files={view.snapshot?.files ?? []}
        directories={view.snapshot?.directories ?? []}
        mode={view.fileViewMode}
        collapsedDirectoryPaths={view.collapsedDirectoryPaths}
        selectedPath={view.route.path ?? ""}
        hasMore={view.canLoadMoreFiles}
        loading={view.status.files === "refreshing"}
        onSelectPath={controller.actions.selectFile}
        onLoadMore={() => void controller.actions.loadMore("files")}
        onModeChange={(mode) => void controller.actions.setFileViewMode(mode)}
        onDirectoryToggle={(path) =>
          void controller.actions.toggleDirectory(path)}
      />{/if}
  </AppShell.Body.Sidebar>
  <button
    type="button"
    class="community-projects__file-tree-resize"
    data-resizing={resizePointerId === null ? undefined : ""}
    data-min-width={fileTreeMinimum}
    data-max-width={fileTreeMaximum}
    data-current-width={fileTreeWidth}
    aria-label="Resize repository file tree"
    title="Drag to resize. Double-click to reset width."
    onpointerdown={beginFileTreeResize}
    onpointermove={moveFileTreeResize}
    onpointerup={finishFileTreeResize}
    onpointercancel={finishFileTreeResize}
    ondblclick={() => (fileTreeWidth = repositoryFileTreeDefaultWidth)}
    onkeydown={resizeFileTreeWithKeyboard}
  >
    <span aria-hidden="true"></span>
  </button>{/if}
<AppShell.Body.Content
  label="Repository file browser"
  scrollbarVisibility="hover"
  class="community-projects__files-content"
>
  <section
    class="community-projects__files-surface"
    aria-label="Repository files"
  >
    <CommunityProjectsRepositoryBreadcrumb
      {controller}
      {editing}
      bind:filename={editFilename}
    />
    {#if initialLoading}<CommunityProjectsRepositorySkeleton
        layout="directory"
        label="Loading repository files"
      />{:else}
      {#if view.targets.length > 1}<CommunityProjectsColumnAlert
          title="Choose a bookmark target"
          description="This bookmark is conflicted. Select an explicit revision to browse its files."
        >
          {#each view.targets as target}<Button
              variant="outline"
              onclick={() => controller.actions.selectRevision(target)}
              >Open target {target.slice(0, 8)}…{target.slice(-4)}</Button
            >{/each}</CommunityProjectsColumnAlert
        >{/if}
      {#if view.route.path}<CommunityProjectsRepositoryFilePanel
          {controller}
          bind:editing
          bind:filename={editFilename}
          blob={view.blob}
          selectedPath={view.route.path ?? ""}
          {latestCommit}
          {now}
          {fileTreeOpen}
          onToggleFileTree={() => (fileTreeOpen = !fileTreeOpen)}
          loading={view.status.detail === "loading"}
          isMarkdown={isMarkdownRepositoryPath(view.route.path ?? "")}
        />{:else}<CommunityProjectsRepositoryDirectory
          bind:fileTreeOpen
          currentPath={view.route.directory}
          {entries}
          {latestCommit}
          {pathCommits}
          {previewDocuments}
          {editUnavailable}
          branch={selectedSnapshot}
          branches={snapshotChoices}
          {now}
          onOpenDirectory={controller.actions.selectDirectory}
          onOpenFile={controller.actions.selectFile}
          onEditFile={editLandingDocument}
          onSelectCommit={async (revision) => {
            await controller.actions.selectPage("log");
            await controller.actions.selectRevision(revision.eventId);
          }}
          onSelectBranch={selectSnapshot}
          hasMore={view.canLoadMoreFiles}
          loading={view.status.files === "refreshing"}
          onLoadMore={() => controller.actions.loadMore("files")}
          hasMoreReferences={view.canLoadMoreBookmarks}
          loadingReferences={view.status.bookmarks === "refreshing"}
          onLoadMoreReferences={() =>
            void controller.actions.loadMore("bookmarks")}
        />{/if}
    {/if}
  </section>
</AppShell.Body.Content>

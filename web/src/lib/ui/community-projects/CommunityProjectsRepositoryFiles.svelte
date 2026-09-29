<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRepositoryFileTree from "./CommunityProjectsRepositoryFileTree.svelte";
  import CommunityProjectsColumnAlert from "./CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsRepositoryFilePanel from "./CommunityProjectsRepositoryFilePanel.svelte";
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

  const selectedSnapshot = $derived.by(() => {
    if (view.route.bookmark) return `bookmark:${view.route.bookmark}`;
    if (!view.route.changeId) return "";
    return (
      view.choices.find(
        (choice) =>
          choice.kind === "change" &&
          (choice.value === `change:${view.route.changeId}` ||
            (view.route.commitId !== null &&
              choice.targets.includes(view.route.commitId))),
      )?.value ?? `change:${view.route.changeId}`
    );
  });
  const previewDocuments = $derived(
    (view.snapshot?.files ?? []).filter(
      (file) =>
        isRepositoryLandingDocumentPath(file.path) &&
        !file.binary &&
        !file.too_large &&
        file.contents !== undefined,
    ),
  );
  function selectSnapshot(value: string | undefined) {
    if (value) controller.actions.selectRef(value);
  }
  $effect(() =>
    saveRepositoryFileTreeLayout(storage, {
      open: fileTreeOpen,
      width: fileTreeWidth,
    }),
  );
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
          blob={view.blob}
          selectedPath={view.route.path ?? ""}
          {latestCommit}
          {now}
          loading={view.status.detail === "loading"}
          isMarkdown={isMarkdownRepositoryPath(view.route.path ?? "")}
        />{:else}<CommunityProjectsRepositoryDirectory
          bind:fileTreeOpen
          currentPath={view.route.directory}
          {entries}
          {latestCommit}
          {pathCommits}
          {previewDocuments}
          branch={selectedSnapshot}
          branches={view.choices}
          {now}
          onOpenDirectory={controller.actions.selectDirectory}
          onOpenFile={controller.actions.selectFile}
          onSelectCommit={(revision) => {
            controller.actions.selectPage("log");
            controller.actions.selectRevision(revision.eventId);
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

<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRepositoryFileTree from "./CommunityProjectsRepositoryFileTree.svelte";
  import CommunityProjectsRepositoryFilePanel from "./CommunityProjectsRepositoryFilePanel.svelte";
  import CommunityProjectsRepositoryDirectory from "./CommunityProjectsRepositoryDirectory.svelte";
  import {
    isMarkdownRepositoryPath,
    isRepositoryLandingDocumentPath,
    repositoryBrowserEntries,
  } from "./repository-browser.js";
  import "./repository-files.css";
  import "./repository-directory.css";
  let { controller, now }: { controller: StoreController; now: number } =
    $props();
  let fileTreeOpen = $state(true);
  let fileTreeWidth = $state(256);
  let resizePointerId = $state<number | null>(null);
  let resizeStartX = 0;
  let resizeStartWidth = 0;
  const fileTreeMinimum = 208;
  const fileTreeMaximum = 420;
  const view = $derived(controller.state);
  const fileTreeStyle = $derived(
    `--ui-shell-body-sidebar-width: ${fileTreeWidth}px`,
  );
  const entries = $derived(
    repositoryBrowserEntries(
      (view.snapshot?.files ?? []).map((file) => ({
        ...file,
        name: file.path,
        type: "blob" as const,
      })),
      view.route.directory,
    ),
  );
  const latestCommit = $derived(
    view.graph.revisions.find(
      (revision) => revision.eventId === view.snapshot?.id,
    ),
  );
  const pathCommits = $derived(
    Object.fromEntries(entries.map((entry) => [entry.path, latestCommit])),
  );

  const selectedSnapshot = $derived(
    view.route.commitId
      ? `commit:${view.route.commitId}`
      : `bookmark:${view.route.bookmark}`,
  );
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
  function clampFileTreeWidth(width: number): number {
    return Math.max(fileTreeMinimum, Math.min(fileTreeMaximum, width));
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
    <CommunityProjectsRepositoryFileTree
      files={view.snapshot?.files ?? []}
      selectedPath={view.route.path ?? ""}
      onSelectPath={controller.actions.selectFile}
    />
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
    ondblclick={() => (fileTreeWidth = 256)}
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
    {#if view.targets.length > 1}<Alert.Root
        ><Alert.Title>Choose a bookmark target</Alert.Title><Alert.Description
          >This bookmark is conflicted. Select an explicit revision to browse
          its files.</Alert.Description
        >
        {#each view.targets as target}<Button
            variant="outline"
            onclick={() => controller.actions.selectRevision(target)}
            >Open target {target.slice(0, 8)}…{target.slice(-4)}</Button
          >{/each}</Alert.Root
      >{/if}
    {#if view.route.path}<CommunityProjectsRepositoryFilePanel
        blob={view.blob}
        selectedPath={view.route.path ?? ""}
        {latestCommit}
        {now}
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
        onLoadMore={() => controller.actions.loadMore("files")}
      />{/if}
  </section>
</AppShell.Body.Content>

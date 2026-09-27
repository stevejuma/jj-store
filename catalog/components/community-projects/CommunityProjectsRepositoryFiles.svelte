<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRepositoryFileTree from "./CommunityProjectsRepositoryFileTree.svelte";
  import CommunityProjectsRepositoryFilePanel from "./CommunityProjectsRepositoryFilePanel.svelte";
  import {
    repositoryBreadcrumbSegments,
    repositoryParentPath,
  } from "./repository-browser.js";
  import "./repository-files.css";
  let { controller }: { controller: StoreController } = $props();
  let fileTreeOpen = $state(true);
  const view = $derived(controller.state);
  const branch = $derived(
    view.route.revisionId
      ? `commit:${view.route.revisionId}`
      : `bookmark:${view.route.bookmark}`,
  );
  function selectBranch(value: string | undefined) {
    if (value) controller.actions.selectRef(value);
  }
</script>

<section
  class="community-projects__files-surface"
  aria-label="Repository files"
>
  {#if view.targets.length > 1}<Alert.Root
      ><Alert.Title>Choose a bookmark target</Alert.Title><Alert.Description
        >This bookmark is conflicted. Select an explicit revision to browse its
        files.</Alert.Description
      >
      {#each view.targets as target}<Button
          variant="outline"
          onclick={() => controller.actions.selectRevision(target)}
          >Open target {target.slice(0, 8)}…{target.slice(-4)}</Button
        >{/each}</Alert.Root
    >{/if}
  <div
    class="community-projects__file-browser-layout"
    data-file-tree-open={fileTreeOpen}
  >
    {#if fileTreeOpen}<CommunityProjectsRepositoryFileTree
        bind:expanded={fileTreeOpen}
        {branch}
        branches={view.choices}
        files={view.snapshot?.files ?? []}
        selectedPath={view.route.path ?? ""}
        onSelectBranch={selectBranch}
        onSelectPath={controller.actions.selectFile}
      />{/if}
    <CommunityProjectsRepositoryFilePanel
      bind:fileTreeOpen
      {branch}
      branches={view.choices}
      blob={view.blob}
      selectedPath={view.route.path ?? ""}
      isMarkdown={false}
      breadcrumbs={repositoryBreadcrumbSegments(
        repositoryParentPath(view.route.path ?? ""),
      )}
      onSelectBranch={selectBranch}
      onOpenDirectory={() => {
        fileTreeOpen = true;
        controller.actions.selectFile(null);
      }}
    />
  </div>
</section>

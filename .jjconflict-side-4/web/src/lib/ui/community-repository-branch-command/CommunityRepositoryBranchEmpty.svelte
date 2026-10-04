<script lang="ts">
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";

  let { filter, query }: { filter: string; query: string } = $props();

  const title = $derived(
    query.trim()
      ? "No matching bookmarks"
      : filter === "workspace"
        ? "No workspace bookmarks"
        : filter === "conflicted"
          ? "No conflicted bookmarks"
          : "No bookmarks",
  );
  const description = $derived(
    query.trim()
      ? "Try another search or bookmark filter."
      : filter === "workspace"
        ? "No bookmarks are associated with a workspace."
        : filter === "conflicted"
          ? "No bookmarks have multiple targets."
          : "This repository has no bookmarks.",
  );
</script>

<CommandView.Empty class="community-bookmark-cards__empty-option">
  <Empty.Root class="community-bookmark-cards__empty" role="status">
    <Empty.Header>
      <Empty.Media variant="icon">
        <GitBranchIcon aria-hidden="true" />
      </Empty.Media>
      <Empty.Title>{title}</Empty.Title>
      <Empty.Description>{description}</Empty.Description>
    </Empty.Header>
  </Empty.Root>
</CommandView.Empty>

<style>
  :global(
    .community-bookmark-cards__empty-option[data-ui-part="command-empty"]
  ) {
    height: auto;
    min-height: 12rem;
    padding: 0;
  }
  :global(.community-bookmark-cards__empty) {
    min-height: 12rem;
    padding: 1.5rem;
  }
  :global(.community-bookmark-cards__empty [data-ui-part="empty-title"]) {
    font-size: var(--community-text-body);
  }
  :global(.community-bookmark-cards__empty [data-ui-part="empty-description"]) {
    font-size: var(--community-text-secondary);
  }
</style>

<script lang="ts">
  // CommunityProjectsTaskDetails and CommunityApplication overview rows (Apache-2.0).
  import type { Component } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import FolderRootIcon from "@lucide/svelte/icons/folder-root";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import LinkIcon from "@lucide/svelte/icons/link";
  import type { StoreController } from "../../controller/contracts.js";
  import { taskStateLabel } from "./work-presentation.js";
  let { controller }: { controller: StoreController } = $props();
  const hub = $derived(controller.projects!),
    project = $derived(hub.detail?.state.project),
    task = $derived(hub.detail?.state.task);
  let copied = $state(false);
  async function copy() {
    await navigator.clipboard.writeText(
      new URL(location.hash, location.href).href,
    );
    copied = true;
  }
</script>

{#snippet item(
  Icon: Component,
  label: string,
  value: string | number = "",
  action?: () => void,
)}
  <Button
    type="button"
    variant="ghost"
    class="community-application__project-overview-item"
    onclick={action}
  >
    <Icon data-icon="inline-start" aria-hidden="true" /><span>{label}</span
    ><span
      class="community-application__project-overview-count"
      title={String(value)}>{value}</span
    >
  </Button>
{/snippet}

<nav class="store-projects__rail" aria-label="Project details">
  <p>Details</p>
  {#if project}
    {@render item(
      FolderKanbanIcon,
      project.title,
      "",
      () => void hub.openProject(hub.route.repository, project.id),
    )}
    {#if task}
      {@render item(CircleDotIcon, "Status", taskStateLabel(task.state))}
      {@render item(
        MessageCircleIcon,
        "Comments",
        `${hub.detail!.state.comments.length}${hub.detail!.state.commentCursor ? "+" : ""}`,
      )}
      {#if task.parent}{@render item(
          ListTodoIcon,
          "Parent task",
          "",
          () =>
            void hub.openProject(hub.route.repository, project.id, task.parent),
        )}{/if}
    {/if}
    {@render item(
      LinkIcon,
      copied ? "Link copied" : "Copy Link",
      "",
      () => void copy(),
    )}
    <p>Repositories</p>
    {#each Object.entries(project.repositories) as [id, active]}
      {@const repo = controller.state.repositories.find((r) => r.id === id)}
      <Button
        variant="ghost"
        class="community-application__project-overview-item"
        disabled={!repo}
        title={id}
        onclick={() => controller.actions.openRepository(id)}
      >
        <GitBranchIcon data-icon="inline-start" aria-hidden="true" /><span
          >{repo?.name ?? id.slice(0, 12)}</span
        ><span class="community-application__project-overview-count"
          >{active ? "" : "Retired"}</span
        >
      </Button>
      {#if !repo}<small>Separate access required</small>{/if}
    {/each}
  {:else}
    {@render item(
      FolderRootIcon,
      "Projects",
      "",
      () => void hub.openHub("projects"),
    )}
    {@render item(ListTodoIcon, "Tasks", "", () => void hub.openHub("tasks"))}
    {@render item(
      HistoryIcon,
      "Activity",
      "",
      () => void hub.openHub("activity"),
    )}
    {@render item(
      MessageCircleIcon,
      "Reviews",
      "",
      () => void hub.openHub("reviews"),
    )}
    {@render item(
      GitBranchIcon,
      "Repositories",
      "",
      () => void hub.openHub("repositories"),
    )}
    <p>Home repositories</p>
    {#each controller.state.repositories as repo}{@render item(
        GitBranchIcon,
        repo.name,
        "",
        () => void hub.filter({ workHome: repo.id }),
      )}{/each}
  {/if}
</nav>

<style>
  :global(
    .store-projects__rail
      .community-application__project-overview-item[data-ui-component="button"]
  ) {
    display: grid;
    width: 100%;
    height: 1.75rem;
    grid-template-columns: 0.875rem minmax(0, 1fr) auto;
    justify-content: stretch;
    gap: 0.75rem;
    padding-inline: 0.5rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
    font-weight: 400;
    line-height: 1.25rem;
    text-align: start;
  }
  :global(
    .store-projects__rail
      .community-application__project-overview-item:hover:not(:disabled)
  ),
  :global(
    .store-projects__rail
      .community-application__project-overview-item:focus-visible
  ) {
    background-color: color-mix(in oklab, currentColor 10%, transparent);
  }
  :global(
    .store-projects__rail .community-application__project-overview-item > span
  ) {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  :global(
    .store-projects__rail .community-application__project-overview-item > svg
  ) {
    width: 0.875rem;
    height: 0.875rem;
  }
  .community-application__project-overview-count {
    max-width: 9rem;
    overflow: hidden;
    color: var(--muted-foreground);
    font-weight: 400;
    font-variant-numeric: tabular-nums;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

<script lang="ts">
  // CommunityProjectFilesRepositoryCommand composition, using owned workspace state.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/circle-check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
  import XIcon from "@lucide/svelte/icons/x";
  import type { StoreController } from "../../controller/contracts.js";
  import { encodeJjChangeId } from "../../projects/repository-log-graph.js";
  let {
    controller,
    disabled = false,
    label = "Repository workspace",
    emptyLabel = "Repository overview",
    fullWidth = false,
  }: {
    controller: StoreController;
    disabled?: boolean;
    label?: string;
    emptyLabel?: string;
    fullWidth?: boolean;
  } = $props();
  let open = $state(false),
    query = $state("");
  const view = $derived(controller.state);
  function handleOpen(next: boolean) {
    open = next;
    if (next) query = "";
  }
  function choose(workspace: string | null) {
    handleOpen(false);
    void controller.actions.selectWorkspace(workspace);
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        size="sm"
        class={["store-workspace-picker__trigger", props.class]}
        role="combobox"
        aria-label={label}
        style={fullWidth ? "max-width: 100%; width: 100%" : undefined}
        aria-expanded={open}
        title={disabled
          ? "Finish editing or resolving the pending command to switch workspaces."
          : "Select workspace"}
        disabled={disabled ||
          !view.route.snapshotOperationId ||
          view.workingCopiesStatus === "loading"}
      >
        <FolderGit2Icon aria-hidden="true" />
        <span>{view.route.workspace ?? emptyLabel}</span>
        <ChevronDownIcon aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="end"
    class="store-workspace-picker__command"
    style="--ui-popover-width: 22rem; --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Header class="store-workspace-picker__header">
      <Popover.Title role="heading" aria-level={2}
        >Select workspace</Popover.Title
      >
      <Popover.Close>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close workspace selector"
          >
            <XIcon aria-hidden="true" />
          </Button>
        {/snippet}
      </Popover.Close>
    </Popover.Header>
    <CommandView.Root label="Select workspace" loop>
      <CommandView.Input
        bind:value={query}
        placeholder="Search loaded workspaces"
        aria-label="Search workspaces"
      />
      <CommandView.List>
        <CommandView.Empty>No matching workspaces.</CommandView.Empty>
        <CommandView.Group heading={view.repository.name}>
          <CommandView.Item value={emptyLabel} onSelect={() => choose(null)}>
            <CommandView.ItemIcon
              >{#if !view.route.workspace}<CheckIcon
                  aria-hidden="true"
                />{:else}<FolderGit2Icon
                  aria-hidden="true"
                />{/if}</CommandView.ItemIcon
            >
            <CommandView.ItemLabel>{emptyLabel}</CommandView.ItemLabel>
            <CommandView.ItemDescription
              >Browse without workspace context</CommandView.ItemDescription
            >
          </CommandView.Item>
          {#each view.workingCopies as copy (copy.workspace)}
            <CommandView.Item
              value={`${copy.workspace} ${copy.changeId} ${copy.message}`}
              onSelect={() => choose(copy.workspace)}
            >
              <CommandView.ItemIcon
                >{#if view.route.workspace === copy.workspace}<CheckIcon
                    aria-hidden="true"
                  />{:else}<FolderGit2Icon
                    aria-hidden="true"
                  />{/if}</CommandView.ItemIcon
              >
              <CommandView.ItemLabel>{copy.workspace}</CommandView.ItemLabel>
              <CommandView.ItemDescription
                >{encodeJjChangeId(copy.changeId).slice(0, 12)} · {copy.message}</CommandView.ItemDescription
              >
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
    {#if view.workingCopiesStatus === "failed"}
      <p role="status">Workspaces could not be loaded.</p>
      <Button
        variant="ghost"
        size="sm"
        onclick={() => controller.actions.loadMore("workspaces")}
        >Retry workspaces</Button
      >
    {:else if view.canLoadMoreWorkingCopies}
      <Button
        variant="ghost"
        size="sm"
        disabled={view.workingCopiesStatus === "refreshing"}
        onclick={() => controller.actions.loadMore("workspaces")}
        >{view.workingCopiesStatus === "refreshing"
          ? "Loading workspaces…"
          : "Load more workspaces"}</Button
      >
    {/if}
  </Popover.Content>
</Popover.Root>

<style>
  :global(.store-workspace-picker__trigger) {
    min-width: 0;
    max-width: min(22rem, 55%);
  }
  :global(.store-workspace-picker__trigger span) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(.store-workspace-picker__trigger svg) {
    flex: none;
  }
  :global(.store-workspace-picker__header[data-ui-part="popover-header"]) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    min-height: 0;
    margin: 0;
    padding: 0;
  }
  :global(.store-workspace-picker__header [data-ui-part="popover-title"]) {
    margin: 0;
    font-weight: 700;
    line-height: 1.2;
  }
  :global(.store-workspace-picker__command[data-ui-part="popover-content"]) {
    --ui-command-view-list-max-height: min(18rem, 50dvh);
    --ui-command-view-list-padding: 0.25rem;
    --ui-command-view-gap: 0.25rem;
    max-width: calc(100vw - 2rem);
  }
</style>

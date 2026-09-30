<script lang="ts">
  // CommunityProjectFilesRepositoryCommand composition, using owned workspace state.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
  import XIcon from "@lucide/svelte/icons/x";
  import type { StoreController } from "../../controller/contracts.js";
  import { encodeJjChangeId } from "../../projects/repository-log-graph.js";
  let {
    controller,
    disabled = false,
  }: { controller: StoreController; disabled?: boolean } = $props();
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
        aria-label="Repository workspace"
        aria-expanded={open}
        title={disabled
          ? "Finish editing or resolving the pending command to switch workspaces."
          : "Select workspace"}
        disabled={disabled ||
          !view.route.snapshotOperationId ||
          view.status.bookmarks === "loading"}
      >
        <FolderGit2Icon aria-hidden="true" />
        <span>{view.route.workspace ?? "Repository overview"}</span>
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
        placeholder="Search workspaces"
        aria-label="Search workspaces"
      />
      <CommandView.List>
        <CommandView.Empty>No matching workspaces.</CommandView.Empty>
        <CommandView.Group heading={view.repository.name}>
          <CommandView.Item
            value="Repository overview"
            onSelect={() => choose(null)}
          >
            <CommandView.ItemIcon
              >{#if !view.route.workspace}<CheckIcon
                  aria-hidden="true"
                />{/if}</CommandView.ItemIcon
            >
            <CommandView.ItemLabel>Repository overview</CommandView.ItemLabel>
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

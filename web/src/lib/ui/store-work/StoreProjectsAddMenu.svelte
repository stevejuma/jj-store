<script lang="ts">
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import ClipboardPlusIcon from "@lucide/svelte/icons/clipboard-plus";
  import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
  import StoreLocalFolders from "../store-settings/StoreLocalFolders.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    disabled = false,
    includeRepository = false,
    onTask,
  }: {
    controller: StoreController;
    disabled?: boolean;
    includeRepository?: boolean;
    onTask?: () => void;
  } = $props();
  let openingDialog = false;
  let button = $state<HTMLButtonElement | null>(null);
  function select(action: () => void) {
    openingDialog = true;
    button?.focus();
    action();
  }
</script>

{#snippet menu(addRepository?: () => void, repositoryDisabled = false)}
  <DropdownMenu.Root
    onOpenChange={(open) => {
      if (open) openingDialog = false;
    }}
  >
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          bind:ref={button}
          variant="ghost"
          size="icon-sm"
          aria-label="Add"
          title="Add"
          disabled={(!onTask || disabled) &&
            (!addRepository || repositoryDisabled)}
        >
          <PlusIcon aria-hidden="true" />
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content
      align="end"
      preventScroll={false}
      aria-label="Add to projects"
      style="width: 14rem; max-width: calc(100vw - 2rem)"
      onCloseAutoFocus={(event) => {
        if (openingDialog) event.preventDefault();
      }}
    >
      <DropdownMenu.Group>
        {#if onTask}<DropdownMenu.Item
            {disabled}
            onSelect={() => select(onTask!)}
          >
            <ClipboardPlusIcon data-icon="inline-start" aria-hidden="true" />Add
            task
          </DropdownMenu.Item>{/if}
        {#if addRepository}<DropdownMenu.Item
            disabled={repositoryDisabled}
            onSelect={() => select(addRepository)}
          >
            <GitBranchPlusIcon data-icon="inline-start" aria-hidden="true" />Add
            local repository
          </DropdownMenu.Item>{/if}
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/snippet}

{#if includeRepository && controller.state.accessMode === "local_trusted"}
  <StoreLocalFolders {controller} mode="add">
    {#snippet trigger(add, unavailable)}{@render menu(
        add,
        unavailable,
      )}{/snippet}
  </StoreLocalFolders>
{:else}{@render menu()}{/if}

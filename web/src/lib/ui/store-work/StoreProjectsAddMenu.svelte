<script lang="ts">
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import ClipboardPlusIcon from "@lucide/svelte/icons/clipboard-plus";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import ServerIcon from "@lucide/svelte/icons/server";
  import StoreLocalFolders from "../store-settings/StoreLocalFolders.svelte";
  import StoreNativeRepositoryCreate from "../store-settings/StoreNativeRepositoryCreate.svelte";
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

{#snippet menu(
  addRepository?: () => void,
  repositoryDisabled = false,
  createRepository?: () => void,
  creationDisabled = false,
)}
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
            (!addRepository || repositoryDisabled) &&
            (!createRepository ||
              creationDisabled ||
              !controller.state.nativeRepositoryCreation)}
        >
          <PlusIcon aria-hidden="true" />
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content
      align="end"
      preventScroll={false}
      aria-label="Add to projects"
      style="width: 16rem; max-width: calc(100vw - 2rem)"
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
            <FolderGitIcon data-icon="inline-start" aria-hidden="true" />Add
            Local Repository
          </DropdownMenu.Item>{/if}
        {#if createRepository && controller.state.nativeRepositoryCreation}<DropdownMenu.Item
            disabled={creationDisabled}
            onSelect={() => select(createRepository)}
            ><ServerIcon data-icon="inline-start" aria-hidden="true" />Add
            Server Repository</DropdownMenu.Item
          >{/if}
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/snippet}

{#if includeRepository && controller.state.accessMode === "local_trusted"}
  <StoreNativeRepositoryCreate {controller}>
    {#snippet trigger(create, creationDisabled)}
      <StoreLocalFolders {controller} mode="add">
        {#snippet trigger(add, unavailable)}{@render menu(
            add,
            unavailable,
            create,
            creationDisabled,
          )}{/snippet}
      </StoreLocalFolders>
    {/snippet}
  </StoreNativeRepositoryCreate>
{:else}{@render menu()}{/if}

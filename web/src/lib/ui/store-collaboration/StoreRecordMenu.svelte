<script lang="ts">
  // Community message menu composition, with native record copy values.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import Trash2Icon from "@lucide/svelte/icons/trash-2";
  let {
    label = "More message actions",
    items,
    disabled = false,
    onEdit,
    onRemove,
    onOpenChange,
  }: {
    label?: string;
    items: readonly { label: string; value: string | null | undefined }[];
    disabled?: boolean;
    onEdit?: (() => void) | undefined;
    onRemove?: (() => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  } = $props();
  let feedback = $state("");
  $effect(() => {
    void items;
    feedback = "";
  });
  async function copy(label: string, value: string) {
    try {
      await navigator.clipboard.writeText(value);
      feedback = `${label.replace(/^Copy /, "")} copied`;
    } catch {
      feedback = "Could not copy. Try again.";
    }
  }
</script>

<DropdownMenu.Root {onOpenChange}>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}<Button
        {...props}
        type="button"
        variant="ghost"
        size="icon-sm"
        aria-label={label}
        title={label}
      >
        <EllipsisIcon aria-hidden="true" />
      </Button>{/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content
    class="community-message__menu-content"
    style="min-width: 13.5rem;"
    align="end"
    sideOffset={4}
    preventScroll={false}
  >
    {#if onEdit}<DropdownMenu.Group
        ><DropdownMenu.Item {disabled} onSelect={onEdit}
          ><PencilIcon data-icon="inline-start" aria-hidden="true" />Edit
          comment</DropdownMenu.Item
        ></DropdownMenu.Group
      ><DropdownMenu.Separator />{/if}
    <DropdownMenu.Group>
      {#each items as item (item.label)}
        {#if item.value != null}<DropdownMenu.Item
            onSelect={() => void copy(item.label, item.value!)}
            ><CopyIcon
              data-icon="inline-start"
              aria-hidden="true"
            />{item.label}</DropdownMenu.Item
          >{/if}
      {/each}
    </DropdownMenu.Group>
    {#if onRemove}<DropdownMenu.Separator /><DropdownMenu.Group
        ><DropdownMenu.Item {disabled} variant="destructive" onSelect={onRemove}
          ><Trash2Icon data-icon="inline-start" aria-hidden="true" />Remove
          comment</DropdownMenu.Item
        ></DropdownMenu.Group
      >{/if}
  </DropdownMenu.Content>
</DropdownMenu.Root>
<span class="sr-only" role="status" aria-live="polite">{feedback}</span>

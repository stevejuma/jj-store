<script lang="ts">
  // Owned StoreRecordMenu copy composition, adapted to Changeyard stack values.
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import Copy from "@lucide/svelte/icons/copy";
  let {
    items,
    submenu = true,
  }: { items: { label: string; value: string }[]; submenu?: boolean } =
    $props();
  const { context } = ownNotifications();
  async function copy(item: { label: string; value: string }) {
    try {
      await navigator.clipboard.writeText(item.value);
      context.service.resolve(context.scope, "stack-copy");
      context.service.publish({
        scope: context.scope,
        key: "stack-copy",
        title: "Copied",
        message: `${item.label.replace(/^Copy /, "")} copied`,
        severity: "info",
        lifetime: "timed",
      });
    } catch {
      context.service.publish({
        scope: context.scope,
        key: "stack-copy",
        title: "Copy failed",
        message: "Could not copy. Try again.",
        severity: "error",
        lifetime: "dismiss",
      });
    }
  }
</script>

{#snippet options()}
  <Menu.Group
    >{#each items as item (item.label)}
      <Menu.Item aria-label={item.label} onSelect={() => void copy(item)}
        ><Copy aria-hidden="true" />{item.label}</Menu.Item
      >
    {/each}</Menu.Group
  >
{/snippet}
{#if submenu}<Menu.Sub
    ><Menu.SubTrigger><Copy aria-hidden="true" />Copy</Menu.SubTrigger>
    <Menu.SubContent class="stack-workspace__action-menu"
      >{@render options()}</Menu.SubContent
    >
  </Menu.Sub>{:else}{@render options()}{/if}

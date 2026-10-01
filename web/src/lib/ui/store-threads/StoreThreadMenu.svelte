<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import type { StoreController } from "../../controller/contracts.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  let { controller }: { controller: StoreController } = $props();
  const threads = $derived(controller.threads);
  const detail = $derived(threads?.detail);
  const panel = useThreadPanel();
  let menuOpen = $state(false);
</script>

{#if detail}<DropdownMenu.Root bind:open={menuOpen}>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}<Button
          {...props}
          variant="ghost"
          size="icon-sm"
          aria-label="Thread actions"><Ellipsis aria-hidden="true" /></Button
        >{/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Item
        onSelect={() => {
          menuOpen = false;
          panel.open("settings");
        }}>Thread settings</DropdownMenu.Item
      >
    </DropdownMenu.Content>
  </DropdownMenu.Root>{/if}

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import FolderCheckIcon from "@lucide/svelte/icons/folder-check";
  import StoreAgentChatRepositories from "../store-agent-chat/StoreAgentChatRepositories.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  let open = $state(false);
  function opening(value: boolean) {
    if (value) {
      controller.execution?.chat.showContext(controller);
      controller.execution?.chat.close();
    }
    open = value;
  }
</script>

<Popover.Root {open} onOpenChange={opening}>
  <Popover.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label="Agent repositories"
        title="Agent repositories"
        ><FolderCheckIcon aria-hidden="true" /></Button
      >{/snippet}</Popover.Trigger
  >
  <Popover.Content
    side="top"
    align="start"
    style="--ui-popover-width: min(36rem, calc(100vw - 2rem))"
    ><h2>Agent repositories</h2>
    <p>Choose which attached repositories the next agent chat uses.</p>
    {#if open}<StoreAgentChatRepositories {controller} />{/if}</Popover.Content
  >
</Popover.Root>

<style>
  h2 {
    font-size: var(--community-text-body);
    font-weight: 600;
  }
  p {
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }
</style>

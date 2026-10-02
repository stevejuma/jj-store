<script lang="ts">
  import { tick } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreProjectsAgentDirectory from "./StoreProjectsAgentDirectory.svelte";
  import StoreProjectsAgentResponse from "./StoreProjectsAgentResponse.svelte";
  let { controller }: { controller: StoreController } = $props();
  const agents = $derived(controller.projects!.agents);
  const open = $derived(!!controller.projects!.route.agentSessionId);
  let opener: HTMLElement | null = null;
  let root: HTMLDivElement;
  async function select(home: string, id: string) {
    opener =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;
    await agents.select(home, id);
  }
  async function close() {
    const target = opener?.isConnected
      ? opener
      : root.querySelector<HTMLElement>('button[aria-pressed="true"]');
    await agents.select(null, null);
    await tick();
    target?.focus({ preventScroll: true });
  }
</script>

<div
  bind:this={root}
  class="store-projects-agents"
  data-open={open ? "" : undefined}
>
  <ScrollArea type="hover" class="store-projects-agents__directory">
    <StoreProjectsAgentDirectory {controller} onSelect={select} />
  </ScrollArea>
  {#if open}<StoreProjectsAgentResponse
      controller={agents.response}
      error={agents.selectionError}
      onClose={close}
    />{/if}
</div>

<style>
  .store-projects-agents {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    height: 100%;
    min-height: 0;
    min-width: 0;
  }
  .store-projects-agents[data-open] {
    grid-template-columns: minmax(0, 1fr) minmax(0, 1.2fr);
  }
  :global(.store-projects-agents__directory) {
    min-height: 0;
    min-width: 0;
    height: 100%;
  }
  @media (max-width: 720px) {
    .store-projects-agents[data-open] {
      grid-template-columns: minmax(0, 1fr);
    }
    .store-projects-agents[data-open]
      :global(.store-projects-agents__directory) {
      display: none;
    }
  }
</style>

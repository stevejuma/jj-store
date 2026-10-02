<script lang="ts">
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreProjectsAgentDirectory from "./StoreProjectsAgentDirectory.svelte";
  let { controller }: { controller: StoreController } = $props();
</script>

<ScrollArea type="hover" class="store-projects-agents__directory">
  <StoreProjectsAgentDirectory
    {controller}
    onSelect={(home, id) => void controller.projects!.agents.select(home, id)}
  />
</ScrollArea>

<style>
  :global(.store-projects-agents__directory) {
    min-height: 0;
    min-width: 0;
    height: 100%;
    flex: 1;
  }
</style>

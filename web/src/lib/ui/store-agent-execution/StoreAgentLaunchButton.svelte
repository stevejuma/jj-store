<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import BotIcon from "@lucide/svelte/icons/bot";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentLaunchContext } from "../../controller/agent-execution-types.js";
  import { useAgentExecution } from "./context.js";
  const inherited = useAgentExecution();
  let {
    manager = inherited,
    context = {},
    label = "New agent session",
  }: {
    manager?: AgentExecution;
    context?: AgentLaunchContext;
    label?: string;
  } = $props();
</script>

{#if manager?.available()}<Button
    variant="outline"
    size="sm"
    onclick={() => manager?.show(context)}
    ><BotIcon aria-hidden="true" />{label}</Button
  >{/if}

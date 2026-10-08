<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import MessageSquarePlusIcon from "@lucide/svelte/icons/message-square-plus";
  import BotIcon from "@lucide/svelte/icons/bot";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentLaunchContext } from "../../controller/agent-execution-types.js";
  import { useAgentExecution } from "./context.js";
  const inherited = useAgentExecution();
  let {
    manager = inherited,
    context,
    showManage = true,
  }: {
    manager?: AgentExecution;
    context: AgentLaunchContext;
    showManage?: boolean;
  } = $props();
</script>

{#if manager?.available()}
  <Tooltip.Provider>
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="New agent session"
            onclick={() => manager?.show(context)}
          >
            <MessageSquarePlusIcon aria-hidden="true" />
          </Button>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content role="tooltip">New agent session</Tooltip.Content>
    </Tooltip.Root>
    {#if showManage}<Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Manage agents"
              onclick={() => manager?.directory()}
            >
              <BotIcon aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content role="tooltip">Manage agents</Tooltip.Content>
      </Tooltip.Root>{/if}
  </Tooltip.Provider>
{/if}

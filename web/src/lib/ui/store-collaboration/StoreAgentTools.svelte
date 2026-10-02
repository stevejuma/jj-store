<!-- @community-copy src/components/community-agent-activity/CommunityAgentTranscriptTools.svelte -->
<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { useDrafts } from "../../appearance/drafts.svelte.js";
  import { anchorAgentTool } from "../../collaboration/presentation.js";
  import StoreAgentToolDetail from "./StoreAgentToolDetail.svelte";
  import StoreAgentToolReview from "./StoreAgentToolReview.svelte";
  let {
    items,
    patches = {},
    controller,
    interactive = true,
  }: {
    items: readonly Extract<Model<"AgentActivity">, { type: "tool" }>[];
    patches?: Readonly<Record<string, string>>;
    controller: StoreController;
    interactive?: boolean;
  } = $props();
  const drafts = useDrafts();
  const byId = $derived(
    new Map(items.map((entry) => [entry.tool.tool_call_id, entry])),
  );
  const selected = $derived(
    controller.state.reviewThread
      ? anchorAgentTool(controller.state.reviewThread.anchor.value)
      : undefined,
  );
  const initialExpanded = $derived(
    items.some(({ tool }) => {
      const session = controller.state.agentSession?.session.session_id;
      return (
        !!drafts.get(`tool:${session}:${tool.tool_call_id}`).open ||
        (selected?.sessionId === session &&
          selected?.toolCallId === tool.tool_call_id)
      );
    }),
  );
  function status(state: Model<"ToolState">) {
    if (state === "succeeded") return "complete" as const;
    if (state === "failed" || state === "cancelled" || state === "interrupted")
      return "error" as const;
    if (state === "running") return "running" as const;
    if (state === "pending") return "pending" as const;
    return "unknown" as const;
  }
</script>

{#snippet detail(call: { id?: string })}
  {@const entry = call.id ? byId.get(call.id) : undefined}
  {#if entry}<StoreAgentToolDetail tool={entry.tool} {patches} />{/if}
{/snippet}

<Chat.ToolCalls
  label={items.length > 1 ? "Tool calls" : "Tool activity"}
  defaultExpanded={initialExpanded}
  calls={items.map(({ tool }) => ({
    id: tool.tool_call_id,
    name: tool.native_name,
    status: status(tool.state),
    target:
      tool.code.flatMap((association) => association.paths)[0] ??
      tool.observed_paths[0] ??
      Chat.toolCallTarget(tool.input?.content),
    detail,
  }))}
>
  {#snippet callWrapper(call, children)}
    {@const entry = call.id ? byId.get(call.id) : undefined}
    {#if entry}<StoreAgentToolReview
        tool={entry.tool}
        {controller}
        {interactive}
      >
        {@render children()}
      </StoreAgentToolReview>{/if}
  {/snippet}
</Chat.ToolCalls>

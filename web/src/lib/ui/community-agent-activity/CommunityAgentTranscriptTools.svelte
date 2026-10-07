<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Chat from "@lapismd/design-core/ai/chat";
  import TranscriptToolDetail from "./TranscriptToolDetail.svelte";
  import { liveTranscriptTool } from "./transcript-tool-adapters.js";
  import type { ActivityRow } from "../store-agent-execution/presentation.js";
  let {
    item,
    items,
    openCallId,
    callWrapper,
  }: {
    item?: ActivityRow;
    items?: readonly ActivityRow[];
    openCallId?: string;
    callWrapper?: Snippet<[ActivityRow, Snippet]>;
  } = $props();
  let rawDetails = $state<Record<string, boolean>>({});
  let expanded = $state(false);
  const entries = $derived(items ?? (item ? [item] : []));
  const byId = $derived(new Map(entries.map((entry) => [entry.id, entry])));
  $effect(() => {
    if (openCallId && byId.has(openCallId)) expanded = true;
  });
  function status(entry: ActivityRow) {
    return entry.value.status === "completed"
      ? ("complete" as const)
      : entry.value.status === "failed"
        ? ("error" as const)
        : entry.value.status === "in_progress"
          ? ("running" as const)
          : ("pending" as const);
  }
</script>

{#snippet detail(call: { id?: string })}
  {@const entry = call.id ? byId.get(call.id) : undefined}
  {#if entry}<TranscriptToolDetail
      tool={liveTranscriptTool(entry)}
      value={entry.value}
      bind:raw={
        () => rawDetails[entry.id] ?? false,
        (value) => (rawDetails[entry.id] = value)
      }
    />{/if}
{/snippet}
{#snippet wrapCall(call: { id?: string }, children: Snippet)}
  {@const entry = call.id ? byId.get(call.id) : undefined}
  {#if entry && callWrapper}{@render callWrapper(entry, children)}
  {:else}{@render children()}{/if}
{/snippet}
<Chat.ToolCalls
  style="--ui-ai-chat-tool-width: 100%; --ui-ai-chat-tool-trigger-padding-inline: 0.5rem;"
  label={entries.length > 1 ? "Tool calls" : "Tool activity"}
  bind:expanded
  callWrapper={wrapCall}
  calls={entries.map((entry) => ({
    id: entry.id,
    name: entry.text || "Tool call",
    status: status(entry),
    detail,
  }))}
/>

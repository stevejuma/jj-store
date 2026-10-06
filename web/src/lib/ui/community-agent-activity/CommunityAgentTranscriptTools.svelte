<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import type { ActivityRow } from "../store-agent-execution/presentation.js";
  let { item }: { item: ActivityRow } = $props();
  const status = $derived(
    item.value.status === "completed"
      ? "complete"
      : item.value.status === "failed"
        ? "error"
        : item.value.status === "in_progress"
          ? "running"
          : "pending",
  );
</script>

{#snippet detail()}<CodeBlock
    code={JSON.stringify(item.value, null, 2)}
    language="json"
    title="Tool activity"
    size="sm"
    width="full"
    isWrapped
    maxHeight="20rem"
  />{/snippet}
<Chat.ToolCalls
  label="Tool activity"
  calls={[{ id: item.id, name: item.text || "Tool call", status, detail }]}
/>

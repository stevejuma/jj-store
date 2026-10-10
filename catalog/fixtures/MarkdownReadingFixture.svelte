<script lang="ts">
  import StoreAgentChatBody from "../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import { markdownBlocks } from "../../web/src/lib/ui/store-agent-chat-body/markdown-blocks.js";
  let selected = $state("");
  let opens = 0;
  const text =
    "Updated [LICENSE](</Users/synthetic/workspace/LICENSE:3>)\n\n- File is ready\n- Commit: `52c87453`\n\nLocation: /Users/synthetic/workspace/LICENSE";
  const items = [
    {
      id: "answer",
      label: "Assistant message",
      kind: "message" as const,
      author: { id: "agent", name: "Agent", handle: "", initials: "AG" },
      blocks: markdownBlocks("answer", "revision", "Assistant message", text),
    },
  ];
</script>

<div style="height: 480px">
  <StoreAgentChatBody
    {items}
    displayPaths={() => [["/Users/synthetic/workspace", "$WORKSPACE"]]}
    resolveFileLink={(_block, destination) =>
      destination === "/Users/synthetic/workspace/LICENSE:3"
        ? () => (selected = `LICENSE line 3 (${++opens})`)
        : undefined}
  />
</div>
<output aria-label="Selected file">{selected}</output>

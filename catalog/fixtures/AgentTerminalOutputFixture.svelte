<script lang="ts">
  import { untrack } from "svelte";
  import { capturedToolProjection } from "../../web/src/lib/conversations/tool-adapter.js";
  import StoreAgentChatBody from "../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import type { ChatBodyItem } from "../../web/src/lib/ui/store-agent-chat-body/model.js";
  let { exact = false }: { exact?: boolean } = $props();
  const output =
    'Sample README contents\nexport const greeting: string = "Hello 🦀";\n';
  const value = {
    kind: "execute",
    status: "completed",
    rawInput: { command: "cat README.md src/example.ts" },
    rawOutput: { formatted_output: output, exit_code: 0 },
    content: [{ type: "terminal", terminalId: "captured-terminal" }],
  };
  const target = {
    type: "tool_output",
    session_id: "1".repeat(32),
    tool_id: "2".repeat(32),
    snapshot: { kind: "tool_snapshot", id: "3".repeat(64) },
    content: { kind: "content", id: "4".repeat(64) },
  } as const;
  const projected = untrack(() =>
    capturedToolProjection(
      {
        id: "execute",
        kind: "tool",
        request: "turn",
        sequence: 1,
        text: "Execute",
        value,
      },
      "execute",
      "revision",
      {
        row_id: "execute",
        target,
        text_parts: exact
          ? [{ part: "/rawOutput/formatted_output", text: output }]
          : [{ part: "/0/terminalId", text: "captured-terminal" }],
        threads: [],
        replies: [],
        answers: [],
      },
      new Map(),
    ),
  );
  const items: ChatBodyItem[] = [
    {
      id: "tools",
      kind: "tools",
      label: "Tools",
      blocks: [],
      tools: [
        {
          id: "execute",
          revision: "revision",
          title: projected.title,
          status: "complete",
          blocks: projected.blocks,
          raw: value,
        },
      ],
    },
  ];
</script>

<div style="height: 650px;"><StoreAgentChatBody {items} /></div>

<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import XIcon from "@lucide/svelte/icons/x";
  import ChatRawToggle from "./ChatRawToggle.svelte";
  import ChatCollapsedComments from "./ChatCollapsedComments.svelte";
  import ChatToolBlocks from "./ChatToolBlocks.svelte";
  import { rawToolBlock } from "./tool-blocks.js";
  import type { ChatBodyTool, ChatBodyProps } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    tool,
    state,
    onClose,
    customBlock,
  }: {
    tool: ChatBodyTool;
    state: ChatBodyComments;
    onClose: () => void;
    customBlock?: ChatBodyProps["customBlock"];
  } = $props();
</script>

<AppShell.Sidebar.Header class="chat-tool-panel-header">
  <strong>{tool.title}</strong>
  <div class="chat-tool-panel-actions">
    <ChatRawToggle
      raw={state.rawTools[tool.id] ?? false}
      label={`Show ${state.rawTools[tool.id] ? "rich" : "raw"} output in read sidebar for ${tool.title}`}
      onToggle={() => (state.rawTools[tool.id] = !state.rawTools[tool.id])}
    />
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Close read tool details"
      onclick={onClose}><XIcon size={16} /></Button
    >
  </div>
</AppShell.Sidebar.Header>
<AppShell.Sidebar.Body>
  <div class="chat-tool-panel-content">
    <ChatToolBlocks
      blocks={state.rawTools[tool.id] ? [rawToolBlock(tool)] : tool.blocks}
      {state}
      {customBlock}
    />
    <ChatCollapsedComments
      {state}
      blocks={state.rawTools[tool.id] ? tool.blocks : [rawToolBlock(tool)]}
    />
  </div>
</AppShell.Sidebar.Body>

<style>
  :global(.chat-tool-panel-header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-inline: 0.75rem;
    gap: 0.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--background);
  }
  .chat-tool-panel-actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
  strong {
    min-width: 0;
    font-size: 0.85rem;
    overflow-wrap: anywhere;
  }
  .chat-tool-panel-content {
    padding: 0.75rem;
  }
</style>

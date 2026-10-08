<script lang="ts">
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

<div class="chat-tool-panel">
  <header>
    <div><span>Captured read</span><strong>{tool.title}</strong></div>
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
  </header>
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
</div>

<style>
  .chat-tool-panel {
    min-width: 0;
    color: var(--foreground);
    background: var(--background);
  }
  header {
    position: sticky;
    top: 0;
    z-index: 2;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding: 0.75rem;
    gap: 0.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--background);
  }
  header > div:first-child {
    display: grid;
    gap: 0.2rem;
    min-width: 0;
  }
  header span {
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
  .chat-tool-panel-actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
  strong {
    font-size: 0.85rem;
    overflow-wrap: anywhere;
  }
  .chat-tool-panel-content {
    padding: 0.75rem;
  }
</style>

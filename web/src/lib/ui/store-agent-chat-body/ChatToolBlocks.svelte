<script lang="ts">
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import ChatContent from "./ChatContent.svelte";
  import type { ChatBodyBlock, ChatBodyProps } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    blocks,
    state,
    customBlock,
  }: {
    blocks: readonly ChatBodyBlock[];
    state: ChatBodyComments;
    customBlock?: ChatBodyProps["customBlock"];
  } = $props();
  const key = (block: ChatBodyBlock) => `file:${block.id}`;
  const open = (block: ChatBodyBlock, index: number) =>
    state.disclosures[key(block)] ?? index === 0;
</script>

{#if blocks.length > 1}
  <div class="chat-tool-files">
    {#each blocks as block, index (block.id)}
      <Collapsible.Root
        open={open(block, index)}
        onOpenChange={(value) => (state.disclosures[key(block)] = value)}
      >
        <div class="chat-tool-file">
          <Collapsible.Trigger
            class="chat-file-trigger"
            aria-label={`${open(block, index) ? "Hide" : "Show"} captured file ${block.path ?? block.label}`}
          >
            <span>{block.path ?? block.label}</span>
            <ChevronRightIcon
              size={14}
              class={open(block, index) ? "chat-file-open" : ""}
            />
          </Collapsible.Trigger>
          <Collapsible.Content>
            <div class="chat-tool-file-content">
              <ChatContent
                blocks={[block]}
                {state}
                {customBlock}
                showHeaders={false}
              />
            </div>
          </Collapsible.Content>
        </div>
      </Collapsible.Root>
    {/each}
  </div>
{:else}<ChatContent {blocks} {state} {customBlock} />{/if}

<style>
  .chat-tool-files {
    display: grid;
    gap: 0.5rem;
    padding: 0.25rem;
  }
  .chat-tool-file {
    border-radius: var(--radius);
  }
  :global(.chat-file-trigger) {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    width: fit-content;
    max-width: 100%;
    padding: 0;
    min-height: 28px;
    border: 0;
    border-radius: var(--radius);
    background: transparent;
    color: var(--muted-foreground);
    cursor: pointer;
    font-family: var(--font-sans, system-ui);
    font-size: 0.875rem;
    font-weight: 400;
    line-height: 1.5;
    text-align: left;
  }
  :global(.chat-file-trigger span) {
    overflow-wrap: anywhere;
    min-width: 0;
  }
  :global(.chat-file-trigger:hover) {
    background: transparent;
    color: var(--foreground);
  }
  :global(.chat-file-trigger svg) {
    flex-shrink: 0;
  }
  :global(.chat-file-open) {
    rotate: 90deg;
  }
  :global(.chat-file-trigger:focus-visible) {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
  }
  .chat-tool-file-content {
    margin: 0.5rem 0.25rem;
  }
</style>

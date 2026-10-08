<script lang="ts">
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ChatDisclosureIcon from "./ChatDisclosureIcon.svelte";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import ChatRawToggle from "./ChatRawToggle.svelte";
  import ChatCollapsedComments from "./ChatCollapsedComments.svelte";
  import ChatToolBlocks from "./ChatToolBlocks.svelte";
  import type { ChatBodyItem, ChatBodyTool, ChatBodyProps } from "./model.js";
  import { rawToolBlock } from "./tool-blocks.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    item,
    state,
    customBlock,
    onInspectTool,
  }: {
    item: ChatBodyItem;
    state: ChatBodyComments;
    customBlock?: ChatBodyProps["customBlock"];
    onInspectTool?: ChatBodyProps["onInspectTool"];
  } = $props();
  const calls = $derived(item.tools ?? []);
  const componentId = $props.id();
  const statusId = (tool: ChatBodyTool) =>
    `${componentId}-status-${encodeURIComponent(tool.id)}`;
  function expanded(id: string, fallback = false) {
    return state.disclosures[id] ?? fallback;
  }
  const statusLabel = (tool: ChatBodyTool) =>
    ({
      complete: "Complete",
      error: "Error",
      running: "Running",
      unknown: "Status unavailable",
    })[tool.status];
  function toggleRaw(tool: ChatBodyTool, opener: HTMLElement) {
    state.rawTools[tool.id] = !state.rawTools[tool.id];
    if (tool.detailPlacement === "sidebar" && onInspectTool)
      onInspectTool(tool, opener);
    else state.disclosures[tool.id] = true;
  }
</script>

{#snippet rawToggle(tool: ChatBodyTool)}
  <ChatRawToggle
    raw={state.rawTools[tool.id] ?? false}
    label={`Show ${state.rawTools[tool.id] ? "rich" : "raw"} output for ${tool.title}`}
    onToggle={(opener) => toggleRaw(tool, opener)}
  />
{/snippet}

{#snippet call(tool: ChatBodyTool)}
  <div class="chat-tool-item">
    <span class="sr-only" id={statusId(tool)}>{statusLabel(tool)}</span>
    {#if tool.detailPlacement === "sidebar" && onInspectTool}
      <div class="chat-tool-heading">
        <Button
          variant="ghost"
          class="chat-tool-trigger"
          aria-label={`Show details for ${tool.title}`}
          aria-describedby={statusId(tool)}
          onclick={(event) => onInspectTool?.(tool, event.currentTarget)}
        >
          {#if tool.status === "error"}<CircleAlertIcon
              size={14}
            />{:else if tool.status === "running"}<LoaderCircleIcon
              size={14}
            />{/if}<span>{tool.title}</span><ChatDisclosureIcon />
        </Button>
        {@render rawToggle(tool)}
      </div>
      <ChatCollapsedComments
        {state}
        blocks={[...tool.blocks, rawToolBlock(tool)]}
      />
    {:else}
      <Collapsible.Root
        open={expanded(tool.id, item.defaultExpanded && calls.length === 1)}
        onOpenChange={(open) => (state.disclosures[tool.id] = open)}
      >
        <div class="chat-tool-heading">
          <Collapsible.Trigger
            class="chat-tool-trigger"
            aria-label={`${expanded(tool.id, item.defaultExpanded && calls.length === 1) ? "Hide" : "Show"} details for ${tool.title}`}
            aria-describedby={statusId(tool)}
          >
            {#if tool.status === "error"}<CircleAlertIcon size={14} />
            {:else if tool.status === "running"}<LoaderCircleIcon size={14} />
            {/if}
            <span>{tool.title}</span>
            <ChatDisclosureIcon />
          </Collapsible.Trigger>
          {@render rawToggle(tool)}
        </div>
        <Collapsible.Content class="chat-tool-detail" aria-label={tool.title}>
          {#if expanded(tool.id, item.defaultExpanded && calls.length === 1)}<ChatToolBlocks
              blocks={state.rawTools[tool.id]
                ? [rawToolBlock(tool)]
                : tool.blocks}
              {state}
              {customBlock}
            />
          {/if}
        </Collapsible.Content>
      </Collapsible.Root>
      {#if !expanded(tool.id, item.defaultExpanded && calls.length === 1)}
        <ChatCollapsedComments
          {state}
          blocks={[...tool.blocks, rawToolBlock(tool)]}
        />
      {:else}
        <ChatCollapsedComments
          {state}
          blocks={state.rawTools[tool.id] ? tool.blocks : [rawToolBlock(tool)]}
        />
      {/if}
    {/if}
  </div>
{/snippet}

{#if calls.length > 1}
  <Collapsible.Root
    open={expanded(item.id, item.defaultExpanded)}
    onOpenChange={(open) => (state.disclosures[item.id] = open)}
  >
    <Collapsible.Trigger
      class="chat-tool-trigger chat-tool-summary"
      aria-label={`${expanded(item.id, item.defaultExpanded) ? "Collapse" : "Expand"} ${item.label}`}
    >
      <span>{item.label}</span><ChatDisclosureIcon />
    </Collapsible.Trigger>
    <Collapsible.Content class="chat-tool-group-content"
      >{#if expanded(item.id, item.defaultExpanded)}{#each calls as tool (tool.id)}{@render call(
            tool,
          )}{/each}{/if}</Collapsible.Content
    >
  </Collapsible.Root>
  {#if !expanded(item.id, item.defaultExpanded)}<ChatCollapsedComments
      {state}
      blocks={calls.flatMap((t) => [...t.blocks, rawToolBlock(t)])}
    />{/if}
{:else if calls[0]}{@render call(calls[0])}{/if}

<style>
  .chat-tool-item {
    padding: 0;
  }
  :global(.chat-tool-summary) {
    margin: 0;
  }
  .chat-tool-heading {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  :global(.chat-tool-trigger) {
    display: inline-flex;
    width: fit-content;
    max-width: 100%;
    flex: 0 1 auto;
    min-width: 0;
    height: auto;
    justify-content: flex-start;
    font-weight: 400;
    gap: 0.5rem;
    align-items: center;
    padding: 0;
    min-height: 28px;
    border: 0;
    border-radius: var(--radius);
    background: transparent;
    color: var(--muted-foreground);
    text-align: start;
    font-family: var(--font-sans, system-ui);
    font-size: 0.875rem;
    line-height: 1.5;
    cursor: pointer;
  }
  :global(.chat-tool-trigger:hover) {
    background: transparent;
    color: var(--foreground);
  }
  :global(.chat-tool-trigger:focus-visible) {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
  }
  :global(.chat-tool-trigger span:not(.sr-only)) {
    overflow-wrap: anywhere;
  }
  :global(.chat-tool-trigger svg) {
    flex-shrink: 0;
  }
  :global(.chat-tool-detail) {
    padding-block-start: 0.5rem;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }
  :global(.chat-tool-group-content:not([hidden])) {
    display: grid;
    gap: 0.5rem;
    padding-block-start: 0.5rem;
  }
</style>

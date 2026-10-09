<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as ToggleGroup from "@lapismd/design-core/shadcn/toggle-group";
  import XIcon from "@lucide/svelte/icons/x";
  import MessageSquarePlus from "@lucide/svelte/icons/message-square-plus";
  import {
    FileDiff,
    type FileDiffLineContext,
  } from "@lapismd/design-core/diff";
  import ChatContent from "../store-agent-chat-body/ChatContent.svelte";
  import type { WorkspaceBrowser } from "../../conversations/workspace-browser.svelte.js";
  let { browser, onClose }: { browser: WorkspaceBrowser; onClose: () => void } =
    $props();
  const file = $derived(browser.file?.file);
  const changed = $derived(!!browser.file?.comparison);
  const comments = $derived(browser.conversation.comments);
  const blocks = $derived(browser.blocks);
  const selectedBlock = $derived(
    browser.presentation === "before" ? blocks.before : blocks.after,
  );
  function commentLine(line: FileDiffLineContext) {
    const side = line.variant === "removed" ? "before" : "after";
    const block = blocks[side];
    if (!block || !line.lineNumber) return;
    const lines = block.text.split("\n"),
      index = line.lineNumber - 1;
    if (index < 0 || index >= lines.length) return;
    const from = lines.slice(0, index).reduce((n, s) => n + s.length + 1, 0);
    browser.presentation = side;
    comments.open(block, {
      from,
      to: from + lines[index].replace(/\r$/, "").length,
    });
  }
</script>

<AppShell.Sidebar.Header>
  <strong class="truncate" title={file?.path}
    >{file?.path ?? "Workspace file"}</strong
  >
  <Button
    variant="ghost"
    size="icon-sm"
    aria-label="Close workspace file"
    onclick={onClose}><XIcon /></Button
  >
</AppShell.Sidebar.Header>
<AppShell.Sidebar.Body>
  <div class="workspace-file-content">
    {#if browser.fileLoading}<p role="status">Loading file…</p>
    {:else if browser.fileError || file?.unavailable}<p role="status">
        {browser.fileError || file?.unavailable}
      </p>
    {:else if file}
      <div class="workspace-inspector-controls">
        {#if blocks.after || blocks.before}<Button
            variant="ghost"
            size="sm"
            onclick={() => {
              const block = selectedBlock ?? blocks.after ?? blocks.before;
              if (block) {
                browser.presentation =
                  block === blocks.before ? "before" : "after";
                comments.open(block, { from: 0, to: block.text.length });
              }
            }}><MessageSquarePlus />Comment on file</Button
          >{/if}
        {#if changed}
          <ToggleGroup.Root
            type="single"
            bind:value={browser.presentation}
            aria-label="File presentation"
            ><ToggleGroup.Item value="diff">Diff</ToggleGroup.Item
            ><ToggleGroup.Item value="before" disabled={!blocks.before}
              >Before</ToggleGroup.Item
            ><ToggleGroup.Item value="after" disabled={!blocks.after}
              >After</ToggleGroup.Item
            ></ToggleGroup.Root
          >
          {#if browser.presentation === "diff"}<ToggleGroup.Root
              type="single"
              bind:value={browser.diffMode}
              aria-label="Diff layout"
              ><ToggleGroup.Item value="unified">Unified</ToggleGroup.Item
              ><ToggleGroup.Item value="split">Split</ToggleGroup.Item
              ></ToggleGroup.Root
            >{/if}
        {/if}
      </div>
      {#if changed && browser.presentation === "diff"}
        <FileDiff
          path={file.path}
          oldText={file.before}
          newText={file.contents}
          viewMode={browser.diffMode}
          wrap
        >
          {#snippet lineAccessory(line: FileDiffLineContext)}<Button
              variant="ghost"
              size="icon-sm"
              aria-label={`Comment on ${line.variant === "removed" ? "before" : "after"} line ${line.lineNumber}`}
              onclick={() => commentLine(line)}><MessageSquarePlus /></Button
            >{/snippet}
        </FileDiff>
      {:else if selectedBlock}
        <ChatContent
          blocks={[selectedBlock]}
          state={comments}
          showHeaders={false}
        />
      {/if}
    {/if}
  </div>
</AppShell.Sidebar.Body>

<style>
  .workspace-file-content {
    display: flex;
    flex-direction: column;
    min-width: 0;
    gap: 0.5rem;
    padding: 0.75rem;
  }
  strong {
    min-width: 0;
    font-size: 0.875rem;
  }
  .workspace-inspector-controls {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
</style>

<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import CommunityDiffControls from "../community-diff-controls/CommunityDiffControls.svelte";
  import FileIcon from "@lucide/svelte/icons/file";
  import XIcon from "@lucide/svelte/icons/x";
  import MessageSquarePlus from "@lucide/svelte/icons/message-square-plus";
  import {
    FileDiff,
    type FileDiffLineContext,
  } from "@lapismd/design-core/diff";
  import RepositoryFileLoading from "../community-projects/RepositoryFileLoading.svelte";
  import { isMarkdownRepositoryPath } from "../community-projects/repository-browser.js";
  import ChatTextBlock from "../store-agent-chat-body/ChatTextBlock.svelte";
  import type { WorkspaceBrowser } from "../../conversations/workspace-browser.svelte.js";
  let { browser, onClose }: { browser: WorkspaceBrowser; onClose: () => void } =
    $props();
  const file = $derived(browser.file?.file);
  const path = $derived(
    browser.fileLoading ? browser.listing.selected : file?.path,
  );
  const changed = $derived(!!browser.file?.comparison);
  const comments = $derived(browser.conversation.comments);
  const blocks = $derived(browser.blocks);
  const sections = $derived([
    { value: "diff", label: "Diff" },
    { value: "before", label: "Before", disabled: !blocks.before },
    { value: "after", label: "After", disabled: !blocks.after },
  ]);
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

<AppShell.Sidebar.Header
  style="padding-inline: 0; --store-view-tabs-padding-inline-end: 0.5rem; --store-view-tabs-min-width: 5rem;"
>
  <StoreViewTabs
    value={browser.presentation}
    {sections}
    hideTabs={!changed || browser.fileLoading || !!browser.fileError}
    label="File presentation"
    onSelect={(value) => {
      if (value === "diff" || value === "before" || value === "after")
        browser.presentation = value;
    }}
  >
    {#snippet leading()}
      <span class="workspace-inspector-title">
        <FileIcon aria-hidden="true" />
        <strong class="truncate" title={path}>{path ?? "Workspace file"}</strong
        >
      </span>
    {/snippet}
    {#snippet trailing()}
      {#if changed && !browser.fileLoading && browser.presentation === "diff"}
        <CommunityDiffControls
          bind:viewMode={browser.diffMode}
          showWrap={false}
        />
      {/if}
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="Close workspace file"
        onclick={onClose}><XIcon /></Button
      >
    {/snippet}
  </StoreViewTabs>
</AppShell.Sidebar.Header>
<AppShell.Sidebar.Body>
  <div class="workspace-file-content">
    {#if browser.fileLoading}<RepositoryFileLoading
        path={path ?? "workspace file"}
        isMarkdown={isMarkdownRepositoryPath(path ?? "")}
      />
    {:else if browser.fileError || file?.unavailable}<p role="status">
        {browser.fileError || file?.unavailable}
      </p>
    {:else if file}
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
        <ChatTextBlock
          block={selectedBlock}
          state={comments}
          showHeader={false}
          fileView
          initialLine={browser.fileLine}
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
    font-size: var(--community-text-body);
  }
  .workspace-inspector-title {
    display: inline-flex;
    flex: 0 1 auto;
    min-width: 0;
    max-width: 35%;
    align-items: center;
    gap: 0.5rem;
  }
  .workspace-inspector-title :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: none;
  }
</style>

<script lang="ts">
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { tick, onDestroy } from "svelte";
  import ChatToolPanel from "../../web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte";
  import { ChatBodyComments } from "../../web/src/lib/ui/store-agent-chat-body/comments.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityMarkdownContent from "../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  import StoreAgentChatBody from "../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import type {
    ChatCommentAnchor,
    ChatCommentThread,
    ChatBodyTool,
  } from "../../web/src/lib/ui/store-agent-chat-body/model.js";
  import reference from "./delta-chat/reference.json";
  import { deltaChatItems } from "./delta-chat/adapter.js";
  import {
    playgroundItems,
    appendedItem,
    failedItem,
  } from "./delta-chat/playground.js";
  let {
    scenario = "reference",
    narrow = false,
  }: {
    scenario?: "reference" | "playground" | "failure" | "empty";
    narrow?: boolean;
  } = $props();
  const referenceItems = deltaChatItems(reference);
  let threads = $state<ChatCommentThread[]>([]);
  let appended = $state(false);
  let changed = $state(false);
  let mounted = $state(true);
  let serial = 0;
  let inspected = $state(false);
  const items = $derived(
    scenario === "reference"
      ? referenceItems
      : scenario === "empty"
        ? []
        : [
            ...playgroundItems.map((item) =>
              changed && item.id === "prose"
                ? {
                    ...item,
                    blocks: item.blocks.map((block) => ({
                      ...block,
                      revision: "demo-2",
                      text: "This is a different captured revision.",
                    })),
                  }
                : item,
            ),
            ...(scenario === "failure" ? [failedItem] : []),
            ...(appended ? [appendedItem] : []),
          ],
  );
  function add(anchor: ChatCommentAnchor, text: string) {
    const id = `comment-${++serial}`;
    threads = [
      ...threads,
      { id, anchor, comments: [{ id, author: "You", text }] },
    ];
  }
  function reply(threadId: string, text: string) {
    threads = threads.map((thread) =>
      thread.id === threadId
        ? {
            ...thread,
            comments: [
              ...thread.comments,
              { id: `comment-${++serial}`, author: "You", text },
            ],
          }
        : thread,
    );
  }
  const shell = new AppShellController({ leftClosed: true, rightClosed: true });
  onDestroy(() => {
    void shell.dispose();
  });
  const commentState = new ChatBodyComments(() => ({
    items,
    threads,
    onAddComment: add,
    onReply: reply,
  }));
  let selectedToolId = $state<string>();
  let toolOpener: HTMLElement | undefined;
  const selectedTool = $derived(
    items
      .flatMap((item) => item.tools ?? [])
      .find((tool) => tool.id === selectedToolId),
  );
  async function inspectTool(tool: ChatBodyTool, opener: HTMLElement) {
    selectedToolId = tool.id;
    toolOpener = opener;
    await tick();
    if (shell.mobile.resolvedMode === "mobile")
      shell.mobile.show("right", "chat-read-tool", opener);
  }
  async function closeTool() {
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    selectedToolId = undefined;
    await tick();
    toolOpener?.focus({ preventScroll: true });
  }
</script>

{#snippet customBlock(
  block: import("../../web/src/lib/ui/store-agent-chat-body/model.js").ChatBodyBlock,
)}
  {#if block.id === "custom/table"}
    <div class="chat-custom-example">
      <p>Custom Svelte view · decision summary</p>
      <dl>
        <dt>Comments</dt>
        <dd>Inline discussion</dd>
        <dt>Storage</dt>
        <dd>Story memory</dd>
      </dl>
      <Button
        variant="outline"
        size="sm"
        onclick={() => (inspected = !inspected)}
        aria-pressed={inspected}>Inspect decision</Button
      >
    </div>
  {:else}<CommunityMarkdownContent content={block.text} fullWidth />{/if}
{/snippet}

<div class="chat-body-fixture" data-narrow={narrow}>
  <AppShell.Root
    controller={shell}
    mobileBreakpoint={600}
    desktopMinMainWidth={0}
    style="height: 100%"
  >
    <AppShell.Main>
      <AppShell.Toolbar>
        <div class="chat-body-fixture-header">
          <div>
            <h1>
              {scenario === "reference"
                ? reference.source.title
                : "Commentable chat body"}
            </h1>
            <p>
              {scenario === "reference"
                ? "Delta reference · main thread"
                : "Synthetic interaction examples"}
            </p>
          </div>
          {#if scenario !== "reference" && scenario !== "empty"}<div
              class="chat-body-fixture-controls"
            >
              <Button
                variant="outline"
                size="sm"
                disabled={appended}
                onclick={() => (appended = true)}>Append response</Button
              >
              <Button
                variant="outline"
                size="sm"
                onclick={() => (changed = !changed)}>Replace content</Button
              >
              <Button
                variant="outline"
                size="sm"
                onclick={() => (mounted = !mounted)}
                >{mounted ? "Hide body" : "Show body"}</Button
              >
            </div>{/if}
        </div>
      </AppShell.Toolbar>
      <AppShell.Body
        layout="regions"
        label="Chat workspace"
        style="--ui-shell-body-sidebar-width: min(26rem, 42vw)"
      >
        <AppShell.Body.Content label="Conversation document">
          <div class="chat-body-fixture-body">
            {#if mounted}<StoreAgentChatBody
                {items}
                {threads}
                onAddComment={add}
                onReply={reply}
                {customBlock}
                {commentState}
                onInspectTool={inspectTool}
              />{/if}
          </div>
        </AppShell.Body.Content>
        {#if selectedTool}<AppShell.Body.Sidebar
            side="right"
            panelId="chat-read-tool"
            label="Read tool details"
            mobileLabel="Captured read"
          >
            <ChatToolPanel
              tool={selectedTool}
              state={commentState}
              onClose={closeTool}
              {customBlock}
            />
          </AppShell.Body.Sidebar>{/if}
      </AppShell.Body>
    </AppShell.Main>
  </AppShell.Root>
</div>

<style>
  .chat-custom-example p {
    margin-block: 0 0.75rem;
    font-weight: 600;
  }
  .chat-custom-example dl {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.25rem 1rem;
  }
  .chat-custom-example dd {
    margin: 0;
  }

  .chat-body-fixture {
    height: min(900px, 100vh);
    display: flex;
    flex-direction: column;
    background: var(--background);
    color: var(--foreground);
  }
  .chat-body-fixture[data-narrow="true"] {
    width: min(100%, 390px);
    margin-inline: auto;
  }
  .chat-body-fixture-header {
    display: flex;
    padding: 0.5rem 1rem;
    gap: 1rem;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
  }
  .chat-body-fixture-header h1 {
    margin: 0;
    font-size: 0.85rem;
    font-weight: 600;
  }
  .chat-body-fixture-header p {
    margin: 0.25rem 0 0;
    font-size: 0.8rem;
    color: var(--muted-foreground);
  }
  .chat-body-fixture-controls {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .chat-body-fixture-body {
    position: absolute;
    inset: 0;
  }
</style>

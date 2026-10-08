<script lang="ts">
  import "./delta-chat/fixture.css";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { tick, onDestroy, untrack } from "svelte";
  import ChatToolPanel from "../../web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte";
  import { ChatBodyComments } from "../../web/src/lib/ui/store-agent-chat-body/comments.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityMarkdownContent from "../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  import StoreAgentChatBody from "../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import type { ChatBodyTool } from "../../web/src/lib/ui/store-agent-chat-body/model.js";
  import helloWorld from "./delta-chat/hello-world.json";
  import { deltaChatThreads } from "./delta-chat/discussions.js";
  import { referenceDiscussions } from "./delta-chat/reference-discussions.js";
  import reference from "./delta-chat/reference.json";
  import { deltaChatItems } from "./delta-chat/adapter.js";
  import { largeCaptureItems } from "./delta-chat/large-captures.js";
  import { DemoReactions } from "./delta-chat/local-reactions.svelte.js";
  import {
    DemoNextMessage,
    demoSender,
  } from "./delta-chat/next-message.svelte.js";
  import ChatNextMessage from "../../web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte";
  import ChatMessageFooter from "../../web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte";
  import ChatRuntimeControls from "../../web/src/lib/ui/store-agent-chat-body/ChatRuntimeControls.svelte";
  import { DemoRuntime, demoAgents } from "./delta-chat/runtime.svelte.js";
  import {
    playgroundItems,
    appendedItem,
    failedItem,
  } from "./delta-chat/playground.js";
  let {
    scenario = "reference",
    narrow = false,
    showComposer,
  }: {
    scenario?:
      "reference" | "playground" | "failure" | "empty" | "large" | "comments";
    narrow?: boolean;
    showComposer?: boolean;
  } = $props();
  const referenceDemo = referenceDiscussions(deltaChatItems(reference));
  const commentItems = deltaChatItems(helloWorld);
  const reactions = new DemoReactions();
  const runtime = new DemoRuntime();
  const nextMessage = new DemoNextMessage(() => runtime.snapshot());
  const composing = $derived(showComposer ?? scenario !== "empty");
  let draftEditor = $state<{ focus: () => void }>();
  async function sendMessage() {
    if (await commentState.send()) {
      await tick();
      draftEditor?.focus();
    }
  }
  let appended = $state(false);
  let changed = $state(false);
  let mounted = $state(true);
  let inspected = $state(false);
  const capturedItems = $derived(
    scenario === "comments"
      ? commentItems
      : scenario === "reference"
        ? referenceDemo.items
        : scenario === "large"
          ? largeCaptureItems
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
  const items = $derived([...capturedItems, ...nextMessage.messages]);
  const shell = new AppShellController({
    leftClosed: true,
    rightClosed: true,
    rightWidth: 416,
    sidebarMinWidth: 280,
    sidebarMaxWidth: 720,
  });
  onDestroy(() => {
    void shell.dispose();
  });
  const commentState = new ChatBodyComments(
    () => ({
      items,
      author: demoSender,
      onSend: (submission) => nextMessage.accept(submission),
      onInspectTool: inspectTool,
      onReact: reactions.toggle,
      reactionOptions: ["👍", "❤️", "✅"],
    }),
    untrack(() =>
      scenario === "comments"
        ? deltaChatThreads(helloWorld, commentItems)
        : scenario === "reference"
          ? referenceDemo.threads
          : [],
    ),
  );
  untrack(() => {
    if (scenario === "comments") commentState.message = helloWorld.draftText;
  });
  $effect(() => {
    commentState.focusMessage = () => draftEditor?.focus();
  });
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
    shell.right.expand();
    await tick();
    if (shell.mobile.resolvedMode === "mobile")
      shell.mobile.show("right", "right", opener);
  }
  async function closeTool() {
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    shell.right.setClosed(true);
    selectedToolId = undefined;
    await tick();
    toolOpener?.focus({ preventScroll: true });
  }
</script>

{#snippet nextDraft()}<ChatNextMessage
    bind:this={draftEditor}
    bind:value={commentState.message}
    member={demoSender}
    disabled={commentState.sending}
    hideAvatar={commentState.pending.length > 0}
    onSubmit={sendMessage}
  />{/snippet}
{#snippet nextActions()}<ChatMessageFooter
    canSend={commentState.canSend}
    onSend={sendMessage}
    error={commentState.error}
    >{#snippet controls()}<ChatRuntimeControls
        agents={demoAgents}
        agent={runtime.agent}
        models={runtime.models}
        model={runtime.model}
        thinkingOptions={runtime.thinkingOptions}
        thinking={runtime.thinking}
        disabled={commentState.sending}
        onAgentChange={(id) => (runtime.agentId = id)}
        onModelChange={(value) => (runtime.model = value)}
        onThinkingChange={(value) => (runtime.thinking = value)}
      />{/snippet}</ChatMessageFooter
  >{/snippet}

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
  <CommunityAppShell
    controller={shell}
    mobileBreakpoint={600}
    desktopMinMainWidth={360}
  >
    <AppShell.Main>
      <AppShell.Toolbar style="border-block-end: 1px solid var(--border)">
        <div class="chat-body-fixture-header">
          <div>
            <h1>
              {scenario === "comments"
                ? helloWorld.source.title
                : scenario === "reference"
                  ? reference.source.title
                  : "Commentable chat body"}
            </h1>
            {#if scenario !== "reference"}<p>
                Synthetic interaction examples
              </p>{/if}
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
              <Button
                variant="outline"
                size="sm"
                onclick={() => nextMessage.completeReplies(commentState)}
                >Simulate replies</Button
              >
              <Button
                variant="outline"
                size="sm"
                onclick={() => (nextMessage.failNext = true)}
                >Fail next send</Button
              >
            </div>{/if}
        </div>
      </AppShell.Toolbar>
      <AppShell.Body layout="regions" label="Chat workspace">
        <AppShell.Body.Content label="Conversation document">
          <div class="chat-body-fixture-body">
            {#if mounted}<StoreAgentChatBody
                {items}
                reactions={reactions.values}
                onReact={reactions.toggle}
                composer={composing ? nextDraft : undefined}
                footer={composing ? nextActions : undefined}
                reactionOptions={["👍", "❤️", "✅"]}
                {customBlock}
                {commentState}
                onInspectTool={inspectTool}
              />{/if}
          </div>
        </AppShell.Body.Content>
      </AppShell.Body>
    </AppShell.Main>
    {#if selectedTool}<AppShell.Sidebar
        side="right"
        label="Read tool details"
        mobileLabel="Captured read"
        resizeLabel="Resize read tool details"
        constrainedPresentation="replace-main"
        closeable
      >
        <ChatToolPanel
          tool={selectedTool}
          state={commentState}
          onClose={closeTool}
          {customBlock}
        />
      </AppShell.Sidebar>{/if}
  </CommunityAppShell>
</div>

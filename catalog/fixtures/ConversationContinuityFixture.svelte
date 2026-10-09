<script lang="ts">
  import type { Snippet } from "svelte";
  let inputActions = $state<Snippet>();
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ConversationSession } from "../../web/src/lib/conversations/session.svelte.js";
  import StoreAgentChatBody from "../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import CommunityChatComposer from "../../web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte";
  import ChatMessageFooter from "../../web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte";
  const conversation = new ConversationSession();
  let view = $state<"document" | "classic">("document"),
    mounted = $state(true),
    output = $state("First"),
    committed = $state(false);
  const user = { id: "1".repeat(32), name: "You", handle: "", initials: "YO" };
  const target = {
    type: "human_message" as const,
    thread_id: "2".repeat(32),
    message_id: "3".repeat(32),
    revision: { kind: "human_message_revision" as const, id: "4".repeat(64) },
  };
  const block = {
    id: "source",
    revision: target.revision.id,
    kind: "markdown" as const,
    label: "Source",
    text: "Hello **world** 🦀",
  };
  $effect(() => {
    const text = output,
      done = committed;
    untrack(() =>
      conversation.adopt({
        items: [
          {
            id: "source",
            kind: "message",
            role: "user",
            author: user,
            label: "Source",
            blocks: [block],
          },
          {
            id: "stream",
            kind: "message",
            role: "assistant",
            author: { ...user, name: "Agent" },
            label: "Stream",
            blocks: [
              {
                id: "stream-text",
                revision: done ? "committed" : "pending",
                kind: "markdown",
                label: "Stream",
                text,
                commentable: done,
              },
            ],
          },
        ],
        sources: new Map([
          [
            block.id,
            {
              target,
              part: null,
              text: block.text,
              from: 0,
              revision: block.revision,
            },
          ],
        ]),
        threads: [],
        targets: new Map(),
      }),
    );
  });
  conversation.host = {
    author: user,
    onSend: () => ({ publication: "published" }),
  };
</script>

<div class="fixture">
  <div role="group" aria-label="Fixture controls">
    <Button
      onclick={() => (view = view === "document" ? "classic" : "document")}
      >Switch renderer</Button
    ><Button
      onclick={() => {
        mounted = false;
        setTimeout(() => (mounted = true), 0);
      }}>Remount body</Button
    ><Button onclick={() => (output += " next 🦀")}>Stream chunk</Button><Button
      onclick={() => (committed = true)}>Commit capture</Button
    >
  </div>
  {#if mounted}<StoreAgentChatBody
      items={conversation.projection.items}
      commentState={conversation.comments}
      {view}
    >
      {#snippet composer(presentation)}<CommunityChatComposer
          {presentation}
          dockActions
          bind:documentActions={inputActions}
          member={user}
          hideAvatar={conversation.comments.pending.length > 0}
          bind:value={conversation.comments.message}
          placeholder=""
          enterInsertsNewline
          clearOnSubmit={false}
          showSubmitButton={false}
          allowEmpty={conversation.comments.pending.length > 0}
          onSubmit={() => conversation.comments.send().then(() => {})}
        />{/snippet}
      {#snippet footer()}<ChatMessageFooter
          actions={inputActions}
          canSend={conversation.comments.canSend}
          onSend={() => void conversation.comments.send()}
        />{/snippet}
    </StoreAgentChatBody>{/if}
</div>

<style>
  .fixture {
    height: 750px;
    display: flex;
    flex-direction: column;
  }
</style>

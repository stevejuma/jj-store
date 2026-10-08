<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ThreadsController } from "../../controller/thread-types.js";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../../community/workspace-model.js";
  import type { ChatBodyItem } from "../store-agent-chat-body/model.js";
  let {
    item,
    messages,
    threads,
    conversation,
    controller,
    disabled,
    user,
    onReply,
    id,
  }: {
    item: ChatBodyItem;
    messages: readonly CommunityMessageModel[];
    threads: ThreadsController;
    conversation: ConversationSession;
    controller?: StoreController;
    disabled: boolean;
    user: CommunityMemberModel;
    onReply?: (id: string) => void;
    id: string;
  } = $props();
  async function saveEdit() {
    const edit = conversation.editing;
    if (!edit || !edit.text.trim() || disabled) return;
    try {
      await threads.submitConversation({
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        command: {
          type: "message",
          thread_id: id,
          expected_thread: edit.thread,
          message_id: edit.id,
          expected_revision: edit.revision,
          reply_to:
            threads.events.find((e) => e.message?.message_id === edit.id)
              ?.message?.reply_to ?? null,
          body: edit.text,
        },
      });
      conversation.editing = undefined;
    } catch (cause) {
      conversation.comments.error =
        cause instanceof Error ? cause.message : String(cause);
    }
  }
  function editMessage(message: CommunityMessageModel) {
    const target = threads.projection.targets[message.id];
    if (target?.type === "human_message" && threads.detail)
      conversation.editing = {
        id: message.id,
        text: message.content,
        revision: target.revision,
        thread: threads.detail.thread.reference,
      };
  }
  function copyLink(messageId: string) {
    if (!controller) return;
    const hash = storeRouteToHash({
      ...controller.state.route,
      replyId: messageId,
    });
    if (hash)
      void navigator.clipboard.writeText(
        new URL(hash, window.location.href).href,
      );
  }
</script>

{#if messages.some((m) => m.id === item.id)}<div
    class="conversation-message-actions"
  >
    {#if onReply}<Button
        variant="ghost"
        size="sm"
        onclick={() => onReply?.(item.id)}
        >{disabled ? "View replies" : "Reply in thread"}</Button
      >{/if}
    {#if controller}<Button
        variant="ghost"
        size="sm"
        onclick={() => copyLink(item.id)}>Copy link</Button
      >{/if}
    {#if !disabled && messages.find((m) => m.id === item.id)?.authorId === user.id}<Button
        variant="ghost"
        size="sm"
        {disabled}
        onclick={() => editMessage(messages.find((m) => m.id === item.id)!)}
        >Edit message</Button
      >{/if}
    <Button
      variant="ghost"
      size="sm"
      onclick={() =>
        navigator.clipboard.writeText(
          messages.find((m) => m.id === item.id)?.content ?? "",
        )}>Copy message</Button
    >
  </div>

  {#if conversation.editing?.id === item.id}<CommunityChatComposer
      bind:value={conversation.editing.text}
      {disabled}
      ariaLabel="Edit message"
      enterInsertsNewline
      clearOnSubmit={false}
      onSubmit={saveEdit}
    /><Button variant="ghost" onclick={() => (conversation.editing = undefined)}
      >Cancel edit</Button
    >{/if}
{/if}

<style>
  .conversation-message-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }
</style>

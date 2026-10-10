<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import MoreIcon from "@lucide/svelte/icons/ellipsis";
  import ReplyIcon from "@lucide/svelte/icons/reply";
  import LinkIcon from "@lucide/svelte/icons/link";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import CopyIcon from "@lucide/svelte/icons/copy";
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
    menu = false,
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
    menu?: boolean;
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

{#if messages.some((m) => m.id === item.id)}
  {#if menu}<DropdownMenu.Root>
      <DropdownMenu.Trigger
        >{#snippet child({ props })}<Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="More message actions"
            title="More message actions"><MoreIcon aria-hidden="true" /></Button
          >{/snippet}</DropdownMenu.Trigger
      >
      <DropdownMenu.Content align="end">
        <DropdownMenu.Group>
          {#if onReply}<DropdownMenu.Item onSelect={() => onReply?.(item.id)}
              ><ReplyIcon aria-hidden="true" />{disabled
                ? "View replies"
                : "Reply in thread"}</DropdownMenu.Item
            >{/if}
          {#if controller}<DropdownMenu.Item onSelect={() => copyLink(item.id)}
              ><LinkIcon aria-hidden="true" />Copy link</DropdownMenu.Item
            >{/if}
          {#if !disabled && messages.find((m) => m.id === item.id)?.authorId === user.id}<DropdownMenu.Item
              onSelect={() =>
                editMessage(messages.find((m) => m.id === item.id)!)}
              ><PencilIcon aria-hidden="true" />Edit message</DropdownMenu.Item
            >{/if}
          <DropdownMenu.Item
            onSelect={() =>
              navigator.clipboard.writeText(
                messages.find((m) => m.id === item.id)?.content ?? "",
              )}><CopyIcon aria-hidden="true" />Copy message</DropdownMenu.Item
          >
        </DropdownMenu.Group>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {:else if conversation.editing?.id === item.id}<CommunityChatComposer
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

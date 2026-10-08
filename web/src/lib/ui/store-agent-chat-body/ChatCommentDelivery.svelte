<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import CheckCheckIcon from "@lucide/svelte/icons/check-check";
  import ArrowDownRightIcon from "@lucide/svelte/icons/arrow-down-right";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatComment } from "./model.js";
  const statusId = $props.id();
  let { state, comment }: { state: ChatBodyComments; comment: ChatComment } =
    $props();
</script>

{#if comment.status === "replied" && comment.replyItemId}
  <span id={statusId} class="sr-only">Replied</span>
  <Tooltip.Provider delayDuration={200}>
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-xs"
            class="chat-comment-reply-action"
            aria-label="Go to reply"
            aria-describedby={`${props["aria-describedby"] ?? ""} ${statusId}`}
            onclick={() => state.goToReply(comment.replyItemId!)}
          >
            <span class="chat-comment-delivered-icon" aria-hidden="true"
              ><CheckCheckIcon /></span
            >
            <span class="chat-comment-jump-icon" aria-hidden="true"
              ><ArrowDownRightIcon /></span
            >
          </Button>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content side="top" role="tooltip">Go to reply</Tooltip.Content>
    </Tooltip.Root>
  </Tooltip.Provider>
{:else if comment.status === "sent"}
  <span class="chat-comment-complete" role="img" aria-label="Sent" title="Sent"
    ><CheckCheckIcon size={16} /></span
  >
{:else if comment.status === "sending"}
  <span class="chat-comment-status" role="status">Sending</span>
{/if}

<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import MessageSquareReplyIcon from "@lucide/svelte/icons/message-square-reply";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import "../editor-focus.css";
  import type { Model } from "../../api/client.js";
  import {
    anchorAgentTool,
    reviewForAgentTool,
  } from "../../collaboration/presentation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreAgentToolDetail from "./StoreAgentToolDetail.svelte";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";

  let {
    tool,
    patches = {},
    controller,
    interactive = true,
  }: {
    tool: Model<"AgentToolActivity">;
    patches?: Readonly<Record<string, string>>;
    controller: StoreController;
    interactive?: boolean;
  } = $props();

  let draftOpen = $state(false);
  let draft = $state("");
  let actionsVisible = $state(false);
  let menuOpen = $state(false);

  const selected = $derived.by(() => {
    const detail = controller.state.reviewThread;
    if (!detail) return false;
    const owner = anchorAgentTool(detail.anchor.value);
    return Boolean(
      owner &&
      owner.sessionId === controller.state.agentSession?.session.session_id &&
      owner.toolCallId === tool.tool_call_id,
    );
  });
  const review = $derived.by(
    () =>
      reviewForAgentTool(
        controller.state.reviewThreads,
        controller.state.agentSession?.session.session_id ?? "",
        tool.tool_call_id,
      ) ?? (selected ? controller.state.reviewThread?.thread : undefined),
  );

  function status(state: Model<"ToolState">) {
    if (state === "succeeded") return "complete" as const;
    if (state === "failed" || state === "cancelled" || state === "interrupted")
      return "error" as const;
    if (state === "running") return "running" as const;
    if (state === "pending") return "pending" as const;
    return "unknown" as const;
  }

  function target(): string | undefined {
    return (
      tool.code.flatMap((association) => association.paths)[0] ??
      tool.observed_paths[0] ??
      Chat.toolCallTarget(tool.input?.content)
    );
  }

  async function beginReply() {
    if (
      review &&
      controller.state.reviewThread?.thread.thread_id !== review.thread_id
    )
      await controller.actions.selectReviewThread(review.thread_id);
    draftOpen = true;
    draft = "";
  }

  async function submit(content: string) {
    await controller.actions.commentOnAgentTool(tool.tool_call_id, content);
    if (controller.state.status.reviewAction === "ready") {
      draft = "";
      draftOpen = false;
    }
  }
</script>

{#snippet detail()}
  <StoreAgentToolDetail {tool} {patches} />
{/snippet}

<div
  class="store-agent-tool-review"
  data-ui-part="agent-tool-review"
  role="group"
  aria-label={`${tool.native_name} tool call`}
  onpointerenter={() => (actionsVisible = true)}
  onpointerleave={() => {
    if (!draftOpen) actionsVisible = false;
  }}
  onfocusin={() => (actionsVisible = true)}
  onfocusout={(event) => {
    const next = event.relatedTarget;
    if (
      next instanceof Node &&
      event.currentTarget instanceof Node &&
      event.currentTarget.contains(next)
    )
      return;
    if (!draftOpen) actionsVisible = false;
  }}
>
  <Chat.ToolCalls
    label="Tool activity"
    calls={[
      {
        id: tool.tool_call_id,
        name: tool.native_name,
        status: status(tool.state),
        target: target(),
        errorMessage: tool.state === "failed" ? "Tool failed" : undefined,
        detail,
      },
    ]}
  />
  <div
    class="store-agent-tool-review__actions"
    role="group"
    aria-label={`Actions for ${tool.native_name} tool call`}
    data-visible={actionsVisible || menuOpen ? "" : undefined}
  >
    {#if interactive && review}
      <span
        class="store-agent-tool-review__count"
        aria-label={`${review.comment_count} comments`}
        title={`${review.comment_count} comments`}
      >
        <MessageSquareIcon aria-hidden="true" />
        {review.comment_count}
      </span>
    {/if}
    {#if interactive}<Button
        type="button"
        variant="ghost"
        size="icon-sm"
        aria-label={`Reply to ${tool.native_name} tool call`}
        title="Reply to tool call"
        onclick={() => void beginReply()}
      >
        <MessageSquareReplyIcon aria-hidden="true" />
      </Button>{/if}
    <StoreRecordMenu
      label={`More ${tool.native_name} tool actions`}
      onOpenChange={(open) => (menuOpen = open)}
      items={[
        { label: "Copy tool call ID", value: tool.tool_call_id },
        { label: "Copy snapshot ID", value: tool.snapshot_id },
        { label: "Copy turn ID", value: tool.turn_id },
        { label: "Copy tool name", value: tool.native_name },
        { label: "Copy input", value: tool.input?.content },
        { label: "Copy output", value: tool.output?.content },
        {
          label: "Copy paths",
          value: tool.observed_paths.length
            ? tool.observed_paths.join("\n")
            : null,
        },
      ]}
    />
  </div>
  {#if interactive && selected}
    <div class="store-agent-tool-review__thread">
      <StoreReviewConversation
        {controller}
        inline
        showComposer={false}
        showDiff={false}
      />
    </div>
  {/if}
  {#if draftOpen}
    <div
      class="store-agent-tool-review__composer"
      data-ui-part="agent-tool-reply-composer"
    >
      <CommunityChatComposer
        mode="form"
        showSubmitButton
        bind:value={draft}
        disabled={controller.state.status.reviewAction === "refreshing" ||
          !!controller.state.reviewActionRequestId}
        busy={controller.state.status.reviewAction === "refreshing"}
        ariaLabel={`Reply to ${tool.native_name} tool call`}
        submitLabel="Reply"
        placeholder="Reply to this tool call"
        onCancel={() => {
          draft = "";
          draftOpen = false;
        }}
        onSubmit={(submission) => submit(submission.content)}
      />
      {#if controller.state.status.reviewAction === "failed" || controller.state.status.reviewAction === "unavailable"}
        <p role="alert">{controller.state.reviewActionMessage}</p>
      {/if}
    </div>
  {/if}
</div>

<style>
  .store-agent-tool-review {
    position: relative;
    min-width: 0;
    border-radius: var(--radius);
  }

  .store-agent-tool-review__actions {
    position: absolute;
    z-index: 3;
    inset-block-start: 0;
    inset-inline-end: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.125rem;
    padding: 0.125rem;
    border: 1px solid var(--border);
    border-radius: 999px;
    opacity: 0;
    background: var(--popover, var(--background));
    box-shadow: var(--shadow-sm);
    pointer-events: none;
    transition:
      opacity 120ms ease,
      transform 120ms ease;
    transform: translateY(calc(-100% + 0.15rem));
  }

  .store-agent-tool-review__actions[data-visible] {
    opacity: 1;
    pointer-events: auto;
    transform: translateY(calc(-100% + 0.3rem));
  }

  .store-agent-tool-review__count {
    display: inline-flex;
    align-items: center;
    gap: 0.2rem;
    padding-inline: 0.35rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .store-agent-tool-review__count :global(svg) {
    width: 0.8rem;
    height: 0.8rem;
  }

  .store-agent-tool-review__thread,
  .store-agent-tool-review__composer {
    min-width: 0;
    margin: 0.35rem 0.25rem 0.5rem 4.85rem;
  }

  .store-agent-tool-review__composer > p {
    margin: 0.4rem 0 0;
    color: var(--destructive);
    font-size: var(--community-text-secondary);
  }

  @media (hover: none) {
    .store-agent-tool-review__actions {
      position: relative;
      inset: auto;
      width: fit-content;
      margin-inline-start: auto;
      opacity: 1;
      pointer-events: auto;
      transform: none;
    }
  }
</style>

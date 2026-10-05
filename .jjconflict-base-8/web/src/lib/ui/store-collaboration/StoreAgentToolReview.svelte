<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import MessageSquareReplyIcon from "@lucide/svelte/icons/message-square-reply";
  import { reviewDraft } from "./review-drafts.svelte.js";
  import StoreAgentToolDiscussion from "./StoreAgentToolDiscussion.svelte";
  import "../editor-focus.css";
  import type { Model } from "../../api/client.js";
  import type { ReviewDetail } from "../../controller/repository-read-types.js";
  import {
    anchorAgentTool,
    reviewForAgentTool,
  } from "../../collaboration/presentation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";

  let {
    tool,
    children,
    controller,
    interactive = true,
  }: {
    tool: Model<"AgentToolActivity">;
    children: Snippet;
    controller: StoreController;
    interactive?: boolean;
  } = $props();

  import { useDrafts } from "../../appearance/drafts.svelte.js";

  const drafts = useDrafts();
  const draft = $derived(
    drafts.get(
      `tool:${controller.state.repository.id}:${controller.state.agentSession?.session.session_id}:${tool.tool_call_id}`,
    ),
  );
  $effect(() => {
    if (
      draft.pendingRequestId &&
      controller.state.committedReviewRequestId === draft.pendingRequestId
    ) {
      draft.value = "";
      draft.pendingRequestId = null;
    }
  });
  let draftOpen = $derived(draft.open ?? false);
  const writable = $derived(
    interactive &&
      ["write", "admin"].includes(controller.state.repository.permission ?? ""),
  );
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

  let comments = $state<ReviewDetail>();
  const visibleComments = $derived.by(() => {
    const owner = comments && anchorAgentTool(comments.anchor.value);
    return owner?.sessionId ===
      controller.state.agentSession?.session.session_id &&
      owner?.toolCallId === tool.tool_call_id
      ? comments
      : undefined;
  });
  let commentsError = $state("");
  $effect(() => {
    const currentDetail = selected ? controller.state.reviewThread : undefined;
    const sessionId = controller.state.agentSession?.session.session_id;
    void controller.state.reviewThreads;
    if (!interactive || !sessionId) {
      comments = undefined;
      return;
    }
    if (currentDetail) {
      comments = currentDetail;
      return;
    }
    const read = controller.actions.readAgentToolReview;
    if (!read) return;
    let active = true;
    untrack(
      () =>
        void read(tool.tool_call_id)
          .then((detail) => {
            if (active) {
              comments = detail;
              commentsError = "";
            }
          })
          .catch((cause) => {
            if (active)
              commentsError =
                cause instanceof Error
                  ? cause.message
                  : "Could not load tool comments.";
          }),
    );
    return () => {
      active = false;
    };
  });
  async function activateComments() {
    const id = visibleComments?.thread.thread_id ?? review?.thread_id;
    if (id && controller.state.reviewThread?.thread.thread_id !== id)
      await controller.actions.selectReviewThread(id);
  }
  async function beginReply() {
    if (!writable || controller.state.reviewActionRequestId) return;
    const owner = controller,
      currentDraft = draft;
    const id = visibleComments?.thread.thread_id ?? review?.thread_id;
    await activateComments();
    if (id) {
      const current = reviewDraft(owner, id);
      current.reply = null;
      current.edit = null;
    }
    currentDraft.open = true;
  }

  async function submit(content: string) {
    const owner = controller,
      currentDraft = draft;
    const committed = owner.state.committedReviewRequestId;
    await owner.actions.commentOnAgentTool(tool.tool_call_id, content);
    if (
      owner.state.status.reviewAction === "ready" &&
      !owner.state.reviewActionRequestId &&
      owner.state.committedReviewRequestId !== committed
    ) {
      currentDraft.value = "";
    } else {
      currentDraft.pendingRequestId = owner.state.reviewActionRequestId;
      throw new Error(
        owner.state.reviewActionMessage ||
          "Tool comment has not been confirmed.",
      );
    }
  }
</script>

<div
  class="store-agent-tool-review"
  data-ui-part="agent-tool-review"
  role="group"
  aria-label={`${tool.native_name} tool call`}
>
  <div
    class="store-agent-tool-review__call"
    role="group"
    aria-label={`${tool.native_name} tool row`}
    onpointerenter={() => (actionsVisible = true)}
    onpointerleave={() => {
      actionsVisible = false;
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
      actionsVisible = false;
    }}
  >
    {@render children()}
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
      {#if writable}<Button
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
  </div>
  {#if commentsError}<p role="alert">{commentsError}</p>{/if}
  {#if visibleComments || (interactive && draftOpen)}
    <StoreAgentToolDiscussion
      {tool}
      {controller}
      {draft}
      detail={visibleComments}
      showComposer={writable && draftOpen}
      onActivate={activateComments}
      onSubmit={submit}
      onChange={(value) => (draft.value = value)}
      onCancel={() => {
        draft.value = "";
        draft.open = false;
      }}
    />
  {/if}
</div>

<style>
  .store-agent-tool-review,
  .store-agent-tool-review__call {
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
    transform: none;
  }

  .store-agent-tool-review__actions[data-visible] {
    opacity: 1;
    pointer-events: auto;
    transform: none;
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

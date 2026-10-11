<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import MessageSquarePlusIcon from "@lucide/svelte/icons/message-square-plus";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import ChatDisclosureIcon from "../store-agent-chat-body/ChatDisclosureIcon.svelte";
  import { principalMember } from "../../collaboration/presentation.js";
  import "../store-agent-chat-body/comment-card.css";
  import type { ReviewDetail } from "../../controller/repository-read-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { useDrafts } from "../../appearance/drafts.svelte.js";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";
  let {
    controller,
    children,
    inlineComposer = false,
  }: {
    controller?: StoreController;
    children: Snippet<[Snippet, Snippet]>;
    inlineComposer?: boolean;
  } = $props();
  const drafts = useDrafts();
  const sessionId = $derived(
    controller?.state.agentSession?.session.session_id,
  );
  const supported = $derived(
    !!controller &&
      controller.state.repository.profile !== "git_sha1_jj045" &&
      !!controller.actions.commentOnAgentSession &&
      !!controller.actions.readAgentSessionReview,
  );
  const writable = $derived(
    supported &&
      ["write", "admin"].includes(
        controller?.state.repository.permission ?? "",
      ),
  );
  const draft = $derived(
    drafts.get(`agent-session:${controller?.state.repository.id}:${sessionId}`),
  );
  let detail = $state<ReviewDetail>();
  const visibleDetail = $derived(
    detail?.anchor.value.target.target === "agent_session" &&
      detail.anchor.value.target.value.session_id === sessionId
      ? detail
      : undefined,
  );
  let error = $state("");
  let loading = $state(true);
  let retry = $state(0);
  $effect(() => {
    const id = sessionId;
    const selected = controller?.state.reviewThread;
    void controller?.state.reviewThreads;
    void retry;
    if (!id || !supported) {
      detail = undefined;
      loading = false;
      return;
    }
    if (
      selected?.anchor.value.target.target === "agent_session" &&
      selected.anchor.value.target.value.session_id === id
    ) {
      detail = selected;
      loading = false;
      error = "";
      return;
    }
    let active = true;
    loading = true;
    error = "";
    untrack(
      () =>
        void controller?.actions
          .readAgentSessionReview?.()
          .then((result) => {
            if (active) {
              detail = result;
              loading = false;
              error = "";
            }
          })
          .catch((cause) => {
            if (active) {
              loading = false;
              error =
                cause instanceof Error
                  ? cause.message
                  : "Could not load session comments.";
            }
          }),
    );
    return () => {
      active = false;
    };
  });
  $effect(() => {
    if (
      draft.pendingRequestId &&
      controller?.state.committedReviewRequestId === draft.pendingRequestId
    ) {
      draft.value = "";
      draft.pendingRequestId = null;
    }
  });
  const disabled = $derived(
    !writable ||
      loading ||
      !!error ||
      controller?.state.status.reviewAction === "refreshing" ||
      !!controller?.state.reviewActionRequestId ||
      !!draft.pendingRequestId,
  );
  async function activate() {
    if (
      controller &&
      visibleDetail &&
      controller.state.reviewThread?.thread.thread_id !==
        visibleDetail.thread.thread_id
    )
      await controller.actions.selectReviewThread(
        visibleDetail.thread.thread_id,
      );
  }
  async function submit(content: string) {
    if (disabled || !controller) return;
    const owner = controller;
    const currentDraft = draft,
      committed = owner.state.committedReviewRequestId;
    await owner.actions.commentOnAgentSession?.(content);
    if (
      owner.state.status.reviewAction === "ready" &&
      owner.state.committedReviewRequestId !== committed
    )
      currentDraft.value = "";
    else {
      currentDraft.pendingRequestId = owner.state.reviewActionRequestId;
      throw new Error(
        owner.state.reviewActionMessage ||
          "Session comment has not been confirmed.",
      );
    }
  }
</script>

{#snippet comments()}
  {#if supported && controller && (visibleDetail || (inlineComposer && writable))}<section
      aria-label="Session comments"
      class="store-agent-comments"
    >
      <h2>Session comments</h2>
      {#if visibleDetail}<StoreReviewConversation
          rootDepth={1}
          {controller}
          reviewDetail={visibleDetail}
          onActivate={activate}
          inline
          showComposer={false}
          showDiff={false}
        />{/if}
      {#if inlineComposer && writable}{@render composer()}{/if}
    </section>{/if}
  {#if supported && error}<div class="store-agent-comments">
      <p role="alert">{error}</p>
      <Button variant="secondary" size="sm" onclick={() => retry++}
        >Retry session comments</Button
      >
    </div>{/if}
{/snippet}
{#snippet composer()}
  {#if writable && sessionId && controller}<div
      class="store-agent-comments__composer"
      data-ui-part="agent-session-review"
    >
      <Collapsible.Root
        class="chat-comment-context"
        open={!!draft.open || !!draft.pendingRequestId}
        disabled={!!draft.pendingRequestId}
        onOpenChange={(open) => (draft.open = open)}
      >
        <Collapsible.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="sm"
              class="store-agent-comments__toggle"
            >
              <MessageSquarePlusIcon data-icon="inline-start" />
              Add review
              <ChatDisclosureIcon />
            </Button>
          {/snippet}
        </Collapsible.Trigger>
        <Collapsible.Content class="store-agent-comments__draft">
          {#if draft.open || draft.pendingRequestId}
            <div class="chat-comment-list" data-ui-part="agent-session-footer">
              <div class="chat-comment-card">
                <CommunityMemberAvatar
                  member={principalMember(
                    controller.state.repository.principal ?? "local",
                  )}
                  size="xsmall"
                  decorative
                />
                <div class="chat-comment-content">
                  <CommunityChatComposer
                    mode="form"
                    hideAvatar
                    showSubmitButton
                    bind:value={draft.value}
                    {disabled}
                    busy={controller?.state.status.reviewAction ===
                      "refreshing"}
                    ariaLabel="Agent session comment"
                    submitLabel="Add comment"
                    placeholder="Add a comment about this session…"
                    onSubmit={(submission) => submit(submission.content)}
                  />
                </div>
              </div>
            </div>
            {#if draft.pendingRequestId}<div class="chat-comment-status">
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => controller?.actions.checkReviewAction()}
                  >Check comment status</Button
                >
              </div>{/if}
          {/if}
        </Collapsible.Content>
      </Collapsible.Root>
    </div>{/if}
{/snippet}
{@render children(comments, composer)}

<style>
  .store-agent-comments {
    min-width: 0;
    padding: 0.75rem;
  }
  h2 {
    margin: 0 0 0.5rem;
    font-size: var(--community-text-body);
    font-weight: 600;
  }
  .store-agent-comments__composer {
    min-width: 0;
    padding: 0.5rem 0.75rem;
  }
  :global(.store-agent-comments__toggle) {
    width: 100%;
    justify-content: flex-start;
  }
  :global(.store-agent-comments__draft) {
    padding-block-start: 0.5rem;
  }
  .store-agent-comments__composer :global(.community-chat-composer__surface) {
    --ui-ai-chat-composer-radius: 0;
    --ui-ai-chat-composer-padding: 0;
  }
  .store-agent-comments__composer
    :global(.community-chat-composer__surface > [data-ui-part="body"]) {
    border: 0;
    padding: 0;
    box-shadow: none;
  }
</style>

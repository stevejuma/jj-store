<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { reviewDraft } from "./review-drafts.svelte.js";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";
  import type { Model } from "../../api/client.js";
  import type { ReviewDetail } from "../../controller/repository-read-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ConversationDraft } from "../../appearance/drafts.svelte.js";
  let {
    tool,
    controller,
    draft,
    detail,
    showComposer,
    onActivate,
    onSubmit,
    onChange,
    onCancel,
  }: {
    tool: Model<"AgentToolActivity">;
    controller: StoreController;
    draft: ConversationDraft;
    detail?: ReviewDetail;
    showComposer: boolean;
    onActivate(): Promise<void>;
    onSubmit: (content: string) => Promise<void>;
    onChange: (value: string) => void;
    onCancel: () => void;
  } = $props();
  const commentDraft = $derived(
    detail ? reviewDraft(controller, detail.thread.thread_id) : undefined,
  );
</script>

{#if detail}
  <StoreReviewConversation
    rootDepth={1}
    {controller}
    reviewDetail={detail}
    {onActivate}
    inline
    showComposer={false}
    showDiff={false}
  />
{/if}
{#if showComposer && !commentDraft?.reply && !commentDraft?.edit}<div
    class="store-agent-tool-discussion"
    data-ui-part="agent-tool-reply-composer"
  >
    <CommunityChatComposer
      mode="form"
      showSubmitButton
      bind:value={() => draft.value, onChange}
      disabled={controller.state.status.reviewAction === "refreshing" ||
        !!controller.state.reviewActionRequestId ||
        !!draft.pendingRequestId}
      busy={controller.state.status.reviewAction === "refreshing"}
      ariaLabel={`Reply to ${tool.native_name} tool call`}
      submitLabel="Reply"
      placeholder="Reply to this tool call"
      onCancel={draft.pendingRequestId ? undefined : onCancel}
      onSubmit={(submission) => onSubmit(submission.content)}
    />
    {#if draft.pendingRequestId}<Button
        variant="secondary"
        size="sm"
        onclick={() => controller.actions.checkReviewAction()}
        >Check comment status</Button
      >{/if}
    {#if controller.state.status.reviewAction === "failed" || controller.state.status.reviewAction === "unavailable"}
      <p role="alert">{controller.state.reviewActionMessage}</p>
    {/if}
  </div>{/if}

<style>
  .store-agent-tool-discussion {
    min-width: 0;
  }
  .store-agent-tool-discussion > p {
    margin: 0.4rem 0 0;
    color: var(--destructive);
    font-size: var(--community-text-secondary);
  }
</style>

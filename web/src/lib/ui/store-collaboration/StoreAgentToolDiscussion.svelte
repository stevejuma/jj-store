<script lang="ts">
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ConversationDraft } from "../../appearance/drafts.svelte.js";
  let {
    tool,
    controller,
    draft,
    selected,
    onSubmit,
    onChange,
    onCancel,
  }: {
    tool: Model<"AgentToolActivity">;
    controller: StoreController;
    draft: ConversationDraft;
    selected: boolean;
    onSubmit: (content: string) => Promise<void>;
    onChange: (value: string) => void;
    onCancel: () => void;
  } = $props();
</script>

{#if selected}
  <StoreReviewConversation
    {controller}
    inline
    showComposer={false}
    showDiff={false}
  />
{/if}
<div
  class="store-agent-tool-discussion"
  data-ui-part="agent-tool-reply-composer"
>
  <CommunityChatComposer
    mode="form"
    showSubmitButton
    bind:value={() => draft.value, onChange}
    disabled={controller.state.status.reviewAction === "refreshing" ||
      !!controller.state.reviewActionRequestId}
    busy={controller.state.status.reviewAction === "refreshing"}
    ariaLabel={`Reply to ${tool.native_name} tool call`}
    submitLabel="Reply"
    placeholder="Reply to this tool call"
    {onCancel}
    onSubmit={(submission) => onSubmit(submission.content)}
  />
  {#if controller.state.status.reviewAction === "failed" || controller.state.status.reviewAction === "unavailable"}
    <p role="alert">{controller.state.reviewActionMessage}</p>
  {/if}
</div>

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

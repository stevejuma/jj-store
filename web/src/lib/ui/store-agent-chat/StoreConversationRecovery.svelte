<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { conversationSubmissionResult } from "../../conversations/submission-result.js";
  let {
    conversation,
    manager,
    standalone = false,
  }: {
    conversation: ConversationSession;
    manager: AgentExecution;
    standalone?: boolean;
  } = $props();
  const pending = $derived(conversation.comments.retainedSubmission);
  const receipt = $derived(
    pending &&
      (standalone
        ? manager.outcome?.request_id === pending.id
          ? {
              request_id: pending.id,
              conversation_id: manager.selected,
              comment_publication: manager.outcome.comment_publication ?? null,
              agents: [manager.outcome],
            }
          : undefined
        : manager.chat.receipts[pending.id]),
  );
  async function check() {
    if (standalone) {
      if (!pending) return;
      try {
        if (manager.pending?.body.request_id === pending.id)
          await manager.checkOutcome();
        else
          manager.outcome = await manager.client.request(
            `agents/outcome?request=${pending.id}`,
          );
      } catch {
        conversation.comments.error =
          "The retained request is still unavailable.";
      }
    } else await manager.chat.retry().catch(() => {});
  }
  $effect(() => {
    if (pending && receipt) {
      const id = pending.id,
        result = conversationSubmissionResult(
          receipt,
          new Map(manager.value?.sessions.map((s) => [s.id, s.title])),
        );
      untrack(() => conversation.comments.recover(id, result));
    }
  });
  function keep() {
    const id = pending?.id;
    conversation.comments.keepUnresolved();
    if (manager.chat.pending?.request_id === id) manager.chat.retainUnknown();
    if (standalone && manager.pending?.body.request_id === id)
      manager.retainAsUnresolved();
  }
</script>

{#if pending && (standalone || manager.chat.pending?.request_id === pending.id)}
  <section aria-label="Comment submission recovery">
    <p role="status">
      {receipt?.comment_publication?.state === "published"
        ? "Comments published. Agent delivery needs attention."
        : "Submission awaiting confirmation."}
    </p>
    {#each conversation.comments.submissionResult?.delivery ?? [] as entry (entry.id)}<p
      >
        {entry.label}: {entry.state}
      </p>{/each}
    <Button
      variant="outline"
      size="sm"
      disabled={manager.chat.busy}
      onclick={() => void check()}>Check retained request</Button
    >
    <Button
      variant="ghost"
      size="sm"
      disabled={manager.chat.busy}
      onclick={keep}>Keep unresolved in history</Button
    >
  </section>
{/if}
{#each conversation.comments.unresolvedHistory as entry (entry.submission.id)}
  <details>
    <summary>Unresolved submission</summary>
    <p>{entry.submission.text}</p>
    {#each entry.submission.comments as comment (comment.commentId)}<p>
        {comment.text}
      </p>{/each}
  </details>
{/each}

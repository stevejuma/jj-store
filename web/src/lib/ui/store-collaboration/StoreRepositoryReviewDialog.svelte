<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose: () => void } = $props();
  const view = $derived(controller.state);
  let body = $state("");
  let checking = $state(false);
  const disabled = $derived(
    view.status.reviewAction === "refreshing" || !!view.reviewActionRequestId,
  );
  async function check() {
    checking = true;
    try {
      await controller.actions.checkReviewAction();
      if (view.status.reviewAction === "ready" && !view.reviewActionRequestId)
        onClose();
    } finally {
      checking = false;
    }
  }
  async function submit(text: string) {
    if (!text.trim() || disabled) return;
    await controller.actions.createRepositoryReview?.(text);
    if (view.status.reviewAction === "ready" && !view.reviewActionRequestId)
      onClose();
    else
      throw new Error(
        view.reviewActionMessage || "Review update has not been confirmed.",
      );
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && !disabled) onClose();
  }}
>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>New repository review</Dialog.Title>
      <Dialog.Description
        >Start a discussion about this repository. Markdown is supported.</Dialog.Description
      >
    </Dialog.Header>
    <CommunityChatComposer
      bind:value={body}
      mode="form"
      showSubmitButton
      submitLabel="Create review"
      onSubmit={(submission) => submit(submission.content)}
      {disabled}
      placeholder="Write a review…"
      ariaLabel="Repository review message"
    />
    {#if view.reviewActionMessage}<p role="status">
        {view.reviewActionMessage}
      </p>{/if}
    {#if view.reviewActionRequestId}<Button
        variant="outline"
        disabled={checking ||
          view.status.reviewAction === "refreshing" ||
          view.status.reviewAction === "loading"}
        onclick={check}>Check request status</Button
      >{/if}
    <Dialog.Footer
      ><Button variant="outline" {disabled} onclick={onClose}>Cancel</Button
      ></Dialog.Footer
    >
  </Dialog.Content>
</Dialog.Root>

<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let { controller }: { controller: StackWorkspaceController } = $props();
  const deleting = $derived(
    controller.preview?.request.command.kind === "delete_workspace",
  );
</script>

<Dialog.Root
  open={Boolean(controller.preview || controller.pending)}
  onOpenChange={(open) => {
    if (!open) controller.cancelPreview();
  }}
>
  <DialogFrame
    title={controller.pending
      ? "Workspace outcome unknown"
      : (controller.preview?.title ?? "Preview workspace command")}
    description={controller.pending
      ? "Check the workspace state before another command."
      : "Review the exact command before applying."}
    preventScroll={false}
  >
    {#if controller.preview}
      <p>{controller.preview.summary}</p>
      {#if controller.preview.conflicts.length}<CommunityProjectsColumnAlert
          title="Applying introduces conflicts"
          description={controller.preview.conflicts.join(", ")}
        />{/if}
    {/if}
    {#if controller.pending}<CommunityProjectsColumnAlert
        title="Outcome awaiting confirmation"
        description={controller.adapter.retryPending === false
          ? "Refresh JJ state and inspect operation history before another attempt."
          : "Check status or retry the original request. New commands stay blocked."}
      />
      <p class="stack-workspace__request">
        Request: {controller.pending.id}
      </p>{/if}
    {#if controller.validation}<p role="alert">
        {controller.validation}
      </p>{/if}
    {#snippet footer()}
      {#if controller.preview}<Button
          variant="outline"
          disabled={controller.busy || Boolean(controller.pending)}
          onclick={() => controller.cancelPreview()}>Cancel preview</Button
        >{/if}
      {#if controller.pending}<Button
          variant="outline"
          disabled={controller.busy}
          onclick={() => controller.checkStatus()}
          >{controller.adapter.retryPending === false
            ? "Refresh JJ state"
            : "Check status"}</Button
        >{/if}
      <Button
        variant={deleting ? "destructive" : "default"}
        disabled={controller.busy ||
          Boolean(controller.validation) ||
          (Boolean(controller.pending) &&
            controller.adapter.retryPending === false)}
        onclick={() => controller.apply()}
        >{controller.pending
          ? "Retry original command"
          : deleting
            ? "Delete workspace"
            : "Apply command"}</Button
      >
    {/snippet}
  </DialogFrame>
</Dialog.Root>

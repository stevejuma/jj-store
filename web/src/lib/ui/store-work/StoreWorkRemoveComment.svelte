<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as AlertDialog from "@lapismd/design-core/shadcn/alert-dialog";
  import type { WorkController } from "../../controller/work-types.js";
  let {
    work,
    commentId = $bindable(null),
  }: { work: WorkController; commentId: string | null } = $props();
  const view = $derived(work.state);
  async function remove() {
    const comment = view.comments.find((row) => row.id === commentId);
    if (!view.task || !comment) return false;
    await work.submit({
      command: "tombstone_comment",
      id: comment.id,
      expected_version: comment.version,
      expected_task_version: view.task.version,
    });
    if (view.error) return false;
    commentId = null;
    return true;
  }
</script>

<AlertDialog.Root
  open={commentId !== null}
  onOpenChange={(open) => {
    if (!open && !view.writing) commentId = null;
  }}
>
  <AlertDialog.Content
    preventScroll={false}
    onEscapeKeydown={(event) => {
      if (view.writing) event.preventDefault();
    }}
  >
    <AlertDialog.Header>
      <AlertDialog.Title>Remove comment?</AlertDialog.Title>
      <AlertDialog.Description
        >The comment body will be removed. Its replies and recorded history will
        remain.</AlertDialog.Description
      >
    </AlertDialog.Header>
    {#if view.error}<p role="alert">{view.error}</p>{/if}
    <AlertDialog.Footer>
      <Button
        variant="outline"
        disabled={view.writing}
        onclick={() => (commentId = null)}>Cancel</Button
      >
      <Button
        variant="destructive"
        disabled={view.writing || !!view.pending}
        onclick={remove}>Remove comment</Button
      >
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>

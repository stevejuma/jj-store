<script lang="ts">
  import { workHierarchy } from "./work-hierarchy.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let { work, writable }: { work: WorkController; writable: boolean } =
    $props();
  const view = $derived(work.state);
  const disabled = $derived(!writable || view.writing || !!view.pending);
  let body = $state(""),
    reply = $state<string | null>(null),
    edit = $state<string | null>(null);
  $effect(() => {
    view.task?.id;
    body = "";
    reply = null;
    edit = null;
  });
  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (!view.task) return;
    const comment = view.comments.find((row) => row.id === edit);
    await work.submit(
      comment
        ? {
            command: "edit_comment",
            id: comment.id,
            expected_version: comment.version,
            expected_task_version: view.task.version,
            body,
          }
        : {
            command: "add_comment",
            id: workId(),
            task: view.task.id,
            expected_task_version: view.task.version,
            reply_to: reply,
            body,
          },
    );
    if (!view.pending && !view.error) {
      body = "";
      reply = null;
      edit = null;
    }
  }
</script>

<section aria-label="Task discussion">
  <h3>Discussion</h3>
  <ol class="store-work__comments">
    {#each workHierarchy(view.comments, (row) => row.reply_to) as { row: comment, depth } (comment.id)}
      <li
        class="store-work__comment"
        style:margin-inline-start={`${Math.min(depth, 4) * 0.6}rem`}
        aria-label={`Comment ${comment.id.slice(0, 8)}`}
      >
        {#if comment.reply_to}<p class="store-work__meta">
            Reply to {comment.reply_to.slice(0, 8)}{view.comments.some(
              (row) => row.id === comment.reply_to,
            )
              ? ""
              : " (earlier page)"}
          </p>{/if}
        <p class="store-work__body">{comment.body ?? "Comment removed"}</p>
        <p class="store-work__meta">
          By {comment.actor.updated_by.slice(0, 8)} · Version {String(
            comment.version,
          )}
        </p>
        {#if writable}
          <Button
            variant="ghost"
            size="sm"
            {disabled}
            onclick={() => {
              reply = comment.id;
              edit = null;
              body = "";
            }}>Reply</Button
          >
          {#if comment.body !== null}
            <Button
              variant="ghost"
              size="sm"
              {disabled}
              onclick={() => {
                edit = comment.id;
                reply = null;
                body = comment.body!;
              }}>Edit comment</Button
            >
            <Button
              variant="ghost"
              size="sm"
              {disabled}
              onclick={() =>
                view.task &&
                work.submit({
                  command: "tombstone_comment",
                  id: comment.id,
                  expected_version: comment.version,
                  expected_task_version: view.task.version,
                })}>Remove comment</Button
            >
          {/if}
        {/if}
      </li>
    {/each}
  </ol>
  {#if view.commentCursor}<Button
      variant="outline"
      disabled={view.loading}
      onclick={() => work.more("comments")}>More comments</Button
    >{/if}
  {#if writable}
    <form
      onsubmit={save}
      class="store-work__form"
      aria-label="Comment composer"
    >
      {#if edit || reply}<p>
          {edit ? "Editing comment" : "Replying to"}
          {(edit ?? reply)?.slice(0, 8)}
          <Button
            variant="ghost"
            size="sm"
            {disabled}
            onclick={() => {
              edit = null;
              reply = null;
              body = "";
            }}>Cancel</Button
          >
        </p>{/if}
      <label
        >Comment<Textarea
          bind:value={body}
          required
          maxlength={65536}
          {disabled}
        /></label
      >
      <Button type="submit" disabled={disabled || !body.trim()}
        >{edit ? "Save comment" : "Add comment"}</Button
      >
    </form>
  {/if}
</section>

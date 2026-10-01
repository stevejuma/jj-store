<script lang="ts">
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreWorkMessageActions from "./StoreWorkMessageActions.svelte";
  import { untrack } from "svelte";
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as AlertDialog from "@lapismd/design-core/shadcn/alert-dialog";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { CommunityMessage } from "@lapismd/lapis-community/components";
  import {
    workCommentRows,
    workMember,
    workMessage,
  } from "./work-presentation.js";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import "./community-work-thread.css";
  import "./community-work-tasks.css";
  import { workDraft } from "./work-drafts.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    writable,
    busy = false,
    activity,
  }: {
    work: WorkController;
    writable: boolean;
    busy?: boolean;
    activity?: Snippet;
  } = $props();
  const view = $derived(work.state),
    disabled = $derived(
      !writable || busy || view.loading || view.writing || !!view.pending,
    );
  let draft = $state(
    untrack(() => workDraft(work, `comments:${view.task?.id ?? ""}`)),
  );
  let removing = $state<string | null>(null);
  let activityOpen = $state(false);
  const rows = $derived(workCommentRows(view.comments));
  const members = $derived([
    ...new Map(
      view.comments.map((c) => [
        c.actor.created_by,
        workMember(c.actor.created_by),
      ]),
    ).values(),
  ]);
  const reply = $derived(view.comments.find((c) => c.id === draft.reply));
  const context = $derived(
    draft.edit
      ? {
          kind: "edit" as const,
          id: draft.edit,
          label: view.comments.find((c) => c.id === draft.edit)?.body ?? "",
        }
      : draft.reply
        ? {
            kind: "reply" as const,
            target: {
              eventId: draft.reply,
              roomId: view.task?.id ?? "",
              authorId: reply?.actor.created_by ?? "",
              kind: "channel" as const,
              authorName: reply
                ? workMember(reply.actor.created_by).name
                : "Earlier comment",
              content: reply?.body ?? "Comment removed",
            },
          }
        : undefined,
  );
  $effect(() => {
    const id = view.task?.id;
    draft = untrack(() => workDraft(work, `comments:${id ?? ""}`));
  });
  async function save(content: string) {
    if (!view.task) return;
    draft.body = content;
    const comment = view.comments.find((row) => row.id === draft.edit);
    await work.submit(
      comment
        ? {
            command: "edit_comment",
            id: comment.id,
            expected_version: comment.version,
            expected_task_version: view.task.version,
            body: draft.body,
          }
        : {
            command: "add_comment",
            id: workId(),
            task: view.task.id,
            expected_task_version: view.task.version,
            reply_to: draft.reply,
            body: draft.body,
          },
    );
    if (!view.pending && !view.error) {
      draft.body = "";
      draft.reply = null;
      draft.edit = null;
    } else
      throw new Error(
        view.error || "Outcome unknown. Check the retained task command.",
      );
  }
  async function remove() {
    const comment = view.comments.find((row) => row.id === removing);
    if (!view.task || !comment) return false;
    await work.submit({
      command: "tombstone_comment",
      id: comment.id,
      expected_version: comment.version,
      expected_task_version: view.task.version,
    });
    if (view.error) return false;
    removing = null;
    return true;
  }
</script>

<!-- CommunityProjects task activity and CommunityThreadTreeRow, native callbacks. -->
<section aria-label="Task discussion">
  <Collapsible.Root
    class="community-projects__task-activity"
    bind:open={activityOpen}
  >
    <Collapsible.Trigger class="community-projects__task-section-trigger">
      <span>Activity</span>
      {#if activityOpen}<ChevronDownIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />
      {:else}<ChevronRightIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />{/if}
    </Collapsible.Trigger>
    <Collapsible.Content>
      {@render activity?.()}
      <div
        class="community-projects__task-timeline"
        role="list"
        aria-label="Task comments"
      >
        {#each rows as row (row.comment.id)}
          {@const comment = row.comment}
          <div
            class="community-thread-tree-row"
            role="listitem"
            aria-label={`Comment ${comment.id.slice(0, 8)}`}
            data-ui-part="thread-tree-row"
            data-thread-depth={row.depth}
            style={`--community-thread-indent: ${row.indentRem}rem; --community-thread-connector-indent: ${row.connectorIndentRem ?? 0}rem;`}
          >
            {#each row.guideIndentRems as guideIndent}<span
                class="community-thread-tree-row__guide"
                data-ui-part="thread-reply-guide"
                style={`--community-thread-guide-indent: ${guideIndent}rem;`}
                aria-hidden="true"
              ></span>{/each}
            {#if row.connectorIndentRem !== undefined}<span
                class="community-thread-tree-row__connector"
                data-ui-part="thread-reply-connector"
                aria-hidden="true"
              ></span>{/if}
            {#if row.connectsToChildren}<span
                class="community-thread-tree-row__descendant-guide"
                data-ui-part="thread-descendant-guide"
                aria-hidden="true"
              ></span>{/if}
            <div class="community-thread-tree-row__content">
              <CommunityMessage
                message={workMessage(comment)}
                author={workMember(comment.actor.created_by)}
                {members}
                density="compact"
              >
                {#snippet supplement()}
                  {#if comment.reply_to && !view.comments.some((c) => c.id === comment.reply_to)}<p
                      class="store-work__meta"
                      title={comment.reply_to}
                    >
                      Reply to {comment.reply_to.slice(0, 8)} (earlier page)
                    </p>{/if}
                  {#if comment.version > 1}<span
                      class="sr-only"
                      title={comment.actor.updated_by}
                      >Last edited by {comment.actor.updated_by}</span
                    >{/if}
                  {#if writable}<StoreWorkMessageActions
                      {disabled}
                      removed={comment.body === null}
                      authorName={workMember(comment.actor.created_by).name}
                      onReply={() => {
                        draft.reply = comment.id;
                        draft.edit = null;
                      }}
                      onEdit={() => {
                        draft.edit = comment.id;
                        draft.reply = null;
                        draft.body = comment.body!;
                      }}
                      onRemove={() => (removing = comment.id)}
                    />{/if}
                {/snippet}
              </CommunityMessage>
            </div>
          </div>
        {/each}
      </div>
      {#if view.commentCursor}<Button
          variant="outline"
          disabled={view.loading}
          onclick={() => work.more("comments")}>More comments</Button
        >{/if}
      {#if !view.comments.length && !view.loading}<StoreWorkEmpty
          kind="comments"
          title="No discussion yet"
          description="Add a comment to start the discussion."
        />{/if}
      {#if writable}<CommunityChatComposer
          class="community-projects__task-composer"
          bind:value={draft.body}
          mode="form"
          showSubmitButton
          ariaLabel="Comment"
          placeholder="Add a comment…"
          submitLabel={draft.edit ? "Save comment" : "Add comment"}
          {disabled}
          {context}
          {members}
          onCancelContext={() => {
            draft.edit = null;
            draft.reply = null;
          }}
          onSubmit={(submission) => save(submission.content)}
        />{/if}
    </Collapsible.Content>
  </Collapsible.Root>
</section>
<AlertDialog.Root
  open={removing !== null}
  onOpenChange={(open) => {
    if (!open && !view.writing) removing = null;
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
        onclick={() => (removing = null)}>Cancel</Button
      >
      <Button
        variant="destructive"
        disabled={view.writing || !!view.pending}
        onclick={remove}>Remove comment</Button
      >
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>

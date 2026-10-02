<script lang="ts">
  // @community-copy CommunityProjects.svelte task Activity (Apache-2.0), native comments.
  import { focusNativeMessage } from "../focus-message.js";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import StoreWorkRemoveComment from "./StoreWorkRemoveComment.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreWorkComment from "./StoreWorkComment.svelte";
  import { workCommentRows, workMember } from "./work-presentation.js";
  import "./community-work-thread.css";
  import "./community-work-tasks.css";
  import { workDraft } from "./work-drafts.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    writable,
    principal,
    busy = false,
  }: {
    work: WorkController;
    writable: boolean;
    principal?: string | undefined;
    busy?: boolean;
  } = $props();
  const view = $derived(work.state),
    disabled = $derived(
      !writable || busy || view.loading || view.writing || !!view.pending,
    );
  let draft = $state(
    untrack(() => workDraft(work, `comments:${view.task?.id ?? ""}`)),
  );
  let removing = $state<string | null>(null);
  let activityOpen = $state(true);
  $effect(() => {
    if (view.focusCommentId) activityOpen = true;
  });
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
          label: draft.editContext?.body ?? "",
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
    if (
      draft.edit &&
      (!comment ||
        !draft.editContext ||
        comment.body === null ||
        comment.actor.created_by !== principal)
    )
      throw new Error(
        "The selected comment cannot be edited. Cancel this edit to start a new comment.",
      );
    await work.submit(
      comment
        ? {
            command: "edit_comment",
            id: comment.id,
            expected_version: draft.editContext!.commentVersion,
            expected_task_version: draft.editContext!.taskVersion,
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
      draft.editContext = undefined;
    } else
      throw new Error(
        view.error || "Outcome unknown. Check the retained task command.",
      );
  }
</script>

<!-- CommunityProjects task activity and CommunityThreadTreeRow, native callbacks. -->
<section aria-label="Task activity">
  <Collapsible.Root
    class="community-projects__task-activity"
    bind:open={activityOpen}
  >
    <Collapsible.Trigger class="community-projects__task-section-trigger">
      <span>Activity</span>
      {#if activityOpen}<ChevronDownIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />{:else}<ChevronRightIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />{/if}
    </Collapsible.Trigger>
    <Collapsible.Content>
      <div
        use:focusNativeMessage={view.comments.some(
          (c) => c.id === view.focusCommentId,
        )
          ? view.focusCommentId
          : null}
      >
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
              data-message-id={comment.id}
              data-highlighted={comment.id === view.focusCommentId || undefined}
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
                <StoreWorkComment
                  {work}
                  {comment}
                  {members}
                  {writable}
                  {disabled}
                  onReply={writable
                    ? () => {
                        draft.reply = comment.id;
                        draft.edit = null;
                        draft.editContext = undefined;
                      }
                    : undefined}
                  onEdit={writable &&
                  comment.body !== null &&
                  comment.actor.created_by === principal
                    ? () => {
                        draft.edit = comment.id;
                        draft.reply = null;
                        draft.body = comment.body!;
                        draft.editContext = {
                          body: comment.body!,
                          commentVersion: comment.version,
                          taskVersion: view.task!.version,
                        };
                      }
                    : undefined}
                  onRemove={writable && comment.body !== null
                    ? () => (removing = comment.id)
                    : undefined}
                />
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
            title="No comments yet"
            description="Add a comment to this task."
          />{/if}
      </div>
    </Collapsible.Content>
  </Collapsible.Root>
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
        if (draft.edit) draft.body = "";
        draft.edit = null;
        draft.reply = null;
        draft.editContext = undefined;
      }}
      onSubmit={(submission) => save(submission.content)}
    />{/if}
</section>
<StoreWorkRemoveComment {work} bind:commentId={removing} />

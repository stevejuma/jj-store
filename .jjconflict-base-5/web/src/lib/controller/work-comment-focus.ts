import type { WorkQuery, WorkState } from "./work-types.js";
export async function loadWorkCommentFocus(
  state: WorkState,
  route: {
    projectId?: string | null;
    taskId?: string | null;
    commentId?: string | null;
  },
  read: (query: WorkQuery, kind: "comments", more: boolean) => Promise<void>,
  current: () => boolean,
) {
  for (
    let page = 0;
    route.taskId &&
    route.commentId &&
    !state.comments.some((c) => c.id === route.commentId) &&
    state.commentCursor &&
    page < 20;
    page++
  ) {
    const previous = state.commentCursor;
    await read(
      {
        project: route.projectId ?? undefined,
        task: route.taskId,
        cursor: previous,
      },
      "comments",
      true,
    );
    if (!current() || previous === state.commentCursor) return;
  }
  if (
    current() &&
    route.commentId &&
    !state.comments.some((c) => c.id === route.commentId)
  )
    state.error = state.commentCursor
      ? "Comment is outside loaded history. Load more comments to continue."
      : "This comment is unavailable.";
}

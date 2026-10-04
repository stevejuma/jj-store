import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";
type Detail = NonNullable<StoreControllerState["reviewThread"]>;
export function editFixtureReviewComment(
  detail: Detail,
  id: string,
  revision: string,
  text: string,
): Detail | undefined {
  const comment = detail.comments.find((row) => row.comment_id === id);
  if (
    !comment ||
    comment.revision_id !== revision ||
    comment.content.state !== "visible"
  )
    return;
  const content = comment.content;
  return {
    ...detail,
    comments: detail.comments.map((row) =>
      row.comment_id === id
        ? {
            ...row,
            previous_revision_id: row.revision_id,
            revision_id: "55".repeat(32),
            content: { ...content, text },
          }
        : row,
    ),
  };
}

export function appendFixtureReviewComment(
  detail: Detail,
  text: string,
  now: number,
  parentCommentId?: string,
): Detail {
  const index = detail.comments.length + 1;
  const byte = index.toString(16).padStart(2, "0");
  return {
    ...detail,
    thread: {
      ...detail.thread,
      comment_count: index,
      content_version: Number(detail.thread.content_version) + 1,
    },
    comments: [
      ...detail.comments,
      {
        comment_id: byte.repeat(16),
        revision_id: byte.repeat(32),
        parent_comment_id: parentCommentId ?? null,
        author: "20".repeat(16),
        actor: "20".repeat(16),
        created_at_millis: now * 1000,
        previous_revision_id: null,
        superseded_revision_ids: [],
        content: {
          state: "visible",
          body_id: "21".repeat(32),
          format: "safe_markdown",
          text: text.trim(),
          sanitizer_version: "review-markdown-v1",
        },
      },
    ],
  };
}

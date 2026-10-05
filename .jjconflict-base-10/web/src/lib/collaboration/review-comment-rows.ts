import type { Model } from "../api/client.js";

const REVIEW_REPLY_INDENT_REM = 2.25;
const MAX_VISIBLE_REVIEW_REPLY_DEPTH = 6;

export interface ReviewCommentRow {
  comment: Model<"ReviewComment">;
  depth: number;
  indentRem: number;
  connectorIndentRem?: number;
  guideIndentRems: readonly number[];
  connectsToChildren: boolean;
}

function reviewReplyIndentRem(depth: number): number {
  const visibleDepth = Math.min(
    Math.max(depth - 1, 0),
    MAX_VISIBLE_REVIEW_REPLY_DEPTH,
  );
  return visibleDepth * REVIEW_REPLY_INDENT_REM;
}

export function reviewCommentRows(
  comments: readonly Model<"ReviewComment">[],
  rootDepth: 0 | 1 = 0,
): ReviewCommentRow[] {
  const byId = new Map(
    comments.map((comment) => [comment.comment_id, comment]),
  );
  const childrenByParent = new Map<string, Model<"ReviewComment">[]>();
  const roots: Model<"ReviewComment">[] = [];

  for (const comment of comments) {
    const parent = comment.parent_comment_id;
    if (!parent || parent === comment.comment_id || !byId.has(parent)) {
      roots.push(comment);
      continue;
    }
    const children = childrenByParent.get(parent) ?? [];
    children.push(comment);
    childrenByParent.set(parent, children);
  }

  const rows: ReviewCommentRow[] = [];
  const visited = new Set<string>();
  const visit = (
    comment: Model<"ReviewComment">,
    depth: number,
    guideDepths: readonly number[],
  ): void => {
    if (visited.has(comment.comment_id)) return;
    visited.add(comment.comment_id);
    const children = (childrenByParent.get(comment.comment_id) ?? []).filter(
      (child) => !visited.has(child.comment_id),
    );
    rows.push({
      comment,
      depth,
      indentRem: reviewReplyIndentRem(depth),
      ...(depth > 1
        ? { connectorIndentRem: reviewReplyIndentRem(depth - 1) }
        : {}),
      guideIndentRems: guideDepths.map(reviewReplyIndentRem),
      connectsToChildren: depth > 0 && children.length > 0,
    });
    children.forEach((child, index) => {
      const childGuides = [...guideDepths];
      if (depth > 0 && index < children.length - 1) childGuides.push(depth);
      visit(child, depth + 1, childGuides);
    });
  };

  for (const root of roots) visit(root, rootDepth, []);
  for (const comment of comments) {
    if (!visited.has(comment.comment_id)) visit(comment, rootDepth, []);
  }
  return rows;
}

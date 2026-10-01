import type { Model } from "../../api/client.js";
import type {
  CommunityMemberModel,
  CommunityMessageModel,
} from "@lapismd/lapis-community/components/community-message";

export function workMember(id: string): CommunityMemberModel {
  const local = id === "local" || id.endsWith(":local");
  return {
    id,
    name: local ? "Local" : `${id.slice(0, 12)}…`,
    handle: id,
    initials: local ? "L" : id.slice(0, 2).toUpperCase(),
  };
}

export function workMessage(
  comment: Model<"WorkCommentV1">,
): CommunityMessageModel {
  const createdAt = new Date(Number(comment.actor.created_at_millis));
  return {
    id: comment.id,
    authorId: comment.actor.created_by,
    content: comment.body ?? "*Comment removed*",
    timestamp: createdAt.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    }),
    createdAt: createdAt.toISOString(),
    edited: comment.version > 1,
    ...(comment.reply_to ? { parentMessageId: comment.reply_to } : {}),
  };
}

/** Community thread-tree-layout: same capped guides and sibling chronology.
 * Native paged fragments retain their requested parent; no synthetic root/event.
 */
export function workCommentRows(comments: readonly Model<"WorkCommentV1">[]) {
  const ids = new Set(comments.map((c) => c.id));
  const children = new Map<string, Model<"WorkCommentV1">[]>();
  const roots: Model<"WorkCommentV1">[] = [];
  for (const comment of comments) {
    if (
      !comment.reply_to ||
      comment.reply_to === comment.id ||
      !ids.has(comment.reply_to)
    )
      roots.push(comment);
    else {
      const group = children.get(comment.reply_to) ?? [];
      group.push(comment);
      children.set(comment.reply_to, group);
    }
  }
  const indent = (depth: number) => Math.min(Math.max(depth, 0), 6) * 2.25;
  type Row = {
    comment: Model<"WorkCommentV1">;
    depth: number;
    indentRem: number;
    connectorIndentRem: number | undefined;
    guideIndentRems: number[];
    connectsToChildren: boolean;
  };
  const rows: Row[] = [],
    visited = new Set<string>();
  const stack = roots
    .slice()
    .reverse()
    .map((comment) => ({ comment, depth: 0, guides: [] as number[] }));
  function visit() {
    while (stack.length) {
      const { comment, depth, guides } = stack.pop()!;
      if (visited.has(comment.id)) continue;
      visited.add(comment.id);
      const replies = (children.get(comment.id) ?? []).filter(
        (c) => !visited.has(c.id),
      );
      rows.push({
        comment,
        depth,
        indentRem: indent(depth),
        connectorIndentRem: depth ? indent(depth - 1) : undefined,
        guideIndentRems: guides,
        connectsToChildren: replies.length > 0,
      });
      for (let index = replies.length - 1; index >= 0; index--) {
        const childGuides =
          index < replies.length - 1
            ? [...new Set([...guides, indent(depth)])]
            : guides;
        stack.push({
          comment: replies[index]!,
          depth: depth + 1,
          guides: childGuides,
        });
      }
    }
  }
  visit();
  // Malformed fragments remain visible; iteration is bounded by native IDs.
  for (const comment of comments)
    if (!visited.has(comment.id)) {
      stack.push({ comment, depth: 0, guides: [] });
      visit();
    }
  return rows;
}

export function taskStateLabel(state: Model<"TaskStateV1">): string {
  return state === "in_progress"
    ? "In progress"
    : state[0]!.toUpperCase() + state.slice(1);
}

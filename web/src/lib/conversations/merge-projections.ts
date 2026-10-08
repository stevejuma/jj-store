import type { ConversationProjection } from "./agent-adapter.js";
export function mergeConversationProjections(
  base: ConversationProjection,
  linked: ConversationProjection,
): ConversationProjection {
  const batches = new Set(
    linked.items.flatMap((i) => (i.commentBatchId ? [i.commentBatchId] : [])),
  );
  const threads = new Map(base.threads.map((t) => [t.id, t]));
  for (const thread of linked.threads) {
    const prior = threads.get(thread.id);
    threads.set(
      thread.id,
      prior
        ? {
            ...thread,
            comments: [
              ...new Map(
                [...prior.comments, ...thread.comments].map((c) => [
                  `${c.messageId ?? c.id}:${c.revision?.id ?? ""}`,
                  c,
                ]),
              ).values(),
            ],
          }
        : thread,
    );
  }
  return {
    authoritativeDiscussions: new Set([
      ...(base.authoritativeDiscussions ?? []),
      ...(linked.authoritativeDiscussions ?? []),
    ]),
    answers: new Map([...(base.answers ?? []), ...(linked.answers ?? [])]),
    items: mergeOrderedItems(
      base.items.filter(
        (i) =>
          !i.commentBatchId ||
          !batches.has(i.commentBatchId) ||
          i.blocks.length,
      ),
      linked.items,
    ),
    sources: new Map([...base.sources, ...linked.sources]),
    targets: new Map([...base.targets, ...linked.targets]),
    threads: [...threads.values()],
  };
}

/** Preserve each capture's order. Only recorded timestamps order the two streams. */
export function mergeOrderedItems(
  a: ConversationProjection["items"],
  b: ConversationProjection["items"],
) {
  const result: ConversationProjection["items"] = [];
  let i = 0,
    j = 0;
  while (i < a.length && j < b.length) {
    const left = a[i].observedAt,
      right = b[j].observedAt;
    if (
      left !== undefined &&
      right !== undefined &&
      Number.isFinite(left) &&
      Number.isFinite(right) &&
      right < left
    )
      result.push(b[j++]);
    else result.push(a[i++]);
  }
  return [...result, ...a.slice(i), ...b.slice(j)];
}

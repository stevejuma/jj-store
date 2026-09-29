import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";

const MAX_CONTEXT_LINKS = 4_096;

export async function readAgentRelations(
  source: StoreDataSource,
  subject: { commit: string } | { operation: string },
): Promise<Model<"AgentRelationLink">[]> {
  const relations: Model<"AgentRelationLink">[] = [];
  const seen = new Set<string>();
  const cursors = new Set<string>();
  let inspected = 0;
  let cursor: string | undefined;
  do {
    const page = await source.agentRelations(subject, cursor);
    inspected += Math.max(1, page.relations.length);
    if (inspected > MAX_CONTEXT_LINKS)
      throw new Error("Agent context exceeds the browser read bound.");
    for (const relation of page.relations) {
      if (seen.has(relation.object_id)) continue;
      seen.add(relation.object_id);
      relations.push(relation);
    }
    cursor = nextCursor(cursors, page.next_cursor, "agent relation");
  } while (cursor && !relations.some((relation) => relation.association));
  return relations;
}

export async function readReviewContext(
  source: StoreDataSource,
  matches: (thread: Model<"ReviewThreadSummary">) => boolean,
  current: readonly Model<"ReviewThreadSummary">[] = [],
  next: string | null = null,
): Promise<{
  threads: Model<"ReviewThreadSummary">[];
  cursor: string | null;
  selected: Model<"ReviewThreadSummary"> | undefined;
}> {
  const threads = [...current];
  const seen = new Set(threads.map((thread) => thread.thread_id));
  const cursors = new Set(next ? [next] : []);
  let inspected = threads.length;
  let selected = threads.find(matches);
  let cursor: string | undefined = next ?? undefined;
  let needsPage = threads.length === 0 || Boolean(cursor);
  while (!selected && needsPage) {
    const page = await source.reviewThreads(cursor);
    inspected += Math.max(1, page.threads.length);
    if (inspected > MAX_CONTEXT_LINKS)
      throw new Error("Review context exceeds the browser read bound.");
    for (const thread of page.threads) {
      if (seen.has(thread.thread_id)) continue;
      seen.add(thread.thread_id);
      threads.push(thread);
    }
    selected = threads.find(matches);
    cursor = nextCursor(cursors, page.next_cursor, "review thread");
    needsPage = Boolean(cursor);
  }
  return { threads, cursor: cursor ?? null, selected };
}

function nextCursor(
  seen: Set<string>,
  next: string | null,
  label: string,
): string | undefined {
  if (next && !seen.add(next))
    throw new Error(`Repeated ${label} cursor from the server.`);
  return next ?? undefined;
}

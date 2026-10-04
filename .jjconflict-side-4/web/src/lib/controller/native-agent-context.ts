import { pretty, type Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { NativeAgentContext } from "./native-activity-types.js";

const errorMessage = (cause: unknown) =>
  cause instanceof Error ? cause.message : "Agent context unavailable";

export async function readNativeAgentContext(
  source: StoreDataSource,
  subject: { commit: string } | { operation: string },
  options: {
    pin?: import("./repository-read-types.js").AgentSnapshot;
    deferSessions?: boolean;
  } = {},
): Promise<NativeAgentContext> {
  try {
    let cursor: string | undefined;
    const cursors = new Set<string>(),
      ids = new Set<string>();
    let pin: import("./repository-read-types.js").AgentSnapshot | undefined =
        options.pin,
      count = 0;
    do {
      const page = await source.agentRelations(subject, cursor);
      if (
        page.snapshot.repo_id !== source.repository ||
        (pin && pretty(pin) !== pretty(page.snapshot))
      )
        throw new Error("Agent context snapshot changed. Refresh to retry.");
      pin = page.snapshot;
      count += Math.max(1, page.relations.length);
      if (count > 4096)
        throw new Error("Agent context exceeds the browser read bound.");
      for (const row of page.relations) {
        const association = row.association;
        if (!association) continue;
        const matches =
          "commit" in subject
            ? [
                ...association.before_commits,
                ...association.after_commits,
              ].includes(subject.commit)
            : [
                ...association.before_operations,
                ...association.after_operations,
              ].includes(subject.operation);
        if (matches) ids.add(association.session_id);
      }
      if (ids.size > 16)
        throw new Error(
          "More than 16 related agent sessions. Open Agents for the complete context.",
        );
      cursor = page.next_cursor ?? undefined;
      if (cursor && cursors.has(cursor))
        throw new Error("Repeated agent context cursor.");
      if (cursor) cursors.add(cursor);
    } while (cursor);
    const load = once(async (): Promise<NativeAgentContext> => {
      try {
        const sessions = [];
        for (const id of ids) {
          const session = await source.agentSession(id);
          if (
            session.session.session_id !== id ||
            pretty(session.snapshot) !== pretty(pin)
          )
            throw new Error(
              "Agent context snapshot changed. Refresh to retry.",
            );
          sessions.push(session);
        }
        return { sessions, error: "" };
      } catch (cause) {
        return { sessions: [], error: errorMessage(cause) };
      }
    });
    return options.deferSessions && ids.size
      ? { sessions: [], error: "", relatedSessionIds: [...ids], load }
      : await load();
  } catch (cause) {
    return { sessions: [], error: errorMessage(cause) };
  }
}

function once(read: () => Promise<NativeAgentContext>) {
  let pending: Promise<NativeAgentContext> | undefined;
  return () => (pending ??= read());
}

/** Shared two-reader budget for a page; memoization survives virtual unmounts. */
export function createDeferredAgentContexts(
  source: StoreDataSource,
  pin?: import("./repository-read-types.js").AgentSnapshot,
) {
  let active = 0;
  const waiting: (() => void)[] = [];
  async function schedule(read: () => Promise<NativeAgentContext>) {
    if (active >= 2)
      await new Promise<void>((resolve) => waiting.push(resolve));
    else active++;
    try {
      return await read();
    } finally {
      const next = waiting.shift();
      if (next) next();
      else active--;
    }
  }
  return (
    subject: { commit: string } | { operation: string },
  ): NativeAgentContext => ({
    sessions: [],
    error: "",
    discover: once(() =>
      schedule(async () => {
        const context = await readNativeAgentContext(source, subject, {
          pin,
          deferSessions: true,
        });
        const load = context.load;
        if (load) context.load = once(() => schedule(load));
        return context;
      }),
    ),
  });
}

export async function readAnchoredAgentContext(
  source: StoreDataSource,
  anchor: Model<"ArtifactAnchorV1">,
): Promise<NativeAgentContext> {
  const target = anchor.target;
  if (
    target.target !== "agent_text" &&
    target.target !== "agent_artifact" &&
    target.target !== "agent_session"
  )
    return { sessions: [], error: "" };
  try {
    const session = await source.agentSession(target.value.session_id);
    if (
      session.snapshot.repo_id !== source.repository ||
      session.session.session_id !== target.value.session_id ||
      session.session.snapshot_id !== target.value.session_snapshot.id
    )
      throw new Error(
        "The original anchored agent snapshot is unavailable in the selected capture.",
      );
    return { sessions: [session], error: "" };
  } catch (cause) {
    return { sessions: [], error: errorMessage(cause) };
  }
}

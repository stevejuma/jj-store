import type { Model } from "../api/client.js";
import type { ThreadsController } from "../controller/thread-types.js";
import type { AgentExecution } from "../controller/agent-execution.svelte.js";
import type {
  ConversationProjection,
  AgentConversationRow,
} from "./agent-adapter.js";

export function useConversationReactions(
  read: () => {
    threads: ThreadsController;
    targets: ReadonlyMap<string, Model<"ReactionTargetV1">>;
    projection: ConversationProjection;
    rows: readonly AgentConversationRow[];
    manager?: AgentExecution;
  },
) {
  const options = $derived(read());
  const entries = $derived(
    new Map(
      options.projection.items.flatMap((item) => {
        const target = options.targets.get(item.id);
        if (target)
          return [[item.id, { threads: options.threads, target }] as const];
        const source = options.projection.targets.get(item.id),
          row = options.rows.find((r) => r.session.id === source?.sessionId);
        const captured =
          source &&
          options.manager?.discussions.message(source.sessionId, source.rowId);
        return row && captured
          ? [
              [
                item.id,
                {
                  threads: options.manager!.discussions.reader(
                    row.session.repository,
                  ),
                  target: captured.target,
                },
              ] as const,
            ]
          : [];
      }),
    ),
  );
  const models = $derived(
    Object.fromEntries(
      [...entries].map(([id, entry]) => [
        id,
        entry.threads.reactionModels(entry.target),
      ]),
    ),
  );
  return {
    get models() {
      return models;
    },
    entry(id: string) {
      return entries.get(id);
    },
    async toggle(
      id: string,
      emoji: string,
      custom?: Parameters<ThreadsController["toggle"]>[2],
    ) {
      const entry = entries.get(id);
      if (entry) await entry.threads.toggle(entry.target, emoji, custom);
    },
  };
}

export function recordedReactionTargets(
  activity: readonly Model<"AgentActivity">[],
  session: string,
  projection: ConversationProjection,
) {
  const originals = new Map(
    activity.flatMap((e) =>
      e.type === "message" && e.reaction_target
        ? [[e.message_id, e.reaction_target] as const]
        : e.type === "tool" && e.tool.reaction_target
          ? [[e.tool.tool_call_id, e.tool.reaction_target] as const]
          : [],
    ),
  );
  return new Map(
    projection.items.flatMap((item) => {
      const row = projection.targets.get(item.id);
      const target = row?.sessionId === session && originals.get(row.rowId);
      return target ? [[item.id, target] as const] : [];
    }),
  );
}

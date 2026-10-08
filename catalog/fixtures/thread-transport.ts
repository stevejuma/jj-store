import { applyThreadSubmission } from "./thread-submission.js";
import type { Model } from "../../web/src/lib/controller/thread-types.js";
import type {
  ReactionItems,
  ReactionRequest,
  ReactionTarget,
  ThreadRequest,
  ThreadTransport,
} from "../../web/src/lib/controller/thread-types.js";
import { targetKey } from "../../web/src/lib/collaboration/thread-presentation.js";
export const THREAD_IDS = {
  repo: "11".repeat(16),
  actor: "22".repeat(16),
  thread: "33".repeat(16),
  message: "44".repeat(16),
  emoji: "55".repeat(16),
};
const ref = (
  kind: Model<"CollabObjectRef">["kind"],
  n: number,
): Model<"CollabObjectRef"> => ({ kind, id: n.toString(16).padStart(64, "0") });
export function createThreadTransport(
  options: { unknownFirst?: boolean; reject?: string } = {},
) {
  let seq = 1,
    uncertain = !!options.unknownFirst;
  const requests: (ThreadRequest | ReactionRequest)[] = [],
    receipts = new Map<
      string,
      Extract<Model<"CollabCommandOutcome">, { status: "committed" }>
    >();
  const threads = new Map<string, Model<"ThreadSummaryV1">>();
  const history = new Map<string, Model<"ThreadEventItemV1">[]>();
  const rows = new Map<string, ReactionItems>();
  const snapshot = (): Model<"CollabReadSnapshotV1"> => ({
    version: 1,
    repo_id: THREAD_IDS.repo,
    epoch: "66".repeat(16),
    manifest_digest: ref("thread_version", seq).id,
    published_seq: seq,
    code_state_version: 1,
    operation_heads: [],
    thread_root: ref("thread_version", seq).id,
    thread_state_version: seq,
    reaction_root: null,
    reaction_state_version: seq,
    agent_root: null,
    review_root: null,
  });
  const page = (
    items: Model<"CollabPageItems">,
    next_cursor: string | null = null,
  ): Model<"CollabPage"> => ({
    snapshot: snapshot(),
    items,
    next_cursor,
  });
  function addThread(id: string, title: string) {
    const event = ref("thread_event", ++seq),
      reference = ref("thread_version", ++seq);
    threads.set(id, {
      reference,
      value: {
        schema_version: 1,
        thread_id: id,
        title,
        status: "active",
        actor: THREAD_IDS.actor,
        accepted_at_millis: seq,
        previous: null,
        event,
        baseline: null,
      },
    });
    history.set(id, []);
  }
  function addMessage(
    thread_id: string,
    id: string,
    body: string | null,
    reply_to: Model<"CollabObjectRef"> | null = null,
    previous: Model<"CollabObjectRef"> | null = null,
  ) {
    const thread = threads.get(thread_id)!;
    const revision = ref("human_message_revision", ++seq),
      event = ref("thread_event", ++seq);
    const message: Model<"HumanMessageRevisionV1"> = {
      schema_version: 1,
      thread_id,
      message_id: id,
      body,
      actor: THREAD_IDS.actor,
      accepted_at_millis: 1725000000000 + seq,
      previous,
      ...(reply_to ? { reply_to } : {}),
    };
    history.get(thread_id)!.unshift({
      reference: event,
      value: {
        schema_version: 1,
        id: seq.toString(16).padStart(32, "0"),
        thread_id,
        actor: THREAD_IDS.actor,
        accepted_at_millis: message.accepted_at_millis,
        previous: thread.value.event,
        event: { type: "message", revision },
      },
      message,
    });
    threads.set(thread_id, {
      ...thread,
      reference: ref("thread_version", ++seq),
      value: { ...thread.value, event, previous: thread.reference },
    });
    return revision;
  }
  addThread(THREAD_IDS.thread, "general");
  addMessage(THREAD_IDS.thread, THREAD_IDS.message, "Welcome to the channel");
  const emojiVersion = ref("custom_emoji_version", 500),
    preview = ref("image_asset", 501);
  const transport = {
    list: async (_cursor?: string) =>
      page({ type: "threads", threads: [...threads.values()] }),
    thread: async (id) => ({ snapshot: snapshot(), thread: threads.get(id)! }),
    events: async (id) =>
      page({ type: "events", thread_id: id, events: history.get(id) ?? [] }),
    emojis: async () =>
      page({
        type: "emojis",
        versions: { [THREAD_IDS.emoji]: emojiVersion },
        definitions: [
          {
            schema_version: 1,
            emoji_id: THREAD_IDS.emoji,
            shortcode: "lapis",
            label: "Lapis",
            actor: THREAD_IDS.actor,
            accepted_at_millis: 1,
            previous: null,
            asset: preview,
            preview,
            retired: false,
          },
        ],
      }),
    asset: async (_id: string) => ({
      type: "image/png",
      bytes: Uint8Array.from(
        atob(
          "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+a3ioAAAAASUVORK5CYII=",
        ),
        (c) => c.charCodeAt(0),
      ),
    }),
    reactions: async (target) =>
      page(
        rows.get(targetKey(target)) ?? {
          type: "reactions",
          target,
          revision: "current",
          reactions: [],
          conflict_count: 0,
          actors: [],
        },
      ),
    async command(kind, request) {
      requests.push(request);
      const prior = receipts.get(request.request_id);
      if (prior) return prior;
      if (kind === "threads") {
        const c = (request as ThreadRequest).command;
        if (c.type === "create" || c.type === "create_agent_discussion") {
          addThread(c.thread_id, c.title);
          if (c.type === "create_agent_discussion") {
            const thread = threads.get(c.thread_id)!;
            threads.set(c.thread_id, {
              ...thread,
              value: { ...thread.value, agent_message: c.target },
            });
          }
        }
        if (c.type === "submit")
          applyThreadSubmission(
            c.submission,
            request as ThreadRequest,
            threads,
            history,
            addThread,
            addMessage,
            ref,
            () => ++seq,
            () => seq,
          );
        if (c.type === "rename" || c.type === "lifecycle") {
          const thread = threads.get(c.thread_id)!;
          threads.set(c.thread_id, {
            ...thread,
            reference: ref("thread_version", ++seq),
            value: {
              ...thread.value,
              ...(c.type === "rename"
                ? { title: c.title }
                : {
                    status: (
                      {
                        activate: "active",
                        idle: "idle",
                        archive: "archived",
                        tombstone: "tombstoned",
                        restore: "idle",
                      } as const
                    )[c.action],
                  }),
            },
          });
        }
        if (c.type === "message")
          addMessage(
            c.thread_id,
            c.message_id,
            c.body ?? null,
            c.reply_to ?? null,
            c.expected_revision,
          );
      } else {
        const c = (request as ReactionRequest).command;
        if (c.type === "set" || c.type === "clear") {
          rows.set(targetKey(c.target), {
            type: "reactions",
            target: c.target,
            revision: "current",
            reactions:
              c.type === "set"
                ? [{ value: c.value, count: 1, viewer_reacted: true }]
                : [],
            conflict_count: 0,
            actors:
              c.type === "set"
                ? [
                    {
                      actor: THREAD_IDS.actor,
                      active: [ref("reaction", ++seq)],
                      value: c.value,
                      conflict: false,
                    },
                  ]
                : [],
          });
        }
      }
      // A transport fixture: durable receipt integrity is qualified by Rust service tests.
      const result: Extract<
        Model<"CollabCommandOutcome">,
        { status: "committed" }
      > = {
        status: "committed",
        receipt: {
          request_id: request.request_id,
        } as Model<"CollabCommandReceipt">,
      };
      receipts.set(request.request_id, result);
      if (uncertain) {
        uncertain = false;
        return { status: "outcome_unknown", request_id: request.request_id };
      }
      return result;
    },
    status: async (_, id) =>
      receipts.get(id) ?? { status: "not_known", request_id: id },
  } satisfies ThreadTransport;
  return {
    transport,
    requests,
    history,
    rows,
    threads,
    snapshot,
    page,
    addMessage,
    emojiVersion,
  };
}

import { createThreadTransport } from "./thread-transport.js";
import { fixtureJsonResponse as json } from "./agent-execution-session.js";
import { targetKey } from "../../web/src/lib/collaboration/thread-presentation.js";
import type {
  AgentAction,
  AgentOutcome,
} from "../../web/src/lib/controller/agent-execution-types.js";
import type {
  Model,
  ThreadRequest,
  ReactionRequest,
} from "../../web/src/lib/controller/thread-types.js";

export const inlineSession = "44".repeat(16);
const request = "55".repeat(16);
export type InlineAgentCommand = {
  version: number;
  request_id: string;
  session_id: string;
  action: AgentAction;
};

/** Current comment-batch transport for the live conversation stories only. */
export function inlineCommentTransport(options: {
  unpublished?: boolean;
  writeUnavailable?: boolean;
  unknown?: boolean;
  observe: (command?: InlineAgentCommand, outcome?: string) => void;
  resume: () => void;
}) {
  const fixture = createThreadTransport();
  const { transport } = fixture;
  const receipts = new Map<string, AgentOutcome>();
  let uncertain = !!options.unknown;
  let failFeedback = false;
  function batchesFor(message: string, revision: string) {
    return [...fixture.history.values()].flatMap((events) =>
      events.flatMap((event) => {
        const target = event.link?.target;
        return target?.type === "comment_batch" &&
          target.comments.some(
            (comment) =>
              comment.type === "human_message" &&
              comment.message_id === message &&
              comment.revision.id === revision,
          )
          ? [target.request_id]
          : [];
      }),
    );
  }
  const captured: Model<"AgentExecutionFeedbackMessage">[] = [
    "user",
    "assistant",
  ].map((role, i) => ({
    row_id: `${request}/${role}`,
    target: {
      type: "agent_message",
      session_id: "99".repeat(16),
      message_id: (i ? "bb" : "aa").repeat(16),
      event_id: (i ? "dd" : "cc").repeat(16),
      batch: { kind: "event_batch", id: "ee".repeat(32) },
      content: { kind: "content", id: (i ? "22" : "11").repeat(32) },
    },
    text_parts: [
      { part: null, text: i ? "Ready to help." : "Show the retained time" },
    ],
    answers: [],
    threads: [],
    replies: [],
  }));
  captured.push({
    row_id: `${request}/tool/read`,
    target: {
      type: "tool_output",
      session_id: "99".repeat(16),
      tool_id: "77".repeat(16),
      snapshot: { kind: "tool_snapshot", id: "88".repeat(32) },
      content: { kind: "content", id: "66".repeat(32) },
    },
    text_parts: [
      {
        part: "/rawOutput/formatted_output",
        text: "export const captured = 42;",
      },
    ],
    answers: [],
    threads: [],
    replies: [],
  });
  async function feedback() {
    if (failFeedback) throw new Error("Feedback refresh failed");
    const messages = captured.map((message) => {
      const threads = [...fixture.threads.values()].filter(
        (thread) =>
          thread.value.comment_source &&
          targetKey(thread.value.comment_source.target) ===
            targetKey(message.target),
      );
      const replies = threads.flatMap((thread) => {
        const latest = new Map<string, Model<"AgentExecutionFeedbackReply">>();
        for (const event of fixture.history.get(thread.value.thread_id) ?? []) {
          if (
            !event.message ||
            event.value.event.type !== "message" ||
            latest.has(event.message.message_id)
          )
            continue;
          latest.set(event.message.message_id, {
            thread_id: thread.value.thread_id,
            thread_revision: thread.reference,
            source: thread.value.comment_source!,
            historical: false,
            batches: batchesFor(
              event.message.message_id,
              event.value.event.revision.id,
            ),
            answers: [],
            message: event.message,
            revision: event.value.event.revision,
          });
        }
        return [...latest.values()];
      });
      return {
        ...message,
        threads: threads.map((t) => t.value.thread_id),
        replies,
      };
    });
    return json({
      messages: options.unpublished ? [] : messages,
      ...(options.writeUnavailable
        ? {
            write_unavailable:
              "Comments are unavailable until collaboration replication is qualified.",
          }
        : {}),
    });
  }
  async function respond(
    path: string,
    init?: RequestInit,
  ): Promise<Response | undefined> {
    const query = new URL(path, "http://fixture");
    if (query.pathname.endsWith("/agents/feedback")) return feedback();
    if (query.pathname.endsWith("/agents/command")) {
      const command: InlineAgentCommand = JSON.parse(String(init?.body));
      options.observe(command);
      if (receipts.has(command.request_id))
        return json(receipts.get(command.request_id));
      if (command.action.action === "resume") options.resume();
      const batch =
        command.action.action === "prompt"
          ? command.action.comments
          : undefined;
      if (batch)
        await transport.command("threads", {
          version: 1,
          request_id: command.request_id,
          command: { type: "submit", submission: batch },
        });
      const outcome: AgentOutcome = {
        version: 1,
        request_id: command.request_id,
        session_id: command.session_id,
        state: uncertain && batch ? "outcome_unknown" : "queued",
        error: uncertain && batch ? "connection_lost" : null,
        comment_publication: batch
          ? {
              request_id: command.request_id,
              state: uncertain ? "unknown" : "published",
              batch: { kind: "thread_link", id: "ab".repeat(32) },
            }
          : null,
      };
      receipts.set(command.request_id, outcome);
      return json(outcome);
    }
    if (query.pathname.endsWith("/agents/outcome")) {
      const id = query.searchParams.get("request")!;
      options.observe(undefined, id);
      return json(receipts.get(id));
    }
    if (!path.startsWith("/v1/repos/")) return undefined;
    const body = init?.body ? JSON.parse(String(init.body)) : undefined;
    if (path.includes("/reaction-assets/")) {
      const asset = await transport.asset(query.pathname.split("/").at(-1)!);
      return new Response(new Uint8Array(asset.bytes).buffer, {
        headers: { "content-type": asset.type },
      });
    }
    if (path.includes("/reaction-emojis"))
      return json(await transport.emojis());
    if (path.includes("/reactions/query"))
      return json(await transport.reactions(body.target));
    const kind = path.includes("/reactions/") ? "reactions" : "threads";
    if (path.includes("/commands/"))
      return json(
        await transport.status(kind, query.pathname.split("/").at(-1)!),
      );
    if (path.endsWith("/commands"))
      return json(
        await transport.command(kind, body as ThreadRequest | ReactionRequest),
      );
    const id = query.pathname.split("/")[5];
    if (path.includes("/timeline")) return json(await transport.events(id));
    if (id)
      return fixture.threads.has(id)
        ? json(await transport.thread(id))
        : new Response(JSON.stringify({ code: "not_found" }), {
            status: 404,
            headers: { "content-type": "application/json" },
          });
    return json(await transport.list());
  }
  return {
    respond,
    failFeedback() {
      failFeedback = true;
    },
    recoverFeedback() {
      failFeedback = false;
    },
    confirm() {
      uncertain = false;
      for (const [id, receipt] of receipts)
        receipts.set(id, {
          ...receipt,
          state: "queued",
          error: null,
          comment_publication: receipt.comment_publication
            ? { ...receipt.comment_publication, state: "published" }
            : null,
        });
    },
  };
}

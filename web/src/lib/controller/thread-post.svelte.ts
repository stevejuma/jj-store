import { ApiError } from "../api/client.js";
import { collabRequestId } from "./collab-actions.svelte.js";
import type { ThreadRequest, ThreadTransport } from "./thread-types.js";

type Draft = {
  title: string;
  body?: string;
  thread: string;
  request: ThreadRequest;
  phase: "create" | "body";
  committed: boolean;
  running: boolean;
  retryBody: boolean;
};

// An inline forum post uses two native receipts. Retain drafts per authorized
// repository/principal; a recreated transport can recover the original request.
export function createThreadPost(options: {
  source: () => ThreadTransport;
  repository: () => string;
  principal: () => string;
  execute: (request: ThreadRequest) => Promise<void>;
  pending: () => boolean;
  open: (id: string) => Promise<void>;
}) {
  const drafts = new Map<string, Draft>();
  let postedThreadId = $state("");
  const identity = () => `${options.repository()}:${options.principal()}`;
  function committed(id: string) {
    for (const draft of drafts.values())
      if (draft.request.request_id === id) draft.committed = true;
  }
  async function resume() {
    const key = identity(),
      current = drafts.get(key);
    if (!current || current.running || options.pending()) return;
    const transport = options.source();
    const valid = () => {
      try {
        return (
          drafts.get(key) === current &&
          identity() === key &&
          options.source() === transport
        );
      } catch {
        return false;
      }
    };
    async function prepareBody() {
      current!.phase = "body";
      current!.retryBody = true;
      const detail = await transport.thread(current!.thread);
      if (!valid()) return false;
      current!.committed = false;
      current!.retryBody = false;
      current!.request = {
        version: 1,
        request_id: collabRequestId(),
        command: {
          type: "message",
          thread_id: current!.thread,
          expected_thread: detail.thread.reference,
          message_id: collabRequestId(),
          expected_revision: null,
          reply_to: null,
          body: current!.body!,
        },
      };
      return true;
    }
    current.running = true;
    try {
      if (current.retryBody && !(await prepareBody())) return;
      if (!current.committed) await options.execute(current.request);
      if (!valid() || !current.committed) return;
      if (current.phase === "create" && current.body) {
        if (!(await prepareBody())) return;
        await options.execute(current.request);
      }
      if (valid() && current.committed) {
        postedThreadId = current.thread;
        drafts.delete(key);
        await options.open(current.thread);
      }
    } catch (error) {
      if (valid() && !options.pending()) {
        if (current.phase === "create") drafts.delete(key);
        // Leave the composer mounted with its content after a definitive body
        // rejection. Publishing it again continues the already-created thread.
        else current.retryBody = true;
      }
      throw error;
    } finally {
      current.running = false;
    }
  }
  return {
    committed,
    resume,
    get postedThreadId() {
      return postedThreadId;
    },
    clear() {
      drafts.clear();
    },
    async create(
      title: string,
      body?: string,
      target?: import("./thread-types.js").ReactionTarget,
    ) {
      const key = identity(),
        existing = drafts.get(key);
      if (
        options.pending() ||
        existing?.running ||
        (existing && !existing.retryBody)
      )
        throw new ApiError("check_pending_action_first");
      if (existing?.retryBody && existing.title === title.trim() && body) {
        existing.body = body;
        await resume();
        return;
      }
      // A changed title is an explicit new post; the rejected initial message
      // has never committed. The earlier created thread remains in the directory.
      const thread = collabRequestId();
      drafts.set(key, {
        title: title.trim(),
        body,
        thread,
        phase: "create",
        committed: false,
        running: false,
        retryBody: false,
        request: {
          version: 1,
          request_id: collabRequestId(),
          command: target
            ? {
                type: "create_agent_discussion",
                thread_id: thread,
                title: title.trim(),
                target,
              }
            : {
                type: "create",
                thread_id: thread,
                title: title.trim(),
                baseline: null,
              },
        },
      });
      await resume();
    },
  };
}

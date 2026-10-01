import { ApiError } from "../api/client.js";
import type {
  ThreadRequest,
  ReactionRequest,
  ThreadTransport,
} from "./thread-types.js";
export const collabRequestId = () => crypto.randomUUID().replaceAll("-", "");
type Intent = {
  kind: "threads" | "reactions";
  request: ThreadRequest | ReactionRequest;
};
const rejected = new Set([
  "access_denied",
  "invalid_request",
  "stale_collaboration",
  "intent_conflict",
  "collaboration_replication_unsupported",
  "not_found",
]);
export function createCollabActions(options: {
  source: () => ThreadTransport;
  repository: () => string;
  principal: () => string;
  generation: () => number;
  valid: (ticket: number, transport: ThreadTransport) => boolean;
  refresh: () => Promise<void>;
  onCommitted?: (id: string) => void;
}) {
  const identity = () => `${options.repository()}:${options.principal()}`;
  let epoch = 0;
  let intents = $state<Record<string, Intent>>({});
  let active = $state<Record<string, boolean>>({});
  let errors = $state<Record<string, string>>({});
  let errorKinds = $state<Record<string, Intent["kind"]>>({});
  const remove = (repo: string) => {
    const next = { ...intents };
    delete next[repo];
    intents = next;
  };
  async function run(intent: Intent) {
    const repo = identity(),
      transport = options.source(),
      ticket = options.generation(),
      stamp = epoch;
    intents = { ...intents, [repo]: intent };
    active = { ...active, [repo]: true };
    errors = { ...errors, [repo]: "" };
    try {
      const outcome = await transport.command(intent.kind, intent.request);
      if (stamp !== epoch) return;
      if (outcome.status !== "committed")
        throw new Error(
          "The action outcome is unknown. Check its status before continuing.",
        );
      if (outcome.receipt.request_id !== intent.request.request_id)
        throw new ApiError("receipt_identity_mismatch");
      remove(repo);
      options.onCommitted?.(intent.request.request_id);
      if (options.valid(ticket, transport)) await options.refresh();
    } catch (cause) {
      if (stamp !== epoch) return;
      if (cause instanceof ApiError && rejected.has(cause.code)) remove(repo);
      const message =
        cause instanceof ApiError &&
        cause.code === "collaboration_replication_unsupported"
          ? "Threads and reactions are unavailable for this replicated code repository until collaboration replication is supported. Use a WorkStore home."
          : cause instanceof Error
            ? cause.message
            : String(cause);
      errors = { ...errors, [repo]: message };
      errorKinds = { ...errorKinds, [repo]: intent.kind };
      throw new Error(message);
    } finally {
      if (stamp === epoch) active = { ...active, [repo]: false };
    }
  }
  return {
    reset(preservePending = false) {
      epoch++;
      active = {};
      errors = {};
      errorKinds = {};
      if (!preservePending) intents = {};
    },
    get pending() {
      return intents[identity()] ?? null;
    },
    get busy() {
      return active[identity()] ?? false;
    },
    get errorKind() {
      return errors[identity()]
        ? (errorKinds[identity()] ?? intents[identity()]?.kind)
        : intents[identity()]?.kind;
    },
    get error() {
      return (
        errors[identity()] ||
        (intents[identity()] && !active[identity()]
          ? "An action is awaiting confirmation. Check its status before continuing."
          : "")
      );
    },
    async execute(kind: Intent["kind"], request: Intent["request"]) {
      if (active[identity()] || intents[identity()])
        throw new ApiError("check_pending_action_first");
      await run({ kind, request });
    },
    async checkPending() {
      const repo = identity(),
        intent = intents[repo];
      if (!intent || active[repo]) return;
      const transport = options.source(),
        ticket = options.generation(),
        stamp = epoch;
      active = { ...active, [repo]: true };
      try {
        const result = await transport.status(
          intent.kind,
          intent.request.request_id,
        );
        if (stamp !== epoch) return;
        if (result.status === "committed") {
          if (result.receipt.request_id !== intent.request.request_id)
            throw new ApiError("receipt_identity_mismatch");
          remove(repo);
          options.onCommitted?.(intent.request.request_id);
          errors = { ...errors, [repo]: "" };
          if (options.valid(ticket, transport)) await options.refresh();
        } else
          errors = {
            ...errors,
            [repo]:
              "The original request is retained. Check again or retry the same action.",
          };
      } catch (cause) {
        if (stamp !== epoch) return;
        errors = {
          ...errors,
          [repo]: cause instanceof Error ? cause.message : String(cause),
        };
      } finally {
        if (stamp === epoch) active = { ...active, [repo]: false };
      }
    },
    async retryPending() {
      const repo = identity(),
        intent = intents[repo];
      if (intent && !active[repo]) await run(intent);
    },
  };
}

import { onDestroy } from "svelte";
import type { AgentExecution } from "../controller/agent-execution.svelte.js";
import {
  object,
  type AgentOutcome,
  type AgentSession,
} from "../controller/agent-execution-types.js";
import type { ConversationSession } from "./session.svelte.js";
import type {
  ChatPromptSubmission,
  ChatSubmissionResult,
} from "../ui/store-agent-chat-body/model.js";
import {
  runtimeMatches,
  waitForRuntime,
} from "../ui/store-agent-execution/runtime-preparation.js";
import { prepareStandaloneRuntime } from "./prepare-standalone-runtime.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";
export function createStandaloneSender(
  read: () => {
    manager: AgentExecution;
    session?: AgentSession;
    conversation: ConversationSession;
    preferences?: AgentSession["preferences"];
    idle: boolean;
    ready: boolean;
    canConfigure: boolean;
  },
) {
  let preparing = $state(false),
    preparationError = $state("");
  let mounted = true;
  onDestroy(() => {
    mounted = false;
    read().manager.runtimeObservation.notify();
  });
  async function send(
    submission: ChatPromptSubmission,
  ): Promise<ChatSubmissionResult> {
    const {
      manager,
      session,
      conversation,
      preferences,
      idle,
      ready,
      canConfigure,
    } = read();
    const id = manager.selected,
      draft = submission.text,
      wanted = preferences && { ...preferences };
    if (
      (!draft?.trim() && !submission.comments.length) ||
      !session ||
      session.state === "cleaned" ||
      (!idle && !session.setup_id) ||
      (!ready && !canConfigure && !preferences?.mode) ||
      manager.pending ||
      manager.busy ||
      preparing
    )
      throw new Error(
        "The agent is unavailable for sending. Your drafts are retained.",
      );
    let dispatched = false;
    preparing = true;
    preparationError = "";
    try {
      if (!(await prepareStandaloneRuntime(manager, id, () => mounted)))
        throw new Error(
          manager.error ||
            "The agent could not be prepared. Your draft is retained.",
        );
      if (
        wanted &&
        (!runtimeMatches(manager.session, wanted) ||
          object(manager.session?.capabilities).modeReady !== true)
      ) {
        const request = await manager.commandSession(id, {
          action: "configure",
          preferences: wanted,
        });
        if (
          !(await waitForRuntime(manager, id, request, wanted, () => mounted))
        ) {
          if (mounted && manager.selected === id)
            preparationError =
              "Runtime configuration was not confirmed. Your draft is retained. Check the pending request outcome before continuing.";
          throw new Error(preparationError);
        }
      }
      if (
        !mounted ||
        manager.selected !== id ||
        !manager.session?.connected ||
        manager.pending ||
        manager.error ||
        !runtimeMatches(manager.session, wanted) ||
        !["idle", "running"].includes(manager.session.state) ||
        object(manager.session.capabilities).modeReady !== true
      )
        throw new Error(
          "The runtime changed before dispatch. Your draft is retained.",
        );
      let comments;
      if (submission.comments.length && session) {
        await manager.discussions.load(id);
        const threads = manager.discussions.reader(session.repository);
        const identity = (suffix: string) =>
          bytesToHex(
            sha256(
              new TextEncoder().encode(
                `conversation/${session.repository}/${id}/${suffix}`,
              ),
            ),
          ).slice(0, 32);
        const parent = await threads.ensureConversation(
          identity("thread"),
          identity("create"),
        );
        comments = conversation.prepare(submission, {
          id: parent.thread.value.thread_id,
          revision: parent.thread.reference,
        });
      }
      dispatched = true;
      await manager.commandSession(
        id,
        { action: "prompt", text: draft, max_duration_ms: 900000, comments },
        undefined,
        submission.id,
      );
      const outcome = await manager.client.request<AgentOutcome>(
        `agents/outcome?request=${submission.id}`,
      );
      return manager.promptQueue.submitted(submission, {
        request_id: submission.id,
        conversation_id: id,
        comment_publication: outcome.comment_publication ?? null,
        agents: [outcome],
      });
    } catch (cause) {
      if (dispatched)
        return {
          publication: "unknown",
          error:
            manager.error || "Check the retained request before sending again.",
        };
      throw cause;
    } finally {
      preparing = false;
    }
  }
  return {
    send,
    get preparing() {
      return preparing;
    },
    get error() {
      return preparationError;
    },
  };
}

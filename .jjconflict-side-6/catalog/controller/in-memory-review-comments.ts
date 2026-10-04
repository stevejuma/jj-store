import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type {
  StoreRoute,
  StoreControllerState,
} from "../../web/src/lib/controller/contracts.js";
import type { ReviewInlinePatch } from "../../web/src/lib/controller/review-anchor-patch.js";
import { createFixtureLineReview } from "./in-memory-inline-review.js";
import { editFixtureReviewComment } from "./fixture-review-edit.js";
import { reviewForAgentTool } from "../../web/src/lib/collaboration/presentation.js";
import { reviewDetail, reviewThreads } from "../fixtures/collaboration.js";

type Detail = NonNullable<StoreControllerState["reviewThread"]>;
type Thread = StoreControllerState["reviewThreads"][number];
export function createFixtureReviewCommentActions(context: {
  session(): NonNullable<StoreControllerState["agentSession"]>;
  threads(): Thread[];
  setThreads(threads: Thread[]): void;
  detail(): Detail;
  setDetail(detail: Detail): void;
  selectReview(thread: Thread): void;
  retainReview(): void;
  readDetail(id: string): Detail | undefined;
  selectThread(id: string): void;
  appendReview(text: string): Promise<void>;
  pending(): boolean;
  route(): StoreRoute;
  patch(value: ReviewInlinePatch): void;
  publish(update: () => void, request?: string): void;
  fail(message: string): void;
}) {
  return {
    async commentOnDiffLine(line: FileDiffLineContext, text: string) {
      if (!text.trim() || line.lineNumber === null) return;
      const { thread, patch } = createFixtureLineReview(
        context.route(),
        line,
        context.threads()[0]!,
      );
      context.setThreads([...context.threads(), thread]);
      context.selectReview(thread);
      context.setDetail({ ...context.detail(), comments: [] });
      context.patch(patch);
      context.selectThread(thread.thread_id);
      await context.appendReview(text);
    },
    async editReviewComment(
      id: string,
      expectedRevisionId: string,
      text: string,
    ) {
      const changed = editFixtureReviewComment(
        context.detail(),
        id,
        expectedRevisionId,
        text,
      );
      if (!changed) {
        context.fail("stale review");
        return;
      }
      context.publish(() => {
        context.setDetail(changed);
        context.retainReview();
      }, "31".repeat(16));
    },
    async commentOnAgentTool(toolCallId: string, text: string) {
      if (!text.trim() || context.pending()) return;
      const sessionId = context.session().session.session_id;
      let thread = reviewForAgentTool(context.threads(), sessionId, toolCallId);
      if (!thread) {
        thread = {
          ...reviewThreads[0]!,
          thread_id: toolCallId.slice(0, 30) + "77",
          comment_count: 0,
          anchor: {
            ...reviewThreads[0]!.anchor,
            target: {
              target: "agent_artifact",
              value: {
                session_id: sessionId,
                session_snapshot: {
                  kind: "session_snapshot",
                  id: context.session().session.snapshot_id,
                },
                owner: {
                  kind: "tool",
                  tool_call_id: toolCallId,
                  attempt_id: toolCallId,
                  snapshot: { kind: "tool_snapshot", id: "aa".repeat(32) },
                },
              },
            },
          },
        };
        context.setThreads([...context.threads(), thread]);
        context.selectReview(thread);
        context.setDetail({ ...context.detail(), comments: [] });
        context.retainReview();
      }
      if (thread) {
        context.selectReview(thread);
        context.selectThread(thread.thread_id);
      }
      await context.appendReview(text);
    },
    async readAgentToolReview(toolCallId: string) {
      const thread = reviewForAgentTool(
        context.threads(),
        context.session().session.session_id,
        toolCallId,
      );
      if (!thread) return;
      return (
        context.readDetail(thread.thread_id) ?? {
          ...reviewDetail(),
          thread,
          anchor: {
            object_id: thread.original_anchor_id,
            value: thread.anchor,
          },
        }
      );
    },
    async readAgentSessionReview() {
      const thread = context
        .threads()
        .find(
          (row) =>
            row.anchor.target.target === "agent_session" &&
            row.anchor.target.value.session_id ===
              context.session().session.session_id,
        );
      return thread ? context.readDetail(thread.thread_id) : undefined;
    },
    async commentOnAgentSession(text: string) {
      if (!text.trim() || context.pending()) return;
      const sessionId = context.session().session.session_id;
      let thread = context
        .threads()
        .find(
          (row) =>
            row.anchor.target.target === "agent_session" &&
            row.anchor.target.value.session_id === sessionId,
        );
      if (!thread) {
        thread = {
          ...reviewThreads[0]!,
          thread_id: sessionId.slice(0, 30) + "88",
          comment_count: 0,
          anchor: {
            ...reviewThreads[0]!.anchor,
            target: {
              target: "agent_session",
              value: {
                session_id: sessionId,
                session_snapshot: {
                  kind: "session_snapshot",
                  id: context.session().session.snapshot_id,
                },
              },
            },
          },
        };
        context.setThreads([...context.threads(), thread]);
        context.selectReview(thread);
        context.setDetail({ ...context.detail(), comments: [] });
        context.retainReview();
      } else context.selectReview(thread);
      context.selectThread(thread.thread_id);
      await context.appendReview(text);
    },
  };
}

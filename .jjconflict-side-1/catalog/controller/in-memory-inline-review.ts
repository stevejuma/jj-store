import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type {
  StoreControllerState,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { ReviewInlinePatch } from "../../web/src/lib/controller/review-anchor-patch.js";
import { reviewPatch } from "../fixtures/collaboration.js";

type ReviewThread = StoreControllerState["reviewThreads"][number];

export function createFixtureLineReview(
  route: StoreRoute,
  line: FileDiffLineContext,
  base: ReviewThread,
): { thread: ReviewThread; patch: ReviewInlinePatch } {
  const threadId = "7a".repeat(16);
  const anchor = {
    schema_version: 1,
    repository_id: route.repository,
    target: {
      target: "code_text" as const,
      value: {
        operation_id: route.snapshotOperationId!,
        commit_id: route.commitId ?? route.inspectedOperationId!,
        change_id: route.changeId,
        path: line.path,
        file_id: "7b".repeat(32),
        value_kind: "file" as const,
        text: {
          encoding: "utf8" as const,
          start_byte: "0",
          end_byte: "1",
          selected_sha256: "7c".repeat(32),
        },
        diff_context: {
          before_operation_id:
            route.comparisonParentId ?? route.snapshotOperationId!,
          before_commit_id: route.commitId ?? route.inspectedOperationId!,
          before_path: line.path,
          after_operation_id: route.snapshotOperationId!,
          after_commit_id: route.commitId ?? route.inspectedOperationId!,
          after_path: line.path,
          side:
            line.variant === "removed"
              ? ("before" as const)
              : ("after" as const),
          comparison: "explicit_pair" as const,
        },
      },
    },
    created_context: {
      publication_position: 25,
      code_state_version: 8,
      agent_root: null,
    },
    sensitivity: "repository" as const,
  };
  return {
    thread: {
      ...base,
      thread_id: threadId,
      original_anchor_id: "7d".repeat(32),
      snapshot_id: "7e".repeat(32),
      anchor,
      comment_count: 0,
    },
    patch: {
      ...reviewPatch,
      path: line.path,
      lineNumber: line.lineNumber ?? undefined,
      side: line.variant === "removed" ? "before" : "after",
    },
  };
}

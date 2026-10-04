import {
  anchorCommit,
  anchorMatchesOperation,
  anchorMatchesPath,
} from "../collaboration/presentation.js";
import type { StoreRoute } from "./contracts.js";
import type { readReviewContext } from "./production-collaboration-context.js";
export async function refreshCollaborationContext(
  route: StoreRoute,
  load: (
    subject: { commit: string } | { operation: string },
    matches: Parameters<typeof readReviewContext>[1],
    retainSelection: boolean,
  ) => Promise<void>,
) {
  const { commitId, inspectedOperationId, path } = route;
  if (!commitId && !inspectedOperationId) return;
  await load(
    inspectedOperationId
      ? { operation: inspectedOperationId }
      : { commit: commitId! },
    (thread) =>
      (inspectedOperationId
        ? anchorMatchesOperation(thread.anchor, inspectedOperationId)
        : anchorCommit(thread.anchor) === commitId) &&
      (!path || anchorMatchesPath(thread.anchor, path)),
    true,
  );
}

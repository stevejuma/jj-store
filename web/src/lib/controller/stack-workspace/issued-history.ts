import { LoadedStackRanges, TARGET_RANGE } from "./loaded-ranges.js";
import type { StackWorkspaceReads } from "./reads.js";
import {
  WorkspaceCommandError,
  type WorkspaceHistoryRequest,
  type WorkspaceStackHistoryRequest,
  type WorkspaceSnapshot,
  type StackCommit,
} from "./types.js";

/** History reads authorize only their exact advertised head and issued comparison scope. */
export async function readIssuedStackHistory(
  owner: {
    reads: StackWorkspaceReads;
    current(): WorkspaceSnapshot;
    commits: Map<string, StackCommit>;
    ranges: LoadedStackRanges;
  },
  request: WorkspaceHistoryRequest | WorkspaceStackHistoryRequest,
  signal?: AbortSignal,
) {
  const snapshot = owner.current(),
    stackRequest = "stack" in request ? request : undefined;
  if (stackRequest) {
    const stack = snapshot.stacks.find(
      (stack) => stack.id === stackRequest.stack,
    );
    const head = stack?.heads.find(
      (head) => head.name === stack.name && head.name === request.branch,
    );
    if (
      !head ||
      head.commit !== request.head ||
      snapshot.target?.commit !== stackRequest.base
    )
      throw new WorkspaceCommandError(
        "stale",
        "The stack head or target changed.",
      );
  } else if (
    !snapshot.target ||
    request.branch !== snapshot.target.name ||
    request.head !== snapshot.target.commit
  )
    throw new WorkspaceCommandError("stale", "The target branch changed.");
  const page = await owner.reads.history({
    head: request.head,
    ...(stackRequest ? { base: stackRequest.base } : {}),
    limit: request.limit,
    cursor: request.cursor,
    signal,
  });
  owner.current();
  for (const commit of page.commits) owner.commits.set(commit.id, commit);
  owner.ranges.record(
    stackRequest?.stack ?? TARGET_RANGE,
    request.head,
    stackRequest?.base,
    page.commits,
    Boolean(request.cursor),
    page.nextCursor,
  );
  return page;
}

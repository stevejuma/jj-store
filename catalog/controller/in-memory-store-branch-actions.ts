import type { Bookmark } from "../../web/src/lib/projects/presentation.js";
import type { RepositoryWorkingCopy } from "../../web/src/lib/projects/repository-log.js";
import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";

export function createInMemoryStoreBranchActions(context: {
  bookmarks(): readonly Bookmark[];
  workingCopies: readonly RepositoryWorkingCopy[];
  route(): StoreRoute;
  push(route: StoreRoute): void;
}) {
  return {
    selectBookmark(name: string) {
      if (!context.bookmarks().some((bookmark) => bookmark.name === name))
        throw new Error("Unknown fixture bookmark");
      context.push({
        ...context.route(),
        bookmark: name,
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
    selectWorkingCopy(commit: string, workspace?: string) {
      const workingCopy = context.workingCopies.find((candidate) =>
        workspace
          ? candidate.workspace === workspace
          : candidate.commitId === commit,
      );
      if (!workingCopy) throw new Error("Unknown fixture working copy");
      context.push({
        ...context.route(),
        bookmark: "",
        workspace: workingCopy.workspace,
        inspectedOperationId: null,
        commitId: workingCopy.commitId,
        changeId: workingCopy.changeId,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
  };
}

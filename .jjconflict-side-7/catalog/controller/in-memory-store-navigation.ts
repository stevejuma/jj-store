import { workspaceSelectedRoute } from "../../web/src/lib/controller/store-workspaces.js";
import {
  createStoreStacksNavigation,
  storePageRoute,
} from "../../web/src/lib/controller/store-stacks-navigation.js";
import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
import type { RepositoryWorkingCopy } from "../../web/src/lib/projects/repository-log.js";
import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
import { repositories } from "../fixtures/snapshots.js";
import * as fixture from "../fixtures/repository.js";
export function createInMemoryStoreNavigation(context: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  workingCopies: readonly RepositoryWorkingCopy[];
}) {
  const stacks = createStoreStacksNavigation(context);
  return {
    selectStacksView: stacks.select,
    leaveStacks: stacks.leave,
    selectWorkspace(workspace: string | null) {
      if (
        workspace &&
        !context.workingCopies.some((copy) => copy.workspace === workspace)
      )
        throw new Error("Unknown fixture workspace");
      context.push(workspaceSelectedRoute(context.route(), workspace));
    },
    openRepository(id: string) {
      if (!repositories.some((repo) => repo.id === id))
        throw new Error("Unknown fixture repository");
      context.push({
        page: "files",
        repository: id,
        workspace: null,
        bookmark: "main",
        snapshotOperationId: fixture.IDS.opLeft,
        inspectedOperationId: null,
        commitId: null,
        evolutionCommitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
        agentSessionId: null,
        reviewThreadId: null,
      });
    },
    selectPage(page: StorePage) {
      if (page === "stacks") {
        stacks.enter();
        return;
      }
      context.push({
        ...storePageRoute(context.route(), page),
        bookmark: page === "files" && !context.route().workspace ? "main" : "",
        inspectedOperationId: null,
        commitId: null,
        evolutionCommitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
        agentSessionId: null,
        reviewThreadId: null,
      });
    },
  };
}

import type {
  StoreController,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { createInMemoryStoreCollaboration } from "./in-memory-store-collaboration.svelte.js";
export function createInMemoryFileActions(context: {
  route(): StoreRoute;
  snapshot(): StoreController["state"]["snapshot"];
  push(route: StoreRoute): void;
  collaboration: ReturnType<typeof createInMemoryStoreCollaboration>;
}) {
  return {
    selectFile(path: string | null) {
      const route = context.route();
      if (
        path &&
        route.page !== "oplog" &&
        !context.snapshot()?.files.some((file) => file.path === path)
      )
        throw new Error("File absent from selected snapshot");
      context.push({
        ...route,
        path,
        directory: path
          ? path.split("/").slice(0, -1).join("/")
          : route.directory,
        reviewThreadId:
          path && route.commitId
            ? context.collaboration.pathReview(route.commitId, path)
            : path && route.inspectedOperationId
              ? context.collaboration.operationPathReview(
                  route.inspectedOperationId,
                  path,
                )
              : null,
      });
    },
    selectDirectory(directory: string) {
      const route = context.route();
      if (
        directory &&
        !context
          .snapshot()
          ?.files.some((file) => file.path.startsWith(`${directory}/`))
      )
        throw new Error("Directory absent from selected snapshot");
      context.push({ ...route, directory, path: null });
    },
  };
}

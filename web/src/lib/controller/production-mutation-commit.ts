import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import type {
  StoreMutationCommand,
  StoreMutationReceipt,
} from "./store-mutation-types.js";

export function createProductionMutationCommitter(
  history: ReturnType<typeof createStoreRouteHistory>,
  refresh: () => Promise<void>,
) {
  return async (
    receipt: StoreMutationReceipt,
    command: StoreMutationCommand,
  ) => {
    if (command.type === "edit_file" && receipt.result.created[0]) {
      const destination = command.destination;
      history.replace({
        ...history.route,
        page: "files",
        bookmark: command.bookmark,
        commitId: receipt.result.created[0],
        evolutionCommitId: null,
        changeId: null,
        path: destination,
        directory: destination.split("/").slice(0, -1).join("/"),
      });
    }
    await refresh();
  };
}

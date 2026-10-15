import type { StoreController } from "../../controller/contracts.js";

export function projectsBusy(controller: StoreController): boolean {
  return Boolean(
    controller.projects?.loading ||
    controller.state.status.connection === "connecting" ||
    controller.state.status.repositories === "loading",
  );
}

/** Initial placeholders include discovery; retained snapshots survive refresh. */
export function projectsInitialLoading(controller: StoreController): boolean {
  const hub = controller.projects!;
  const detail = hub.detail?.state;
  if (
    !(projectsBusy(controller) || detail?.loading) ||
    hub.error ||
    detail?.error
  )
    return false;
  if (hub.route.taskId) return detail?.task?.id !== hub.route.taskId;
  if (hub.route.projectId && detail?.project?.id !== hub.route.projectId)
    return true;
  if (hub.route.projectId && hub.route.workTab === "repositories") return false;
  switch (hub.route.workTab) {
    case "activity":
      return !hub.activity.homes.some((home) => home.page || home.error);
    case "agents":
      return !hub.agents.homes.some(
        (home) => home.snapshot || home.error || home.unsupported,
      );
    case "reviews":
      return !hub.reviews.homes.some(
        (home) => home.snapshot || home.error || home.unsupported,
      );
    case "repositories":
      return !hub.repositories.entries.some(
        (entry) => entry.summary || entry.error,
      );
    default:
      return !hub.homes.some((home) => home.snapshot || home.error);
  }
}

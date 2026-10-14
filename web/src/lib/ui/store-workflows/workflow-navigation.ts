import type {
  StoreController,
  StoreRoute,
} from "../../controller/contracts.js";
import type { NotificationContext } from "../../notifications/notification-context.svelte.js";
import { storeRouteToHash } from "../../controller/store-route-writer.js";
import {
  workflowsRoute,
  type WorkflowSelection,
} from "../../controller/workflows-route-codec.js";
export function workflowViewSelection(
  route: StoreRoute,
  repository?: string,
): WorkflowSelection {
  return repository
    ? { ...route.workflow, tab: "history", source: repository }
    : (route.workflow ?? {});
}
export function workflowViewRoute(
  route: StoreRoute,
  selection: WorkflowSelection,
  fields: WorkflowSelection,
  repository?: string,
): StoreRoute {
  const selected = { ...selection, ...fields };
  return repository
    ? {
        ...route,
        page: "actions",
        workflow: { ...selected, tab: "history", source: repository },
      }
    : workflowsRoute(selected);
}

export function navigateWorkflowView(
  controller: StoreController,
  selection: WorkflowSelection,
  fields: WorkflowSelection,
  context: NotificationContext,
  repository?: string,
) {
  const next = workflowViewRoute(
    controller.state.route,
    selection,
    fields,
    repository,
  );
  if (!storeRouteToHash(next)) {
    context.service.publish({
      scope: context.scope,
      key: "workflow-route",
      title: "Workflow search unavailable",
      message:
        "Search is limited to 512 UTF-8 bytes without control characters.",
      severity: "error",
      lifetime: "dismiss",
    });
    return;
  }
  context.service.resolve(context.scope, "workflow-route");
  void controller.actions.openRoute?.(next);
}

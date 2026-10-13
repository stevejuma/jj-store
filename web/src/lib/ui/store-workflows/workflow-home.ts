import type { StoreController } from "../../controller/contracts.js";
import type { NotificationContext } from "../../notifications/notification-context.svelte.js";
/** Reject an inappropriate metadata home before allocating a recoverable command identity. */
export async function validateWorkflowHome(
  controller: StoreController,
  home: string,
  context: NotificationContext,
) {
  const snapshot = (
    await controller
      .projects!.transport(home)
      .browse({ kind: "projects", limit: 1 })
  ).snapshot;
  if (!("work_root" in snapshot) || !snapshot.work_root) {
    context.service.publish({
      scope: context.scope,
      key: `workflow-home-${home}`,
      title: "Choose a metadata home",
      message:
        "This repository has no WorkStore project yet. Choose an existing project home or create a project here first.",
      severity: "error",
      lifetime: "dismiss",
    });
    return false;
  }
  context.service.resolve(context.scope, `workflow-home-${home}`);
  return true;
}

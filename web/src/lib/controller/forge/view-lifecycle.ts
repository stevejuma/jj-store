import { untrack } from "svelte";
import type { ForgeController } from "./controller.svelte.js";
import type { NotificationContext } from "../../notifications/notification-context.svelte.js";
/** Connect only this view's feedback owner; polling belongs to the repository. */
export function connectForgeView(
  forge: ForgeController | undefined,
  notifications: NotificationContext,
) {
  if (!forge) return;
  forge.notifications = notifications;
  untrack(() => void forge.refresh());
}

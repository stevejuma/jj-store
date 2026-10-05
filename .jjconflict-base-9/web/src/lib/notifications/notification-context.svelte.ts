import { getContext, setContext, onDestroy, untrack } from "svelte";
import { NotificationService } from "./notification-service.svelte.js";
import type { StoreController } from "../controller/contracts.js";

import { observeNotificationRecovery } from "./notification-recovery.svelte.js";

const key = Symbol("store-notifications");
export interface NotificationContext {
  service: NotificationService;
  scope: string;
}

/** The application owns one service; isolated catalog surfaces get their own owner. */
export function ownNotifications() {
  const inherited = getContext<NotificationContext | undefined>(key);
  if (inherited) return { context: inherited, owned: false };
  const context = $state<NotificationContext>({
    service: new NotificationService(),
    scope: "application",
  });
  setContext(key, context);
  onDestroy(() => context.service.dispose());
  return { context, owned: true };
}

export function ownApplicationNotifications(controller: () => StoreController) {
  const owner = ownNotifications();
  if (owner.owned) observeNotificationScope(owner.context, controller);
  return owner;
}

export function observeNotificationScope(
  context: NotificationContext,
  controller: () => StoreController,
) {
  observeNotificationRecovery(context, controller);
  let previousSession: string | undefined;
  $effect(() => {
    const state = controller().state;
    const session = JSON.stringify([
      state.authenticated,
      state.accessMode,
      state.repository.principal,
    ]);
    const scope = JSON.stringify([
      state.route.repository,
      state.route.page,
      state.route.projectsHub,
      state.route.workspace,
      state.route.bookmark,
      state.route.directory,
      state.route.commitId,
      state.route.evolutionCommitId,
      state.route.inspectedOperationId,
      state.route.comparisonParentId,
      state.route.path,
      state.route.agentSessionId,
      state.route.reviewThreadId,
      state.route.threadId,
      state.route.projectId,
      state.route.taskId,
    ]);
    untrack(() => {
      if (previousSession !== undefined && previousSession !== session)
        context.service.reset();
      else if (context.scope !== scope)
        context.service.leaveScope(context.scope);
      context.scope = scope;
      previousSession = session;
    });
  });
}

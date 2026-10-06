import { untrack } from "svelte";
import type { StoreController } from "../controller/contracts.js";
import type { NotificationContext } from "./notification-context.svelte.js";

/** A hidden Projects tab can recover without remounting its notice producer. */
export function observeProjectNotificationRecovery(
  context: NotificationContext,
  controller: () => StoreController,
) {
  const observed = new Set<string>();
  $effect(() => {
    const hub = controller().projects;
    const collections = [
      ["home", hub?.homes ?? []],
      ["navigation", hub?.navigation.homes ?? []],
      ["reviews", hub?.reviews.homes ?? []],
      ["agents", hub?.agents.homes ?? []],
      ["activity", hub?.activity.homes ?? []],
    ] as const;
    const conditions = collections.flatMap(([kind, homes]) =>
      homes.map((home) => ({
        key: `project-${kind}:${home.repository.id}`,
        error: home.error,
        loading: home.loading,
      })),
    );
    const current = new Set(conditions.map(({ key }) => key));
    untrack(() => {
      for (const key of observed)
        if (!current.has(key)) context.service.resolve("projects-reads", key);
      observed.clear();
      for (const condition of conditions) {
        observed.add(condition.key);
        if (!condition.error && !condition.loading)
          context.service.resolve("projects-reads", condition.key);
      }
    });
  });
}

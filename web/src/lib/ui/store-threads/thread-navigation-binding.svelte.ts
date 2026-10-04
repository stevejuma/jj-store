import { untrack } from "svelte";
import type { StoreController } from "../../controller/contracts.js";

export function observeRepositoryThreadNavigation(
  controller: () => StoreController,
) {
  $effect(() => {
    const current = controller();
    if (!current.freshness || !current.threadNavigation) return;
    const affected = new Set<string>();
    const watch = current.freshness.watch({
      initial: false,
      matches: (change) => {
        if (change.domain === "threads" && change.repository) {
          affected.add(change.repository);
          return true;
        }
        return (
          change.repository === null &&
          ["repository", "discovery"].includes(change.domain)
        );
      },
      refresh: async () => {
        const batch = new Set(affected);
        await current.threadNavigation!.load(
          true,
          batch.size ? batch : undefined,
        );
        const failed = Object.entries(current.threadNavigation!.entries).some(
          ([id, entry]) => (!batch.size || batch.has(id)) && entry.error,
        );
        if (!failed) for (const id of batch) affected.delete(id);
        return !failed;
      },
    });
    return watch.stop;
  });
  const repositories = $derived(controller().state.repositories);
  const repository = $derived(controller().state.repository.id);
  $effect(() => {
    const current = controller(),
      navigation = current.threadNavigation;
    void repositories;
    untrack(() => void navigation?.load());
  });
  $effect(() => {
    const current = controller(),
      threads = current.threads;
    void threads?.threads;
    if (threads?.status === "ready" && repository)
      untrack(() => void current.threadNavigation?.read(repository));
  });
}

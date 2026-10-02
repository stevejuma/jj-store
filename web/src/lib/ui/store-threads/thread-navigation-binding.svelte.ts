import { untrack } from "svelte";
import type { StoreController } from "../../controller/contracts.js";

export function observeRepositoryThreadNavigation(
  controller: () => StoreController,
) {
  $effect(() => {
    const current = controller(),
      navigation = current.threadNavigation;
    void current.state.repositories;
    untrack(() => void navigation?.load());
  });
  $effect(() => {
    const current = controller(),
      threads = current.threads;
    const repository = current.state.repository.id;
    void threads?.threads;
    if (threads?.status === "ready" && repository)
      untrack(() => void current.threadNavigation?.read(repository));
  });
}

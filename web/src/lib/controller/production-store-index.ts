import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export function createProductionStoreIndex(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
  restore: () => Promise<void>;
}) {
  return async function prepareIndex() {
    const { data } = options;
    if (!data.neededIndex) return;
    const source = options.source();
    const ticket = options.version();
    let task = await source.prepareIndex(data.neededIndex.operation_id);
    while (
      options.current(ticket, source) &&
      ["running", "canceling"].includes(task.status)
    ) {
      await new Promise((resolve) => setTimeout(resolve, 500));
      task = await source.indexTask(task.id);
    }
    if (options.current(ticket, source) && task.status === "ready") {
      data.neededIndex = null;
      data.status.error = "";
      await options.restore();
    }
  };
}

import { createProductionStoreMutations } from "./production-store-mutations.js";
import { createProductionStoreConflicts } from "./production-store-conflicts.js";
import { createProductionMutationCommitter } from "./production-mutation-commit.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";

/** Command services share the selected repository and confirmed refresh boundary. */
export function createProductionCommandServices(options: {
  data: ProductionStoreState;
  source(): StoreDataSource;
  current(source: StoreDataSource): boolean;
  history: Parameters<typeof createProductionMutationCommitter>[0];
  refresh: Parameters<typeof createProductionMutationCommitter>[1];
}) {
  const context = {
    source: options.source,
    head: () => options.data.head,
    current: options.current,
  };
  return {
    mutations: createProductionStoreMutations({
      ...context,
      state: options.data.mutation,
      committed: createProductionMutationCommitter(
        options.history,
        options.refresh,
      ),
    }),
    conflictUploads: createProductionStoreConflicts({
      ...context,
      uploadLimit: () => options.data.conflictUploadBytes,
    }),
  };
}

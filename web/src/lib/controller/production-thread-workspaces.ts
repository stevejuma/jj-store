import { ThreadWorkspaces } from "./thread-workspaces.svelte.js";
import { LocalClient } from "../api/local-client.js";
import { repositories as loadRepositories } from "../api/repositories.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { RepositorySources } from "./repository-sources.js";
import type { StoreController } from "./contracts.js";
export function createProductionThreadWorkspaces(context: {
  data: ProductionStoreState;
  token(): string | null;
  threadNavigation: NonNullable<StoreController["threadNavigation"]>;
  sources: RepositorySources;
  openThreads(thread: string, home?: string): Promise<void>;
  fetcher?: typeof fetch;
}) {
  return new ThreadWorkspaces(
    new LocalClient("", context.fetcher),
    async (result) => {
      context.data.repositories = (
        await loadRepositories(context.token(), context.fetcher)
      ).map((r) => ({ ...r }));
      // Entries and the shared transport have separate caches. Publication
      // must invalidate the metadata home's reads before refreshing aliases.
      context.threadNavigation.invalidate(new Set([result.home]));
      await context.sources.refreshThreads(
        context.threadNavigation,
        result.home,
      );
      await context.openThreads(result.thread, result.home);
    },
  );
}

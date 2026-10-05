import { createProductionRepositoryThreadNavigation } from "./repository-thread-navigation.svelte.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { repositorySource } from "./repository-source.js";
import { storeQueryKeys } from "./store-query-keys.js";
import type { StoreRepositorySummary } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";

/** Authorization-session ownership; view closure releases a lease, never another view's source. */
export class RepositorySources {
  readonly queries = createStoreQueryClient();
  readonly session = crypto.randomUUID();
  private token: string | null | undefined;
  private entries = new Map<
    string,
    {
      source: StoreDataSource;
      leases: number;
      profile: string;
      idle?: ReturnType<typeof setTimeout>;
    }
  >();
  constructor(
    private readonly fetcher?: typeof fetch,
    private readonly denied: () => void = () => {},
  ) {}
  acquire(
    repository: StoreRepositorySummary,
    token: string | null,
  ): StoreDataSource {
    if (this.token !== token) {
      this.clear();
      this.token = token;
    }
    let entry = this.entries.get(repository.id);
    const profile = repository.profile ?? "native";
    if (entry && entry.profile !== profile) {
      this.remove(repository.id);
      entry = undefined;
    }
    if (!entry) {
      entry = {
        source: repositorySource(
          repository,
          token,
          this.session,
          this.denied,
          this.fetcher,
          this.queries,
        ),
        leases: 0,
        profile,
      };
      this.entries.set(repository.id, entry);
    }
    clearTimeout(entry.idle);
    entry.leases++;
    const retained = entry;
    let closed = false;
    return new Proxy(entry.source, {
      get: (source, property) => {
        if (property === "close")
          return () => {
            if (closed) return;
            closed = true;
            retained.leases--;
            if (!retained.leases) {
              retained.idle = setTimeout(() => {
                if (
                  this.entries.get(repository.id) === retained &&
                  !retained.leases
                )
                  this.remove(repository.id);
              }, 10 * 60_000);
              (retained.idle as unknown as { unref?: () => void }).unref?.();
              void this.queries.cancelQueries({
                queryKey: storeQueryKeys.root({
                  repository: repository.id,
                  profile: profile as "native",
                  session: this.session,
                }),
              });
            }
          };
        const value = Reflect.get(source, property, source);
        return typeof value === "function"
          ? (...args: unknown[]) => {
              if (closed) throw new DOMException("Closed", "AbortError");
              return value.apply(source, args);
            }
          : value;
      },
    });
  }
  invalidate(repository?: string, domains?: readonly string[]) {
    const root = ["jj-store", this.session];
    return this.queries.invalidateQueries({
      refetchType: "none",
      predicate: (query) => {
        const key = query.queryKey;
        if (
          !root.every((v, i) => key[i] === v) ||
          (repository && key[3] !== repository)
        )
          return false;
        const kind = String(key[4]);
        if (!domains || domains.includes("repository")) return key[5] === null;
        return domains.some((domain) =>
          domain === "code"
            ? kind === "snapshot"
            : domain === "threads"
              ? kind.startsWith("thread-") || kind.startsWith("reaction-")
              : kind.startsWith(
                  domain === "agents"
                    ? "agent-"
                    : domain === "reviews"
                      ? "review-"
                      : domain,
                ),
        );
      },
    });
  }
  async refreshThreads(
    navigation: { load(fresh?: boolean): Promise<void> },
    repository: string,
  ) {
    await this.invalidate(repository);
    await navigation.load(true);
  }
  remove(repository: string) {
    clearTimeout(this.entries.get(repository)?.idle);
    this.entries.get(repository)?.source.close();
    this.entries.delete(repository);
    this.queries.removeQueries({
      predicate: (q) =>
        q.queryKey[1] === this.session && q.queryKey[3] === repository,
    });
  }
  clear() {
    for (const entry of this.entries.values()) {
      clearTimeout(entry.idle);
      entry.source.close();
    }
    this.entries.clear();
    void this.queries.cancelQueries();
    this.queries.clear();
  }
}

export function repositoryReaders(
  repositories: () => readonly StoreRepositorySummary[],
  token: () => string | null,
  fetcher: typeof fetch | undefined,
  denied: () => void,
) {
  const sources = new RepositorySources(fetcher, denied);
  const threadNavigation = createProductionRepositoryThreadNavigation(
    repositories,
    token,
    fetcher,
    sources,
  );
  return { sources, threadNavigation };
}

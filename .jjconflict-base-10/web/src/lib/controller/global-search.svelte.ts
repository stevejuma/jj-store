import { Client, ApiError, type Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type {
  GlobalSearchResult,
  GlobalSearchScope,
} from "./global-search-types.js";
import { contentResults, emptyRoute } from "./global-search-presentation.js";
import {
  presentSearchResults,
  assertSearchPage,
  routeForSearchResult,
} from "./production-store-search.js";
export function createGlobalSearch(options: {
  repositories: () => readonly StoreRepositorySummary[];
  token: () => string | null;
  route: () => StoreRoute;
  open: (route: StoreRoute) => Promise<void>;
  fetcher?: typeof fetch;
}) {
  let results = $state<GlobalSearchResult[]>([]),
    status = $state("idle"),
    errors = $state<string[]>([]);
  let scope = $state<GlobalSearchScope>();
  let abort = new AbortController(),
    generation = 0;
  const continuations = new Map<
    string,
    { content?: string; code?: string; operation?: string }
  >();
  let query = "";
  function clear() {
    generation++;
    abort.abort();
    abort = new AbortController();
    results = [];
    errors = [];
    continuations.clear();
    status = "idle";
    scope = undefined;
  }
  async function run(
    text: string,
    selectedScope?: GlobalSearchScope,
    more = false,
  ) {
    abort.abort();
    abort = new AbortController();
    const signal = abort.signal;
    const ticket = ++generation;
    query = text.trim();
    scope = selectedScope;
    const repositories = options
      .repositories()
      .filter((r) => !scope?.repository || r.id === scope.repository);
    if (!more) {
      results = [];
      continuations.clear();
    }
    errors = [];
    status = more ? "refreshing" : "loading";
    const denied = new Set<string>();
    const incoming: GlobalSearchResult[] = [];
    let index = 0;
    async function read() {
      while (index < repositories.length && !signal.aborted) {
        const repository = repositories[index++]!;
        const sourceName = repository.name;
        if (repository.profile && repository.profile !== "native") {
          errors.push(
            `${sourceName}: Search is unavailable for local Git-backed repositories.`,
          );
          continuations.delete(repository.id);
          continue;
        }
        const client = new Client(
          repository.id,
          options.token(),
          () => {
            denied.add(repository.id);
            if (ticket === generation) {
              continuations.delete(repository.id);
              results = results.filter(
                (result) => result.route.repository !== repository.id,
              );
            }
          },
          options.fetcher,
        );
        const cursors = continuations.get(repository.id);
        const next: { content?: string; code?: string; operation?: string } =
          {};
        if (selectedScope?.section !== "code" && (!more || cursors?.content)) {
          try {
            const page = await client.request<Model<"ContentPage">>(
              "content/search",
              {
                q: query,
                section: selectedScope?.section ?? "all",
                thread: selectedScope?.thread,
                project: selectedScope?.project,
                task: selectedScope?.task,
                limit: 50,
                cursor: more ? cursors?.content : undefined,
              },
              { signal },
            );
            if (page.version !== 1 || page.snapshot.repo_id !== repository.id)
              throw new ApiError("snapshot_mismatch");
            incoming.push(...contentResults(page, sourceName));
            next.content = page.next_cursor ?? undefined;
          } catch (cause) {
            if (!signal.aborted)
              errors.push(
                `${sourceName}: ${cause instanceof Error ? cause.message : String(cause)}`,
              );
          }
        }
        if (
          query &&
          !denied.has(repository.id) &&
          (!selectedScope?.section ||
            selectedScope.section === "all" ||
            selectedScope.section === "code") &&
          (!more || cursors?.code)
        ) {
          try {
            const capabilities = await client.request<Model<"Capabilities">>(
              "capabilities",
              {},
              { signal },
            );
            if (capabilities.search) {
              const head = await client.request<Model<"HeadSnapshot">>(
                "snapshot",
                {},
                { signal },
              );
              const operation = more
                ? cursors?.operation
                : options.route().repository === repository.id
                  ? (options.route().snapshotOperationId ??
                    (head.heads.length === 1 ? head.heads[0] : undefined))
                  : head.heads.length === 1
                    ? head.heads[0]
                    : undefined;
              if (!operation)
                throw new ApiError(
                  "Choose a code operation for this repository to search its code.",
                );
              const page = await client.request<Model<"SearchPage">>(
                "search",
                {
                  q: query,
                  mode: "visible_commits",
                  at_op: operation,
                  page_size: 50,
                  cursor: more ? cursors?.code : undefined,
                },
                { signal },
              );
              assertSearchPage(page, repository.id, operation);
              incoming.push(
                ...presentSearchResults(page.results).map((result) => ({
                  id: `${repository.id}:${result.id}`,
                  kind: result.kind,
                  title: result.title,
                  description: result.description,
                  timestamp: result.timestamp,
                  source: sourceName,
                  route: routeForSearchResult(
                    {
                      ...emptyRoute(repository.id),
                      snapshotOperationId: operation,
                    },
                    result,
                  ),
                })),
              );
              next.code = page.next_cursor ?? undefined;
              next.operation = operation;
            }
          } catch (cause) {
            if (!signal.aborted)
              errors.push(
                `${sourceName} code: ${cause instanceof Error ? cause.message : String(cause)}`,
              );
          }
        }
        if (ticket === generation && !signal.aborted)
          continuations.set(
            repository.id,
            denied.has(repository.id) ? {} : next,
          );
        client.close();
      }
    }
    await Promise.all([read(), read()]);
    if (ticket !== generation || signal.aborted) return;
    if (query && !more && (!selectedScope || selectedScope.section === "all"))
      incoming.push(
        ...repositories
          .filter((r) =>
            r.name.toLocaleLowerCase().includes(query.toLocaleLowerCase()),
          )
          .map((r) => ({
            id: `repo:${r.id}`,
            kind: "repository" as const,
            title: r.name,
            description: "Repository",
            source: r.name,
            timestamp: 0,
            route: { ...emptyRoute(r.id), page: "log" as const },
          })),
      );
    const authorized = new Set(
      options.repositories().map((source) => source.id),
    );
    for (const id of continuations.keys())
      if (denied.has(id) || !authorized.has(id)) continuations.delete(id);
    results = [
      ...new Map(
        [...results, ...incoming]
          .filter(
            (r) =>
              !denied.has(r.route.repository) &&
              authorized.has(r.route.repository),
          )
          .map((r) => [r.id, r]),
      ).values(),
    ].sort((a, b) => b.timestamp - a.timestamp || a.id.localeCompare(b.id));
    status = results.length ? "ready" : errors.length ? "failed" : "empty";
  }
  return {
    get results() {
      return results;
    },
    get status() {
      return status;
    },
    get errors() {
      return errors;
    },
    get canLoadMore() {
      return [...continuations.values()].some((c) => c.content || c.code);
    },
    run,
    clear,
    more: () => run(query, scope, true),
    select: (result: GlobalSearchResult) => options.open(result.route),
  };
}

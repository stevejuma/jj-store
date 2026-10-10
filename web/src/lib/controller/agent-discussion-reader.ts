import { Client } from "../api/client.js";
import { HttpThreadsDataSource } from "./http-threads-data-source.js";
import { LocalInboxThreads } from "./local-inbox-threads.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createThreadsController } from "./production-threads.svelte.js";
import { initialStoreRoute } from "./store-route-history.svelte.js";
export function createAgentDiscussionReader(
  repository: string,
  profile = "native",
  fetcher?: typeof fetch,
  principal: (repository: string) => string = () => "",
) {
  const client = new Client(repository, null, () => {}, fetcher);
  const queries = createStoreQueryClient();
  let active = true;
  let route = initialStoreRoute();
  const local =
    profile === "local_jj_git"
      ? new LocalInboxThreads(repository, fetcher)
      : undefined;
  const source =
    local ??
    new HttpThreadsDataSource(
      client,
      queries,
      { repository, profile: "native", session: crypto.randomUUID() },
      () => active,
    );
  const threads = createThreadsController({
    source: () => source,
    repository: () => repository,
    principal: () => local?.actor ?? principal(repository),
    route: () => route,
    select: (id) => {
      route = { ...route, threadId: id };
    },
  });
  return {
    threads,
    profile,
    principal: () => local?.actor ?? principal(repository),
    prepare: () => local?.prepare() ?? Promise.resolve(),
    async invalidate() {
      await source.invalidate("reactions");
      threads.invalidateReactions();
    },
    close() {
      local?.close();
      active = false;
      threads.clear();
      client.close();
      void queries.cancelQueries();
      queries.clear();
    },
  };
}

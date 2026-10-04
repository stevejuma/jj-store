import type { createHttpQuery } from "./http-query.js";
import { Client, fullId, pretty, type Model } from "../api/client.js";
import type { WorkQuery, WorkRequest, WorkTransport } from "./work-types.js";
import { readRepositoryDirectory } from "./http-repository-directory.js";
import { readNativeActivityPage } from "./native-activity-reader.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { NativeActivityQuery } from "./native-activity-types.js";
export class HttpWorkDataSource implements WorkTransport {
  constructor(
    private client: Client,
    private native?: StoreDataSource,
    private cached?: ReturnType<typeof createHttpQuery>,
    private changed?: () => Promise<void>,
  ) {}
  agents = async (cursor?: string, signal?: AbortSignal) => {
    signal?.throwIfAborted();
    if (!this.native) throw new Error("Agent sessions unavailable");
    const page = await this.native.agentSessions(cursor);
    signal?.throwIfAborted();
    return page;
  };
  get reactions() {
    return this.native?.threads;
  }
  activity = async (query: NativeActivityQuery, signal?: AbortSignal) => {
    signal?.throwIfAborted();
    if (!this.native) throw new Error("Repository activity unavailable");
    const page = await readNativeActivityPage(this.native, query);
    signal?.throwIfAborted();
    return page;
  };
  directory = (signal?: AbortSignal) =>
    this.cached
      ? this.cached(
          "work-directory",
          null,
          {},
          (shared) => readRepositoryDirectory(this.client, this, shared),
          signal,
        )
      : readRepositoryDirectory(this.client, this, signal);
  page(query: WorkQuery, signal?: AbortSignal) {
    const args = {
      ...query,
      project: query.project ? fullId(query.project, 32) : undefined,
      task: query.task ? fullId(query.task, 32) : undefined,
    };
    if (this.cached)
      return this.cached<Model<"WorkPage">>(
        "work-page",
        null,
        args,
        (shared) => this.client.request("work", args, { signal: shared }),
        signal,
      );
    return this.client.request<Model<"WorkPage">>(
      "work",
      {
        ...query,
        project: query.project ? fullId(query.project, 32) : undefined,
        task: query.task ? fullId(query.task, 32) : undefined,
      },
      { signal },
    );
  }
  browse(
    query: import("./projects-hub-types.js").WorkBrowseQuery,
    signal?: AbortSignal,
  ) {
    if (this.cached)
      return this.cached<Model<"WorkBrowsePage">>(
        "work-browse",
        null,
        { ...query },
        (shared) =>
          this.client.request("work/browse", { ...query }, { signal: shared }),
        signal,
      );
    return this.client.request<Model<"WorkBrowsePage">>(
      "work/browse",
      { ...query },
      { signal },
    );
  }
  reviews(cursor?: string, signal?: AbortSignal) {
    if (this.cached)
      return this.cached<Model<"ReviewThreadPage">>(
        "review-threads",
        null,
        { cursor, limit: 50 },
        (shared) =>
          this.client.request(
            "reviews/threads",
            { cursor, limit: 50 },
            { signal: shared },
          ),
        signal,
      );
    return this.client.request<Model<"ReviewThreadPage">>(
      "reviews/threads",
      { cursor, limit: 50 },
      { signal },
    );
  }
  async apply(request: WorkRequest) {
    const result = await this.client.request<Model<"WorkCommandOutcome">>(
      "work/commands",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
    if (result.status === "committed") await this.changed?.();
    return result;
  }
  status(id: string) {
    return this.client.request<Model<"WorkCommandStatus">>(
      `work/commands/${fullId(id, 32)}`,
    );
  }
}

export function cachedWorkSource(
  client: Client,
  native: StoreDataSource,
  cached: ReturnType<typeof createHttpQuery>,
  queries: import("@tanstack/svelte-query").QueryClient,
  scope: import("./store-query-keys.js").StoreQueryScope,
) {
  return new HttpWorkDataSource(client, native, cached, async () => {
    const root = [
      "jj-store",
      scope.session,
      scope.profile ?? "native",
      scope.repository,
    ];
    await queries.invalidateQueries({
      refetchType: "none",
      predicate: (q) =>
        root.every((v, i) => q.queryKey[i] === v) &&
        String(q.queryKey[4]).startsWith("work-"),
    });
  });
}

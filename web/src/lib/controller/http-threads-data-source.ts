import { readQuery } from "./read-query.js";
import type { QueryClient } from "@tanstack/svelte-query";
import { storeQueryKeys, type StoreQueryScope } from "./store-query-keys.js";
import { targetKey } from "../collaboration/thread-presentation.js";
import { Client, fullId, pretty, type Model } from "../api/client.js";
import type {
  ThreadTransport,
  ThreadRequest,
  ReactionRequest,
  ReactionTarget,
} from "./thread-types.js";
export class HttpThreadsDataSource implements ThreadTransport {
  constructor(
    private client: Client,
    private queryClient: QueryClient,
    private scope: StoreQueryScope,
    private active: () => boolean,
  ) {}
  private query<T>(
    kind: string,
    args: Record<string, string | number | undefined>,
    read: (signal: AbortSignal) => Promise<T>,
  ) {
    if (!this.active())
      return Promise.reject<T>(new DOMException("Closed", "AbortError"));
    return readQuery(this.queryClient, {
      staleTime: args.cursor ? 30_000 : Infinity,
      queryKey: storeQueryKeys.resource(this.scope, kind, null, args),
      queryFn: async ({ signal }) => {
        const value = await read(signal);
        if (!this.active()) throw new DOMException("Closed", "AbortError");
        return value;
      },
    });
  }
  private matches(
    key: readonly unknown[],
    scope: "all" | "threads" | "reactions" | "emojis",
    target?: ReactionTarget,
  ) {
    const root = storeQueryKeys.root(this.scope);
    if (!root.every((value, i) => key[i] === value)) return false;
    const kind = String(key[root.length]);
    if (scope === "threads") return kind.startsWith("thread-");
    if (scope === "reactions")
      return (
        kind === "reaction-target" &&
        (!target ||
          (key[root.length + 2] as { target?: string } | undefined)?.target ===
            targetKey(target))
      );
    if (scope === "emojis") return kind === "reaction-emojis";
    return kind.startsWith("thread-") || kind.startsWith("reaction-");
  }
  async invalidate(
    scope: "all" | "threads" | "reactions" | "emojis",
    target?: ReactionTarget,
  ) {
    const predicate = (query: { queryKey: readonly unknown[] }) =>
      this.matches(query.queryKey, scope, target);
    await this.queryClient.cancelQueries({ predicate });
    await this.queryClient.invalidateQueries({
      predicate,
      refetchType: "none",
    });
  }
  async cancelReads() {
    const root = storeQueryKeys.root(this.scope);
    await this.queryClient.cancelQueries({
      predicate: (query) =>
        root.every((v, i) => query.queryKey[i] === v) &&
        ["thread-detail", "thread-events", "reaction-target"].includes(
          String(query.queryKey[root.length]),
        ),
    });
  }
  list(cursor?: string) {
    const args = { limit: 50, cursor };
    return this.query<Model<"CollabPage">>("thread-list", args, (signal) =>
      this.client.request("threads", args, { signal }),
    );
  }
  thread(id: string) {
    return this.query<Model<"ThreadDetailV1">>(
      "thread-detail",
      { id },
      (signal) =>
        this.client.request(`threads/${fullId(id, 32)}`, {}, { signal }),
    );
  }
  events(id: string, cursor?: string) {
    const args = { limit: 50, cursor };
    return this.query<Model<"CollabPage">>(
      "thread-events",
      { id, ...args },
      (signal) =>
        this.client.request(`threads/${fullId(id, 32)}/timeline`, args, {
          signal,
        }),
    );
  }
  emojis(cursor?: string) {
    const args = { limit: 50, cursor };
    return this.query<Model<"CollabPage">>("reaction-emojis", args, (signal) =>
      this.client.request("reaction-emojis", args, { signal }),
    );
  }
  reactions(target: ReactionTarget, cursor?: string) {
    return this.query<Model<"CollabPage">>(
      "reaction-target",
      { target: targetKey(target), cursor, limit: 50 },
      (signal) =>
        this.client.request(
          "reactions/query",
          {},
          {
            signal,
            method: "POST",
            headers: { "content-type": "application/json" },
            body: pretty({ target, limit: 50, ...(cursor ? { cursor } : {}) }),
          },
        ),
    );
  }
  asset(id: string) {
    return this.client.asset(id);
  }
  command(
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ) {
    return this.client.request<Model<"CollabCommandOutcome">>(
      `${kind}/commands`,
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
  }
  status(kind: "threads" | "reactions", id: string) {
    return this.client.request<Model<"CollabCommandStatus">>(
      `${kind}/commands/${fullId(id, 32)}`,
    );
  }
}

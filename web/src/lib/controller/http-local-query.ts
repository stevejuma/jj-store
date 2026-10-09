import { createHttpBlame } from "./http-blame.js";
import { readQuery } from "./read-query.js";
import {
  ApiError,
  decode,
  fullId,
  pretty,
  type Exact,
  type Model,
} from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import { LocalInboxThreads } from "./local-inbox-threads.js";
import { createStoreQueryClient } from "./store-query-client.js";
import {
  validateLocalHead,
  type LocalHead,
  type LocalReadSnapshot,
} from "./repository-profile.js";

import type { operations } from "../api/generated.js";
export type LocalCodeRequest = Exact<
  operations["local_code_query"]["requestBody"]["content"]["application/json"]
>["request"];
type Response = Model<"LocalCodeResponse">;
type DataByKind = {
  [K in Response["kind"]]: Extract<Response, { kind: K }>["data"];
};
export class HttpLocalQuery {
  readonly profile = "git_sha1_jj045" as const;
  protected readonly client: LocalClient;
  readonly threads: LocalInboxThreads;
  readonly #reads: import("@tanstack/svelte-query").QueryClient;
  protected head: LocalHead | null = null;
  #capture = 0;
  #accepted = 0;
  #pending: Promise<LocalHead> | null = null;
  constructor(
    readonly repository: string,
    fetcher?: typeof fetch,
    reads?: import("@tanstack/svelte-query").QueryClient,
    private readonly session: string = crypto.randomUUID(),
  ) {
    this.client = new LocalClient(repository, fetcher);
    this.threads = new LocalInboxThreads(this.client);
    this.#reads = reads ?? createStoreQueryClient();
  }
  #blameScope = "";
  #blame: ReturnType<typeof createHttpBlame> | undefined;
  get blame() {
    const generation = this.head?.generation;
    const scope = JSON.stringify([
      this.session,
      this.profile,
      this.repository,
      generation,
    ]);
    if (!this.#blame || scope !== this.#blameScope) {
      this.#blameScope = scope;
      this.#blame = createHttpBlame({
        client: this.#reads,
        scope,
        active: () =>
          !this.client.closed.signal.aborted &&
          this.head?.generation === generation,
        request: (selection, continuation, signal) => {
          const query = new URLSearchParams({
            at_op: fullId(selection.operation),
            path: selection.path,
          });
          if (continuation) query.set("continuation", continuation);
          return this.client.request(
            `commits/${fullId(selection.commit, 40)}/blame?${query}`,
            undefined,
            signal,
          );
        },
      });
    }
    return this.#blame;
  }
  protected read<T>(
    path: string,
    body?: unknown,
    signal?: AbortSignal,
    _method?: "POST",
    staleTime?: number,
  ) {
    this.client.closed.signal.throwIfAborted();
    if (path === "snapshot") return this.client.request<T>(path, body, signal);
    const encoded = pretty(body);
    const retained = body === undefined ? undefined : decode(encoded);
    return readQuery<T>(
      this.#reads,
      {
        queryKey: [
          "jj-store",
          this.session,
          this.profile,
          this.repository,
          path,
          this.head
            ? `${this.head.generation}:${this.head.heads.join(",")}:${this.head.metadata_commit ?? ""}`
            : null,
          encoded,
        ],
        queryFn: ({ signal: shared }) =>
          this.client.request<T>(path, retained, shared),
        staleTime:
          staleTime ??
          (path.startsWith("work/") && body && path !== "work/query-info"
            ? Infinity
            : 0),
        gcTime: 10 * 60_000,
        retry: (failures, error) =>
          error instanceof ApiError && error.code === "busy" && failures < 2,
      },
      signal,
    );
  }
  async snapshot(fresh = false): Promise<LocalHead> {
    if (this.head && !fresh) return this.head;
    if (this.#pending && !fresh) return this.#pending;
    const ticket = ++this.#capture;
    const pending: Promise<LocalHead> = this.read<
      Model<"LocalRepositorySnapshot">
    >("snapshot").then((result) => {
      const value = validateLocalHead(result, this.repository);
      if (ticket !== this.#capture) {
        if (fresh && !this.client.closed.signal.aborted) {
          if (this.#pending && this.#pending !== pending) return this.#pending;
          if (this.#accepted === this.#capture && this.head) return this.head;
        }
        throw new DOMException("Snapshot superseded", "AbortError");
      }
      this.#accepted = ticket;
      this.head = value;
      return value;
    });
    this.#pending = pending;
    try {
      return await pending;
    } finally {
      if (this.#pending === pending) this.#pending = null;
    }
  }
  protected async query<K extends Response["kind"]>(
    operation: string,
    request: LocalCodeRequest & { kind: K },
    cursor?: string,
    signal?: AbortSignal,
  ): Promise<{
    snapshot: LocalReadSnapshot;
    data: DataByKind[K];
    next_cursor: string | null;
    previous_cursor: string | null;
  }> {
    const selection: LocalCodeRequest = request;
    const sourceOperation =
      selection.kind === "references" ? selection.query.operation : operation;
    const head = await this.snapshot();
    let response: Model<"LocalCodeQueryResponse">;
    try {
      response = await this.read<Model<"LocalCodeQueryResponse">>(
        "query",
        {
          operation: fullId(operation),
          request,
          cursor: cursor ?? null,
        },
        signal,
        undefined,
        // Names are immutable at this operation/commit. Keep expiring cursors
        // short lived; quick/history attribution retains its own progress policy.
        selection.kind === "paths" && !selection.query.include_last_change
          ? 30_000
          : 0,
      );
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        cause.code === "index_rebuild_required"
      ) {
        const pin = cause.localSnapshot;
        if (
          !pin ||
          pin.version !== 1 ||
          pin.profile !== this.profile ||
          pin.repository !== this.repository ||
          pin.operation !== sourceOperation ||
          String(pin.generation) !== String(head.generation)
        )
          throw new ApiError("snapshot_mismatch");
        throw new ApiError(cause.code, {
          ...pin,
          profile: this.profile,
          heads: [...head.heads],
        });
      }
      throw cause;
    }
    if (
      response.snapshot.profile !== this.profile ||
      response.snapshot.repository !== this.repository ||
      response.snapshot.operation !== operation ||
      String(response.snapshot.generation) !== String(head.generation) ||
      response.data.kind !== request.kind ||
      (request.kind === "references" &&
        response.data.kind === "references" &&
        response.data.data.operation !== sourceOperation)
    )
      throw new ApiError("snapshot_mismatch");
    const snapshot: LocalReadSnapshot = {
      ...response.snapshot,
      profile: this.profile,
      heads: [...head.heads],
    };
    const data: DataByKind[K] = response.data.data as DataByKind[K];
    return {
      snapshot,
      data,
      next_cursor: response.next_cursor,
      previous_cursor: response.previous_cursor,
    };
  }
  close() {
    this.threads.close();
    this.#capture++;
    this.client.close();

    this.head = null;
  }
}

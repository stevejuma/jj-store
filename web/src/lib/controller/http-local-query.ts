import {
  ApiError,
  decode,
  fullId,
  pretty,
  type Exact,
  type Model,
} from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
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
  readonly #reads = createStoreQueryClient();
  protected head: LocalHead | null = null;
  #capture = 0;
  #accepted = 0;
  #pending: Promise<LocalHead> | null = null;
  constructor(
    readonly repository: string,
    fetcher?: typeof fetch,
  ) {
    this.client = new LocalClient(repository, fetcher);
  }
  protected read<T>(path: string, body?: unknown, signal?: AbortSignal) {
    if (signal) return this.client.request<T>(path, body, signal);
    const encoded = pretty(body);
    const retained = body === undefined ? undefined : decode(encoded);
    return this.#reads.fetchQuery<T>({
      queryKey: ["local-read", this.repository, this.#capture, path, encoded],
      queryFn: () => this.client.request<T>(path, retained, signal),
      staleTime: 0,
      gcTime: 0,
      retry: (failures, error) =>
        error instanceof ApiError && error.code === "busy" && failures < 2,
    });
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
    const head = await this.snapshot();
    const response = await this.read<Model<"LocalCodeQueryResponse">>(
      "query",
      {
        operation: fullId(operation),
        request,
        cursor: cursor ?? null,
      },
      signal,
    );
    if (
      response.snapshot.profile !== this.profile ||
      response.snapshot.repository !== this.repository ||
      response.snapshot.operation !== operation ||
      String(response.snapshot.generation) !== String(head.generation) ||
      response.data.kind !== request.kind
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
    this.#capture++;
    this.client.close();
    this.#reads.clear();
    this.head = null;
  }
}

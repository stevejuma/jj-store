import { LocalClient } from "../api/local-client.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { readQuery } from "./read-query.js";
import { ApiError, fullId, type Exact, type Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
type InboxQuery = NonNullable<
  operations["local_collab_query"]["requestBody"]
>["content"]["application/json"]["query"];
import type {
  ThreadTransport,
  ThreadRequest,
  ReactionRequest,
  ReactionTarget,
  ThreadPage,
  ThreadDetail,
} from "./thread-types.js";
/** Current companion pins and the device actor remain local, without native snapshot fields. */
export class LocalInboxThreads implements ThreadTransport {
  readonly client: LocalClient;
  actor = "";
  #selection?: Promise<Model<"LocalInboxSelection">>;
  #epoch = 0;
  readonly #queries = createStoreQueryClient();
  #reads = new AbortController();
  constructor(repository: string | LocalClient, fetcher?: typeof fetch) {
    this.client =
      typeof repository === "string"
        ? new LocalClient(repository, fetcher)
        : repository;
  }
  async prepare() {
    await this.selection();
  }
  private selection() {
    return (this.#selection ??= this.client
      .request<Model<"LocalInboxSelection">>("collaboration/snapshot")
      .then((value) => {
        if (
          value.snapshot.profile !== "local_jj_git" ||
          value.snapshot.repo_id !== this.client.repository
        )
          throw new ApiError("local_identity_changed");
        this.actor = value.actor ?? "";
        return value;
      })
      .catch((cause) => {
        this.#selection = undefined;
        throw cause;
      }));
  }
  async invalidate(
    _scope?: "all" | "threads" | "reactions" | "emojis",
    _target?: ReactionTarget,
  ) {
    this.#epoch++;
    this.#selection = undefined;
    this.#queries.clear();
  }
  async cancelReads() {
    this.#reads.abort();
    this.#reads = new AbortController();
  }
  close() {
    this.#epoch++;
    this.#reads.abort();
    this.#queries.clear();
    this.client.close();
  }
  private cached<T>(
    path: string,
    body: Record<string, unknown>,
    signal: AbortSignal,
  ) {
    return readQuery(
      this.#queries,
      {
        queryKey: [this.client.repository, path, body],
        staleTime: body.cursor ? 30_000 : Infinity,
        queryFn: ({ signal }) => this.client.request<T>(path, body, signal),
      },
      signal,
    );
  }
  private async query(
    query: Exact<InboxQuery>,
    cursor?: string,
  ): Promise<ThreadPage> {
    const epoch = this.#epoch;
    const signal = this.#reads.signal;
    const pin = await this.selection();
    const response = await this.cached<Model<"LocalInboxPageResponse">>(
      "collaboration/query",
      {
        metadata_commit: pin.snapshot.metadata_commit,
        query,
        cursor: cursor ?? null,
      },
      signal,
    );
    if (epoch !== this.#epoch)
      throw new DOMException("Superseded", "AbortError");
    if (
      response.data.snapshot.repo_id !== this.client.repository ||
      response.data.snapshot.metadata_commit !== pin.snapshot.metadata_commit
    )
      throw new ApiError("local_identity_changed");
    return {
      snapshot: response.data.snapshot,
      items: response.data.items,
      next_cursor: response.next_cursor,
    };
  }
  list(cursor?: string) {
    return this.query({ type: "threads", limit: 50 }, cursor);
  }
  events(id: string, cursor?: string) {
    return this.query(
      { type: "events", thread_id: fullId(id, 32), limit: 50 },
      cursor,
    );
  }
  emojis(cursor?: string) {
    return this.query({ type: "emojis", limit: 50 }, cursor);
  }
  reactions(target: ReactionTarget, cursor?: string) {
    return this.query({ type: "reactions", target, limit: 50 }, cursor);
  }
  async thread(id: string): Promise<ThreadDetail> {
    const epoch = this.#epoch;
    const signal = this.#reads.signal;
    const pin = await this.selection();
    const value = await this.cached<Model<"LocalInboxThreadDetail">>(
      "collaboration/thread",
      {
        metadata_commit: pin.snapshot.metadata_commit,
        thread_id: fullId(id, 32),
      },
      signal,
    );
    if (epoch !== this.#epoch)
      throw new DOMException("Superseded", "AbortError");
    if (
      value.snapshot.repo_id !== this.client.repository ||
      value.snapshot.metadata_commit !== pin.snapshot.metadata_commit
    )
      throw new ApiError("local_identity_changed");
    return value;
  }
  async asset(id: string) {
    const value = await this.client.request<Model<"ImageAssetV1">>(
      `collaboration/assets/${fullId(id, 64)}`,
    );
    const types = {
      png: "image/png",
      jpeg: "image/jpeg",
      gif: "image/gif",
      webp: "image/webp",
    };
    if (
      !Array.isArray(value.bytes) ||
      value.bytes.length > 2097152 ||
      !types[value.media_type]
    )
      throw new ApiError("invalid_reaction_asset");
    const bytes = value.bytes.map(Number);
    if (
      !bytes.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255)
    )
      throw new ApiError("invalid_reaction_asset");
    return {
      bytes: new Uint8Array(bytes),
      type: types[value.media_type],
    };
  }
  command(
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ) {
    return this.client.request<Model<"LocalCollabOutcome">>(
      "collaboration/commands",
      {
        version: request.version,
        request_id: request.request_id,
        command:
          kind === "threads"
            ? { domain: "thread", intent: request.command }
            : { domain: "reaction", intent: request.command },
      },
    );
  }
  status(_kind: "threads" | "reactions", id: string) {
    return this.client.request<Model<"LocalCollabOutcome">>(
      `collaboration/commands/${fullId(id, 32)}`,
    );
  }
}

import type { QueryClient } from "@tanstack/svelte-query";
import {
  ApiError,
  Client,
  fullId,
  pretty,
  type Envelope,
  type Graph,
  type Head,
  type Model,
  type Task,
} from "../api/client.js";
import type {
  GraphQuery,
  PathQuery,
  SearchQuery,
  StoreDataSource,
} from "./native-data-source.js";
import type { StoreMutationRequest } from "./store-mutation-types.js";
import { storeQueryKeys, type StoreQueryScope } from "./store-query-keys.js";

export class HttpStoreDataSource implements StoreDataSource {
  readonly repository: string;
  readonly #client: Client;
  readonly #scope: StoreQueryScope;
  #active = true;

  constructor(
    repository: string,
    token: string,
    readonly queryClient: QueryClient,
    session: string,
    denied: () => void,
    fetcher?: typeof fetch,
  ) {
    this.#client = new Client(repository, token, denied, fetcher);
    this.repository = this.#client.repository;
    this.#scope = { repository, session };
  }

  #query<T>(
    kind: string,
    snapshot: string | null,
    query: Record<string, string | number | bigint | boolean | undefined>,
    read: (signal: AbortSignal) => Promise<T>,
  ): Promise<T> {
    return this.queryClient.fetchQuery({
      queryKey: storeQueryKeys.resource(this.#scope, kind, snapshot, query),
      queryFn: async ({ signal }) => {
        const value = await read(signal);
        if (!this.#active) throw new DOMException("Closed", "AbortError");
        return value;
      },
    });
  }

  capabilities() {
    return this.#query<Model<"Capabilities">>(
      "capabilities",
      null,
      {},
      (signal) => this.#client.request("capabilities", {}, { signal }),
    );
  }
  async snapshot(fresh = false) {
    const key = storeQueryKeys.resource(this.#scope, "snapshot", null);
    if (fresh)
      await this.queryClient.invalidateQueries({ queryKey: key, exact: true });
    return this.#query<Head>("snapshot", null, {}, (signal) =>
      this.#client.request("snapshot", {}, { signal }),
    );
  }
  references(operation: string, cursor?: string) {
    const query = { at_op: fullId(operation), cursor, limit: 50 };
    return this.#query<Model<"Window">>(
      "references",
      operation,
      query,
      (signal) => this.#client.request("references", query, { signal }),
    );
  }
  graph(kind: "log" | "operations" | "evolution", input: GraphQuery) {
    const query = {
      at_op: fullId(input.operation),
      cursor: input.cursor,
      limit: input.limit ?? 50,
      ...(kind === "log"
        ? {
            revset: input.revset ?? "all()",
            workspace: input.workspace,
            user_email: input.userEmail ?? "",
            at_time_millis: input.atTimeMillis ?? Date.now(),
          }
        : {}),
      ...(kind === "evolution" ? { commit: fullId(input.commit ?? "") } : {}),
    };
    return this.#query<Envelope<Graph>>(
      kind,
      input.operation,
      query,
      (signal) => this.#client.request(kind, query, { signal }),
    );
  }
  commit(id: string, operation: string) {
    const query = { at_op: fullId(operation) };
    return this.#query<Envelope<Model<"CommitDetail">>>(
      "commit",
      operation,
      { id: fullId(id), ...query },
      (signal) => this.#client.request(`commits/${id}`, query, { signal }),
    );
  }
  paths(commit: string, operation: string, input: PathQuery = {}) {
    const query = {
      at_op: fullId(operation),
      path: input.path ?? "",
      recursive: input.recursive ?? false,
      limit: input.limit ?? 50,
      cursor: input.cursor,
    };
    return this.#query<Model<"Window3">>(
      "paths",
      operation,
      { commit, ...query },
      (signal) =>
        this.#client.request(`commits/${fullId(commit)}/paths`, query, {
          signal,
        }),
    );
  }
  file(id: string, operation: string) {
    return this.#query("file", operation, { id }, (signal) =>
      this.#client.file(fullId(id), fullId(operation), signal),
    );
  }
  operation(id: string) {
    return this.#query<Envelope<Model<"OperationDetail">>>(
      "operation",
      id,
      { id },
      (signal) =>
        this.#client.request(`operations/${fullId(id)}`, {}, { signal }),
    );
  }
  operationDiff(id: string, parent?: string) {
    return this.#query<Envelope<Model<"OperationDiffResponse">>>(
      "operation-diff",
      id,
      { id, parent },
      (signal) =>
        this.#client.request(
          `operations/${fullId(id)}/diff`,
          { parent },
          { signal },
        ),
    );
  }
  operationChanges(id: string, parent: string, cursor?: string) {
    const query = { parent: fullId(parent), cursor, limit: 50 };
    return this.#query<Model<"Window2">>(
      "operation-changes",
      id,
      query,
      (signal) =>
        this.#client.request(`operations/${fullId(id)}/changes`, query, {
          signal,
        }),
    );
  }
  diffPaths(operation: string, from: string, to: string, cursor?: string) {
    const query = { at_op: operation, from, to, cursor, limit: 50 };
    return this.#query<Model<"Window4">>(
      "diff-paths",
      operation,
      query,
      (signal) => this.#client.request("diff/paths", query, { signal }),
    );
  }
  diff(operation: string, from: string, to: string, path: string) {
    const query = { at_op: operation, from, to, path, content: true };
    return this.#query<Envelope<Model<"FileDiffResponse">>>(
      "diff",
      operation,
      query,
      (signal) => this.#client.request("diff", query, { signal }),
    );
  }
  conflicts(operation: string, commit: string) {
    return this.#query<Envelope<Model<"ConflictResponse">>>(
      "conflicts",
      operation,
      { commit },
      (signal) =>
        this.#client.request(
          `commits/${fullId(commit)}/conflicts`,
          { at_op: operation },
          { signal },
        ),
    );
  }
  uploadConflict(
    head: Head,
    commit: string,
    path: string,
    fingerprint: string,
    bytes: Uint8Array,
  ) {
    return this.#client.request<Model<"ConflictUpload">>(
      "conflict-uploads",
      {
        epoch: head.epoch,
        position: head.published_seq,
        operation: fullId(head.heads[0]!),
        commit: fullId(commit),
        path,
        fingerprint: fullId(fingerprint),
      },
      {
        method: "POST",
        headers: { "content-type": "application/octet-stream" },
        body: Uint8Array.from(bytes).buffer,
      },
    );
  }
  search(input: SearchQuery) {
    const query = {
      q: input.query,
      mode: "visible_commits",
      at_op: fullId(input.operation),
      page_size: input.limit ?? 50,
      cursor: input.cursor,
    };
    return this.#query<Model<"SearchPage">>(
      "search",
      input.operation,
      query,
      (signal) => this.#client.request("search", query, { signal }),
    );
  }
  previewMutation(request: StoreMutationRequest) {
    return this.#client.request<Model<"Preview">>(
      "mutations/preview",
      {},
      mutationInit(request),
    );
  }
  applyMutation(request: StoreMutationRequest) {
    return this.#client.request<Model<"Outcome">>(
      "mutations",
      {},
      mutationInit(request),
    );
  }
  mutationStatus(requestId: string) {
    return this.#client.request<Model<"Status">>(
      `mutations/${fullId(requestId, 32)}`,
    );
  }
  prepareIndex(operation: string) {
    return this.#client.request<Task>(
      "index/tasks",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ operation }),
      },
    );
  }
  indexTask(id: string) {
    return this.#client.request<Task>(`index/tasks/${encodeURIComponent(id)}`);
  }
  events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: Parameters<Client["events"]>[2],
  ) {
    return this.#client.events(after, signal, deliver);
  }
  close() {
    this.#active = false;
    this.#client.close();
    void this.queryClient.cancelQueries();
    this.queryClient.clear();
  }
}

function mutationInit(request: StoreMutationRequest): RequestInit {
  return {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: pretty(request),
  };
}

export function storeErrorMessage(cause: unknown): string {
  if (cause instanceof ApiError) return cause.message;
  if (cause instanceof DOMException && cause.name === "AbortError") return "";
  return "The repository request could not be completed.";
}

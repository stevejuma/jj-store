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
  codeId,
  validateLocalHead,
  type LocalHead,
  type LocalReadSnapshot,
} from "./repository-profile.js";
import type { GraphQuery, PathQuery } from "./native-data-source.js";

import type { operations } from "../api/generated.js";
type Request = Exact<
  operations["local_code_query"]["requestBody"]["content"]["application/json"]
>["request"];
type Response = Model<"LocalCodeResponse">;
type DataByKind = {
  [K in Response["kind"]]: Extract<Response, { kind: K }>["data"];
};
export class HttpLocalCodeDataSource {
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
    request: Request & { kind: K },
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
  references(operation: string, cursor?: string) {
    return this.query<"references">(
      operation,
      {
        kind: "references",
        query: {
          operation,
          query: "",
          kind: "all",
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
    );
  }
  async queryInfo(operation: string) {
    return (await this.query<"query_info">(operation, { kind: "query_info" }))
      .data;
  }
  graph(kind: "log" | "operations" | "evolution", input: GraphQuery) {
    const graph: Extract<Request, { kind: "graph" }>["query"][0] =
      kind === "log"
        ? {
            graph: "log",
            revset: input.revset ?? "all()",
            workspace: input.workspace ?? null,
            user_email: input.userEmail ?? "",
            at_time_millis: input.atTimeMillis ?? Date.now(),
          }
        : kind === "operations"
          ? input.operationQuery
            ? { graph: "operations_filtered", query: input.operationQuery }
            : { graph: "operations" }
          : {
              graph: "evolution",
              commit: codeId(input.commit ?? "", this.profile),
            };
    return this.query<"graph">(
      input.operation,
      {
        kind: "graph",
        query: [graph, { limit: input.limit ?? 50, after: null }],
      },
      input.cursor,
      input.signal,
    );
  }
  commit(id: string, operation: string) {
    return this.query<"commit">(operation, {
      kind: "commit",
      query: { commit: codeId(id, this.profile) },
    });
  }
  paths(commit: string, operation: string, input: PathQuery = {}) {
    return this.query<"paths">(
      operation,
      {
        kind: "paths",
        query: {
          commit: codeId(commit, this.profile),
          path: input.path ?? "",
          recursive: input.recursive ?? false,
          limit: input.limit ?? 50,
          anchor: null,
          direction: "forward",
        },
      },
      input.cursor,
    );
  }
  async file(id: string, operation: string) {
    const result = await this.query<"file">(operation, {
      kind: "file",
      query: { file: codeId(id, this.profile) },
    });
    return {
      ...result.data,
      text: result.data.text ?? undefined,
      bytes: Number(result.data.bytes),
    };
  }
  operation(id: string) {
    return this.query<"operation">(id, {
      kind: "operation",
      query: { operation: fullId(id) },
    });
  }
  operationDiff(id: string, parent?: string) {
    return this.query<"operation_diff">(id, {
      kind: "operation_diff",
      query: { operation: fullId(id), parent: parent ? fullId(parent) : null },
    });
  }
  operationChanges(id: string, parent: string, cursor?: string) {
    return this.query<"operation_changes">(
      id,
      {
        kind: "operation_changes",
        query: {
          operation: fullId(id),
          parent: fullId(parent),
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
    );
  }
  diffPaths(operation: string, from: string, to: string, cursor?: string) {
    return this.query<"diff_paths">(
      operation,
      {
        kind: "diff_paths",
        query: {
          from: codeId(from, this.profile),
          to: codeId(to, this.profile),
          path: "",
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
    );
  }
  diff(operation: string, from: string, to: string, path: string) {
    return this.query<"diff">(operation, {
      kind: "diff",
      query: {
        from: codeId(from, this.profile),
        to: codeId(to, this.profile),
        path,
        content: true,
      },
    });
  }
  conflicts(operation: string, commit: string) {
    return this.query<"conflicts">(operation, {
      kind: "conflicts",
      query: { commit: codeId(commit, this.profile), path: "" },
    });
  }
  protected async metadata() {
    const head = await this.snapshot();
    if (!head.metadata_commit) throw new ApiError("local_metadata_unavailable");
    return head.metadata_commit;
  }
  close() {
    this.#capture++;
    this.client.close();
    this.#reads.clear();
    this.head = null;
  }
}

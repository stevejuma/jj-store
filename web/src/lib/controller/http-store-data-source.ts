import { createNativeBlame } from "./http-blame.js";
import { readHttpGraph } from "./http-graph-reads.js";
import { httpReferences } from "./http-reference-reads.js";
import { HttpFileReads } from "./http-file-reads.js";
import { createHttpQuery } from "./http-query.js";
import { HttpRepositoryTransports } from "./http-repository-transports.js";
import type { QueryClient } from "@tanstack/svelte-query";
import {
  Client,
  fullId,
  type Envelope,
  type Head,
  type Model,
} from "../api/client.js";
import type {
  GraphQuery,
  ReferenceQuery,
  PathQuery,
  SearchQuery,
  StoreDataSource,
} from "./native-data-source.js";
import type { StoreMutationRequest } from "./store-mutation-types.js";
import { storeQueryKeys, type StoreQueryScope } from "./store-query-keys.js";
import { HttpCollaborationDataSource } from "./http-collaboration-data-source.js";

import { HttpThreadsDataSource } from "./http-threads-data-source.js";
import {
  cachedWorkSource,
  type HttpWorkDataSource,
} from "./http-work-data-source.js";

export class HttpStoreDataSource
  extends HttpRepositoryTransports
  implements StoreDataSource
{
  readonly profile = "native" as const;
  readonly #client: Client;
  readonly #scope: StoreQueryScope;
  readonly #collaboration: HttpCollaborationDataSource;
  readonly #files: HttpFileReads;
  readonly work: HttpWorkDataSource;
  readonly threads: HttpThreadsDataSource;
  #active = true;
  readonly #query: ReturnType<typeof createHttpQuery>;

  constructor(
    readonly repository: string,
    token: string | null,
    readonly queryClient: QueryClient,
    session: string,
    denied: () => void,
    fetcher?: typeof fetch,
    private readonly ownsQueries = true,
  ) {
    const client = new Client(repository, token, denied, fetcher);
    super(client);
    this.#client = client;
    this.#scope = { repository, session, profile: this.profile };
    this.#blame = createNativeBlame(
      client,
      queryClient,
      () => JSON.stringify(this.#scope),
      () => this.#active,
    );
    this.#query = createHttpQuery(queryClient, this.#scope, () => this.#active);
    this.#files = new HttpFileReads(client, this.#query);
    this.threads = new HttpThreadsDataSource(
      client,
      queryClient,
      this.#scope,
      () => this.#active,
    );
    this.work = cachedWorkSource(
      this.#client,
      this,
      this.#query,
      queryClient,
      this.#scope,
    );
    this.#collaboration = new HttpCollaborationDataSource(
      this.#client,
      queryClient,
      this.#scope,
      () => this.#active,
    );
  }

  readonly #blame: ReturnType<typeof createNativeBlame>;
  get blame() {
    return this.#blame();
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
      await this.queryClient.invalidateQueries({
        queryKey: key,
        exact: true,
        refetchType: "none",
      });
    const head = await this.#query<Head>("snapshot", null, {}, (signal) =>
      this.#client.request("snapshot", {}, { signal }),
    );
    this.#scope.epoch = head.epoch;
    return head;
  }
  references(operation: string, cursor?: string, input: ReferenceQuery = {}) {
    return httpReferences(this.#client, this.#query, operation, cursor, input);
  }

  graph(kind: "log" | "operations" | "evolution", input: GraphQuery) {
    return readHttpGraph(this.#client, this.#query, kind, input);
  }
  commitSummary(id: string, operation: string, signal?: AbortSignal) {
    return this.#files.commitSummary(id, operation, signal);
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
    return this.#files.paths(commit, operation, input);
  }
  file(
    id: string,
    operation: string,
    context?: { commit: string; path: string; signal?: AbortSignal },
  ) {
    return this.#files.file(id, operation, context);
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
  diffPaths(
    operation: string,
    from: string,
    to: string,
    cursor?: string,
    signal?: AbortSignal,
  ) {
    const query = { at_op: operation, from, to, cursor, limit: 50 };
    return this.#query<Model<"Window4">>(
      "diff-paths",
      operation,
      query,
      (shared) => this.#client.request("diff/paths", query, { signal: shared }),
      signal,
    );
  }
  diff(
    operation: string,
    from: string,
    to: string,
    path: string,
    signal?: AbortSignal,
  ) {
    const query = { at_op: operation, from, to, path, content: true };
    return this.#query<Envelope<Model<"FileDiffResponse">>>(
      "diff",
      operation,
      query,
      (shared) => this.#client.request("diff", query, { signal: shared }),
      signal,
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
    return this.writes.uploadConflict(head, commit, path, fingerprint, bytes);
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
  agentSessions(cursor?: string) {
    return this.#collaboration.agentSessions(cursor);
  }
  agentSession(id: string, fresh = false) {
    return this.#collaboration.agentSession(id, fresh);
  }
  agentRelations(
    subject: { commit: string } | { operation: string },
    cursor?: string,
  ) {
    return this.#collaboration.agentRelations(subject, cursor);
  }
  agentTranscript(
    session: string,
    transcript: Model<"AgentTranscriptDescriptor">,
    cursor?: string,
  ) {
    return this.#collaboration.agentTranscript(session, transcript, cursor);
  }
  reviewThreads(cursor?: string) {
    return this.#collaboration.reviewThreads(cursor);
  }
  reviewThread(id: string) {
    return this.#collaboration.reviewThread(id);
  }
  async applyReviewCommand(
    request: import("./native-data-source.js").StoreReviewCommandRequest,
  ) {
    return this.#collaboration.applyReviewCommand(request);
  }
  reviewCommandStatus(id: string) {
    return this.#collaboration.reviewCommandStatus(id);
  }
  previewMutation(request: StoreMutationRequest) {
    return this.writes.previewMutation(request);
  }
  applyMutation(request: StoreMutationRequest) {
    return this.writes.applyMutation(request);
  }
  mutationStatus(requestId: string) {
    return this.writes.mutationStatus(requestId);
  }
  close() {
    this.#active = false;
    this.#client.close();
    if (this.ownsQueries) {
      void this.queryClient.cancelQueries({
        queryKey: storeQueryKeys.root(this.#scope),
      });
      this.queryClient.removeQueries({
        queryKey: storeQueryKeys.root(this.#scope),
      });
    }
  }
}

export { storeErrorMessage } from "./store-error.js";

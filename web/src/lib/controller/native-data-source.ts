import type { Envelope, Graph, Head, Model, Task } from "../api/client.js";
import type {
  StoreMutationOutcome,
  StoreMutationPreview,
  StoreMutationRequest,
  StoreMutationStatusResponse,
} from "./store-mutation-types.js";

export interface GraphQuery {
  operation: string;
  cursor?: string;
  limit?: number;
  revset?: string;
  workspace?: string;
  userEmail?: string;
  atTimeMillis?: number;
  commit?: string;
}

export interface PathQuery {
  path?: string;
  recursive?: boolean;
  cursor?: string;
  limit?: number;
}

export interface SearchQuery {
  query: string;
  operation: string;
  cursor?: string;
  limit?: number;
}

export interface StoreDataSource {
  readonly repository: string;
  capabilities(): Promise<Model<"Capabilities">>;
  snapshot(fresh?: boolean): Promise<Head>;
  references(operation: string, cursor?: string): Promise<Model<"Window">>;
  graph(
    kind: "log" | "operations" | "evolution",
    query: GraphQuery,
  ): Promise<Envelope<Graph>>;
  commit(
    id: string,
    operation: string,
  ): Promise<Envelope<Model<"CommitDetail">>>;
  paths(
    commit: string,
    operation: string,
    query?: PathQuery,
  ): Promise<Model<"Window3">>;
  file(
    id: string,
    operation: string,
  ): Promise<{
    text?: string;
    bytes: number;
  }>;
  operation(id: string): Promise<Envelope<Model<"OperationDetail">>>;
  operationDiff(
    id: string,
    parent?: string,
  ): Promise<Envelope<Model<"OperationDiffResponse">>>;
  operationChanges(
    id: string,
    parent: string,
    cursor?: string,
  ): Promise<Model<"Window2">>;
  diffPaths(
    operation: string,
    from: string,
    to: string,
    cursor?: string,
  ): Promise<Model<"Window4">>;
  diff(
    operation: string,
    from: string,
    to: string,
    path: string,
  ): Promise<Envelope<Model<"FileDiffResponse">>>;
  conflicts(
    operation: string,
    commit: string,
  ): Promise<Envelope<Model<"ConflictResponse">>>;
  uploadConflict(
    head: Head,
    commit: string,
    path: string,
    fingerprint: string,
    bytes: Uint8Array,
  ): Promise<Model<"ConflictUpload">>;
  search(query: SearchQuery): Promise<Model<"SearchPage">>;
  previewMutation(request: StoreMutationRequest): Promise<StoreMutationPreview>;
  applyMutation(request: StoreMutationRequest): Promise<StoreMutationOutcome>;
  mutationStatus(requestId: string): Promise<StoreMutationStatusResponse>;
  prepareIndex(operation: string): Promise<Task>;
  indexTask(id: string): Promise<Task>;
  events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: Parameters<import("../api/client.js").Client["events"]>[2],
  ): Promise<void>;
  close(): void;
}

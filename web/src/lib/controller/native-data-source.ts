import type {
  RepositoryCapabilities,
  RepositoryHead,
  RepositoryProfile,
} from "./repository-profile.js";
import type * as Browse from "./repository-read-types.js";
import type {
  Envelope,
  Exact,
  Graph,
  Head,
  Model,
  Task,
} from "../api/client.js";
import type { operations } from "../api/generated.js";
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
  operationQuery?: string;
  signal?: AbortSignal;
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
export type StoreReviewCommandRequest = Exact<
  NonNullable<
    operations["review_command_apply"]["requestBody"]
  >["content"]["application/json"]
>;

export interface StoreDataSource {
  readonly repository: string;
  readonly workingCopy?: import("./working-copy-types.js").WorkingCopyTransport;
  readonly threads?: import("./thread-types.js").ThreadTransport;
  readonly profile?: RepositoryProfile;
  readonly searchSettings?: import("./search-settings.js").SearchSettingsTransport;
  readonly storage?: import("./storage-settings.js").StorageTransport;
  readonly sync?: import("./sync-settings.js").SyncTransport;
  readonly localWorkspaces?: import("./workspace-link-types.js").WorkspaceLinkTransport;
  readonly work?: import("./projects-hub-types.js").WorkBrowseTransport;
  queryInfo?(
    operation: string,
  ): Promise<import("./history-query-types.js").HistoryQueryInfo>;
  capabilities(): Promise<RepositoryCapabilities>;
  snapshot(fresh?: boolean): Promise<RepositoryHead>;
  references(
    operation: string,
    cursor?: string,
  ): Promise<Browse.CodeWindow<Browse.CodeReferences>>;
  graph(
    kind: "log" | "operations" | "evolution",
    query: GraphQuery,
  ): Promise<Browse.CodeEnvelope<Browse.CodeGraph>>;
  commit(
    id: string,
    operation: string,
  ): Promise<Browse.CodeEnvelope<Browse.CodeCommitDetail>>;
  paths(
    commit: string,
    operation: string,
    query?: PathQuery,
  ): Promise<Browse.CodeWindow<Browse.CodePaths>>;
  file(
    id: string,
    operation: string,
  ): Promise<{
    text?: string;
    bytes: number;
  }>;
  operation(
    id: string,
  ): Promise<Browse.CodeEnvelope<Browse.CodeOperationDetail>>;
  operationDiff(
    id: string,
    parent?: string,
  ): Promise<Browse.CodeEnvelope<Browse.CodeOperationDiff>>;
  operationChanges(
    id: string,
    parent: string,
    cursor?: string,
  ): Promise<Browse.CodeWindow<Browse.CodeOperationChanges>>;
  diffPaths(
    operation: string,
    from: string,
    to: string,
    cursor?: string,
  ): Promise<Browse.CodeWindow<Browse.CodeDiffPaths>>;
  diff(
    operation: string,
    from: string,
    to: string,
    path: string,
  ): Promise<Browse.CodeEnvelope<Browse.CodeDiff>>;
  conflicts(
    operation: string,
    commit: string,
  ): Promise<Browse.CodeEnvelope<Browse.CodeConflicts>>;
  uploadConflict(
    head: Head,
    commit: string,
    path: string,
    fingerprint: string,
    bytes: Uint8Array,
  ): Promise<Model<"ConflictUpload">>;
  search(query: SearchQuery): Promise<Model<"SearchPage">>;
  searchStatus(fresh?: boolean): Promise<Model<"SearchStatus">>;
  agentSessions(cursor?: string): Promise<Browse.AgentPage>;
  agentSession(id: string): Promise<Browse.AgentDetail>;
  agentRelations(
    subject: { commit: string } | { operation: string },
    cursor?: string,
  ): Promise<Browse.AgentRelations>;
  agentTranscript(
    session: string,
    transcript: Model<"AgentTranscriptDescriptor">,
    cursor?: string,
  ): Promise<Browse.AgentTranscript>;
  reviewThreads(cursor?: string): Promise<Browse.ReviewPage>;
  reviewThread(id: string): Promise<Browse.ReviewDetail>;
  applyReviewCommand(
    request: StoreReviewCommandRequest,
  ): Promise<Browse.ReviewOutcome>;
  reviewCommandStatus(id: string): Promise<Browse.ReviewStatus>;
  previewMutation(request: StoreMutationRequest): Promise<StoreMutationPreview>;
  applyMutation(request: StoreMutationRequest): Promise<StoreMutationOutcome>;
  mutationStatus(requestId: string): Promise<StoreMutationStatusResponse>;
  prepareIndex(operation: string): Promise<Task>;
  indexTask(id: string): Promise<Task>;
  events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: (event: Browse.RepositoryFreshness, id?: string) => void,
  ): Promise<void>;
  close(): void;
}

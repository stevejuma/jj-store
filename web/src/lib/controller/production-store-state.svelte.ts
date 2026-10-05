import type {
  RepositoryHead as Head,
  RepositoryCapabilities,
} from "./repository-profile.js";
import type { AgentDetail } from "./repository-read-types.js";
import type { CodeSnapshot as Snapshot } from "./repository-profile.js";
import { emptySearchSettings } from "./search-settings.js";
import { emptyStorageSettings } from "./storage-settings.js";
import { emptySyncSettings } from "./sync-settings.js";
import {
  emptyWorkspaceLinks,
  type WorkspaceLinkSettings,
} from "./workspace-link-types.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
import type {
  RepositoryLogGraph,
  RepositoryWorkingCopy,
} from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type {
  RepositoryBlob,
  RepositoryChoice,
  RepositorySnapshot,
  StoreRepositorySummary,
  StoreControllerStatus,
  StoreSearchResult,
} from "./contracts.js";
import type { Comparison } from "./production-store-loaders.js";
import type { InspectionContinuation } from "./production-inspection-pages.js";
import type { StoredPathPage } from "./production-file-pages.js";
import {
  emptyEvolution,
  type ProductionEvolutionState,
} from "./production-evolution-reader.js";
import {
  initialStoreMutationState,
  type MutableStoreMutationState,
} from "./store-mutation-types.js";

type MutableStatus = {
  -readonly [K in keyof StoreControllerStatus]: StoreControllerStatus[K];
};

export interface ProductionStoreState extends WorkspaceLinkSettings {
  searchSettings: import("./search-settings.js").SearchSettings;
  storageSettings: import("./storage-settings.js").StorageSettings;
  syncSettings: import("./sync-settings.js").SyncSettings;
  accessMode: "bootstrapping" | "authenticated" | "local_trusted";
  storageReady: boolean;
  nativeRepositoryCreation: boolean;
  repositoryConversion: boolean;
  captureSupported: boolean;
  settingsSupported: boolean;
  backups: Model<"BackupStatus">[];
  heads: string[];
  repositories: StoreRepositorySummary[];
  capabilities: RepositoryCapabilities | null;
  searchStatus: Model<"SearchStatus"> | null;
  searchStatusState: import("./contracts.js").StoreLoadStatus;
  searchStatusError: string;
  indexPreparation: import("./contracts.js").StoreLoadStatus;
  indexPreparationKind: import("./native-data-source.js").IndexPreparationKind;
  indexProgress: Model<"KindPreparationProgress"> | null;
  indexCancellationUnconfirmed: boolean;
  indexPreparationError: string;
  indexTaskStatus: import("../api/client.js").Task["status"] | null;
  bookmarks: Bookmark[];
  choices: RepositoryChoice[];
  workingCopies: RepositoryWorkingCopy[];
  workingCopiesStatus: import("./contracts.js").StoreLoadStatus;
  workingCopiesCursor: string | null;
  workingCopiesSnapshot: Snapshot | null;
  graph: RepositoryLogGraph;
  evolution: ProductionEvolutionState;
  targets: string[];
  snapshot: RepositorySnapshot | undefined;
  blob: RepositoryBlob | undefined;
  detail: RevisionDetail | undefined;
  fileRevision:
    import("../projects/repository-log.js").RepositoryCommitSummary | undefined;
  fileRevisionStatus: import("./contracts.js").StoreLoadStatus;
  comparisonParents: string[];
  conflictCount: number | null;
  conflicts: import("./repository-read-types.js").CodeConflict[];
  conflictResolutionActions: string[];
  conflictUploadBytes: number | bigint;
  unavailableComparisonCount: number;
  status: MutableStatus;
  refsCursor: string | null;
  graphCursor: string | null;
  graphAtTimeMillis: number | null;
  historyEmail: string;
  fileViewMode: FileListingViewMode;
  pathPages: Map<string, StoredPathPage>;
  knownDirectories: Set<string>;
  collapsedDirectoryPaths: Set<string>;
  fileTerms: Map<string, string>;
  comparisons: Map<string, Comparison>;
  inspectionContinuation: InspectionContinuation | null;
  neededIndex: Snapshot | null;
  searchAvailable: boolean;
  searchQuery: string;
  searchResults: StoreSearchResult[];
  searchCursor: string | null;
  searchStale: boolean;
  searchDegraded: boolean;
  searchTruncated: boolean;
  searchError: string;
  refsPageSnapshot: Snapshot | null;
  graphPageSnapshot: Snapshot | null;
  head: Head | null;
  mutation: MutableStoreMutationState;
  agentSessions: Model<"AgentSessionSummary">[];
  agentSession: AgentDetail | undefined;
  agentRelations: Model<"AgentRelationLink">[];
  agentTranscript: Model<"AgentTranscriptChunk">[];
  agentCursor: string | null;
  transcriptCursor: string | null;
  transcriptDescriptor: Model<"AgentTranscriptDescriptor"> | undefined;
  reviewThreads: Model<"ReviewThreadSummary">[];
  reviewThread: import("./repository-read-types.js").ReviewDetail | undefined;
  reviewContext:
    import("./native-activity-types.js").NativeReviewContext | undefined;
  reviewPatch: import("./review-anchor-patch.js").ReviewInlinePatch | undefined;
  reviewCursor: string | null;
  reviewActionMessage: string;
  reviewActionRequestId: string | null;
  committedReviewRequestId: string | null;
}

export const emptyGraph = (): RepositoryLogGraph => ({
  revisions: [],
  devices: [],
});

export const initialStatus = (): MutableStatus => ({
  connection: "disconnected",
  navigation: "idle",
  repositories: "idle",
  bookmarks: "idle",
  graph: "idle",
  files: "idle",
  detail: "idle",
  search: "idle",
  agents: "idle",
  transcript: "idle",
  reviews: "idle",
  reviewAction: "idle",
  live: "disconnected",
  refreshAvailable: false,
  error: "",
});

export function createProductionStoreState(): ProductionStoreState {
  const state: ProductionStoreState = $state({
    ...emptyWorkspaceLinks(),
    searchSettings: emptySearchSettings(),
    storageSettings: emptyStorageSettings(),
    syncSettings: emptySyncSettings(),
    accessMode: "bootstrapping",
    storageReady: false,
    nativeRepositoryCreation: false,
    repositoryConversion: false,
    captureSupported: false,
    settingsSupported: false,
    backups: [],
    heads: [],
    repositories: [],
    capabilities: null,
    searchStatus: null,
    searchStatusState: "idle",
    searchStatusError: "",
    indexPreparation: "idle",
    indexPreparationKind: "graph",
    indexProgress: null,
    indexCancellationUnconfirmed: false,
    indexPreparationError: "",
    indexTaskStatus: null,
    bookmarks: [],
    choices: [],
    workingCopies: [],
    workingCopiesStatus: "idle",
    workingCopiesCursor: null,
    workingCopiesSnapshot: null,
    graph: emptyGraph(),
    evolution: emptyEvolution(),
    targets: [],
    snapshot: undefined,
    blob: undefined,
    detail: undefined,
    fileRevision: undefined,
    fileRevisionStatus: "idle",
    comparisonParents: [],
    conflictCount: null,
    conflicts: [],
    conflictResolutionActions: [],
    conflictUploadBytes: 0,
    unavailableComparisonCount: 0,
    status: initialStatus(),
    refsCursor: null,
    graphCursor: null,
    graphAtTimeMillis: null,
    historyEmail: "",
    fileViewMode: "tree",
    pathPages: new Map(),
    knownDirectories: new Set(),
    collapsedDirectoryPaths: new Set(),
    fileTerms: new Map(),
    comparisons: new Map(),
    inspectionContinuation: null,
    neededIndex: null,
    searchAvailable: false,
    searchQuery: "",
    searchResults: [],
    searchCursor: null,
    searchStale: false,
    searchDegraded: false,
    searchTruncated: false,
    searchError: "",
    refsPageSnapshot: null,
    graphPageSnapshot: null,
    head: null,
    mutation: initialStoreMutationState(),
    agentSessions: [],
    agentSession: undefined,
    agentRelations: [],
    agentTranscript: [],
    agentCursor: null,
    transcriptCursor: null,
    transcriptDescriptor: undefined,
    reviewThreads: [],
    reviewThread: undefined,
    reviewContext: undefined,
    reviewPatch: undefined,
    reviewCursor: null,
    reviewActionMessage: "",
    reviewActionRequestId: null,
    committedReviewRequestId: null,
  });
  return state;
}

export function clearProductionStoreView(state: ProductionStoreState) {
  state.bookmarks = [];
  state.choices = [];
  state.workingCopies = [];
  state.workingCopiesStatus = "idle";
  state.workingCopiesCursor = null;
  state.workingCopiesSnapshot = null;
  state.graph = emptyGraph();
  state.evolution = emptyEvolution();
  state.targets = [];
  state.snapshot = undefined;
  state.blob = undefined;
  state.detail = undefined;
  state.fileRevision = undefined;
  state.fileRevisionStatus = "idle";
  state.comparisonParents = [];
  state.conflictCount = null;
  state.conflicts = [];
  state.unavailableComparisonCount = 0;
  state.refsCursor = null;
  state.graphCursor = null;
  state.graphAtTimeMillis = null;
  state.historyEmail = "";
  state.fileViewMode = "tree";
  state.pathPages = new Map();
  state.knownDirectories = new Set();
  state.collapsedDirectoryPaths = new Set();
  state.fileTerms = new Map();
  state.comparisons = new Map();
  state.inspectionContinuation = null;
  state.refsPageSnapshot = null;
  state.graphPageSnapshot = null;
  state.agentSessions = [];
  state.agentSession = undefined;
  state.agentRelations = [];
  state.agentTranscript = [];
  state.agentCursor = null;
  state.transcriptCursor = null;
  state.transcriptDescriptor = undefined;
  state.reviewThreads = [];
  state.reviewThread = undefined;
  state.reviewContext = undefined;
  state.reviewPatch = undefined;
  state.reviewCursor = null;
  state.reviewActionMessage = "";
  state.reviewActionRequestId = null;
  state.committedReviewRequestId = null;
}

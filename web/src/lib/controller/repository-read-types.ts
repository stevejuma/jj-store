import type { Model } from "../api/client.js";
import type { CodeSnapshot } from "./repository-profile.js";
export type CodeEnvelope<T> = {
  snapshot: CodeSnapshot;
  data: T;
  next_cursor: string | null;
};
export type CodeWindow<T> = CodeEnvelope<T> & {
  previous_cursor: string | null;
};
export type CodeGraph = Model<"GraphPage"> | Model<"LocalGraphPage">;
export type CodeCommitDetail =
  Model<"CommitDetail"> | Model<"LocalCommitDetail">;
export type CodeOperationDetail =
  Model<"OperationDetail"> | Model<"LocalOperationDetail">;
export type CodeOperationDiff =
  Model<"OperationDiffResponse"> | Model<"LocalOperationDiffResponse">;
export type CodeReferences = Model<"Page"> | Model<"LocalReferencesPage">;
export type CodePaths = Model<"Window3">["data"] | Model<"LocalPathsPage">;
export type CodeEntry = Model<"Entry"> | Model<"LocalPathsEntry">;
export type CodeTreeValue = Model<"NativeTreeValue"> | Model<"LocalTreeValue">;
export type CodeOperationChanges =
  Model<"Window2">["data"] | Model<"LocalOperationChangesPage">;
export type CodeDiffPaths =
  Model<"Window4">["data"] | Model<"LocalDiffSummaryPage">;
export type CodeDiff =
  Model<"FileDiffResponse"> | Model<"LocalFileDiffResponse">;
export type CodeDiffChange =
  Model<"FileDiffChange"> | Model<"LocalFileDiffChange">;
export type CodeConflicts =
  Model<"ConflictResponse"> | Model<"LocalConflictResponse">;
export type CodeConflict =
  Model<"ConflictDescriptor"> | Model<"LocalConflictDescriptor">;
export type AgentSnapshot =
  Model<"AgentReadSnapshot"> | Model<"LocalAgentSnapshot">;
export type AgentAssociation =
  Model<"AgentCodeAssociation"> | Model<"LocalBrowserCodeAssociation">;
export type AgentDetail = Omit<
  Model<"AgentSessionDetail">,
  "snapshot" | "code"
> & { snapshot: AgentSnapshot; code: AgentAssociation[] };
export type AgentPage = Omit<Model<"AgentSessionPage">, "snapshot"> & {
  snapshot: AgentSnapshot;
};
export type AgentRelations = Omit<
  Model<"AgentRelationPage">,
  "snapshot" | "subject"
> & {
  snapshot: AgentSnapshot;
  subject: Model<"AgentRelationSubject"> | Model<"LocalAgentSubject">;
};
export type AgentTranscript = Omit<Model<"AgentTranscriptPage">, "snapshot"> & {
  snapshot: AgentSnapshot;
};
export type RepositoryFreshness =
  Model<"CommittedEvent"> | Model<"LocalFreshnessEvent">;
export function isLocalAssociation(
  value: AgentAssociation,
): value is Model<"LocalBrowserCodeAssociation"> {
  return "source_repositories" in value;
}

export type ReviewSnapshot =
  Model<"ReviewReadSnapshot"> | Model<"LocalReviewSnapshot">;
export type ReviewActivitySummary = Omit<
  Model<"ReviewActivitySummary">,
  "snapshot"
> & {
  snapshot: ReviewSnapshot;
};
export type ReviewPage = Omit<Model<"ReviewThreadPage">, "snapshot"> & {
  snapshot: ReviewSnapshot;
};
export type ReviewDetail = Omit<Model<"ReviewThreadDetail">, "snapshot"> & {
  snapshot: ReviewSnapshot;
};
export type ReviewOutcome =
  Model<"ReviewCommandOutcome"> | Model<"LocalReviewOutcome">;
export type ReviewStatus =
  Model<"ReviewCommandStatus"> | Model<"LocalReviewOutcome">;

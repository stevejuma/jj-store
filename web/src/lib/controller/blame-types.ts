import type { RepositoryCommitSummary } from "../projects/repository-log.js";

/** Presentation boundary shared by native and local annotation readers. */
export interface BlameSelection {
  repository: string;
  operation: string;
  commit: string;
  path: string;
  lineCount: number;
}
export interface BlameRange {
  /** One-based, end-exclusive source line numbers. */
  start: number;
  end: number;
  commitId: string;
}
export interface BlameProgress {
  lineCount: number;
  ranges: readonly BlameRange[];
  commits: Readonly<Record<string, RepositoryCommitSummary>>;
  complete: boolean;
  continuation: string | null;
}
export interface BlameViewState {
  progress?: BlameProgress;
  status: "loading" | "ready" | "failed";
  error?: string;
  preparing?: boolean;
}
export interface BlameController {
  /** Provider owns source/access invalidation; inactive scopes may never replay data. */
  observe(
    selection: BlameSelection,
    publish: (state: BlameViewState) => void,
  ): () => void;
}

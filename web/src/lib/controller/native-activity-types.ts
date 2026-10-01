import type { Model, Snapshot } from "../api/client.js";
import type { RepositoryLogRevision } from "../projects/repository-log.js";

export interface NativeAgentContext {
  sessions: Model<"AgentSessionDetail">[];
  error: string;
  relatedSessionIds?: readonly string[];
  /** Read association metadata when the virtual row mounts. */
  discover?: () => Promise<NativeAgentContext>;
  /** Read the pinned sanitized sessions only when the disclosure opens. */
  load?: () => Promise<NativeAgentContext>;
}
export interface NativeActivityCommit {
  repository: string;
  operation: string;
  revision: RepositoryLogRevision;
  agents: NativeAgentContext;
}
export interface NativeActivityQuery {
  operation?: string;
  cursor?: string;
  atTimeMillis?: number;
  refresh?: boolean;
  agentSnapshot?: Model<"AgentReadSnapshot">;
}
export interface NativeActivityPage {
  snapshot: Snapshot;
  atTimeMillis: number;
  commits: NativeActivityCommit[];
  cursor: string | null;
  agentSnapshot?: Model<"AgentReadSnapshot">;
}
export interface NativeReviewContext {
  commits: NativeActivityCommit[];
  files: import("@lapismd/design-core/diff").FileListingFile[];
  patches: Record<string, string>;
  agents: NativeAgentContext;
  codeError: string;
}

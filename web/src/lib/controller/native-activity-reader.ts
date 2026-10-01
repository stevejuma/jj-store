import { snapshotRepository } from "./repository-profile.js";
import type { StoreDataSource } from "./native-data-source.js";
import type {
  NativeActivityPage,
  NativeActivityQuery,
  NativeActivityCommit,
} from "./native-activity-types.js";
import type { RepositoryLogRevision } from "../projects/repository-log.js";
import { assertPinned } from "./production-snapshot.js";
import { presentGraph } from "./native-presentation.js";
import { createDeferredAgentContexts } from "./native-agent-context.js";

export async function readCommitActivity(
  source: StoreDataSource,
  operation: string,
  revision: RepositoryLogRevision,
): Promise<NativeActivityCommit> {
  return {
    repository: source.repository,
    operation,
    revision,
    agents: createDeferredAgentContexts(source)({ commit: revision.eventId }),
  };
}

export async function readNativeActivityPage(
  source: StoreDataSource,
  query: NativeActivityQuery = {},
): Promise<NativeActivityPage> {
  let operation = query.operation;
  if (!operation) {
    const head = await source.snapshot(query.refresh);
    if (
      snapshotRepository(head) !== source.repository ||
      head.heads.length !== 1
    )
      throw new Error(
        "Select an explicit code operation before browsing activity.",
      );
    operation = head.heads[0]!;
  }
  const atTimeMillis = query.atTimeMillis ?? Date.now();
  const page = await source.graph("log", {
    operation,
    cursor: query.cursor,
    atTimeMillis,
    limit: 20,
  });
  assertPinned(page.snapshot, source, operation);
  const revisions = presentGraph(page.data, page.snapshot.heads).revisions;
  let agentSnapshot = query.agentSnapshot;
  let agentError = "";
  if (revisions.length && !agentSnapshot) {
    try {
      agentSnapshot = (await source.agentSessions()).snapshot;
    } catch (cause) {
      agentError =
        cause instanceof Error ? cause.message : "Agent context unavailable";
    }
  }
  const agentContext = createDeferredAgentContexts(source, agentSnapshot);
  const commits = revisions.map((revision) => ({
    repository: source.repository,
    operation: operation!,
    revision,
    agents: agentError
      ? { sessions: [], error: agentError }
      : agentContext({ commit: revision.eventId }),
  }));
  return {
    snapshot: page.snapshot,
    atTimeMillis,
    commits,
    cursor: page.next_cursor,
    ...(agentSnapshot ? { agentSnapshot } : {}),
  };
}

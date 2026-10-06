import { Client, ApiError, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import type { AgentTarget } from "./agent-execution-types.js";

/** Bounded, operation-pinned reads from only the selected launch repository. */
export async function agentRevisions(
  target: AgentTarget,
  fetcher: typeof fetch,
  query: string,
  signal: AbortSignal,
  cursor?: string,
) {
  const client =
    target.profile === "native"
      ? new Client(target.repository, null, () => {}, fetcher)
      : new LocalClient(target.repository, fetcher);
  try {
    const page =
      client instanceof Client
        ? await client.request<Model<"Window">>(
            "references",
            {
              at_op: target.operation,
              kind: "all",
              limit: 50,
              q: query,
              cursor,
            },
            { signal },
          )
        : await client.request<Model<"LocalCodeQueryResponse">>(
            "query",
            {
              operation: target.operation,
              cursor: cursor ?? null,
              request: {
                kind: "references",
                query: {
                  operation: target.operation,
                  kind: "all",
                  query,
                  exact: null,
                  limit: 50,
                  anchor: null,
                  direction: "forward",
                },
              },
            },
            signal,
          );
    signal.throwIfAborted();
    const repository =
      "repo_id" in page.snapshot
        ? page.snapshot.repo_id
        : page.snapshot.repository;
    const operation =
      "operation_id" in page.snapshot
        ? page.snapshot.operation_id
        : page.snapshot.operation;
    if (repository !== target.repository || operation !== target.operation)
      throw new ApiError("snapshot_mismatch");
    const data =
      "references" in page.data
        ? page.data
        : page.data.kind === "references"
          ? page.data.data
          : null;
    if (!data || data.operation !== target.operation)
      throw new ApiError("snapshot_mismatch");
    if (cursor && page.next_cursor === cursor)
      throw new ApiError("invalid_cursor");
    const items = new Map<
      string,
      { id: string; name: string; description: string }
    >();
    for (const reference of data.references)
      for (const revision of reference.targets) {
        if (!/^(?:[a-f0-9]{40}|[a-f0-9]{128})$/.test(revision.commit))
          throw new ApiError("invalid_code_identity");
        items.set(revision.commit, {
          id: revision.commit,
          name:
            reference.kind === "change"
              ? revision.commit.slice(0, 12)
              : reference.name,
          description: `${revision.description || "No description"} · ${revision.commit.slice(0, 12)}`,
        });
      }
    return { items: [...items.values()], next: page.next_cursor };
  } finally {
    client.close();
  }
}

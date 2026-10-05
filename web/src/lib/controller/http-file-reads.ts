import { Client, fullId, type Envelope, type Model } from "../api/client.js";
import type { PathQuery } from "./native-data-source.js";
import type { createHttpQuery } from "./http-query.js";
export class HttpFileReads {
  constructor(
    private readonly client: Client,
    private readonly query: ReturnType<typeof createHttpQuery>,
  ) {}
  commitSummary(id: string, operation: string, signal?: AbortSignal) {
    return this.query<Envelope<Model<"ContentObject">>>(
      "commit-summary",
      operation,
      { id },
      (shared) =>
        this.client.request(
          `objects/commit/${fullId(id)}`,
          { at_op: fullId(operation) },
          { signal: shared },
        ),
      signal,
    );
  }
  paths(commit: string, operation: string, input: PathQuery = {}) {
    const query = {
      at_op: fullId(operation),
      path: input.path ?? "",
      recursive: input.recursive ?? false,
      include_last_change: input.includeLastChange ?? true,
      ...(input.lastChangeMode
        ? { last_change_mode: input.lastChangeMode }
        : {}),
      anchor: input.anchor,
      limit: input.limit ?? 50,
      cursor: input.cursor,
    };
    return this.query<Model<"Window3">>(
      "paths",
      operation,
      { commit, ...query },
      (signal) =>
        this.client.request(`commits/${fullId(commit)}/paths`, query, {
          signal,
        }),
      input.signal,
    );
  }
  file(
    id: string,
    operation: string,
    context?: { commit: string; path: string; signal?: AbortSignal },
  ) {
    return this.query(
      "file",
      operation,
      { id },
      (signal) => this.client.file(fullId(id), fullId(operation), signal),
      context?.signal,
    );
  }
}

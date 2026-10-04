import { Client, fullId, type Envelope, type Model } from "../api/client.js";
import type { PathQuery } from "./native-data-source.js";
import type { createHttpQuery } from "./http-query.js";
export class HttpFileReads {
  constructor(
    private readonly client: Client,
    private readonly query: ReturnType<typeof createHttpQuery>,
  ) {}
  commitSummary(id: string, operation: string, signal?: AbortSignal) {
    return this.client.request<Envelope<Model<"ContentObject">>>(
      `objects/commit/${fullId(id)}`,
      { at_op: fullId(operation) },
      { signal },
    );
  }
  paths(commit: string, operation: string, input: PathQuery = {}) {
    const query = {
      at_op: fullId(operation),
      path: input.path ?? "",
      recursive: input.recursive ?? false,
      include_last_change: input.includeLastChange ?? true,
      anchor: input.anchor,
      limit: input.limit ?? 50,
      cursor: input.cursor,
    };
    if (input.signal)
      return this.client.request<Model<"Window3">>(
        `commits/${fullId(commit)}/paths`,
        query,
        { signal: input.signal },
      );
    return this.query<Model<"Window3">>(
      "paths",
      operation,
      { commit, ...query },
      (signal) =>
        this.client.request(`commits/${fullId(commit)}/paths`, query, {
          signal,
        }),
    );
  }
  file(
    id: string,
    operation: string,
    context?: { commit: string; path: string; signal?: AbortSignal },
  ) {
    if (context?.signal)
      return this.client.file(fullId(id), fullId(operation), context.signal);
    return this.query("file", operation, { id }, (signal) =>
      this.client.file(fullId(id), fullId(operation), signal),
    );
  }
}

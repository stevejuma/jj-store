import { Client, fullId, pretty, type Model } from "../api/client.js";
import type { WorkQuery, WorkRequest, WorkTransport } from "./work-types.js";
import { readRepositoryDirectory } from "./http-repository-directory.js";
export class HttpWorkDataSource implements WorkTransport {
  constructor(private client: Client) {}
  directory = (signal?: AbortSignal) =>
    readRepositoryDirectory(this.client, this, signal);
  page(query: WorkQuery, signal?: AbortSignal) {
    return this.client.request<Model<"WorkPage">>(
      "work",
      {
        ...query,
        project: query.project ? fullId(query.project, 32) : undefined,
        task: query.task ? fullId(query.task, 32) : undefined,
      },
      { signal },
    );
  }
  browse(
    query: import("./projects-hub-types.js").WorkBrowseQuery,
    signal?: AbortSignal,
  ) {
    return this.client.request<Model<"WorkBrowsePage">>(
      "work/browse",
      { ...query },
      { signal },
    );
  }
  reviews(cursor?: string, signal?: AbortSignal) {
    return this.client.request<Model<"ReviewThreadPage">>(
      "reviews/threads",
      { cursor, limit: 50 },
      { signal },
    );
  }
  apply(request: WorkRequest) {
    return this.client.request<Model<"WorkCommandOutcome">>(
      "work/commands",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
  }
  status(id: string) {
    return this.client.request<Model<"WorkCommandStatus">>(
      `work/commands/${fullId(id, 32)}`,
    );
  }
}

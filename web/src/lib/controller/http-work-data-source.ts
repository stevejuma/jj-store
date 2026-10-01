import { Client, fullId, pretty, type Model } from "../api/client.js";
import type { WorkQuery, WorkRequest, WorkTransport } from "./work-types.js";
export class HttpWorkDataSource implements WorkTransport {
  constructor(private client: Client) {}
  page(query: WorkQuery) {
    return this.client.request<Model<"WorkPage">>("work", {
      ...query,
      project: query.project ? fullId(query.project, 32) : undefined,
      task: query.task ? fullId(query.task, 32) : undefined,
    });
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

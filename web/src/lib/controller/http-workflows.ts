import { Client, fullId, pretty, type Model } from "../api/client.js";
import type {
  WorkflowTransport,
  WorkflowQuery,
  WorkflowRequest,
} from "./workflow-types.js";
export class HttpWorkflows implements WorkflowTransport {
  constructor(private client: Client) {}
  page(query: WorkflowQuery, signal?: AbortSignal) {
    return this.client.request<Model<"WorkflowPage">>(
      "workflows",
      { ...query },
      { signal },
    );
  }
  apply(request: WorkflowRequest) {
    return this.client.request<Model<"WorkflowCommandOutcome">>(
      "workflows/commands",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
  }
  status(id: string) {
    return this.client.request<Model<"WorkflowCommandStatus">>(
      `workflows/commands/${fullId(id, 32)}`,
    );
  }
  events(signal: AbortSignal, changed: () => void) {
    return this.client.events(undefined, signal, changed);
  }
}

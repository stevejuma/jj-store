import { Client, pretty, type Model } from "../api/client.js";
import type {
  WorkspaceLinkRequest,
  WorkspaceLinkTransport,
} from "./workspace-link-types.js";

export class HttpWorkspaceLinks implements WorkspaceLinkTransport {
  constructor(private client: Client) {}
  pick() {
    return this.client.request<Model<"FolderSelection">>(
      "local/workspaces/pick",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: "{}",
      },
    );
  }
  list() {
    return this.client.request<Model<"WorkspaceLinks">>("local/workspaces");
  }
  link(request: WorkspaceLinkRequest) {
    return this.client.request<Model<"WorkspaceLinks">>(
      "local/workspaces/link",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
  }
}

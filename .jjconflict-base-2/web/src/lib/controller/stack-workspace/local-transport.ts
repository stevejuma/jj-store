import type { LocalClient } from "../../api/local-client.js";
import type { Exact, Model } from "../../api/client.js";
import type { operations } from "../../api/generated.js";
export type LocalStacksRequest = Exact<
  NonNullable<
    operations["local_stacks_execute"]["requestBody"]
  >["content"]["application/json"]
>;
export type LocalStacksCommand = LocalStacksRequest["command"];
export interface LocalStacksTransport {
  execute(request: LocalStacksRequest): Promise<Model<"LocalStacksState">>;
}

export class HttpLocalStacksTransport implements LocalStacksTransport {
  constructor(readonly client: LocalClient) {}
  execute(request: LocalStacksRequest) {
    return this.client.request<Model<"LocalStacksState">>("stacks", request);
  }
}

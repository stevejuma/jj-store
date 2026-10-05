import type { LocalClient } from "../../api/local-client.js";
import type { Exact, Model } from "../../api/client.js";
import type { operations } from "../../api/generated.js";
export type LocalStacksRequest = Exact<
  NonNullable<
    operations["local_stacks_execute"]["requestBody"]
  >["content"]["application/json"]
>;
export type LocalStacksCommand = LocalStacksRequest["command"];
export type LocalStacksInspectionRequest = Exact<
  NonNullable<
    operations["local_stacks_inspect"]["requestBody"]
  >["content"]["application/json"]
>;
export type LocalStacksHunkRequest = Exact<
  NonNullable<
    operations["local_stacks_hunks"]["requestBody"]
  >["content"]["application/json"]
>;
export interface LocalStacksTransport {
  hunks?(
    request: LocalStacksHunkRequest,
  ): Promise<Model<"LocalStacksHunkResponse">>;
  execute(request: LocalStacksRequest): Promise<Model<"LocalStacksState">>;
  preview?(request: LocalStacksRequest): Promise<Model<"LocalStacksPreview">>;
  inspect?(
    request: LocalStacksInspectionRequest,
  ): Promise<Model<"LocalStacksInspection">>;
}

export class HttpLocalStacksTransport implements LocalStacksTransport {
  constructor(readonly client: LocalClient) {}
  hunks(request: LocalStacksHunkRequest) {
    return this.client.request<Model<"LocalStacksHunkResponse">>(
      "stacks/hunks",
      request,
    );
  }
  inspect(request: LocalStacksInspectionRequest) {
    return this.client.request<Model<"LocalStacksInspection">>(
      "stacks/inspect",
      request,
    );
  }
  execute(request: LocalStacksRequest) {
    return this.client.request<Model<"LocalStacksState">>("stacks", request);
  }
  preview(request: LocalStacksRequest) {
    return this.client.request<Model<"LocalStacksPreview">>(
      "stacks/preview",
      request,
    );
  }
}

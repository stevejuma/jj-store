import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
import type { StoreLoadStatus } from "./contracts.js";

export type WorkspaceLinkRequest = Exact<
  NonNullable<
    operations["link_workspace"]["requestBody"]
  >["content"]["application/json"]
>;
export interface WorkspaceLinkTransport {
  list(): Promise<Model<"WorkspaceLinks">>;
  link(request: WorkspaceLinkRequest): Promise<Model<"WorkspaceLinks">>;
}
export interface WorkspaceLinkSettings {
  workspaceLinks: Model<"WorkspaceLinks"> | null;
  workspaceLinksState: StoreLoadStatus;
  workspaceLinksError: string;
  workspacePath: string;
  workspaceLinking: boolean;
}
export function emptyWorkspaceLinks(): WorkspaceLinkSettings {
  return {
    workspaceLinks: null,
    workspaceLinksState: "idle",
    workspaceLinksError: "",
    workspacePath: "",
    workspaceLinking: false,
  };
}

import type { Exact, Model } from "../api/client.js";
export type WorkflowDefinition = Model<"WorkflowDefinition">;
export type WorkflowVersion = Model<"WorkflowDefinitionVersion">;
export type WorkflowRun = Model<"WorkflowRun">;
export type WorkflowEvidence = Model<"WorkflowEvidence">;
export type WorkflowPage = Model<"WorkflowPage">;
export type WorkflowRequest = Exact<
  NonNullable<
    import("../api/generated.js").operations["workflow_apply"]["requestBody"]
  >["content"]["application/json"]
>;
export type WorkflowCommand = WorkflowRequest["command"];
export type WorkflowQuery = {
  workflow?: string;
  run?: string;
  at?: number | bigint;
  after?: string;
  limit?: number;
  attempt?: string;
  evidence_after?: number | bigint;
};
export interface WorkflowTransport {
  page(query: WorkflowQuery, signal?: AbortSignal): Promise<WorkflowPage>;
  apply(request: WorkflowRequest): Promise<Model<"WorkflowCommandOutcome">>;
  status(id: string): Promise<Model<"WorkflowCommandStatus">>;
  events?(signal: AbortSignal, changed: () => void): Promise<void>;
}

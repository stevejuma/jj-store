import type { Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
export type AgentSetup = Model<"AgentExecutionSetup">;
export type AgentTarget = Model<"AgentExecutionTarget">;
export type AgentSession = Model<"AgentExecutionSession">;
export type AgentEvent = Model<"AgentExecutionEvent">;
export type AgentStatus = Model<"AgentExecutionStatus">;
export type AgentAction = NonNullable<
  operations["local_agents_command"]["requestBody"]
>["content"]["application/json"]["action"];
export type AgentOutcome = Model<"AgentExecutionOutcome">;
export interface AgentLaunchContext {
  repository?: string;
  commit?: string;
  text?: string;
}
export function object(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}
export function text(value: unknown): string {
  return typeof value === "string" ? value : "";
}

export type AgentView = NonNullable<
  import("./store-route-types.js").StoreRoute["agentView"]
>;

export function agentLaunchDraft(commit = "") {
  return { targetId: "", runner: "codex", commit };
}

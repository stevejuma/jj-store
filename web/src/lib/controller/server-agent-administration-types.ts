import type { Model } from "../api/client.js";
export type ServerAgentDefinition = Model<"ServerAgentDefinition">;
export type ServerAgentConfiguration = Model<"ServerAgentConfiguration">;
export type ServerAgentReport = Model<"ServerAgentAdministrationStatus">;
export type OperatorAccess = Model<"OperatorAccess">;
export type IdentityBinding = Model<"IdentityBinding">;
export type AdministrationPending =
  | {
      kind: "configuration";
      body: {
        expected_revision: string;
        configuration: ServerAgentConfiguration | null;
      };
    }
  | {
      kind: "identity";
      path: "identities" | "identity-revoke";
      body: Record<string, unknown> & { request_id: string };
    }
  | {
      kind: "grant";
      repository: string;
      body: Record<string, unknown> & { request_id: string };
    };
export const newAgentDefinition = (): ServerAgentDefinition => ({
  setup: {
    id: crypto.randomUUID().replaceAll("-", ""),
    name: "",
    avatar: "🤖",
    description: "",
    instructions: "",
    runner: "codex",
    preferences: { model: null, effort: null, mode: "read-only" },
    options: {
      inherit_defaults: false,
      provider: null,
      endpoint: null,
      account: null,
      environment: {},
    },
  },
  principals: [],
});

export function withAgentDefinition(
  agents: ServerAgentDefinition[],
  definition: ServerAgentDefinition,
): ServerAgentDefinition[] {
  return agents.map((agent) =>
    agent.setup.id === definition.setup.id ? definition : agent,
  );
}

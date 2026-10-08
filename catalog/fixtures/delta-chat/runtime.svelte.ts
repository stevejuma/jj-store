import type { CommunityMemberModel } from "../../../web/src/lib/ui/community-workspace/model.js";

export const demoAgents: readonly CommunityMemberModel[] = [
  "Codex",
  "Cursor",
  "OpenCode",
].map((name) => ({
  id: `demo-${name.toLowerCase()}`,
  name,
  handle: "",
  initials: name.slice(0, 2),
  kind: "agent",
  avatarUrl: `/agent-icons/${name.toLowerCase()}.svg`,
}));
export const demoAgent = demoAgents[0];
// Catalog-only choices: no live harness catalog, settings or dispatch.
export class DemoRuntime {
  agentId = $state(demoAgent.id);
  model = $state("gpt-6.1-sol");
  thinking = $state("medium");
  models = [
    { value: "gpt-6.1-sol", label: "6.1 Sol" },
    { value: "gpt-6-astra", label: "6 Astra" },
  ];
  thinkingOptions = ["Low", "Medium", "High"].map((label) => ({
    value: label.toLowerCase(),
    label,
  }));
  get agent() {
    return demoAgents.find((agent) => agent.id === this.agentId) ?? demoAgent;
  }
  snapshot() {
    return { agent: this.agent, model: this.model, thinking: this.thinking };
  }
}

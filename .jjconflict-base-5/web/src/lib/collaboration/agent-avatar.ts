import type { Model } from "../api/client.js";

type Integration = Model<"AgentSessionSummary">["integration"];

const integrationAvatars: Partial<Record<Integration, string>> = {
  codex: "/agent-icons/codex.svg",
  cursor: "/agent-icons/cursor.svg",
  open_code: "/agent-icons/opencode.svg",
};

/** Bundled product artwork; source URLs are recorded beside the static assets. */
export function agentAvatarUrl(integration: Integration): string | undefined {
  return integrationAvatars[integration];
}

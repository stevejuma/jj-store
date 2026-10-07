import { agentMentionToken } from "../../community/agent-mentions.js";
export function communityComposerMentionToken(id: string): string | undefined {
  return /^[0-9a-f]{32,64}$/u.test(id) ? `jj-store-member:${id}` : undefined;
}

export function communityComposerMentionInsert(member: {
  id: string;
  name: string;
  acpAgentId?: string;
}): string {
  return `${(member.acpAgentId ? agentMentionToken(member.acpAgentId) : communityComposerMentionToken(member.id)) ?? `@${member.name.trim()}`} `;
}

import { agentMentionMatches as matches } from "@lapismd/jj-conversations";
export {
  agentMentionToken,
  mentionedAgentIds,
} from "@lapismd/jj-conversations";

/** Legacy presentation names setup IDs pubkey; the shared codec keeps them distinct. */
export function agentMentionMatches(value: string) {
  return matches(value).map(({ agentId, value: token, start, end }) => ({
    value: token,
    pubkey: agentId,
    start,
    end,
  }));
}

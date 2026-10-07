/** Exact saved setup identity, independent of display names and member public keys. */
export function agentMentionToken(id: string): string {
  return `jj-store-agent:${Array.from(new TextEncoder().encode(id), (b) => b.toString(16).padStart(2, "0")).join("")};`;
}
export function agentMentionMatches(value: string) {
  return [...value.matchAll(/jj-store-agent:([0-9a-f]{2,512});/gu)].flatMap(
    (m) => {
      if (m[1]!.length % 2) return [];
      try {
        const id = new TextDecoder("utf-8", { fatal: true }).decode(
          Uint8Array.from(m[1]!.match(/../gu)!, (b) => parseInt(b, 16)),
        );
        if (
          !id ||
          id.length > 64 ||
          /[\x00-\x1f\x7f]/u.test(id) ||
          agentMentionToken(id) !== m[0]
        )
          return [];
        return [
          {
            value: m[0],
            pubkey: id,
            start: m.index,
            end: m.index + m[0].length,
          },
        ];
      } catch {
        return [];
      }
    },
  );
}
export function mentionedAgentIds(value: string): string[] {
  return [...new Set(agentMentionMatches(value).map((m) => m.pubkey))];
}

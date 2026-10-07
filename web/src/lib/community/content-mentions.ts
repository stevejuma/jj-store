import { agentMentionMatches } from "./agent-mentions.js";
import type { CommunityMemberModel } from "./workspace-model.js";

export function contentMentionMatches(
  value: string,
): Array<{ value: string; pubkey: string; start: number; end: number }> {
  return [
    ...agentMentionMatches(value),
    ...[...value.matchAll(/jj-store-member:([0-9a-f]{32,64})\b/gu)].map(
      (m) => ({
        value: m[0],
        pubkey: m[1]!,
        start: m.index,
        end: m.index + m[0].length,
      }),
    ),
  ].sort((a, b) => a.start - b.start);
}

export function contentMentionPubkeys(value: string): string[] {
  return [
    ...new Set(
      contentMentionMatches(value)
        .filter((m) => m.value.startsWith("jj-store-member:"))
        .map((m) => m.pubkey),
    ),
  ];
}

export function contentMentionFromHref(
  value: string | undefined,
): { pubkey: string; value: string } | undefined {
  if (value?.startsWith("jj-store-agent:")) {
    const match = agentMentionMatches(value)[0];
    return match?.value === value ? { pubkey: match.pubkey, value } : undefined;
  }
  const id = value
    ? /^jj-store-member:([0-9a-f]{32,64})$/u.exec(value)?.[1]
    : undefined;
  return id ? { pubkey: id, value: value! } : undefined;
}

export function contentMentionLabel(
  id: string,
  value: string,
  members: readonly CommunityMemberModel[],
): string {
  return `@${members.find((m) => m.id === id)?.name ?? id.slice(0, 12)}`;
}

export function contentMentionFallbackMember(
  id: string,
  _value: string,
): CommunityMemberModel {
  return {
    id,
    name: id.slice(0, 12),
    handle: id.slice(0, 12),
    initials: id.slice(0, 2).toUpperCase(),
    presence: "offline",
  };
}

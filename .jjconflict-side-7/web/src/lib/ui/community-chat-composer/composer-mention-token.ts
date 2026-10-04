export function communityComposerMentionToken(id: string): string | undefined {
  return /^[0-9a-f]{32,64}$/u.test(id) ? `jj-store-member:${id}` : undefined;
}

export function communityComposerMentionInsert(member: {
  id: string;
  name: string;
}): string {
  return `${communityComposerMentionToken(member.id) ?? `@${member.name.trim()}`} `;
}

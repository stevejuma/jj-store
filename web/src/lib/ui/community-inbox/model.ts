import type { CommunityMemberModel } from "../community-workspace/model.js";

export interface SessionInboxItem {
  id: string;
  targetId?: string;
  label?: string;
  author: CommunityMemberModel;
  context: string;
  excerpt?: string;
  lastActivityAt?: number;
  status: string;
  agents?: readonly { id: string; name: string }[];
  harness?: string;
  repository?: { id: string; name: string; href: string; label: string };
}

export interface SessionInboxGroup {
  id: string;
  repositoryId?: string;
  pinned?: boolean;
  name: string;
  items: SessionInboxItem[];
}

/** Group loaded chats by exact repository identity, preserving their order. */
export function groupSessionInboxItems(
  items: readonly SessionInboxItem[],
  pinned: readonly string[] = [],
): SessionInboxGroup[] {
  const groups = new Map<string, SessionInboxGroup>();
  const selected = new Set(pinned),
    seen = new Set<string>();
  const pins: SessionInboxGroup = {
    id: "pinned",
    name: "Pinned",
    pinned: true,
    items: [],
  };
  for (const item of items) {
    const identity = item.targetId ?? item.id;
    if (selected.has(identity)) {
      if (!seen.has(identity)) pins.items.push(item);
      seen.add(identity);
      continue;
    }
    const repository = item.repository;
    const id = repository ? `repository:${repository.id}` : "unbound";
    let group = groups.get(id);
    if (!group) {
      group = {
        id,
        repositoryId: repository?.id,
        name: repository?.name ?? "No repository",
        items: [],
      };
      groups.set(id, group);
    }
    group.items.push(item);
  }
  return [...(pins.items.length ? [pins] : []), ...groups.values()];
}

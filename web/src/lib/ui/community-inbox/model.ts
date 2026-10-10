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
  repository?: { id: string; name: string; href: string; label: string };
}

export interface SessionInboxGroup {
  id: string;
  repositoryId?: string;
  name: string;
  items: SessionInboxItem[];
}

/** Group loaded chats by exact repository identity, preserving their order. */
export function groupSessionInboxItems(
  items: readonly SessionInboxItem[],
): SessionInboxGroup[] {
  const groups = new Map<string, SessionInboxGroup>();
  for (const item of items) {
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
  return [...groups.values()];
}

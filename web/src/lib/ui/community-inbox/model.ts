import type { CommunityMemberModel } from "../community-workspace/model.js";

export interface SessionInboxItem {
  id: string;
  author: CommunityMemberModel;
  context: string;
  excerpt?: string;
  lastActivityAt?: number;
  status: string;
  repository?: { href: string; label: string };
}

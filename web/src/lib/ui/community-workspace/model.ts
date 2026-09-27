/** Presentation-only author excerpt from CommunityMemberModel. */
export interface CommunityMemberModel {
  id: string;
  name: string;
  handle: string;
  initials: string;
  status?: string;
  avatarUrl?: string;
  kind?: "agent" | "person";
  presence?: "online" | "away" | "offline";
}

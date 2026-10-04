import type { CommunityMemberModel } from "../../web/src/lib/ui/community-workspace/model.js";

export function inMemoryStoreMember(
  authenticated: boolean,
): CommunityMemberModel {
  return authenticated
    ? {
        id: "maya",
        name: "Maya Chen",
        handle: "maya",
        initials: "MC",
        presence: "online",
        status: "Local fixture account",
      }
    : {
        id: "guest",
        name: "Guest",
        handle: "guest",
        initials: "G",
        presence: "offline",
      };
}

import type { StoreController } from "../../web/src/lib/controller/contracts.js";
import type { CommunityMemberModel } from "../../web/src/lib/community/workspace-model.js";
/** Only the read-only controller inputs consumed by host Settings. */
export function workflowHostFixtureController(
  profileFailure = false,
): StoreController {
  const principal = "02".repeat(16);
  const profile = {
    principal,
    display_name: "Steve",
    description: "",
    avatar: null,
  };
  return {
    state: {
      repositories: [
        {
          id: "03".repeat(16),
          name: "Demo source",
          permission: "admin",
          principal,
        },
        {
          id: "04".repeat(16),
          name: "Demo home",
          permission: "admin",
          principal,
        },
      ],
      graph: { revisions: [] },
    },
    directory: {
      state: {
        profile: { principal, details: { display_name: "Steve" } },
        profiles: { [principal]: profile },
        urls: {},
      },
      available: () => true,
      loadProfiles: async () => {
        if (profileFailure) throw Error("offline");
      },
      member(id: string, fallback: CommunityMemberModel) {
        return id === principal
          ? { ...fallback, name: "Steve", initials: "ST" }
          : fallback;
      },
    },
    execution: {
      value: {
        setups: [{ id: "demo-agent", name: "Demo agent", avatar: "🤖" }],
      },
      targets: [
        {
          id: "demo-target",
          repository: "03".repeat(16),
          name: "Demo attachment",
          folder: "/private/code",
        },
      ],
      ensure: async () => true,
    },
  } as unknown as StoreController;
}

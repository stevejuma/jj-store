import type { StoreController } from "../../controller/contracts.js";
import type { CommunityNavigationModel } from "../community-navigation/model.js";
export function withThreadNavigation(
  controller: StoreController,
  model: CommunityNavigationModel,
): CommunityNavigationModel {
  if (!controller.threads) return model;
  return {
    ...model,
    sections: [
      ...model.sections,
      {
        id: "threads",
        label: "Threads",
        items: [
          { id: "threads", label: "Browse threads", kind: "threads" },
          ...controller.threads.threads.map((t) => ({
            id: `thread:${controller.state.route.repository}:${t.value.thread_id}`,
            label: t.value.title,
            kind: "channel" as const,
            thread: t.value.thread_id,
          })),
        ],
      },
    ],
  };
}

import type { StoreController } from "../../controller/contracts.js";
import { threadInboxRows } from "./thread-inbox.js";
import type { CommunityNavigationModel } from "../community-navigation/model.js";
export function withThreadNavigation(
  controller: StoreController,
  model: CommunityNavigationModel,
): CommunityNavigationModel {
  const rows = threadInboxRows(controller);
  return {
    ...model,
    sections: model.sections
      .filter((s) => s.id !== "threads")
      .map((section) =>
        section.id !== "repositories"
          ? section
          : {
              ...section,
              action: undefined,
              items: section.items.map((item) => ({
                ...item,
                children: rows
                  .filter((row) => row.repository?.id === item.id)
                  .map((row) => {
                    const [home, thread] = row.targetId!.split(":");
                    return {
                      id: `thread:${home}:${thread}`,
                      label: row.context,
                      kind: "channel" as const,
                      repository: home,
                      thread,
                      description: item.label,
                    };
                  }),
                initiallyExpanded: rows.some(
                  (row) =>
                    row.repository?.id === item.id &&
                    row.targetId ===
                      `${controller.state.route.repository}:${controller.state.route.threadId}`,
                ),
              })),
            },
      ),
  };
}

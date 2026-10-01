import type { StoreController } from "../../controller/contracts.js";
import type {
  CommunityNavigationItemModel,
  StorePage,
} from "../community-navigation/model.js";
export function selectStoreNavigation(
  controller: StoreController,
  item: CommunityNavigationItemModel,
) {
  if (item.kind === "repository") controller.actions.openRepository(item.id);
  else if (
    item.kind === "project" &&
    controller.projects &&
    item.home &&
    item.project
  )
    void controller.projects.openProject(item.home, item.project);
  else if (item.kind === "more-projects" && controller.projects && item.home)
    void controller.projects.navigation.more(item.home);
  else if (item.kind === "retry-projects" && controller.projects)
    void controller.projects.refresh();
  else if (item.kind === "projects" && controller.projects)
    void controller.projects.openHub();
  else if (
    [
      "branches",
      "log",
      "oplog",
      "files",
      "agents",
      "reviews",
      "projects",
    ].includes(item.kind)
  )
    controller.actions.selectPage(item.kind as StorePage);
}

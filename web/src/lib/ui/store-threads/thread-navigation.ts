import type { StoreController } from "../../controller/contracts.js";
import { supportsRepositoryThreads } from "../../controller/repository-thread-navigation.svelte.js";
import type {
  CommunityNavigationModel,
  CommunityNavigationItemModel,
} from "../community-navigation/model.js";
export function withThreadNavigation(
  controller: StoreController,
  model: CommunityNavigationModel,
): CommunityNavigationModel {
  const eligible = controller.state.repositories.filter(
    supportsRepositoryThreads,
  );
  const current =
    eligible.find((repo) => repo.id === controller.state.repository.id) ??
    eligible[0];
  return {
    ...model,
    sections: model.sections
      .filter((section) => section.id !== "threads")
      .map((section) =>
        section.id !== "repositories"
          ? section
          : {
              ...section,
              action:
                current && (controller.threads || controller.threadNavigation)
                  ? {
                      id: "browse-threads",
                      label: "Browse threads",
                      kind: "threads",
                      repository: current.id,
                    }
                  : undefined,
              items: section.items.map((item) => {
                const repo = eligible.find((repo) => repo.id === item.id);
                if (
                  !repo ||
                  !(controller.threads || controller.threadNavigation)
                )
                  return item;
                const entry = controller.threadNavigation?.entries[repo.id];
                const connected =
                  repo.id === controller.state.repository.id
                    ? controller.threads
                    : undefined;
                const threads = entry?.threads ?? connected?.threads ?? [];
                const children: CommunityNavigationItemModel[] = [
                  {
                    id: `threads:${repo.id}`,
                    label: "Browse threads",
                    kind: "threads",
                    repository: repo.id,
                    description: repo.name,
                  },
                  ...threads.map((thread) => ({
                    id: `thread:${repo.id}:${thread.value.thread_id}`,
                    label: thread.value.title,
                    kind: "channel" as const,
                    thread: thread.value.thread_id,
                    repository: repo.id,
                    description: repo.name,
                  })),
                ];
                if (entry?.cursor)
                  children.push({
                    id: `more-threads:${repo.id}`,
                    label: `More threads in ${repo.name}`,
                    kind: "more-threads",
                    repository: repo.id,
                  });
                if (entry?.error)
                  children.push({
                    id: `retry-threads:${repo.id}`,
                    label: `Retry threads in ${repo.name}`,
                    kind: "retry-threads",
                    repository: repo.id,
                  });
                return {
                  ...item,
                  children,
                  initiallyExpanded:
                    controller.state.route.page === "threads" &&
                    controller.state.route.repository === repo.id,
                };
              }),
            },
      ),
  };
}

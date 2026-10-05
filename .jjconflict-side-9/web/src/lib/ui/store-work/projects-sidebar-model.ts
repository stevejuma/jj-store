import type { CommunityNavigationModel } from "../community-navigation/model.js";
import type { ProjectHome } from "../../controller/projects-hub-types.js";
import type { StoreRepositorySummary } from "../../controller/contracts.js";
export const projectSidebarId = (home: string, project: string) =>
  `project:${home}:${project}`;
export function projectsSidebarModel(
  homes: readonly ProjectHome[],
  repositories: readonly StoreRepositorySummary[],
  enabled: boolean,
): CommunityNavigationModel {
  const projects = homes.flatMap((home) => [
    ...home.projects.map((project) => ({
      id: projectSidebarId(home.repository.id, project.id),
      label: project.title,
      kind: "project" as const,
      home: home.repository.id,
      project: project.id,
      children: Object.entries(project.repositories)
        .filter(([, active]) => active)
        .map(([id]) => ({
          id,
          label:
            repositories.find((repo) => repo.id === id)?.name ??
            `Repository ${id.slice(0, 12)} · separate access required`,
          kind: "repository" as const,
        })),
    })),
    ...(home.error
      ? [
          {
            id: `retry:${home.repository.id}`,
            label: `Retry projects in ${home.repository.name}`,
            kind: "retry-projects" as const,
          },
        ]
      : []),
    ...(home.cursor
      ? [
          {
            id: `more:${home.repository.id}`,
            home: home.repository.id,
            label: `More projects in ${home.repository.name}`,
            kind: "more-projects" as const,
          },
        ]
      : []),
  ]);
  return {
    primaryItems: enabled
      ? [{ id: "projects", label: "Projects", kind: "projects" }]
      : [],
    sections: [
      ...(enabled
        ? [{ id: "projects", label: "Your projects", items: projects }]
        : []),
      {
        id: "repositories",
        label: "Repositories",
        items: repositories.map((repo) => ({
          id: repo.id,
          label: repo.name,
          kind: "repository" as const,
        })),
      },
    ],
  };
}

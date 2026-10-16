import type { TagListItem } from "@lapismd/design-core/shadcn/tag-list";
import type { RepositoryLogRevision } from "../../projects/repository-log.js";

export function commitTags(revision?: RepositoryLogRevision): TagListItem[] {
  return [
    ...(revision?.bookmarks ?? [])
      .filter((bookmark) => bookmark.added && !bookmark.removed)
      .map((bookmark) => ({
        id: `bookmark:${JSON.stringify([bookmark.name, bookmark.remote])}`,
        label: bookmark.remote
          ? `${bookmark.name}@${bookmark.remote}`
          : bookmark.name,
        variant: "secondary" as const,
      })),
    ...(revision?.workspaces ?? []).map((workspace) => ({
      id: `workspace:${workspace}`,
      label: workspace,
      tone: "success" as const,
    })),
  ];
}

import type { RepositoryDirectoryEntry } from "../../controller/projects-repositories.svelte.js";
import type { CommunityMemberModel } from "@lapismd/lapis-community/components/community-message";
import type { Model } from "../../api/client.js";

export function repositoryActivity(
  entries: readonly RepositoryDirectoryEntry[],
  id: string,
  project?: { home: string; id: string },
) {
  const own = entries.find((entry) => entry.repository.id === id)?.summary;
  const tasks = entries.flatMap((entry) =>
    (entry.summary?.tasks ?? []).filter(
      (task) =>
        (task.repository ?? entry.repository.id) === id &&
        (!project ||
          (entry.repository.id === project.home &&
            task.project === project.id)),
    ),
  );
  const dates = [
    ...(own?.updatedAtMillis !== null && own?.updatedAtMillis !== undefined
      ? [own.updatedAtMillis]
      : []),
    ...tasks.map((task) => task.actor.updated_at_millis),
  ];
  return {
    commits: own?.commits ?? 0,
    reviews: own?.threads.length ?? 0,
    tasks: tasks.length,
    partial: Boolean(
      own?.commitsPartial ||
      own?.reviewsPartial ||
      entries.some(
        (entry) =>
          entry.loading ||
          entry.error ||
          !entry.summary ||
          entry.summary.tasksPartial,
      ),
    ),
    updatedAtMillis: dates.length
      ? dates.reduce((max, value) => (value > max ? value : max))
      : null,
  };
}

export function repositoryContributor(
  signature: Model<"GraphSignature"> | null | undefined,
): CommunityMemberModel {
  const name = signature?.name || signature?.email || "No recorded contributor";
  return {
    id: signature?.email || name,
    name,
    handle: signature?.email || "",
    initials: signature
      ? name
          .split(/\s+/)
          .slice(0, 2)
          .map((part) => part[0])
          .join("")
          .toUpperCase()
      : "?",
  };
}

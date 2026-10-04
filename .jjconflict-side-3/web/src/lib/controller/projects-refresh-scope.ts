import type { FreshnessChange } from "../api/application-events.js";
import type {
  ProjectsHubContext,
  ProjectsHubController,
  WorkBrowseTransport,
} from "./projects-hub-types.js";
import type { WorkController } from "./work-types.js";
/** Installation changes require discovery; semantic repository notices retain independent windows. */
export function projectsRefreshScope(changes?: readonly FreshnessChange[]) {
  const affected =
    changes && !changes.some((change) => change.repository === null)
      ? new Set(changes.map((change) => change.repository!))
      : undefined;
  const work =
    changes && affected
      ? new Set(
          changes
            .filter((change) => ["repository", "work"].includes(change.domain))
            .map((change) => change.repository!),
        )
      : undefined;
  return { affected, work };
}
export async function refreshProjectSources(options: {
  changes?: readonly FreshnessChange[];
  context: ProjectsHubContext;
  sources: Map<string, WorkBrowseTransport>;
  works: Map<string, WorkController>;
  views: { clear(): void };
  navigation: ProjectsHubController["navigation"];
  current(): boolean;
  restore(refresh: boolean, affected?: ReadonlySet<string>): Promise<void>;
}) {
  const { affected, work } = projectsRefreshScope(options.changes);
  if (!affected) await options.context.discover?.();
  options.views.clear();
  for (const [id, controller] of options.works)
    if (!work || work.has(id)) controller.invalidate?.();
  if (!options.current()) return;
  if (!options.changes)
    await Promise.allSettled(
      [...options.sources]
        .filter(([id]) => !work || work.has(id))
        .map(([, source]) => source.refresh?.()),
    );
  if (!work || work.size) await options.navigation.load(true, work);
  if (options.current()) await options.restore(true, affected);
}

export async function refreshDirtyLocalProjects(
  context: ProjectsHubContext,
  affected: ReadonlySet<string>,
  transport: (id: string) => WorkBrowseTransport,
) {
  await Promise.all(
    context
      .repositories()
      .filter(
        (repo) => affected.has(repo.id) && repo.profile === "git_sha1_jj045",
      )
      .map((repo) => transport(repo.id).refresh?.()),
  );
}

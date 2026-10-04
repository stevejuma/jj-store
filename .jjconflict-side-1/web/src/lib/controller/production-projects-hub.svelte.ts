import { ProjectsDirtiness } from "./projects-dirtiness.js";
import {
  pendingProjectCommands,
  projectProjectionAccepted,
  invalidateProjectViews,
} from "./projects-command-state.js";
import {
  refreshProjectSources,
  refreshDirtyLocalProjects,
} from "./projects-refresh-scope.js";
import { createProjectHomeReader } from "./projects-hub-homes.js";
import { projectHubHomes } from "./projects-hub-homes.js";
import { createProjectsAgents } from "./projects-agents.svelte.js";
import { restoreProjectAuxiliary } from "./projects-auxiliary-restore.js";
import { createProjectsWorkRegistry } from "./projects-work-registry.svelte.js";
import { reloadProjectHomeWindow } from "./projects-home-read.js";
import { createProjectsReviews } from "./projects-reviews.svelte.js";
import { createProjectsNavigation } from "./projects-navigation.svelte.js";
import { createProjectsRepositories } from "./projects-repositories.svelte.js";
import { createProjectsActivity } from "./projects-activity.svelte.js";
import {
  createProjectsViewCache,
  projectsBrowseQuery,
} from "./projects-view-cache.js";
import { projectsRouteActions } from "./projects-route-actions.js";
import type {
  ProjectHome,
  ProjectsHubController,
  ProjectsHubContext,
} from "./projects-hub-types.js";

export function createProjectsHub(
  context: ProjectsHubContext,
): ProjectsHubController {
  let homes = $state<ProjectHome[]>([]),
    loading = $state(false),
    error = $state(""),
    refreshing = $state(false),
    refreshGeneration = 0,
    generation = 0;
  let reads = new AbortController();
  const scroll = new Map<string, number>();
  const views = createProjectsViewCache();
  const dirty = new ProjectsDirtiness();
  const { sources, works, selections, transport, work } =
    createProjectsWorkRegistry(context, () => {
      views.clear();
      context.changed();
    });
  function denied(home: string) {
    works.get(home)?.reset();
    views.clear();
  }
  function report(message: string) {
    error = message;
  }
  const navigation = createProjectsNavigation({
    repositories: context.repositories,
    transport,
    denied,
  });
  const reviews = createProjectsReviews({
    source: transport,
    denied,
  });
  const agents = createProjectsAgents({
    source: transport,
    route: context.route,
    push: context.push,
    response: context.agentResponse,
    denied: () => views.clear(),
  });
  const repositoryDirectory = createProjectsRepositories({ source: transport });
  const activity = createProjectsActivity({ source: transport });
  const read = createProjectHomeReader(
    transport,
    () => reads.signal,
    () => generation,
    (id) => {
      works.get(id)?.reset();
      views.clear();
    },
  );
  async function restore(refresh = false, affected?: ReadonlySet<string>) {
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation,
      route = context.route();
    const pending = dirty.capture(route);
    if (pending.affected.size) {
      affected = new Set([...(affected ?? []), ...pending.affected]);
      refresh = true;
    }
    loading = true;
    error = "";
    if (route.workTab === "agents")
      agents.prepare(
        JSON.stringify([
          route.repository,
          route.projectId,
          route.workHome,
          route.workRepository,
        ]),
      );
    for (const [id, controller] of works) {
      if (!context.repositories().some((r) => r.id === id)) {
        controller.reset();
        works.delete(id);
        sources.delete(id);
        selections.delete(id);
      }
    }
    const cached = refresh ? undefined : views.get(route);
    const selection = projectHubHomes(
      route,
      context.repositories(),
      homes,
      cached,
    );
    const repositories = selection.repositories;
    homes = selection.homes;
    try {
      if (pending.affected.size)
        await refreshDirtyLocalProjects(context, pending.affected, transport);
      if (ticket !== generation) return;
      if (route.projectId && !repositories.length) {
        if (route.workTab === "agents") agents.reset();
        error = "Home repository unavailable. Separate access is required.";
        return;
      }
      if (route.projectId) {
        selections.set(route.repository, {
          projectId: route.projectId,
          taskId: route.taskId ?? null,
          commentId: route.commentId ?? null,
        });
        await work(route.repository).restore(
          refresh && (!affected || pending.work.has(route.repository)),
        );
      }
      if (ticket !== generation) return;
      context.activate?.(route.projectId ? route.repository : null);
      if (
        (route.taskId ||
          ["agents", "reviews", "repositories", "activity"].includes(
            route.workTab ?? "",
          )) &&
        (await restoreProjectAuxiliary({
          route,
          repositories: context.repositories(),
          detail: route.projectId ? work(route.repository).state : null,
          activity,
          agents,
          navigation,
          directory: repositoryDirectory,
          reviews,
          refresh,
          affected,
          current: () => ticket === generation,
          report,
        }))
      )
        return;
      const query = projectsBrowseQuery(route);
      if (cached) return;
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            if (affected && !affected.has(home.repository.id) && home.snapshot)
              continue;
            await reloadProjectHomeWindow(
              home,
              (more) => read(home, query, ticket, more),
              refresh,
              () => ticket === generation,
            );
          }
        }),
      );
    } finally {
      if (ticket === generation) {
        loading = false;
        views.save(route, homes);
        if (!error)
          pending.accept((domain, id) =>
            projectProjectionAccepted(domain, id, route, works, {
              agents: agents.homes,
              reviews: reviews.homes,
              activity: activity.homes,
              directory: repositoryDirectory.entries,
              work: homes,
            }),
          );
      }
    }
  }
  return {
    agents,
    activity,
    navigation,
    reviews,
    repositories: repositoryDirectory,
    scroll,
    get commands() {
      return pendingProjectCommands(works);
    },
    get homes() {
      return homes;
    },
    get loading() {
      return loading || refreshing;
    },
    get error() {
      return error;
    },
    get route() {
      return context.route();
    },
    get detail() {
      return context.route().projectId &&
        context.repositories().some((r) => r.id === context.route().repository)
        ? works.get(context.route().repository)
        : undefined;
    },
    restore,
    invalidate(changes) {
      dirty.record(
        changes,
        context.repositories().map((repo) => repo.id),
      );
      invalidateProjectViews(views, works, changes);
    },
    async refresh(changes) {
      if (changes)
        dirty.record(
          changes,
          context.repositories().map((repo) => repo.id),
        );
      error = "";
      refreshing = true;
      reads.abort();
      const ticket = ++generation,
        refreshTicket = ++refreshGeneration;
      try {
        await refreshProjectSources({
          context,
          sources,
          works,
          views,
          navigation,
          restore,
          changes,
          current: () =>
            ticket === generation && context.route().page === "projects",
        });
      } catch (cause) {
        if (refreshTicket === refreshGeneration)
          error =
            cause instanceof Error
              ? cause.message
              : "Repository discovery failed";
      } finally {
        if (refreshTicket === refreshGeneration) refreshing = false;
      }
    },
    transport,
    work,
    async more(id) {
      const home = homes.find((h) => h.repository.id === id),
        query = projectsBrowseQuery(context.route());
      if (home?.cursor && query && !home.loading)
        await read(home, query, generation, true);
      views.save(context.route(), homes);
    },
    ...projectsRouteActions(context, restore, report),
    reset() {
      generation++;
      refreshGeneration++;
      refreshing = false;
      reads.abort();
      loading = false;
      homes = [];
      error = "";
      for (const w of works.values()) w.reset();
      navigation.reset();
      reviews.reset();
      repositoryDirectory.reset();
      activity.reset();
      agents.reset();
      sources.clear();
      works.clear();
      selections.clear();
      views.clear();
      scroll.clear();
      dirty.clear();
    },
  };
}

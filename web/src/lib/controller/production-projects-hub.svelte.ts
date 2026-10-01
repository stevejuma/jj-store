import { createProjectsWorkRegistry } from "./projects-work-registry.svelte.js";
import { readProjectHome } from "./projects-home-read.js";
import { createProjectsReviews } from "./projects-reviews.svelte.js";
import { createProjectsNavigation } from "./projects-navigation.svelte.js";
import { createProjectsRepositories } from "./projects-repositories.svelte.js";
import { createProjectsActivity } from "./projects-activity.svelte.js";
import { projectsActivityScope } from "./projects-activity-scope.js";
import {
  createProjectsViewCache,
  projectsBrowseQuery,
} from "./projects-view-cache.js";
import { projectsRouteActions } from "./projects-route-actions.js";
import type {
  ProjectHome,
  ProjectsHubController,
  ProjectsHubContext,
  WorkBrowseQuery,
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
  const { sources, works, selections, transport, work } =
    createProjectsWorkRegistry(context);
  const navigation = createProjectsNavigation({
    repositories: context.repositories,
    transport,
    denied: (home) => {
      works.get(home)?.reset();
      views.clear();
    },
  });
  const reviews = createProjectsReviews({
    source: transport,
    denied: (home) => {
      works.get(home)?.reset();
      views.clear();
    },
  });
  const repositoryDirectory = createProjectsRepositories({ source: transport });
  const activity = createProjectsActivity({ source: transport });
  function read(
    home: ProjectHome,
    query: WorkBrowseQuery,
    ticket: number,
    more = false,
  ) {
    return readProjectHome(
      {
        transport,
        signal: reads.signal,
        current: () => ticket === generation,
        denied: (id) => {
          works.get(id)?.reset();
          views.clear();
        },
      },
      home,
      query,
      more,
    );
  }
  async function restore(refresh = false) {
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation,
      route = context.route();
    loading = true;
    error = "";
    for (const [id, controller] of works) {
      if (!context.repositories().some((r) => r.id === id)) {
        controller.reset();
        works.delete(id);
        sources.delete(id);
        selections.delete(id);
      }
    }
    const cached = refresh ? undefined : views.get(route);
    const repositories = context
      .repositories()
      .filter((r) =>
        route.projectId
          ? r.id === route.repository
          : !route.workHome || r.id === route.workHome,
      );
    homes =
      cached ??
      repositories.map((repository) => ({
        ...(homes.find((h) => h.repository.id === repository.id) ?? {
          repository,
          projects: [],
          tasks: [],
          snapshot: null,
          cursor: null,
          total: 0,
          loading: false,
          error: "",
        }),
        repository,
      }));
    try {
      if (route.projectId && !repositories.length) {
        error = "Home repository unavailable. Separate access is required.";
        return;
      }
      if (route.projectId) {
        selections.set(route.repository, {
          projectId: route.projectId,
          taskId: route.taskId ?? null,
        });
        await work(route.repository).restore(refresh);
      }
      if (ticket !== generation) return;
      context.activate?.(route.projectId ? route.repository : null);
      if (route.workTab === "activity" || route.taskId) {
        const detail = route.projectId ? work(route.repository).state : null;
        try {
          await activity.load(
            projectsActivityScope(
              context.repositories(),
              route,
              detail?.project,
              detail?.task,
            ),
            refresh,
            route.snapshotOperationId
              ? { [route.repository]: route.snapshotOperationId }
              : {},
          );
        } catch (cause) {
          activity.reset();
          error =
            cause instanceof Error ? cause.message : "Activity unavailable";
          return;
        }
        if (ticket !== generation || route.workTab === "activity") return;
      }
      if (route.workTab === "repositories") {
        const project = route.projectId
          ? work(route.repository).state.project
          : null;
        if (route.projectId && !project) {
          repositoryDirectory.reset();
          error = "Project unavailable. Refresh or return to Projects.";
          return;
        }
        await repositoryDirectory.load(context.repositories(), refresh);
        return;
      }
      if (route.workTab === "reviews") {
        const project = route.projectId
          ? work(route.repository).state.project
          : null;
        if (route.projectId && !project) {
          reviews.reset();
          error = "Project unavailable. Refresh or return to Projects.";
          return;
        }
        await reviews.load(
          route,
          context
            .repositories()
            .filter((repo) => !project || repo.id in project.repositories),
          refresh,
        );
        return;
      }
      const query = projectsBrowseQuery(route);
      if (cached) return;
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            await read(home, query, ticket);
          }
        }),
      );
    } finally {
      if (ticket === generation) {
        loading = false;
        views.save(route, homes);
      }
    }
  }
  return {
    activity,
    navigation,
    reviews,
    repositories: repositoryDirectory,
    scroll,
    get commands() {
      return [...works]
        .filter(([, work]) => work.state.pending)
        .map(([home, work]) => ({ home, work }));
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
    async refresh() {
      error = "";
      refreshing = true;
      reads.abort();
      const ticket = ++generation,
        refreshTicket = ++refreshGeneration;
      try {
        await context.discover?.();
        views.clear();
        if (ticket !== generation || context.route().page !== "projects")
          return;
        await navigation.load(true);
        if (ticket !== generation || context.route().page !== "projects")
          return;
        await restore(true);
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
    ...projectsRouteActions(context, restore, (message) => {
      error = message;
    }),
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
      sources.clear();
      works.clear();
      selections.clear();
      views.clear();
      scroll.clear();
    },
  };
}

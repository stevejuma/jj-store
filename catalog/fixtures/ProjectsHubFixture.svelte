<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import type { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type {
    StoreRoute,
    StoreController,
  } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    fixtureTransport,
    homeA,
    homeB,
    projectId,
    childId,
  } from "./projects-hub-fixture.js";
  let {
    layoutStorage = createMemoryStoryStorage(),
    detail = false,
    projectDetail = false,
    longRepositoryDescriptions = false,
    failedHome = false,
    localReviews = false,
    failedReviews = false,
    agents = false,
    readonly = false,
    mobile = false,
    sharedRepository = false,
    loading = false,
    discoveryLoading = false,
    bootstrapLoading = false,
    deferredRefresh = false,
    variedProjectMetadata = false,
    longTaskDetail = false,
    repositoryProjects = false,
    execution,
  }: {
    layoutStorage?: Storage;
    detail?: boolean;
    projectDetail?: boolean;
    longRepositoryDescriptions?: boolean;
    failedHome?: boolean;
    localReviews?: boolean;
    failedReviews?: boolean;
    agents?: boolean;
    readonly?: boolean;
    mobile?: boolean;
    sharedRepository?: boolean;
    loading?: boolean;
    discoveryLoading?: boolean;
    bootstrapLoading?: boolean;
    deferredRefresh?: boolean;
    variedProjectMetadata?: boolean;
    longTaskDetail?: boolean;
    repositoryProjects?: boolean;
    execution?: AgentExecution;
  } = $props();
  let refreshPending = $state(false);
  let finishRefresh: (() => void) | undefined;
  let refreshGate = Promise.resolve();
  const base = untrack(() => createInMemoryStoreController("projects"));
  let route = $state<StoreRoute>({
    ...initialStoreRoute(),
    page: "projects",
    projectsHub: true,
    workTab: "projects",
    ...untrack(() =>
      repositoryProjects ? { projectsHub: false, repository: homeA } : {},
    ),
  });
  const repositories = $derived([
    {
      id: homeA,
      name: "Parser repository",
      permission: readonly ? ("read" as const) : ("admin" as const),
      principal: "01".repeat(16),
    },
    {
      id: homeB,
      name: "UI repository",
      permission: "read" as const,
      profile: localReviews ? ("git_sha1_jj045" as const) : undefined,
    },
  ]);
  const hub = createProjectsHub({
    agentResponse: (home) => {
      const response = createInMemoryStoreController("agents");
      return {
        controller: {
          ...response,
          get state() {
            return {
              ...response.state,
              repository: {
                ...response.state.repository,
                ...repositories.find((r) => r.id === home),
                id: home,
              },
            };
          },
        },
        async load(id) {
          response.actions.selectAgentSession(id);
        },
        reset() {
          response.actions.selectAgentSession(null);
        },
      };
    },
    repositories: () => repositories,
    route: () => route,
    push: (next) => (route = next),
    source: (home) => {
      const source = fixtureTransport(home, failedHome, sharedRepository);
      if (localReviews && home === homeB) source.reviews = undefined;
      const page = source.page;
      source.page = async (query, signal) => {
        if (refreshPending) await refreshGate;
        const result = await page(query, signal);
        return longTaskDetail && query.task
          ? {
              ...result,
              tasks: result.tasks.map((task) => ({
                ...task,
                body: "Task details remain readable while the title and comment composer stay in view.\n\n".repeat(
                  60,
                ),
              })),
            }
          : result;
      };
      const activity = source.activity;
      if (activity)
        source.activity = async (query, signal) => {
          if (refreshPending) await refreshGate;
          return activity(query, signal);
        };
      const reviews = source.reviews;
      if (reviews)
        source.reviews = async (cursor, signal) => {
          if (refreshPending) await refreshGate;
          return reviews(cursor, signal);
        };
      const directoryRead = source.directory;
      if (directoryRead)
        source.directory = async (signal) => {
          if (refreshPending) await refreshGate;
          return directoryRead(signal);
        };
      const browse = source.browse;
      source.browse = async (query, signal) => {
        if (refreshPending) await refreshGate;
        const page = await browse(query, signal);
        if (!variedProjectMetadata || !page.projects.length) return page;
        const first = page.projects[0]!;
        return {
          ...page,
          total_matches: 3,
          projects: [
            first,
            ...[1, 2].map((index) => ({
              ...first,
              id: String(index + 5).repeat(32),
              title:
                index === 1
                  ? "Long project title and description"
                  : "Small project",
              repositories: index === 1 ? {} : first.repositories,
              actor: {
                ...first.actor,
                updated_at_millis: Date.now() - index * 60000,
              },
              task_counts: {
                active: index === 1 ? 4095 : 0,
                done: index,
                total: index === 1 ? 4096 : 2,
              },
            })),
          ],
        };
      };
      if (longRepositoryDescriptions && source.directory) {
        const directory = source.directory;
        source.directory = async (signal) => {
          const summary = await directory(signal);
          return {
            ...summary,
            description:
              `${summary.description} with repository navigation and native projects, tasks, reviews and activity. `.repeat(
                4,
              ),
          };
        };
      }
      if (failedReviews)
        source.reviews = async () => {
          throw new Error("Reviews temporarily unavailable");
        };
      if (!loading) return source;
      const pending = () => new Promise<never>(() => {});
      return {
        ...source,
        browse: pending,
        directory: pending,
        reviews: pending,
        agents: pending,
        activity: pending,
        ...(detail || projectDetail ? { page: pending } : {}),
      };
    },
    discover: () =>
      discoveryLoading ? new Promise<void>(() => {}) : Promise.resolve(),
    changed: () => {},
  });
  const controller: StoreController = {
    ...base,
    projects: hub,
    get execution() {
      return execution;
    },
    get state() {
      return {
        ...base.state,
        route,
        repositories,
        accessMode: "authenticated" as const,
        installationReady: true,
        ...(bootstrapLoading
          ? {
              status: {
                ...base.state.status,
                connection: "connecting" as const,
                repositories: "loading" as const,
              },
            }
          : {}),
      };
    },
    actions: {
      ...base.actions,
      refresh: () => hub.refresh(),
      async openReview(repository, thread) {
        base.actions.selectPage("reviews");
        base.actions.selectReviewThread(thread);
        route = { ...base.state.route, repository, projectsHub: false };
      },
      selectReviewThread(id) {
        base.actions.selectReviewThread(id);
        route = { ...route, reviewThreadId: id };
      },
    },
  };
  $effect(() => {
    if (bootstrapLoading) return;
    void untrack(() => hub.navigation.load());
    void untrack(() =>
      discoveryLoading
        ? hub.refresh()
        : repositoryProjects
          ? hub.restore()
          : detail
            ? hub.openProject(homeA, projectId, childId)
            : projectDetail
              ? hub.openProject(homeA, projectId)
              : hub.openHub(agents ? "agents" : "projects"),
    );
  });
  onDestroy(() => hub.reset());
</script>

<StoreRepositoryApplication
  {controller}
  {layoutStorage}
  initiallyCollapsed={false}
  displayMode={mobile ? "mobile" : "desktop"}
/>
{#if deferredRefresh}<Button
    onclick={() => {
      refreshGate = new Promise<void>((resolve) => (finishRefresh = resolve));
      refreshPending = true;
      void hub.refresh();
    }}>Pause project refresh</Button
  >
  <Button
    onclick={() => {
      refreshPending = false;
      finishRefresh?.();
    }}>Finish project refresh</Button
  >{/if}

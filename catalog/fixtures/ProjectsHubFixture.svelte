<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type {
    StoreRoute,
    StoreController,
  } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import {
    fixtureTransport,
    homeA,
    homeB,
    projectId,
    childId,
  } from "./projects-hub-fixture.js";
  let {
    detail = false,
    failedHome = false,
    readonly = false,
    mobile = false,
    sharedRepository = false,
    loading = false,
    repositoryProjects = false,
  }: {
    detail?: boolean;
    failedHome?: boolean;
    readonly?: boolean;
    mobile?: boolean;
    sharedRepository?: boolean;
    loading?: boolean;
    repositoryProjects?: boolean;
  } = $props();
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
    { id: homeB, name: "UI repository", permission: "read" as const },
  ]);
  const hub = createProjectsHub({
    repositories: () => repositories,
    route: () => route,
    push: (next) => (route = next),
    source: (home) => {
      const source = fixtureTransport(home, failedHome, sharedRepository);
      if (!loading) return source;
      const pending = () => new Promise<never>(() => {});
      return {
        ...source,
        browse: pending,
        directory: pending,
        reviews: pending,
        activity: pending,
      };
    },
    changed: () => {},
  });
  const controller: StoreController = {
    ...base,
    projects: hub,
    get state() {
      return {
        ...base.state,
        route,
        repositories,
        accessMode: "authenticated" as const,
        installationReady: true,
      };
    },
    actions: { ...base.actions, refresh: () => hub.refresh() },
  };
  $effect(() => {
    void untrack(() => hub.navigation.load());
    void untrack(() =>
      repositoryProjects
        ? hub.restore()
        : detail
          ? hub.openProject(homeA, projectId, childId)
          : hub.openHub(),
    );
  });
  onDestroy(() => hub.reset());
</script>

<StoreRepositoryApplication
  {controller}
  initiallyCollapsed={false}
  displayMode={mobile ? "mobile" : "desktop"}
/>

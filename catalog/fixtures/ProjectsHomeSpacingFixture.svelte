<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    fixtureTransport,
    homeA,
    homeB,
    project,
  } from "./projects-hub-fixture.js";
  const base = createInMemoryStoreController("projects");
  const storage = createMemoryStoryStorage();
  const repositories = [homeA, homeB].map((id, index) => ({
    id,
    name: `Project home ${index + 1}`,
    permission: "read" as const,
    principal: project.actor.created_by,
  }));
  let route = $state<StoreRoute>({
    ...initialStoreRoute(),
    page: "projects",
    projectsHub: true,
    workTab: "projects",
  });
  const hub = createProjectsHub({
    repositories: () => repositories,
    route: () => route,
    push: (next) => (route = next),
    changed: () => {},
    source: (home) => {
      const source = fixtureTransport(home);
      const browse = source.browse;
      source.browse = async (query, signal) => {
        const page = await browse(query, signal);
        if (query.kind === "tasks") return page;
        const summary = {
          ...project,
          excerpt: project.body,
          task_counts: { active: 0, done: 0, total: 0 },
        };
        const projects =
          home === homeA
            ? [{ ...summary, title: "Personal project" }]
            : [1, 2, 3].map((index) => ({
                ...summary,
                id: String(index + 4).repeat(32),
                title: `Team project ${index}`,
              }));
        return { ...page, projects, total_matches: projects.length };
      };
      return source;
    },
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
  };
  $effect(() => {
    void untrack(() => hub.openHub("projects"));
  });
  onDestroy(() => hub.reset());
</script>

<StoreRepositoryApplication
  {controller}
  layoutStorage={storage}
  initiallyCollapsed
/>

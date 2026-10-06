<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../web/src/lib/controller/contracts.js";
  import type { ProjectsTab } from "../../web/src/lib/controller/projects-hub-types.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    fixtureTransport,
    homeA,
    homeB,
    projectId,
  } from "./projects-hub-fixture.js";
  let {
    tab = "projects",
    failedRead = "browse",
    selectedProject = false,
    activityContextError = false,
  }: {
    tab?: ProjectsTab;
    failedRead?: "browse" | "reviews" | "agents" | "activity" | "directory";
    selectedProject?: boolean;
    activityContextError?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController("projects"));
  const repositories = [
    { id: homeA, name: "Parser repository", permission: "admin" as const },
    { id: homeB, name: "UI repository", permission: "read" as const },
  ];
  let route = $state<StoreRoute>({
    ...initialStoreRoute(),
    page: "projects",
    projectsHub: true,
    workTab: untrack(() => tab),
  });
  let failing = $state(untrack(() => !activityContextError)),
    reads = $state(0),
    paused = $state(false),
    hiddenResult = $state("");
  let finish: (() => void) | undefined;
  let gate = Promise.resolve();
  const hub = createProjectsHub({
    repositories: () => repositories,
    route: () => route,
    push: (next) => (route = next),
    changed: () => {},
    source: (home) => {
      const source = fixtureTransport(home);
      if (activityContextError) {
        const activity = source.activity!;
        source.activity = async (...args) => {
          const page = await activity(...args);
          return {
            ...page,
            commits: page.commits.map((row) => ({
              ...row,
              agents: { ...row.agents, error: "Agent context unavailable" },
            })),
          };
        };
      }
      const read = source[failedRead]!;
      // Wrap only the selected transport read; all other homes/data stay native.
      Object.assign(source, {
        [failedRead]: async (...args: never[]) => {
          if (home === homeB) {
            reads++;
            if (paused) await gate;
            if (failing)
              throw new Error(`${failedRead} temporarily unavailable`);
          }
          return (read as (...args: never[]) => Promise<unknown>)(...args);
        },
      });
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
    actions: { ...base.actions, refresh: () => hub.refresh() },
  };
  $effect(() => {
    void untrack(() =>
      selectedProject ? hub.openProject(homeB, projectId) : hub.openHub(tab),
    );
  });
  onDestroy(() => hub.reset());
  function retry() {
    void hub.refresh();
  }
</script>

<StoreRepositoryApplication
  {controller}
  layoutStorage={createMemoryStoryStorage()}
  initiallyCollapsed={false}
/>
<aside aria-label="Feedback fixture controls">
  <output aria-label="Read attempts">{reads}</output>
  <output aria-label="Hidden review result">{hiddenResult}</output>
  <Button
    onclick={() => {
      failing = false;
      retry();
    }}>Recover read</Button
  >
  <Button
    onclick={() => {
      failing = true;
      retry();
    }}>Fail read</Button
  >
  <Button
    onclick={() => {
      paused = true;
      gate = new Promise<void>((resolve) => (finish = resolve));
      retry();
    }}>Pause read</Button
  >
  <Button
    onclick={() => {
      paused = false;
      finish?.();
    }}>Finish read</Button
  >
  <Button
    onclick={async () => {
      failing = false;
      await hub.reviews.load(
        { ...route, workTab: "reviews" },
        repositories,
        true,
      );
      hiddenResult = "recovered";
    }}>Recover hidden reviews</Button
  >
  <Button
    onclick={async () => {
      failing = true;
      await hub.reviews.load(
        { ...route, workTab: "reviews" },
        repositories,
        true,
      );
      hiddenResult = "failed";
    }}>Fail hidden reviews</Button
  >
</aside>

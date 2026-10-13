<script lang="ts">
  import { untrack } from "svelte";
  import StoreWorkflows from "../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { fixtureTransport } from "./projects-hub-fixture.js";
  import { workflowFixture, repositories, home, code } from "./workflows.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { workflowCollaboration } from "./workflow-collaboration.js";
  let {
    unknown = false,
    shell = false,
    noHome = false,
    loading = false,
  }: {
    unknown?: boolean;
    shell?: boolean;
    noHome?: boolean;
    loading?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController("workflows"));
  const source = untrack(() => workflowFixture(unknown));
  const emptySource = untrack(() => workflowFixture(unknown, code));
  const projects = createProjectsHub({
    repositories: () => repositories,
    route: () => base.state.route,
    push: () => {},
    source: (selected) => ({
      ...fixtureTransport(selected),
      async browse(query, signal) {
        const page = await fixtureTransport(selected).browse(query, signal);
        if ("work_root" in page.snapshot)
          page.snapshot.work_root = noHome ? null : "37".repeat(32);
        return page;
      },
      workflows: loading
        ? { ...source, page: async () => new Promise(() => {}) }
        : selected === home
          ? source
          : emptySource,
    }),
    changed: () => {},
  });
  const controller: StoreController = {
    ...base,
    acquireRepository: workflowCollaboration,
    projects,
    get state() {
      return {
        ...base.state,
        repositories: repositories.map((r) => ({
          ...r,
          principal: "22".repeat(16),
        })),
        repository: repositories[0]!,
        accessMode: "local_trusted" as const,
      };
    },
  };
</script>

{#if shell}<StoreRepositoryApplication
    {controller}
    initiallyCollapsed={false}
  />{:else}<StoreWorkflows {controller} />{/if}

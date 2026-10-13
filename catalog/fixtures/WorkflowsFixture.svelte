<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreWorkflows from "../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createProjectsHub } from "../../web/src/lib/controller/production-projects-hub.svelte.js";
  import { fixtureTransport } from "./projects-hub-fixture.js";
  import { workflowFixture, repositories, home, code } from "./workflows.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { workflowCollaboration } from "./workflow-collaboration.js";
  import { createRepositoryThreadNavigation } from "../../web/src/lib/controller/repository-thread-navigation.svelte.js";
  import { createDirectory } from "../../web/src/lib/controller/directory.svelte.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { fixtureExecutionSetup } from "./agent-execution-status.js";
  let {
    unknown = false,
    choiceFailures = false,
    shell = false,
    noHome = false,
    loading = false,
  }: {
    unknown?: boolean;
    choiceFailures?: boolean;
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
  const threadNavigation = createRepositoryThreadNavigation({
    repositories: () => repositories,
    credential: () => null,
    source: (repository) => {
      const reader = workflowCollaboration(repository, choiceFailures);
      return { list: reader.threads!.list, close: reader.close };
    },
  });
  const directory = createDirectory({
    available: () => true,
    token: () => null,
    fetcher: async (input) => {
      const url = String(input);
      if (choiceFailures && url.includes("profiles?"))
        throw Error("Profile lookup failed");
      const details = {
        display_name: "Server author",
        description: "",
        email: "author@example.invalid",
        avatar: "🧑",
      };
      const value = url.includes("profiles?")
        ? [{ principal: "22".repeat(16), ...details }]
        : url.endsWith("profile")
          ? { principal: "22".repeat(16), revision: "77".repeat(32), details }
          : [];
      return new Response(JSON.stringify(value), {
        headers: { "content-type": "application/json" },
      });
    },
  });
  const execution = new AgentExecution(
    () => true,
    async () => {
      if (choiceFailures) throw Error("Agent lookup failed");
      return new Response(
        JSON.stringify({
          version: 1,
          runtime: "ready",
          revision: 1,
          setups: [fixtureExecutionSetup(null)],
          sessions: [],
          runners: [],
          live_agents: [],
          conversations: [],
        }),
        { headers: { "content-type": "application/json" } },
      );
    },
  );
  onDestroy(() => {
    directory.destroy();
    execution.destroy();
    threadNavigation.clear();
  });
  const controller: StoreController = {
    ...base,
    acquireRepository: (repository) =>
      workflowCollaboration(repository, choiceFailures),
    threadNavigation,
    directory,
    execution,
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

<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { createStoreRouteHistory } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import { storeRouteToHash } from "../../web/src/lib/controller/store-route-codec.js";
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
  import { consoleExample, type ConsoleExample } from "./workflow-console.js";
  let {
    unknown = false,
    choiceFailures = false,
    shell = false,
    noHome = false,
    loading = false,
    initialHash = "#/workflows",
    historyFailure,
    pendingRun = false,
    multiStep = false,
    discovering = false,
    consoleMode,
    logReadDelay = 0,
    logReadFailure = false,
    downloadFailure = false,
  }: {
    unknown?: boolean;
    choiceFailures?: boolean;
    shell?: boolean;
    noHome?: boolean;
    loading?: boolean;
    initialHash?: string;
    historyFailure?: "list" | "detail";
    pendingRun?: boolean;
    multiStep?: boolean;
    discovering?: boolean;
    consoleMode?: ConsoleExample;
    logReadDelay?: number;
    logReadFailure?: boolean;
    downloadFailure?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController("workflows"));
  let location = $state(untrack(() => initialHash));
  let locations = untrack(() => [location]),
    locationIndex = 0,
    notify = () => {};
  const navigation = untrack(() =>
    createStoreRouteHistory({
      hash: () => location,
      push: (hash) => {
        locations = [...locations.slice(0, locationIndex + 1), hash];
        locationIndex++;
        location = hash;
      },
      replace: (hash) => {
        locations[locationIndex] = hash;
        location = hash;
      },
      listen(listener) {
        notify = listener;
        return () => {
          notify = () => {};
        };
      },
      go(delta) {
        locationIndex = Math.max(
          0,
          Math.min(locations.length - 1, locationIndex + delta),
        );
        location = locations[locationIndex]!;
        notify();
      },
    }),
  );
  let failed = false;
  const source = untrack(() => {
    const base = workflowFixture(unknown, home, pendingRun, multiStep);
    return consoleMode ? consoleExample(base, consoleMode) : base;
  });
  let logReads = $state(0),
    lastCommand = $state(""),
    logFailed = false,
    downloadFailed = false;
  const emptySource = untrack(() => workflowFixture(unknown, code));
  const projects = createProjectsHub({
    repositories: () => repositories,
    route: () => navigation.route,
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
          ? {
              ...source,
              async apply(request) {
                lastCommand = JSON.stringify(request.command);
                return source.apply(request);
              },
              async page(query, signal) {
                if (query.attempt && query.run) {
                  logReads++;
                  if (logReadDelay)
                    await new Promise((resolve) =>
                      setTimeout(resolve, logReadDelay),
                    );
                  if (
                    logReadFailure &&
                    !logFailed &&
                    query.evidence_after !== undefined
                  ) {
                    logFailed = true;
                    throw Error("Demo step read failed");
                  }
                  if (
                    downloadFailure &&
                    !downloadFailed &&
                    query.evidence_after === undefined
                  ) {
                    downloadFailed = true;
                    throw Error("Demo download failed");
                  }
                }
                if (
                  !failed &&
                  ((historyFailure === "list" && query.workflow) ||
                    (historyFailure === "detail" && query.run))
                ) {
                  failed = true;
                  throw Error("Workflow history lookup failed");
                }
                return source.page(query, signal);
              },
            }
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
    navigation.destroy();
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
    actions: {
      ...base.actions,
      async openRoute(route) {
        navigation.push(route);
      },
      async navigate(delta) {
        navigation.navigate(delta);
      },
    },
    get state() {
      return {
        ...base.state,
        route: navigation.route,
        canNavigateBack: navigation.canBack,
        canNavigateForward: navigation.canForward,
        status: {
          ...base.state.status,
          repositories: discovering ? ("loading" as const) : ("ready" as const),
        },
        repositories: discovering
          ? []
          : repositories.map((r) => ({
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
<span class="sr-only" aria-label="Workflow location"
  >{storeRouteToHash(navigation.route)}</span
>
<span class="sr-only" aria-label="Log read count">{logReads}</span>
{#if lastCommand}<span class="sr-only" aria-label="Last workflow command"
    >{lastCommand}</span
  >{/if}
<button class="sr-only" onclick={() => navigation.navigate(-1)}
  >Fixture back</button
>
<button class="sr-only" onclick={() => navigation.navigate(1)}
  >Fixture forward</button
>

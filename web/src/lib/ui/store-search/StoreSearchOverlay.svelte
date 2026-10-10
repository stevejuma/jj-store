<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import type {
    StoreController,
    StoreSearchResult,
  } from "../../controller/contracts.js";
  import type { GlobalSearchScope } from "../../controller/global-search-types.js";
  import type { CommunitySearchRow } from "../community-search-command/model.js";
  import StoreSearchCommand from "./StoreSearchCommand.svelte";
  import StoreThreadCreate from "../store-threads/StoreThreadCreate.svelte";
  import StoreWorkProjectForm from "../store-work/StoreWorkProjectForm.svelte";
  import StoreWorkTaskForm from "../store-work/StoreWorkTaskForm.svelte";
  import { currentSearchScope } from "./search-context.js";
  let {
    controller,
    open = $bindable(false),
  }: { controller: StoreController; open?: boolean } = $props();
  let query = $state(""),
    scope = $state<GlobalSearchScope>();
  let timer: ReturnType<typeof setTimeout> | undefined;
  let pending = $state<(() => void) | undefined>();
  let createThreadOpen = $state(false),
    createProjectOpen = $state(false),
    createTaskOpen = $state(false);
  const global = $derived(controller.globalSearch);
  const work = $derived(controller.projects?.detail ?? controller.work);
  const context = $derived(currentSearchScope(controller));
  const sectionKinds = {
    all: "repository",
    threads: "thread",
    projects: "project",
    tasks: "task",
    code: "file",
  } as const satisfies Record<
    NonNullable<GlobalSearchScope["section"]>,
    CommunitySearchRow["kind"]
  >;
  const scopeKind = $derived(
    context.task ? "task" : sectionKinds[context.section ?? "all"],
  );
  const homePermission = $derived(
    controller.state.repositories.find(
      (r) => r.id === controller.projects?.route.repository,
    )?.permission ?? controller.state.repository.permission,
  );
  const workWritable = $derived(
    ["write", "admin"].includes(homePermission ?? ""),
  );
  const writable = $derived(
    ["write", "admin"].includes(controller.state.repository.permission ?? ""),
  );
  function cancelTimer() {
    if (timer) clearTimeout(timer);
    timer = undefined;
  }
  function run(value: string) {
    if (global) void global.run(value, scope);
    else void controller.actions.search(value);
  }
  function updateQuery(value: string) {
    query = value;
    cancelTimer();
    timer = setTimeout(() => {
      timer = undefined;
      run(value);
    }, 250);
  }
  function changeScope(next?: GlobalSearchScope) {
    scope = next;
    cancelTimer();
    run(query);
  }
  function choose(action: () => void) {
    cancelTimer();
    pending = action;
    open = false;
  }
  function selectCode(result: StoreSearchResult) {
    choose(() => {
      void controller.actions.selectSearchResult(result);
    });
  }
  $effect(() => {
    const shown = open;
    untrack(() => {
      cancelTimer();
      query = "";
      scope = undefined;
      if (shown) run("");
      else global?.clear();
    });
  });
  onDestroy(() => {
    cancelTimer();
    global?.clear();
  });
  const actions = $derived([
    {
      id: "threads",
      label: "Threads",
      kind: "thread" as const,
      run: () =>
        choose(() => {
          void controller.actions.openThreads?.();
        }),
    },
    {
      id: "browse-projects",
      label: "Browse projects",
      kind: "project" as const,
      run: () =>
        choose(() => {
          void controller.projects?.openHub("projects");
        }),
    },
    {
      id: "browse-tasks",
      label: "Browse tasks",
      kind: "task" as const,
      run: () =>
        choose(() => {
          void controller.projects?.openHub("tasks");
        }),
    },
    {
      id: "browse-repositories",
      label: "Browse repositories",
      kind: "repository" as const,
      run: () =>
        choose(() => {
          void controller.projects?.openHub("repositories");
        }),
    },
    ...(writable && controller.threads
      ? [
          {
            id: "create-thread",
            label: "Create new thread",
            kind: "thread" as const,
            create: true,
            run: () =>
              choose(() => {
                createThreadOpen = true;
              }),
          },
        ]
      : []),
    ...(workWritable && (controller.projects?.detail ?? controller.work)
      ? [
          {
            id: "create-project",
            label: "Create new project",
            kind: "project" as const,
            create: true,
            run: () =>
              choose(() => {
                createProjectOpen = true;
              }),
          },
        ]
      : []),
    ...(workWritable &&
    (controller.projects?.detail ?? controller.work)?.state.project
      ? [
          {
            id: "create-task",
            label: "Create new task",
            kind: "task" as const,
            create: true,
            run: () =>
              choose(() => {
                createTaskOpen = true;
              }),
          },
        ]
      : []),
  ]);
  const sections = $derived(
    (["threads", "projects", "tasks", "code"] as const).map((section) => ({
      id: `scope:${section}`,
      label: `Search ${section}`,
      kind: sectionKinds[section],
      run: () =>
        changeScope({
          label: section[0]!.toUpperCase() + section.slice(1),
          section: section as GlobalSearchScope["section"],
        }),
    })),
  );
</script>

<StoreSearchCommand
  bind:open
  {query}
  search={controller.state.search}
  now={controller.state.now}
  onQueryChange={updateQuery}
  onSelect={selectCode}
  rows={global?.results}
  status={global?.status}
  errors={global?.errors}
  scopeLabel={context.label}
  {scopeKind}
  activeScopeLabel={scope?.label}
  onScope={() => changeScope(context)}
  onClearScope={() => changeScope()}
  {actions}
  {sections}
  canLoadMore={global?.canLoadMore}
  onSelectRow={global
    ? (id) => {
        const result = global.results.find((r) => r.id === id);
        if (result)
          choose(() => {
            void global.select(result);
          });
      }
    : undefined}
  onOpenChangeComplete={(shown) => {
    if (!shown && pending) {
      const action = pending;
      pending = undefined;
      requestAnimationFrame(action);
    }
  }}
  onLoadMore={() => {
    if (global) void global.more();
    else void controller.actions.loadMore("search");
  }}
/>
{#if createThreadOpen && controller.threadWorkspaces}<StoreThreadCreate
    {controller}
    onClose={() => (createThreadOpen = false)}
  />{/if}
{#if work}<Dialog.Root bind:open={createProjectOpen}
    ><Dialog.Content preventScroll={false}
      ><Dialog.Header
        ><Dialog.Title>New project</Dialog.Title><Dialog.Description
          >Create a project in this home.</Dialog.Description
        ></Dialog.Header
      ><StoreWorkProjectForm
        {work}
        disabled={!workWritable}
        onSaved={() => (createProjectOpen = false)}
      /></Dialog.Content
    ></Dialog.Root
  >
  {#if work.state.project}<Dialog.Root bind:open={createTaskOpen}
      ><Dialog.Content preventScroll={false}
        ><Dialog.Header
          ><Dialog.Title>New task</Dialog.Title><Dialog.Description
            >Create a task in this project.</Dialog.Description
          ></Dialog.Header
        ><StoreWorkTaskForm
          {work}
          project={work.state.project}
          repositories={controller.state.repositories}
          disabled={!workWritable}
          onSaved={() => (createTaskOpen = false)}
        /></Dialog.Content
      ></Dialog.Root
    >{/if}
{/if}

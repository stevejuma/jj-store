<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { untrack } from "svelte";
  import { projectsViewKey } from "../../controller/projects-view-cache.js";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreProjectsViewport from "./StoreProjectsViewport.svelte";
  import StoreWorkOwnershipGroups from "./StoreWorkOwnershipGroups.svelte";
  import StoreProjectsReviews from "./StoreProjectsReviews.svelte";
  import StoreProjectsAgents from "./StoreProjectsAgents.svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import StoreProjectsActivity from "./StoreProjectsActivity.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import ListTodo from "@lucide/svelte/icons/list-todo";
  import MessageSquare from "@lucide/svelte/icons/message-square";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ProjectsTab } from "../../controller/projects-hub-types.js";
  import StoreProjectsDirectory from "./StoreProjectsDirectory.svelte";
  import StoreProjectsLoading from "./StoreProjectsLoading.svelte";
  import {
    projectsBusy,
    projectsInitialLoading,
  } from "./projects-initial-loading.js";
  import StoreWorkTaskRows from "./StoreWorkTaskRows.svelte";
  import StoreWorkRepositories from "./StoreWorkRepositories.svelte";
  import StoreWorkTaskDetail from "./StoreWorkTaskDetail.svelte";
  import StoreWorkSubtasks from "./StoreWorkSubtasks.svelte";
  import "./community-work-rows.css";
  import "./community-work-layout.css";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import StoreProjectsNavigation from "./StoreProjectsNavigation.svelte";
  import StoreProjectsActions from "./StoreProjectsActions.svelte";
  import StoreWorkDialog from "./StoreWorkDialog.svelte";
  import StoreWorkStatus from "./StoreWorkStatus.svelte";
  import "./store-work.css";
  import "./store-projects.css";
  let { controller }: { controller: StoreController } = $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route),
    work = $derived(hub.detail),
    project = $derived(work?.state.project),
    task = $derived(work?.state.task);
  const busy = $derived(projectsBusy(controller));
  const repositoryName = (id: string) =>
    controller.state.repositories.find((r) => r.id === id)?.name ?? id;
  const writable = $derived(
    controller.state.accessMode === "local_trusted" ||
      controller.state.repositories.some(
        (r) =>
          r.id === route.repository &&
          (r.permission === "write" || r.permission === "admin"),
      ),
  );
  const canCreate = $derived(
    controller.state.accessMode === "local_trusted" ||
      controller.state.repositories.some(
        (r) => r.permission === "write" || r.permission === "admin",
      ),
  );
  let layout = $state<"grid" | "list">("list");
  let element = $state<HTMLElement | null>(null);
  const scrollKey = $derived(projectsViewKey(route));
  $effect(() => {
    const key = scrollKey,
      ready = !hub.loading,
      root = element;
    if (ready && root)
      untrack(() => root.scrollTo(0, hub.scroll.get(key) ?? 0));
  });
  $effect(() => {
    const root = element,
      key = scrollKey;
    if (!root) return;
    const save = () => {
      if (!hub.loading) hub.scroll.set(key, root.scrollTop);
    };
    root.addEventListener("scroll", save);
    return () => root.removeEventListener("scroll", save);
  });
  let dialog = $state<
      "createProject" | "editProject" | "createTask" | "editTask" | null
    >(null),
    initialParent = $state("");
  let linkOpen = $state(false);
  $effect(() => {
    // A dialog and its draft belong to the selected project and tab.
    route.repository;
    route.projectId;
    route.workTab;
    linkOpen = false;
  });
  const tabs = $derived<ProjectsTab[]>(
    project
      ? ["activity", "tasks", "reviews", "agents", "repositories"]
      : ["activity", "projects", "tasks", "reviews", "agents", "repositories"],
  );
  const tab = $derived(
    route.workTab ?? (route.projectId ? "tasks" : "projects"),
  );
  function open(mode: typeof dialog, parent = "") {
    initialParent = parent;
    dialog = mode;
  }
  function changeTab(value: ProjectsTab) {
    void hub.filter({ workTab: value });
  }
</script>

<section
  class="store-projects community-projects"
  aria-label="Projects and tasks"
  aria-busy={busy}
  data-task-detail={task ? "" : undefined}
  data-agents-view={tab === "agents" ? "" : undefined}
>
  <StoreProjectsViewport
    taskDetail={!!task}
    bounded={tab === "agents"}
    bind:viewportRef={element}
  >
    <StoreNotice
      noticeKey="projects"
      title="Projects request failed"
      message={hub.error}
      pending={hub.loading}
    />
    {#if work}<StoreWorkStatus {work} />{/if}
    {#each hub.commands.filter((command) => command.work !== work) as command (command.home)}
      <section
        aria-label={`Pending command in ${repositoryName(command.home)}`}
      >
        <h2>Pending command in {repositoryName(command.home)}</h2>
        <StoreWorkStatus work={command.work} />
      </section>
    {/each}
    <Tabs.Root
      value={tab}
      onValueChange={(value) => changeTab(value as ProjectsTab)}
      class="store-projects__tab-root"
    >
      {#if !task}<StoreProjectsNavigation
          {controller}
          {project}
          {tabs}
          {tab}
        />{/if}
      {#if !task}<header class="community-projects__surface-header">
          <div>
            {#if tab === "tasks"}<ListTodo
                aria-hidden="true"
              />{:else if tab === "reviews"}<MessageSquare
                aria-hidden="true"
              />{:else if tab === "repositories"}<FolderGit
                aria-hidden="true"
              />{:else if tab === "agents"}<BotIcon
                aria-hidden="true"
              />{:else if tab === "activity"}<HistoryIcon
                aria-hidden="true"
              />{:else}<FolderKanban aria-hidden="true" />{/if}
            <h1>{tab[0]!.toUpperCase() + tab.slice(1)}</h1>
          </div>
          <StoreProjectsActions
            {controller}
            {project}
            {work}
            {writable}
            {canCreate}
            {busy}
            {tab}
            bind:layout
            bind:linkOpen
            onOpen={open}
          />
        </header>{/if}
      <Tabs.Content value={tab} aria-label={`${tab} view`}>
        {#if projectsInitialLoading(controller)}<StoreProjectsLoading
            {route}
            {layout}
          />
        {:else if tab === "activity"}<StoreProjectsActivity {controller} />
        {:else if tab === "agents"}<StoreProjectsAgents {controller} />
        {:else if tab === "reviews"}<StoreProjectsReviews
            {controller}
            {layout}
          />
        {:else if project && work}
          {#if tab === "repositories"}{#key project.id}<StoreWorkRepositories
                bind:open={linkOpen}
                {project}
                entries={hub.repositories.entries}
                home={route.repository}
                query={route.workQuery ?? ""}
                {layout}
                {work}
                repositories={controller.state.repositories}
                {writable}
                busy={hub.loading}
                onOpenRepository={controller.actions.openRepository}
              />{/key}
          {:else if task}{#key task.id}<StoreWorkTaskDetail
                bind:viewportRef={element}
                {work}
                repositories={controller.state.repositories}
                principal={controller.state.repositories.find(
                  (r) => r.id === route.repository,
                )?.principal}
                {writable}
                busy={hub.loading}
                onClose={() =>
                  void hub.openProject(route.repository, task.project)}
                onSelectTask={(id) =>
                  void hub.openProject(route.repository, task.project, id)}
                onOpenRepository={controller.actions.openRepository}
                onRefresh={() => controller.actions.refresh()}
                onEdit={() => open("editTask")}
                onSubtask={() => open("createTask", task.id)}
              >
                {#snippet subtasks()}<StoreWorkSubtasks
                    {hub}
                    home={route.repository}
                    {task}
                    snapshot={work.state.snapshot}
                    {repositoryName}
                  />{/snippet}
              </StoreWorkTaskDetail>{/key}
          {:else}{#each hub.homes as home}<StoreWorkOwnershipGroups
                rows={home.tasks}
                home={home.repository.id}
                isMine={(row) =>
                  Boolean(
                    home.repository.principal &&
                    row.actor.created_by === home.repository.principal,
                  )}
                kind="tasks"
                Icon={ListTodo}
                >{#snippet children(tasks)}<StoreWorkTaskRows
                    {layout}
                    {tasks}
                    {repositoryName}
                    hierarchy
                    select={(row) =>
                      void hub.openProject(
                        route.repository,
                        row.project,
                        row.id,
                      )}
                  />{/snippet}</StoreWorkOwnershipGroups
              >
              {#if !home.tasks.length && !home.loading && !home.error}<StoreWorkEmpty
                  kind="tasks"
                  title="No matching tasks in this project"
                  description="Change the filters or create a task to begin."
                />{/if}
              {#if home.error}<p role="alert">{home.error}</p>{/if}
              {#if home.cursor}<Button
                  variant="outline"
                  disabled={home.loading}
                  onclick={() => hub.more(home.repository.id)}
                  >More tasks</Button
                >{/if}
            {/each}{/if}
        {:else if !route.projectId}<StoreProjectsDirectory
            {controller}
            {layout}
          />{/if}
      </Tabs.Content>
    </Tabs.Root>
  </StoreProjectsViewport>
</section>
{#if dialog}<StoreWorkDialog
    {controller}
    mode={dialog}
    home={project ? route.repository : ""}
    {project}
    task={dialog === "editTask" ? task : null}
    {initialParent}
    onClose={() => (dialog = null)}
  />{/if}

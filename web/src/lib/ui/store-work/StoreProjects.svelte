<script lang="ts">
  import { untrack } from "svelte";
  import { projectsViewKey } from "../../controller/projects-view-cache.js";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import StoreWorkLayoutControls from "./StoreWorkLayoutControls.svelte";
  import StoreProjectsFilterControls from "./StoreProjectsFilterControls.svelte";
  import StoreWorkOwnershipGroups from "./StoreWorkOwnershipGroups.svelte";
  import StoreProjectsReviews from "./StoreProjectsReviews.svelte";
  import StoreProjectsActivity from "./StoreProjectsActivity.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import ListTodo from "@lucide/svelte/icons/list-todo";
  import MessageSquare from "@lucide/svelte/icons/message-square";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ProjectsTab } from "../../controller/projects-hub-types.js";
  import StoreProjectsDirectory from "./StoreProjectsDirectory.svelte";
  import StoreWorkTaskRows from "./StoreWorkTaskRows.svelte";
  import StoreWorkRepositories from "./StoreWorkRepositories.svelte";
  import StoreWorkTaskDetail from "./StoreWorkTaskDetail.svelte";
  import StoreWorkSubtasks from "./StoreWorkSubtasks.svelte";
  import "./community-work-rows.css";
  import "./community-work-layout.css";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import PencilIcon from "@lucide/svelte/icons/pencil";
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
    const root = element;
    if (!root) return;
    const save = () => {
      if (!hub.loading) hub.scroll.set(scrollKey, root.scrollTop);
    };
    root.addEventListener("scroll", save);
    return () => root.removeEventListener("scroll", save);
  });
  let dialog = $state<
      "createProject" | "editProject" | "createTask" | "editTask" | null
    >(null),
    initialParent = $state("");
  const tabs = $derived<ProjectsTab[]>(
    project
      ? ["activity", "tasks", "reviews", "repositories"]
      : ["activity", "projects", "tasks", "reviews", "repositories"],
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

{#snippet actions()}
  <div class="community-projects__surface-actions">
    {#if tab !== "activity"}<StoreWorkLayoutControls bind:value={layout} />{/if}
    {#key `${project?.id ?? "hub"}:${tab}`}
      <StoreProjectsFilterControls {controller} {project} />
    {/key}
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Refresh projects"
      title="Refresh projects"
      disabled={hub.loading}
      onclick={() => controller.actions.refresh()}
      ><RefreshIcon aria-hidden="true" /></Button
    >
    {#if canCreate && !project && (tab === "projects" || tab === "tasks")}<Button
        variant="ghost"
        size="icon-sm"
        aria-label={tab === "tasks" ? "New task" : "New project"}
        title={tab === "tasks" ? "New task" : "New project"}
        disabled={hub.loading}
        onclick={() => open(tab === "tasks" ? "createTask" : "createProject")}
        ><PlusIcon aria-hidden="true" /></Button
      >{/if}
    {#if project && writable}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="Edit project"
        title="Edit project"
        disabled={hub.loading ||
          work?.state.loading ||
          work?.state.writing ||
          !!work?.state.pending}
        onclick={() => open("editProject")}
        ><PencilIcon aria-hidden="true" /></Button
      ><Button
        variant="ghost"
        size="icon-sm"
        aria-label="New task"
        title="New task"
        disabled={hub.loading ||
          work?.state.loading ||
          work?.state.writing ||
          !!work?.state.pending}
        onclick={() => open("createTask")}
        ><PlusIcon aria-hidden="true" /></Button
      >{/if}
  </div>
{/snippet}

<section
  class="store-projects community-projects"
  aria-label="Projects and tasks"
  aria-busy={hub.loading}
>
  <ScrollArea
    type="hover"
    class="store-projects__scroll"
    bind:viewportRef={element}
  >
    {#if controller.state.status.refreshAvailable}<p role="status">
        Updates available. Refresh projects to view them.
      </p>{/if}
    {#if hub.error}<p role="alert">{hub.error}</p>{/if}
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
      {#if !task}<div class="community-projects__tabs">
          <Tabs.List
            class="community-projects__tab-list"
            aria-label={project ? "Project views" : "Projects views"}
          >
            {#each tabs as value}<Tabs.Trigger
                class="community-projects__tab-trigger"
                {value}>{value[0]!.toUpperCase() + value.slice(1)}</Tabs.Trigger
              >{/each}
          </Tabs.List>
        </div>{/if}
      {#if !task}<header class="community-projects__surface-header">
          <div>
            {#if tab === "tasks"}<ListTodo
                aria-hidden="true"
              />{:else if tab === "reviews"}<MessageSquare
                aria-hidden="true"
              />{:else if tab === "repositories"}<FolderGit
                aria-hidden="true"
              />{:else if tab === "activity"}<HistoryIcon
                aria-hidden="true"
              />{:else}<FolderKanban aria-hidden="true" />{/if}
            <h1>{tab[0]!.toUpperCase() + tab.slice(1)}</h1>
          </div>
          {@render actions()}
        </header>{/if}
      <Tabs.Content value={tab} aria-label={`${tab} view`}>
        {#if tab === "activity"}<StoreProjectsActivity {controller} />
        {:else if tab === "reviews"}<StoreProjectsReviews
            {controller}
            {layout}
          />
        {:else if project && work}
          {#if tab === "repositories"}<StoreWorkRepositories
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
            />
          {:else if task}{#key task.id}<StoreWorkTaskDetail
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
                {#snippet activity()}<StoreProjectsActivity
                    {controller}
                    task
                  />{/snippet}
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
          />{:else if hub.loading}<p role="status">Loading project…</p>{/if}
      </Tabs.Content>
    </Tabs.Root>
  </ScrollArea>
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

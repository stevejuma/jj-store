<script lang="ts">
  // CommunityProjects single-surface tabs, rows and details (Apache-2.0).
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import StoreWorkLayoutControls from "./StoreWorkLayoutControls.svelte";
  import StoreWorkOwnershipGroups from "./StoreWorkOwnershipGroups.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreWorkRepositories from "./StoreWorkRepositories.svelte";
  import type { WorkController } from "../../controller/work-types.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import StoreWorkProjectForm from "./StoreWorkProjectForm.svelte";
  import StoreWorkTaskForm from "./StoreWorkTaskForm.svelte";
  import StoreWorkProjectRows from "./StoreWorkProjectRows.svelte";
  import StoreWorkTaskRows from "./StoreWorkTaskRows.svelte";
  import StoreWorkTaskDetail from "./StoreWorkTaskDetail.svelte";
  import StoreWorkStatus from "./StoreWorkStatus.svelte";
  import "./store-work.css";
  import "./store-projects.css";
  import "./community-work-layout.css";
  let {
    work,
    repositories = [],
    writable = false,
    onOpenRepository,
    initialTab = "tasks",
    principal,
  }: {
    work: WorkController;
    repositories?: readonly StoreRepositorySummary[];
    writable?: boolean;
    onOpenRepository?: (id: string) => void | Promise<void>;
    initialTab?: string;
    principal?: string;
  } = $props();
  const view = $derived(work.state);
  const disabled = $derived(
    !writable || view.loading || view.writing || !!view.pending,
  );
  let layout = $state<"grid" | "list">("list");
  let tab = $state(untrack(() => initialTab));
  let dialog = $state<
    "createProject" | "editProject" | "createTask" | "editTask" | null
  >(null);
  const titles = {
    createProject: "Create project",
    editProject: "Edit project",
    createTask: "Create task",
    editTask: "Edit task",
  };
  const repoName = (id: string) =>
    repositories.find((repo) => repo.id === id)?.name ?? id;
</script>

<section
  class="store-projects community-projects"
  aria-label="Projects and tasks"
  aria-busy={view.loading}
>
  <StoreWorkStatus {work} />
  <Tabs.Root bind:value={tab} class="store-projects__tab-root">
    {#if !view.task || tab !== "tasks"}<div class="community-projects__tabs">
        <Tabs.List
          class="community-projects__tab-list"
          aria-label="Project views"
        >
          {#each ["projects", "tasks", "repositories"] as value}<Tabs.Trigger
              class="community-projects__tab-trigger"
              {value}>{value[0]!.toUpperCase() + value.slice(1)}</Tabs.Trigger
            >{/each}
        </Tabs.List>
      </div>
      <header class="community-projects__surface-header">
        <div>
          {#if tab === "tasks"}<ListTodoIcon
              aria-hidden="true"
            />{:else if tab === "repositories"}<GitBranchIcon
              aria-hidden="true"
            />{:else}<FolderKanbanIcon aria-hidden="true" />{/if}
          <h2>{tab[0]!.toUpperCase() + tab.slice(1)}</h2>
        </div>
        <div class="community-projects__surface-actions">
          {#if !view.task}<StoreWorkLayoutControls bind:value={layout} />{/if}
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Refresh projects"
            title="Refresh projects"
            disabled={view.loading}
            onclick={() => work.refresh()}
            ><RefreshIcon aria-hidden="true" /></Button
          >
          {#if writable}{#if view.project}<Button
                variant="ghost"
                size="icon-sm"
                aria-label="Edit project"
                title="Edit project"
                {disabled}
                onclick={() => (dialog = "editProject")}
                ><PencilIcon aria-hidden="true" /></Button
              >{/if}
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={tab === "projects" ? "New project" : "New task"}
              title={tab === "projects" ? "New project" : "New task"}
              disabled={disabled || (tab !== "projects" && !view.project)}
              onclick={() =>
                (dialog = tab === "projects" ? "createProject" : "createTask")}
              ><PlusIcon aria-hidden="true" /></Button
            >
          {/if}
        </div>
      </header>{/if}
    <Tabs.Content value={tab}>
      {#if tab === "projects"}
        <StoreWorkOwnershipGroups
          rows={view.projects}
          {principal}
          kind="projects"
          Icon={FolderKanbanIcon}
        >
          {#snippet children(projects)}<StoreWorkProjectRows
              {projects}
              {layout}
              select={(p) => {
                void work.select(p.id);
                tab = "tasks";
              }}
            />
          {/snippet}</StoreWorkOwnershipGroups
        >
        {#if !view.projects.length && !view.loading}<StoreWorkEmpty
            title="No projects yet"
            description="Create a project to organize work across repositories."
          />{/if}
        {#if view.projectCursor}<Button
            variant="outline"
            disabled={view.loading}
            onclick={() => work.more("projects")}>More projects</Button
          >{/if}
      {:else if tab === "repositories" && view.project}
        <StoreWorkRepositories
          project={view.project}
          {work}
          {repositories}
          {layout}
          {writable}
          initialManual
          onOpenRepository={onOpenRepository ?? (() => {})}
        />
      {:else if view.task && view.project}
        <StoreWorkTaskDetail
          {work}
          {repositories}
          {writable}
          onEdit={() => (dialog = "editTask")}
          onClose={() => void work.select(view.project!.id)}
          onSelectTask={(id) => void work.select(view.project!.id, id)}
          onRefresh={() => work.refresh()}
          {onOpenRepository}
        />
      {:else if view.project}
        <StoreWorkOwnershipGroups
          rows={view.tasks}
          {principal}
          kind="tasks"
          Icon={ListTodoIcon}
        >
          {#snippet children(tasks)}<StoreWorkTaskRows
              {tasks}
              {layout}
              hierarchy
              repositoryName={repoName}
              select={(t) => void work.select(t.project, t.id)}
            />
          {/snippet}</StoreWorkOwnershipGroups
        >
        {#if !view.tasks.length && !view.loading}<StoreWorkEmpty
            kind="tasks"
            title="No tasks yet"
            description="Create a task in this project to begin."
          />{/if}
        {#if view.taskCursor}<Button
            variant="outline"
            disabled={view.loading}
            onclick={() => work.more("tasks")}>More tasks</Button
          >{/if}
      {:else}<StoreWorkEmpty
          title="Select a project"
          description="Open a project to see its tasks and repositories."
        />{/if}
    </Tabs.Content>
  </Tabs.Root>
</section>
{#if dialog}<Dialog.Root
    open
    onOpenChange={(open) => {
      if (!open && !view.writing) dialog = null;
    }}
  >
    <DialogFrame
      title={titles[dialog]}
      preventScroll={false}
      busy={view.writing}
    >
      {#if dialog === "createProject" || dialog === "editProject"}<StoreWorkProjectForm
          {work}
          project={dialog === "editProject" ? view.project : null}
          {disabled}
        />
      {:else if view.project}<StoreWorkTaskForm
          {work}
          project={view.project}
          task={dialog === "editTask" ? view.task : null}
          {repositories}
          {disabled}
        />{/if}
      <StoreWorkStatus {work} />
    </DialogFrame>
  </Dialog.Root>{/if}

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import type { WorkController } from "../../controller/work-types.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import StoreWorkProjectForm from "./StoreWorkProjectForm.svelte";
  import StoreWorkTaskForm from "./StoreWorkTaskForm.svelte";
  import StoreWorkComments from "./StoreWorkComments.svelte";
  import "./store-work.css";
  import { workHierarchy } from "./work-hierarchy.js";
  let {
    work,
    repositories = [],
    writable = false,
    onOpenRepository,
  }: {
    work: WorkController;
    repositories?: readonly StoreRepositorySummary[];
    writable?: boolean;
    onOpenRepository?: (id: string) => void | Promise<void>;
  } = $props();
  const view = $derived(work.state);
  const disabled = $derived(!writable || view.writing || !!view.pending);
  let link = $state(""),
    createProject = $state(false),
    createTask = $state(false),
    editProject = $state(false),
    editTask = $state(false);
  $effect(() => {
    view.project?.id;
    createTask = false;
    editProject = false;
  });
  $effect(() => {
    view.task?.id;
    editTask = false;
  });
  const repoName = (id: string) =>
    repositories.find((repo) => repo.id === id)?.name ?? id;
</script>

<section
  class="store-work"
  aria-label="Projects and tasks"
  aria-busy={view.loading}
>
  <header class="store-work__toolbar">
    <h2>Projects</h2>
    <Button
      variant="outline"
      disabled={view.loading}
      onclick={() => work.refresh()}>Refresh projects</Button
    >
    {#if writable}<Button
        {disabled}
        onclick={() => {
          createProject = !createProject;
        }}>New project</Button
      >{/if}
  </header>
  {#if view.error}<p role="alert">{view.error}</p>{/if}
  {#if view.message}<p role="status">{view.message}</p>{/if}
  {#if view.pending}<div class="store-work__toolbar">
      <span>Pending request {view.pending.request_id.slice(0, 8)}</span><Button
        disabled={view.writing}
        onclick={() => work.check()}>Check task command</Button
      ><Button disabled={view.writing} onclick={() => work.retry()}
        >Retry task command</Button
      >
    </div>{/if}
  {#if createProject}<StoreWorkProjectForm {work} {disabled} />{/if}
  <div class="store-work__columns">
    <nav aria-label="Projects">
      <ul class="store-work__list">
        {#each view.projects as project (project.id)}<li>
            <Button
              variant="ghost"
              aria-current={view.project?.id === project.id
                ? "page"
                : undefined}
              onclick={() => work.select(project.id)}>{project.title}</Button
            >
          </li>{/each}
      </ul>
      {#if !view.projects.length && !view.loading}<p>No projects yet.</p>{/if}
      {#if view.projectCursor}<Button
          variant="outline"
          disabled={view.loading}
          onclick={() => work.more("projects")}>More projects</Button
        >{/if}
    </nav>
    <div>
      {#if view.project}
        <header class="store-work__toolbar">
          <h3>{view.project.title}</h3>
          {#if writable}<Button
              variant="ghost"
              {disabled}
              onclick={() => {
                editProject = !editProject;
              }}>Edit project</Button
            ><Button
              {disabled}
              onclick={() => {
                createTask = !createTask;
              }}>New task</Button
            >{/if}
        </header>
        <p class="store-work__body">{view.project.body}</p>
        {#if editProject}<StoreWorkProjectForm
            {work}
            project={view.project}
            {disabled}
          />{/if}
        <section aria-label="Linked repositories">
          <h4>Repositories</h4>
          <ul class="store-work__list">
            {#each Object.entries(view.project.repositories) as [id, active]}<li
                class="store-work__toolbar"
              >
                <span>{repoName(id)}{active ? "" : " (retired)"}</span>
                {#if repositories.some((repo) => repo.id === id)}<Button
                    variant="ghost"
                    size="sm"
                    onclick={() => onOpenRepository?.(id)}
                    >Open repository</Button
                  >{:else}<span class="store-work__meta"
                    >Separate access required</span
                  >{/if}
                {#if writable}<Button
                    variant="ghost"
                    size="sm"
                    {disabled}
                    onclick={() =>
                      view.project &&
                      work.submit({
                        command: "set_repository",
                        project: view.project.id,
                        expected_version: view.project.version,
                        repository: id,
                        active: !active,
                      })}>{active ? "Retire link" : "Restore link"}</Button
                  >{/if}
              </li>{/each}
          </ul>
          {#if writable}<form
              class="store-work__toolbar"
              onsubmit={(event) => {
                event.preventDefault();
                if (view.project)
                  void work.submit({
                    command: "set_repository",
                    project: view.project.id,
                    expected_version: view.project.version,
                    repository: link,
                    active: true,
                  });
              }}
            >
              <label
                >Repository ID<Input
                  bind:value={link}
                  required
                  pattern={"[0-9a-f]{32}"}
                  {disabled}
                  list="work-repositories"
                /></label
              >
              <datalist id="work-repositories"
                >{#each repositories as repo}<option value={repo.id}
                    >{repo.name}</option
                  >{/each}</datalist
              >
              <Button type="submit" {disabled}>Link repository</Button>
            </form>{/if}
        </section>
        {#if createTask}<StoreWorkTaskForm
            {work}
            project={view.project}
            {repositories}
            {disabled}
          />{/if}
        <h4>Tasks</h4>
        <ul class="store-work__list" aria-label="Project tasks">
          {#each workHierarchy(view.tasks, (row) => row.parent) as { row: task, depth } (task.id)}<li
              style:margin-inline-start={`${Math.min(depth, 4) * 0.6}rem`}
            >
              <Button
                variant="ghost"
                aria-current={view.task?.id === task.id ? "page" : undefined}
                onclick={() => work.select(task.project, task.id)}
                >{task.title}</Button
              ><span class="store-work__meta">
                {task.state.replaceAll("_", " ")}{task.parent
                  ? ` · Subtask of ${view.tasks.find((row) => row.id === task.parent)?.title ?? task.parent.slice(0, 8)}`
                  : ""}{task.repository
                  ? ` · ${repoName(task.repository)}`
                  : ""}</span
              >
            </li>{/each}
        </ul>
        {#if view.taskCursor}<Button
            variant="outline"
            disabled={view.loading}
            onclick={() => work.more("tasks")}>More tasks</Button
          >{/if}
      {/if}
    </div>
    <article aria-label="Task details">
      {#if view.task && view.project}<header class="store-work__toolbar">
          <h3>{view.task.title}</h3>
          {#if writable}<Button
              variant="ghost"
              {disabled}
              onclick={() => {
                editTask = !editTask;
              }}>Edit task</Button
            >{/if}
        </header>
        <p>{view.task.state.replaceAll("_", " ")}</p>
        <p class="store-work__body">{view.task.body}</p>
        {#if editTask}<StoreWorkTaskForm
            {work}
            project={view.project}
            task={view.task}
            {repositories}
            {disabled}
          />{/if}
        <StoreWorkComments {work} {writable} />
      {:else}<p>Select a task to read its discussion.</p>{/if}
    </article>
  </div>
</section>

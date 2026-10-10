<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import StoreWorkCollection from "./StoreWorkCollection.svelte";
  import StoreWorkProjectRows from "./StoreWorkProjectRows.svelte";
  import StoreProjectsRepositories from "./StoreProjectsRepositories.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreWorkRowsSkeleton from "./StoreWorkRowsSkeleton.svelte";
  import StoreWorkTaskRows from "./StoreWorkTaskRows.svelte";
  let {
    controller,
    layout = "list",
  }: { controller: StoreController; layout?: "grid" | "list" } = $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route);
  const kind = $derived(route.workTab === "tasks" ? "tasks" : "projects");
  const repositoryName = (id: string) =>
    controller.state.repositories.find((repo) => repo.id === id)?.name ?? id;
  const owned = (actor: { created_by: string }, principal?: string) =>
    Boolean(principal && actor.created_by === principal);
  function count(mine: boolean) {
    return hub.homes.reduce(
      (sum, home) =>
        sum +
        home[kind].filter(
          (row) => owned(row.actor, home.repository.principal) === mine,
        ).length,
      0,
    );
  }
</script>

{#if route.workTab === "repositories"}<StoreProjectsRepositories
    {controller}
    {layout}
  />
{:else}
  {#if !hub.homes.length && !hub.loading}<StoreWorkEmpty
      title="No accessible home repositories"
      description="Create or register a repository with the jj-store CLI to start a project."
    />{/if}
  <div class="community-projects__collection-groups">
    {#each [true, false] as mine}
      {#if count(mine)}<StoreWorkCollection
          id={`${kind}:${mine ? "mine" : "other"}`}
          title={mine ? "Mine" : `Other ${kind}`}
          count={count(mine)}
          Icon={kind === "tasks" ? ListTodoIcon : FolderKanbanIcon}
        >
          {#each hub.homes as home (home.repository.id)}
            {#if kind === "tasks"}
              {@const tasks = home.tasks.filter(
                (row) => owned(row.actor, home.repository.principal) === mine,
              )}
              {#if tasks.length}<StoreWorkTaskRows
                  {tasks}
                  {layout}
                  {repositoryName}
                  select={(task) =>
                    void hub.openProject(
                      home.repository.id,
                      task.project,
                      task.id,
                    )}
                />{/if}
            {:else}
              {@const projects = home.projects.filter(
                (row) => owned(row.actor, home.repository.principal) === mine,
              )}
              {#if projects.length}<StoreWorkProjectRows
                  {projects}
                  {layout}
                  select={(project) =>
                    void hub.openProject(home.repository.id, project.id)}
                />{/if}
            {/if}
          {/each}
        </StoreWorkCollection>{/if}
    {/each}
  </div>
  {#each hub.homes.filter((home) => home.loading || home.error || home.cursor) as home (home.repository.id)}
    <section
      class="store-projects__home"
      aria-label={`Home ${home.repository.name}`}
      aria-busy={home.loading}
    >
      <StoreNotice
        scope="projects-reads"
        noticeKey={`project-home:${home.repository.id}`}
        title={`Could not load ${kind} in ${home.repository.name}`}
        message={home.error
          ? `${home.error}${home.snapshot ? " Previous results retained; they may use earlier filters." : ""} Results cover available homes only.`
          : ""}
        pending={home.loading}
        action={{
          id: "retry-home",
          label: "Retry home",
          run: () => hub.refresh(),
        }}
      />
      {#if home.loading && !home.snapshot}<div class="store-projects__loading">
          <StoreWorkRowsSkeleton
            {layout}
            {kind}
            label={`Loading ${home.repository.name} projects and tasks`}
          />
        </div>{/if}
      {#if home.cursor}<Button
          variant="outline"
          disabled={home.loading}
          onclick={() => hub.more(home.repository.id)}
        >
          More {kind} in {home.repository.name}</Button
        >{/if}
    </section>
  {/each}
  {#if hub.homes.length && !hub.loading && hub.homes.every((home) => home.snapshot && !home.total && !home.error)}
    <StoreWorkEmpty
      {kind}
      title={route.workQuery || route.workRepository || route.workHome
        ? "No matching items"
        : `No ${kind} yet`}
      description="Create an item or change the filters to see work across your repositories."
    />{/if}
{/if}

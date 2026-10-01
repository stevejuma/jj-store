<script lang="ts">
  import { untrack } from "svelte";
  import { pretty } from "../../api/client.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ProjectsHubController } from "../../controller/projects-hub-types.js";
  import type { Model } from "../../api/client.js";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreWorkTaskRows from "./StoreWorkTaskRows.svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  let open = $state(false);
  let {
    hub,
    home,
    task,
    snapshot,
    repositoryName,
  }: {
    hub: ProjectsHubController;
    home: string;
    task: Model<"TaskV1">;
    snapshot: unknown;
    repositoryName: (id: string) => string;
  } = $props();
  let rows = $state<Model<"WorkTaskSummary">[]>([]),
    cursor = $state<string | null>(null),
    error = $state(""),
    loading = $state(false),
    generation = 0;
  let reads = new AbortController(),
    pin = "";
  async function load(more = false) {
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation;
    loading = true;
    error = "";
    try {
      const page = await hub.transport(home).browse(
        {
          kind: "tasks",
          project: task.project,
          parent: task.id,
          limit: 50,
          ...(more && cursor ? { cursor } : {}),
        },
        reads.signal,
      );
      if (ticket !== generation) return;
      if (more && pin !== pretty(page.snapshot))
        throw new Error("Snapshot changed. Refresh the list.");
      pin = pretty(page.snapshot);
      rows = more ? [...rows, ...page.tasks] : page.tasks;
      cursor = page.next_cursor;
    } catch (cause) {
      if (ticket === generation)
        error = cause instanceof Error ? cause.message : "Subtasks unavailable";
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  $effect(() => {
    home;
    task.id;
    snapshot;
    untrack(() => void load());
    return () => {
      generation++;
      reads.abort();
    };
  });
</script>

<section aria-label="Subtasks">
  <Collapsible.Root class="community-projects__task-activity" bind:open>
    <Collapsible.Trigger class="community-projects__task-section-trigger"
      ><span>Subtasks</span>
      {#if open}<ChevronDownIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />{:else}<ChevronRightIcon
          class="community-projects__task-section-chevron"
          aria-hidden="true"
        />{/if}
    </Collapsible.Trigger>
    <Collapsible.Content>
      <StoreWorkTaskRows
        tasks={rows}
        {repositoryName}
        select={(row) => void hub.openProject(home, row.project, row.id)}
      />
      {#if !rows.length && !loading && !error}<StoreWorkEmpty
          kind="tasks"
          title="No subtasks yet"
          description="Break this task into smaller tasks with Add subtask."
        />{/if}
      {#if error}<p role="alert">{error}</p>
        <Button onclick={() => load()}>Retry subtasks</Button>{/if}
      {#if cursor}<Button
          variant="outline"
          disabled={loading}
          onclick={() => load(true)}>More subtasks</Button
        >{/if}
    </Collapsible.Content>
  </Collapsible.Root>
</section>

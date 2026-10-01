<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import type { Model } from "../../api/client.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    project,
    task = null,
    repositories,
    disabled = false,
  }: {
    work: WorkController;
    project: Model<"ProjectV1">;
    task?: Model<"TaskV1"> | null;
    repositories: readonly StoreRepositorySummary[];
    disabled?: boolean;
  } = $props();
  let title = $state(""),
    body = $state(""),
    repository = $state(""),
    parent = $state(""),
    status = $state<Model<"TaskStateV1">>("open");
  $effect(() => {
    title = task?.title ?? "";
    body = task?.body ?? "";
    repository = task?.repository ?? "";
    parent = task?.parent ?? "";
    status = task?.state ?? "open";
  });
  async function save(event: SubmitEvent) {
    event.preventDefault();
    const fields = {
      title,
      body,
      repository: repository || null,
      parent: parent || null,
      expected_project_version: project.version,
    };
    await work.submit(
      task
        ? {
            command: "update_task",
            id: task.id,
            expected_version: task.version,
            state: status,
            ...fields,
          }
        : {
            command: "create_task",
            id: workId(),
            project: project.id,
            ...fields,
          },
    );
  }
</script>

<form
  onsubmit={save}
  class="store-work__form"
  aria-label={task ? "Edit task" : "Create task"}
>
  <label
    >Task title<Input
      bind:value={title}
      required
      maxlength={512}
      {disabled}
    /></label
  >
  <label
    >Task description<Textarea
      bind:value={body}
      maxlength={65536}
      {disabled}
    /></label
  >
  {#if task}<label
      >Task state<select bind:value={status} {disabled}
        ><option value="open">Open</option><option value="in_progress"
          >In progress</option
        ><option value="blocked">Blocked</option><option value="done"
          >Done</option
        ></select
      ></label
    >{/if}
  <label
    >Target repository<select bind:value={repository} {disabled}>
      <option value="">Project only</option>
      {#each Object.entries(project.repositories) as [id, active]}
        {#if active || id === task?.repository}<option value={id}
            >{repositories.find((repo) => repo.id === id)?.name ?? id}{active
              ? ""
              : " (retired)"}</option
          >{/if}
      {/each}
    </select></label
  >
  <label
    >Parent task<select bind:value={parent} {disabled}>
      <option value="">No parent</option>
      {#if task?.parent && !work.state.tasks.some((row) => row.id === task?.parent)}<option
          value={task.parent}>Parent {task.parent.slice(0, 8)}</option
        >{/if}
      {#each work.state.tasks.filter((row) => row.id !== task?.id) as row}<option
          value={row.id}>{row.title}</option
        >{/each}
    </select></label
  >
  <Button type="submit" disabled={disabled || !title.trim()}
    >{task ? "Save task" : "Create task"}</Button
  >
</form>

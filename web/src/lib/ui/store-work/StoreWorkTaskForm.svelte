<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { workDraft } from "./work-drafts.js";
  import StoreWorkPicker from "./StoreWorkPicker.svelte";
  import { untrack } from "svelte";
  import type { Model } from "../../api/client.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    project,
    task = null,
    repositories,
    disabled = false,
    initialParent = "",
    transport,
  }: {
    work: WorkController;
    project: Model<"ProjectV1">;
    task?: Model<"TaskV1"> | null;
    repositories: readonly StoreRepositorySummary[];
    disabled?: boolean;
    initialParent?: string;
    transport?: import("../../controller/projects-hub-types.js").WorkBrowseTransport;
  } = $props();
  const formId = $props.id();
  const draft = untrack(() =>
    workDraft(
      work,
      `task:${task?.id ?? `${project.id}:${initialParent}:new`}`,
      {
        title: task?.title ?? "",
        body: task?.body ?? "",
        repository: task?.repository ?? "",
        parent: task?.parent ?? initialParent,
        status: task?.state ?? "open",
      },
    ),
  );
  async function save(event: SubmitEvent) {
    event.preventDefault();
    const fields = {
      title: draft.title,
      body: draft.body,
      repository: draft.repository || null,
      parent: draft.parent || null,
      expected_project_version: project.version,
    };
    await work.submit(
      task
        ? {
            command: "update_task",
            id: task.id,
            expected_version: task.version,
            state: draft.status,
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
  <Field.Group class="store-work__fields community-settings__option-group">
    <StoreWorkField
      label="Task title"
      id={`${formId}-title`}
      description="Name the work to complete."
      {disabled}
    >
      <Input
        id={`${formId}-title`}
        bind:value={draft.title}
        required
        maxlength={512}
        {disabled}
      />
    </StoreWorkField>
    <StoreWorkField
      label="Task description"
      compact
      description="Add context and acceptance criteria with Markdown."
      {disabled}
    >
      <CommunityChatComposer
        bind:value={draft.body}
        mode="form"
        ariaLabel="Task description"
        placeholder="Describe the task"
        {disabled}
      />
    </StoreWorkField>
    {#if task}<StoreWorkField
        label="Task state"
        id={`${formId}-state`}
        {disabled}
      >
        <select id={`${formId}-state`} bind:value={draft.status} {disabled}
          ><option value="open">Open</option><option value="in_progress"
            >In progress</option
          ><option value="blocked">Blocked</option><option value="done"
            >Done</option
          ></select
        >
      </StoreWorkField>{/if}
    <StoreWorkField
      label="Target repository"
      id={`${formId}-repository`}
      description="Choose a linked repository, or keep the task at project level."
      {disabled}
    >
      <StoreRepositoryPicker
        id={`${formId}-repository`}
        label="Target repository"
        bind:value={draft.repository}
        {disabled}
        allowEmpty
        emptyLabel="Project only"
        repositories={Object.entries(project.repositories)
          .filter(([id, active]) => active || id === task?.repository)
          .map(([id, active]) => ({
            id,
            name: `${repositories.find((repo) => repo.id === id)?.name ?? id}${active ? "" : " (retired)"}`,
          }))}
      />
    </StoreWorkField>
    <StoreWorkPicker
      {transport}
      project={project.id}
      label="Parent task"
      bind:value={draft.parent}
      rows={work.state.tasks}
      repositoryName={(id) =>
        repositories.find((repo) => repo.id === id)?.name ?? id.slice(0, 8)}
      selectedLabel={work.state.tasks.find((row) => row.id === draft.parent)
        ?.title ?? ""}
      excluded={task?.id ?? ""}
      {disabled}
    />
  </Field.Group>
  <div class="store-work__actions">
    <Button type="submit" disabled={disabled || !draft.title.trim()}
      >{task ? "Save task" : "Create task"}</Button
    >
  </div>
</form>

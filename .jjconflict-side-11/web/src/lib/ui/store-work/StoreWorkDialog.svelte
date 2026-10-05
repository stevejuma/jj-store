<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import { untrack } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  import type { WorkController } from "../../controller/work-types.js";
  import StoreWorkProjectForm from "./StoreWorkProjectForm.svelte";
  import StoreWorkTaskForm from "./StoreWorkTaskForm.svelte";
  import StoreWorkPicker from "./StoreWorkPicker.svelte";
  import StoreWorkStatus from "./StoreWorkStatus.svelte";
  let {
    controller,
    mode,
    home: initialHome = "",
    project = null,
    task = null,
    initialParent = "",
    onClose,
  }: {
    controller: StoreController;
    mode: "createProject" | "editProject" | "createTask" | "editTask";
    home?: string;
    project?: Model<"ProjectV1"> | null;
    task?: Model<"TaskV1"> | null;
    initialParent?: string;
    onClose: () => void;
  } = $props();
  const dialogId = $props.id();
  const hub = $derived(controller.projects!);
  const writableHomes = $derived(
    controller.state.repositories.filter(
      (r) =>
        controller.state.accessMode === "local_trusted" ||
        r.permission === "write" ||
        r.permission === "admin",
    ),
  );
  let home = $state(untrack(() => initialHome || writableHomes[0]?.id || "")),
    projectId = $state(untrack(() => project?.id ?? "")),
    selectedProject = $state(untrack(() => project)),
    error = $state("");
  let work = $state<WorkController>();
  $effect(() => {
    const selectedHome = home;
    work = untrack(() => (selectedHome ? hub.work(selectedHome) : undefined));
  });
  const editing = $derived(mode.startsWith("edit")),
    taskForm = $derived(mode.endsWith("Task"));
  const title = $derived(
    `${editing ? "Edit" : "Create"} ${taskForm ? "task" : "project"}`,
  );
  $effect(() => {
    const id = projectId,
      selectedHome = home;
    if (!taskForm || project) return;
    let current = true;
    selectedProject = null;
    error = "";
    if (id && selectedHome)
      void hub
        .transport(selectedHome)
        .page({ project: id, limit: 1 })
        .then((page) => {
          if (current) selectedProject = page.projects[0] ?? null;
        })
        .catch((cause) => {
          if (current)
            error =
              cause instanceof Error ? cause.message : "Project unavailable";
        });
    return () => {
      current = false;
    };
  });
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open) onClose();
  }}
>
  <DialogFrame
    size="wide"
    preventScroll={false}
    {title}
    description="Save native metadata in the selected home repository."
  >
    {#if !home || !writableHomes.length}<StoreWorkEmpty
        title="Choose a writable home"
        description="Create or register a repository using the jj-store CLI if no home is available."
      />{/if}
    {#if !editing && !project}<Field.Group
        class="store-work__fields community-settings__option-group"
      >
        <StoreWorkField
          label="Home repository"
          id={`${dialogId}-home`}
          description="This repository owns the project metadata and discussion."
        >
          <StoreRepositoryPicker
            id={`${dialogId}-home`}
            label="Home repository"
            repositories={writableHomes}
            bind:value={home}
            allowEmpty
            emptyLabel="Choose a writable home"
            onChange={() => (projectId = "")}
          />
        </StoreWorkField>
      </Field.Group>{/if}
    {#if work}<StoreWorkStatus {work} />{/if}
    {#if error}<p role="alert">{error}</p>{/if}
    {#if home && work}
      {#key home}
        {#if taskForm && !project}<StoreWorkPicker
            transport={hub.transport(home)}
            kind="projects"
            label="Task project"
            bind:value={projectId}
            disabled={hub.loading ||
              work.state.loading ||
              work.state.writing ||
              !!work.state.pending}
          />{/if}
        {#if taskForm && selectedProject}<StoreWorkTaskForm
            {work}
            project={selectedProject}
            {task}
            repositories={controller.state.repositories}
            transport={hub.transport(home)}
            {initialParent}
            disabled={hub.loading ||
              work.state.loading ||
              work.state.writing ||
              !!work.state.pending}
          />
        {:else if !taskForm}<StoreWorkProjectForm
            {work}
            project={editing ? project : null}
            disabled={hub.loading ||
              work.state.loading ||
              work.state.writing ||
              !!work.state.pending}
          />{/if}
      {/key}
    {/if}
    <Button variant="outline" onclick={onClose}>Done</Button>
  </DialogFrame>
</Dialog.Root>

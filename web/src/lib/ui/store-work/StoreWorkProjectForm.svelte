<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { workDraft } from "./work-drafts.js";
  import { untrack } from "svelte";
  import type { Model } from "../../api/client.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    project = null,
    disabled = false,
  }: {
    work: WorkController;
    project?: Model<"ProjectV1"> | null;
    disabled?: boolean;
  } = $props();
  const formId = $props.id();
  const draft = untrack(() =>
    workDraft(work, `project:${project?.id ?? "new"}`, {
      title: project?.title ?? "",
      body: project?.body ?? "",
    }),
  );
  async function save(event: SubmitEvent) {
    event.preventDefault();
    await work.submit(
      project
        ? {
            command: "update_project",
            id: project.id,
            expected_version: project.version,
            title: draft.title,
            body: draft.body,
          }
        : {
            command: "create_project",
            id: workId(),
            title: draft.title,
            body: draft.body,
          },
    );
  }
</script>

<form
  onsubmit={save}
  class="store-work__form"
  aria-label={project ? "Edit project" : "Create project"}
>
  <Field.Group class="store-work__fields community-settings__option-group">
    <StoreWorkField
      label="Project title"
      id={`${formId}-title`}
      description="A short name for this project."
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
      label="Project description"
      compact
      description="Explain the project with Markdown."
      {disabled}
    >
      <CommunityChatComposer
        bind:value={draft.body}
        mode="form"
        ariaLabel="Project description"
        placeholder="Describe the project"
        {disabled}
      />
    </StoreWorkField>
  </Field.Group>
  <div class="store-work__actions">
    <Button type="submit" disabled={disabled || !draft.title.trim()}
      >{project ? "Save project" : "Create project"}</Button
    >
  </div>
</form>

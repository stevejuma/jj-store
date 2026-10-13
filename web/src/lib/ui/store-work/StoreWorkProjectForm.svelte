<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreSettingsFooter from "../store-settings/StoreSettingsFooter.svelte";
  import { provideSettingsPresentation } from "../store-settings/settings-presentation.js";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import TagIcon from "@lucide/svelte/icons/tag";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import "../store-settings/StoreSettings.css";
  import "../store-settings/creation-forms.css";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { workDraft } from "./work-drafts.js";
  import { untrack } from "svelte";
  import type { Model } from "../../api/client.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    project = null,
    disabled = false,
    id,
    externalActions = false,
    submitDisabled = $bindable(true),
    onSaved,
  }: {
    work: WorkController;
    project?: Model<"ProjectV1"> | null;
    disabled?: boolean;
    id?: string;
    externalActions?: boolean;
    submitDisabled?: boolean;
    onSaved?: () => void;
  } = $props();
  const formId = $props.id();
  provideSettingsPresentation({ value: "global" });
  const projectVersion = untrack(() => project?.version);
  const draft = untrack(() =>
    workDraft(work, `project:${project?.id ?? "new"}`, {
      title: project?.title ?? "",
      body: project?.body ?? "",
    }),
  );
  $effect(() => {
    submitDisabled = disabled || !draft.title.trim();
  });
  async function save(event: SubmitEvent) {
    event.preventDefault();
    await work.submit(
      project
        ? {
            command: "update_project",
            id: project.id,
            expected_version: projectVersion!,
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
    if (!work.state.pending && !work.state.error) onSaved?.();
  }
</script>

<form
  id={id ?? formId}
  onsubmit={save}
  class="store-work__form store-creation-form store-settings--global"
  aria-label={project ? "Edit project" : "Create project"}
>
  <StoreSettingsCard
    id={`${formId}-details`}
    title="Project details"
    Icon={FolderKanbanIcon}
  >
    <Field.Group class="store-settings__editable-rows">
      <StoreWorkField
        class="store-settings__option-row"
        Icon={TagIcon}
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
        class="store-settings__option-row store-creation-form__description"
        Icon={FileTextIcon}
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
  </StoreSettingsCard>
  {#if !externalActions}<StoreSettingsFooter standalone>
      <Button type="submit" disabled={disabled || !draft.title.trim()}
        >{project ? "Save project" : "Create project"}</Button
      >
    </StoreSettingsFooter>{/if}
</form>

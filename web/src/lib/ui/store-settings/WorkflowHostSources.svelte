<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Plus from "@lucide/svelte/icons/plus";
  import type { Model } from "../../api/client.js";
  import type { WorkflowHostChoices } from "../../controller/workflow-host-choices.svelte.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  type Source = Model<"WorkflowHostSource">;
  let {
    sources,
    choices,
    repositories = [],
    targets = [],
    disabled,
    onLoadTargets,
    onChange,
  }: {
    sources: Source[];
    choices: WorkflowHostChoices;
    repositories?: readonly StoreRepositorySummary[];
    targets?: readonly Model<"AgentExecutionTarget">[];
    disabled: boolean;
    onLoadTargets?(): void;
    onChange(value: Source[]): void;
  } = $props();
  let query = $state("");
  const candidates = $derived([
    ...new Map(
      [
        ...repositories.filter((r) => r.profile !== "git_sha1_jj045"),
        ...choices.state.bindings.map((b) => ({
          id: b.repository,
          name: b.name,
        })),
        ...sources
          .filter((s) => s.repository)
          .map((s) => ({
            id: s.repository,
            name:
              repositories.find((r) => r.id === s.repository)?.name ||
              s.repository,
          })),
        ...(/^[0-9a-f]{32}$/.test(query.trim())
          ? [{ id: query.trim(), name: query.trim() }]
          : []),
      ].map((r) => [r.id, r]),
    ).values(),
  ]);
  function change(index: number, patch: Partial<Source>) {
    onChange(sources.map((s, i) => (i === index ? { ...s, ...patch } : s)));
  }
  function repository(index: number, id: string) {
    const path =
      choices.state.bindings.find(
        (b) => b.repository === id && b.availability === "ready",
      )?.path ||
      sources.find((s) => s.repository === id)?.workspace ||
      "";
    change(index, { repository: id, workspace: path, target_id: null });
  }
</script>

{#each sources as source, index}
  <Field.Field class="store-settings__option-row">
    <Field.Content
      ><Field.Title
        ><StoreSettingsIcon Icon={FolderGit} />Source {index + 1}</Field.Title
      >
      <Field.Description
        >Select its repository and native CLI attachment. Source access remains
        separately authorized.</Field.Description
      ></Field.Content
    >
    <StoreRepositoryPicker
      id={`workflow-source-${index}`}
      label={`Source ${index + 1} repository`}
      repositories={candidates.map((r) => ({
        ...r,
        disabled: sources.some((s, i) => i !== index && s.repository === r.id),
      }))}
      value={source.repository}
      onChange={(id) => repository(index, id)}
      {disabled}
      onOpen={() => void choices.refresh()}
      loading={choices.state.loading}
      onSearch={(v) => (query = v)}
      searchLabel="Find a source repository"
      groupLabel="Repositories"
    />
    <Field.Label for={`workflow-source-path-${index}`}
      >Native attachment path</Field.Label
    >
    <Input
      id={`workflow-source-path-${index}`}
      aria-label={`Source ${index + 1} native attachment path`}
      value={source.workspace}
      {disabled}
      oninput={(e) => change(index, { workspace: e.currentTarget.value })}
    />
    <StoreRepositoryPicker
      id={`workflow-source-target-${index}`}
      label={`Source ${index + 1} agent target`}
      repositories={[
        ...targets
          .filter((t) => t.repository === source.repository)
          .map((t) => ({ id: t.id, name: t.name, description: t.folder })),
        ...(source.target_id && !targets.some((t) => t.id === source.target_id)
          ? [{ id: source.target_id, name: source.target_id }]
          : []),
      ]}
      value={source.target_id ?? ""}
      allowEmpty
      emptyLabel="No agent target"
      onChange={(id) => change(index, { target_id: id || null })}
      {disabled}
      searchLabel="Find an admitted agent attachment"
      groupLabel="Agent attachments"
      onOpen={onLoadTargets}
    />
    <Button
      type="button"
      variant="ghost"
      {disabled}
      onclick={() => onChange(sources.filter((_, i) => i !== index))}
      ><Trash aria-hidden="true" />Remove source {index + 1}</Button
    >
  </Field.Field>
{/each}
<Field.Field class="store-settings__option-row"
  ><Button
    type="button"
    variant="outline"
    disabled={disabled || sources.length >= 32}
    onclick={() =>
      onChange([
        ...sources,
        { repository: "", workspace: "", target_id: null },
      ])}><Plus aria-hidden="true" />Add source repository</Button
  ></Field.Field
>

<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { EditorModelRow } from "../../community/agent-editor-model-rows.js";
  import CommunityAgentEditorModelPicker from "./CommunityAgentEditorModelPicker.svelte";

  let {
    id,
    pending = false,
    rows,
    runtimeAgent,
    model,
    onSelect,
    onHarnessChange,
  }: {
    id: string;
    pending?: boolean;
    rows: readonly EditorModelRow[];
    runtimeAgent: string;
    model: string;
    onSelect: (row: EditorModelRow) => void;
    onHarnessChange?: (runner: string) => void;
  } = $props();

  const selected = $derived(
    rows.find(
      (row) =>
        !row.disabled &&
        row.runtimeAgent === runtimeAgent &&
        row.model === model,
    ),
  );
</script>

<Field.Field>
  <Field.Label for={id}>Model</Field.Label>
  <CommunityAgentEditorModelPicker
    {id}
    {pending}
    {rows}
    {runtimeAgent}
    {model}
    {onSelect}
    {onHarnessChange}
  />
  {#if selected}
    <Field.Description>Selected model: {selected.name}</Field.Description>
  {/if}
</Field.Field>

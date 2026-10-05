<script lang="ts">
  import { untrack } from "svelte";
  import StoreStackConflictVersionPicker from "./StoreStackConflictVersionPicker.svelte";
  import StoreConflictText from "../store-conflicts/StoreConflictText.svelte";
  import type { WorkspaceConflict } from "../../controller/stack-workspace/types.js";
  let {
    value,
    locked,
    onDraft,
    onPickerOpen,
  }: {
    value: WorkspaceConflict;
    locked: boolean;
    onDraft: (content: string, resolved: boolean) => void;
    onPickerOpen?: (open: boolean) => void;
  } = $props();
  let version = $state(2);
  let reviewed = $state<string | undefined>();
  let left = $state(untrack(() => value.left));
  const right = $derived(value.terms?.[version] ?? value.right);
  const base = $derived(value.terms?.[Math.max(1, version - 1)] ?? value.base);
</script>

<StoreStackConflictVersionPicker
  terms={value.terms ?? []}
  selected={version}
  disabled={locked}
  onOpenChange={onPickerOpen}
  onSelect={(index) => {
    if (locked || value.terms?.[index] === null || index === version) return;
    left = reviewed ?? left;
    version = index;
    onDraft(left, false);
  }}
/>
<StoreConflictText
  path={value.path}
  {left}
  {base}
  {right}
  leftLabel="Current resolution"
  rightLabel={`Version ${version / 2 + 1}`}
  editable
  readOnly={locked}
  onResolvedChange={(change) => {
    reviewed = change.content;
    // A pairwise merge cannot prove that all native versions were reviewed.
    onDraft(change.content, false);
  }}
  onBaseChange={(content) => {
    reviewed = content;
    onDraft(content, false);
  }}
/>

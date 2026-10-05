<script lang="ts">
  import { untrack } from "svelte";
  import StoreConflictTerms from "../store-conflicts/StoreConflictTerms.svelte";
  import StoreConflictText from "../store-conflicts/StoreConflictText.svelte";
  import type { WorkspaceConflict } from "../../controller/stack-workspace/types.js";
  let {
    value,
    locked,
    onDraft,
  }: {
    value: WorkspaceConflict;
    locked: boolean;
    onDraft: (content: string, resolved: boolean) => void;
  } = $props();
  let version = $state(2);
  let reviewed = $state<string | undefined>();
  let left = $state(untrack(() => value.left));
  const right = $derived(value.terms?.[version] ?? value.right);
  const base = $derived(value.terms?.[Math.max(1, version - 1)] ?? value.base);
</script>

<p>
  Compare each native version with your current resolution. Review all versions
  before staging the complete file.
</p>
<StoreConflictTerms
  actionLabel="Compare"
  terms={(value.terms ?? []).flatMap((text, index) =>
    index % 2 === 0
      ? [
          {
            id: index,
            label: `version ${index / 2 + 1}`,
            disabled: text === null,
          },
        ]
      : [],
  )}
  selected={version}
  disabled={locked}
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

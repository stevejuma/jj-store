<script lang="ts" generics="Id extends number | bigint">
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    terms,
    selected,
    disabled = false,
    actionLabel = "Use",
    onSelect,
  }: {
    terms: readonly { id: Id; label?: string | null; disabled?: boolean }[];
    selected?: Id;
    disabled?: boolean;
    actionLabel?: string;
    onSelect: (id: Id) => void;
  } = $props();
</script>

<div class="store-conflict-path__terms" aria-label="Native conflict terms">
  {#each terms as term (term.id)}<Button
      type="button"
      size="sm"
      variant={selected === term.id ? "secondary" : "outline"}
      disabled={disabled || term.disabled}
      aria-pressed={selected === term.id}
      onclick={() => onSelect(term.id)}
      >{actionLabel} {term.label ?? `term ${term.id}`}</Button
    >{/each}
</div>

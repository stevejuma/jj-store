<script lang="ts">
  import type { ComponentProps } from "svelte";
  import StoreNumericSettingRow from "./StoreNumericSettingRow.svelte";
  let {
    value,
    onValueChange,
    defaultValue,
    ...props
  }: Omit<
    ComponentProps<typeof StoreNumericSettingRow>,
    "value" | "onValueChange" | "defaultValue"
  > & {
    value: number | bigint;
    defaultValue?: number | bigint;
    onValueChange: (value: number) => void;
  } = $props();
  let draft = $state<{ observed: number; text: string }>();
  const text = $derived(draft?.observed === value ? draft.text : String(value));
  function update(text: string) {
    const parsed = Number(text);
    // Numeric configuration drafts cannot carry an empty string or NaN through
    // their JSON copy boundary. -1 remains invalid for these nonnegative fields.
    const next = text.trim() && Number.isSafeInteger(parsed) ? parsed : -1;
    draft = { observed: next, text };
    onValueChange(next);
  }
</script>

<StoreNumericSettingRow
  {...props}
  value={text}
  defaultValue={defaultValue === undefined ? undefined : Number(defaultValue)}
  onValueChange={update}
/>

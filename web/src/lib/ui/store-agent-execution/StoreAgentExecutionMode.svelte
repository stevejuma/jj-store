<script lang="ts">
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsSelect from "../store-settings/StoreSettingsSelect.svelte";
  let {
    id,
    value = $bindable(),
    options,
    disabled = false,
  }: {
    id: string;
    value: string | null;
    options: { id: string; name: string }[];
    disabled?: boolean;
  } = $props();
</script>

<StoreWorkField label="Execution mode" {id}>
  <StoreSettingsSelect
    {id}
    bind:value
    disabled={disabled || !options.length}
    options={[
      { value: null, label: "Harness default" },
      ...options.map((o) => ({ value: o.id, label: o.name })),
      ...(value && !options.some((o) => o.id === value)
        ? [{ value, label: `${value} (saved preference)`, disabled: true }]
        : []),
    ]}
  />
</StoreWorkField>

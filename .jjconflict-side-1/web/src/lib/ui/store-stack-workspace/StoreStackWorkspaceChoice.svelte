<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  let {
    label,
    value,
    items,
    disabled = false,
    onChange,
  }: {
    label: string;
    value: string;
    items: { id: string; name: string }[];
    disabled?: boolean;
    onChange: (id: string) => void;
  } = $props();
</script>

<Select.Root type="single" {value} {disabled} onValueChange={onChange}>
  <Select.Trigger aria-label={label}>
    {items.find((item) => item.id === value)?.name ?? label}
  </Select.Trigger>
  <Select.Content preventScroll={false}>
    {#each items as item (item.id)}<Select.Item
        value={item.id}
        label={item.name}>{item.name}</Select.Item
      >{/each}
  </Select.Content>
</Select.Root>

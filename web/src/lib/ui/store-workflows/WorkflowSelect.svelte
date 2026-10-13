<script lang="ts">
  import type { Component } from "svelte";
  import * as Select from "@lapismd/design-core/shadcn/select";
  let {
    id,
    label,
    value,
    options,
    disabled = false,
    onChange,
  }: {
    id: string;
    label: string;
    value: string;
    options: readonly { value: string; label: string; Icon?: Component }[];
    disabled?: boolean;
    onChange(value: string): void;
  } = $props();
  const selected = $derived(options.find((option) => option.value === value));
</script>

<Select.Root type="single" {value} {disabled} onValueChange={onChange}
  ><Select.Trigger {id} aria-label={label}
    >{#if selected?.Icon}<selected.Icon aria-hidden="true" />{/if}<span
      >{selected?.label ?? value}</span
    ></Select.Trigger
  ><Select.Content
    ><Select.Group
      >{#each options as option (option.value)}<Select.Item
          value={option.value}
          label={option.label}
          >{#if option.Icon}<option.Icon
              aria-hidden="true"
            />{/if}{option.label}</Select.Item
        >{/each}</Select.Group
    ></Select.Content
  ></Select.Root
>

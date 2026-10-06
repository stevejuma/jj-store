<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  let {
    id,
    pending = false,
    value = "",
    options,
    onValueChange,
  }: {
    id: string;
    pending?: boolean;
    value?: string;
    options: readonly { value: string; name: string }[];
    onValueChange: (value: string) => void;
  } = $props();
</script>

<Field.Field
  ><Field.Label for={id}>Execution mode</Field.Label><Select.Root
    type="single"
    {value}
    {onValueChange}
    disabled={pending}
    ><Select.Trigger {id}
      >{options.find((o) => o.value === value)?.name ??
        (value || "Choose execution mode")}</Select.Trigger
    ><Select.Content
      ><Select.Group
        >{#each options as option (option.value)}<Select.Item
            value={option.value}>{option.name}</Select.Item
          >{/each}</Select.Group
      ></Select.Content
    ></Select.Root
  ></Field.Field
>

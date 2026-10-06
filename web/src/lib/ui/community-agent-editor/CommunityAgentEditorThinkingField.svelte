<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import type { CommunityAgentRuntimeOption } from "../../community/agent-runtime-contracts.js";

  let {
    id,
    pending = false,
    thinking,
    options,
    onValueChange,
  }: {
    id: string;
    pending?: boolean;
    thinking?: string;
    options: readonly CommunityAgentRuntimeOption[];
    onValueChange: (value: string) => void;
  } = $props();
</script>

<Field.Field>
  <Field.Label for={id}>Thinking</Field.Label>
  <Select.Root
    type="single"
    value={thinking ?? ""}
    onValueChange={(value) => onValueChange(value as string)}
    disabled={pending}
  >
    <Select.Trigger {id}
      >{options.find(({ value }) => value === thinking)?.name ??
        thinking}</Select.Trigger
    >
    <Select.Content side="top"
      ><Select.Group>
        {#each options as option (option.value)}
          <Select.Item value={option.value}>{option.name}</Select.Item>
        {/each}
      </Select.Group></Select.Content
    >
  </Select.Root>
</Field.Field>

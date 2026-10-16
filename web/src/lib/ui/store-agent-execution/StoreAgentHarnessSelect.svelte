<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  import HarnessIcon from "../community-agent-editor/CommunityAgentEditorHarnessIcon.svelte";
  let {
    id,
    value,
    options,
    disabled = false,
    onValueChange,
  }: {
    id: string;
    value: string;
    options: readonly { id: string; label: string }[];
    disabled?: boolean;
    onValueChange: (value: string) => void;
  } = $props();
</script>

<Select.Root
  type="single"
  {value}
  {disabled}
  onValueChange={(next) => {
    if (options.some((option) => option.id === next)) onValueChange(next);
  }}
>
  <Select.Trigger {id} class="w-full">
    <span class="store-agent-harness-choice">
      <HarnessIcon runtimeAgent={value} />
      <span
        >{options.find((option) => option.id === value)?.label ?? value}</span
      >
    </span>
  </Select.Trigger>
  <Select.Content aria-labelledby={`${id}-label`}>
    <Select.Group>
      {#each options as option (option.id)}
        <Select.Item value={option.id} label={option.label}>
          <span class="store-agent-harness-choice">
            <HarnessIcon runtimeAgent={option.id} />
            <span>{option.label}</span>
          </span>
        </Select.Item>
      {/each}
    </Select.Group>
  </Select.Content>
</Select.Root>

<style>
  .store-agent-harness-choice {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }
  .store-agent-harness-choice :global(svg) {
    width: 1em;
    height: 1em;
    flex-shrink: 0;
  }
</style>

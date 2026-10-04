<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ListFilterIcon from "@lucide/svelte/icons/list-filter";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import StoreWorkTaskStateIcon from "./StoreWorkTaskStateIcon.svelte";
  import type { StoreRoute } from "../../controller/contracts.js";
  type State = NonNullable<StoreRoute["workState"]>;
  let {
    id,
    value,
    onChange,
  }: {
    id: string;
    value: State;
    onChange: (state: State) => void | Promise<void>;
  } = $props();
  const choices = [
    { value: "active", label: "Active", state: null },
    { value: "done", label: "Done", state: "done" },
    { value: "all", label: "All", state: null },
    { value: "open", label: "Open", state: "open" },
    { value: "in_progress", label: "In progress", state: "in_progress" },
    { value: "blocked", label: "Blocked", state: "blocked" },
  ] as const;
  const selected = $derived(
    choices.find((choice) => choice.value === value) ?? choices[0],
  );
</script>

{#snippet icon(choice: (typeof choices)[number])}
  <span aria-hidden="true" class="store-work-state-filter__icon">
    {#if choice.state}<StoreWorkTaskStateIcon state={choice.state} />
    {:else if choice.value === "active"}<ListFilterIcon />{:else}<ListTodoIcon
      />{/if}
  </span>
{/snippet}

<DropdownMenu.Root>
  <DropdownMenu.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        {id}
        type="button"
        variant="outline"
        class={["store-work-state-filter__trigger", props.class]}
        aria-label="Task state"
      >
        {@render icon(selected)}<span>{selected.label}</span><ChevronDownIcon
          aria-hidden="true"
        />
      </Button>{/snippet}</DropdownMenu.Trigger
  >
  <DropdownMenu.Content align="start" preventScroll={false}>
    <DropdownMenu.RadioGroup
      {value}
      onValueChange={(next) => onChange(next as State)}
      aria-label="Task state"
    >
      {#each choices as choice (choice.value)}<DropdownMenu.RadioItem
          value={choice.value}
          aria-label={choice.label}
        >
          {@render icon(choice)}{choice.label}
        </DropdownMenu.RadioItem>{/each}
    </DropdownMenu.RadioGroup>
  </DropdownMenu.Content>
</DropdownMenu.Root>

<style>
  :global(.store-work-state-filter__trigger) {
    width: 100%;
    justify-content: flex-start;
  }
  :global(
    .store-work-state-filter__trigger > span:not(.store-work-state-filter__icon)
  ) {
    flex: 1;
    text-align: start;
  }
  .store-work-state-filter__icon {
    display: inline-flex;
    align-items: center;
  }
  .store-work-state-filter__icon :global(svg) {
    width: 1rem;
    height: 1rem;
  }
</style>

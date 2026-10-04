<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import { captureSourceStatuses } from "./capture-source-presentation.js";
  let { value = $bindable([]) }: { value?: string[] } = $props();
  let open = $state(false);
  let query = $state("");
  const uid = $props.id();
  const label = $derived(
    value.length === 0
      ? "All statuses"
      : value.length === 1
        ? captureSourceStatuses.find((s) => s.value === value[0])?.label
        : `${value.length} statuses`,
  );
  function toggle(status: string) {
    value =
      status === "all"
        ? []
        : value.includes(status)
          ? value.filter((s) => s !== status)
          : [...value, status];
  }
</script>

<Popover.Root
  {open}
  onOpenChange={(next) => {
    open = next;
    if (next) query = "";
  }}
>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button {...props} variant="outline" aria-label="Filter by source status">
        {label}<ChevronDownIcon data-icon="inline-end" aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="end"
    role="dialog"
    aria-label="Source status filter"
    style="--ui-popover-width: min(20rem, calc(100vw - 2rem)); --ui-popover-padding: 0.25rem"
  >
    <CommandView.Root label="Search source statuses" loop>
      <CommandView.Input
        bind:value={query}
        aria-label="Search source statuses"
        placeholder="Search statuses…"
      />
      <CommandView.List>
        <CommandView.Empty>No matching statuses</CommandView.Empty>
        <CommandView.Group heading="Statuses">
          {#each captureSourceStatuses as status, index (status.value)}
            {@const checked =
              status.value === "all"
                ? value.length === 0
                : value.includes(status.value)}
            <CommandView.Item
              value={status.label}
              keywords={[status.value]}
              aria-label={status.label}
              aria-describedby={`${uid}-${index}`}
              data-checked={checked}
              onSelect={() => toggle(status.value)}
            >
              <CommandView.ItemLabel>{status.label}</CommandView.ItemLabel>
              <span id={`${uid}-${index}`} class="sr-only"
                >{checked ? "Selected" : "Not selected"}</span
              >
              {#if checked}
                <span data-ui-component="command-view" data-ui-part="trailing">
                  <CheckIcon aria-hidden="true" size={16} />
                </span>
              {/if}
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>

<script lang="ts">
  import { onDestroy } from "svelte";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Check from "@lucide/svelte/icons/check";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import FileText from "@lucide/svelte/icons/file-text";
  import X from "@lucide/svelte/icons/x";
  let {
    terms,
    selected,
    disabled,
    onSelect,
    onOpenChange,
  }: {
    terms: readonly (string | null)[];
    selected: number;
    disabled: boolean;
    onSelect: (index: number) => void;
    onOpenChange?: (open: boolean) => void;
  } = $props();
  let open = $state(false),
    query = $state("");
  const options = $derived(
    terms.flatMap((text, index) => {
      if (index % 2) return [];
      const label = `Version ${index / 2 + 1}`;
      if (text === null)
        return [
          {
            index,
            label,
            disabled: true,
            detail: "Deleted version · unavailable for text comparison",
          },
        ];
      const lines = text.split(/\r?\n/u);
      if (lines.at(-1) === "") lines.pop();
      const base = (terms[Math.max(1, index - 1)] ?? "").split(/\r?\n/u);
      const preview = (
        lines.find((line, n) => line !== base[n]) ??
        lines.find((line) => line.trim()) ??
        "Empty file"
      )
        .trim()
        .replace(/\s+/gu, " ")
        .slice(0, 100);
      const bytes = new TextEncoder().encode(text).length;
      return [
        {
          index,
          label,
          disabled: false,
          detail: `${lines.length} ${lines.length === 1 ? "line" : "lines"} · ${bytes.toLocaleString()} bytes · ${preview}`,
        },
      ];
    }),
  );
  const visible = $derived(
    options.filter((option) =>
      `${option.label} ${option.detail}`
        .toLocaleLowerCase()
        .includes(query.trim().toLocaleLowerCase()),
    ),
  );
  function handleOpen(next: boolean) {
    open = next;
    if (next) query = "";
    onOpenChange?.(next);
  }
  onDestroy(() => onOpenChange?.(false));
  function choose(index: number) {
    if (
      disabled ||
      !options.some((option) => option.index === index && !option.disabled)
    )
      return;
    handleOpen(false);
    onSelect(index);
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        size="xs"
        role="combobox"
        aria-label="Comparison version"
        aria-expanded={open}
        {disabled}
      >
        <FileText data-icon="inline-start" aria-hidden="true" />Version {selected /
          2 +
          1}<ChevronDown data-icon="inline-end" aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="start"
    role="dialog"
    aria-label="Choose conflict version"
    style="--ui-popover-width: min(34rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Title class="sr-only">Choose conflict version</Popover.Title>
    <CommandView.Root
      label="Search conflict versions"
      shouldFilter={false}
      loop
    >
      <div class="stack-conflict-version-picker__search">
        <CommandView.Input
          bind:value={query}
          placeholder="Search conflict versions"
          aria-label="Search conflict versions"
        />
        <Popover.Close
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Close version picker"><X aria-hidden="true" /></Button
            >{/snippet}</Popover.Close
        >
      </div>
      <CommandView.List scrollStrategy="styled">
        {#if !visible.length}<CommandView.Empty
            >No matching versions</CommandView.Empty
          >{/if}
        <CommandView.Group heading="Native versions">
          {#each visible as option (option.index)}
            <CommandView.Item
              value={`version:${option.index}`}
              disabled={disabled || option.disabled}
              onSelect={() => choose(option.index)}
            >
              <CommandView.ItemIcon
                >{#if option.index === selected}<Check
                    aria-hidden="true"
                  />{:else}<FileText
                    aria-hidden="true"
                  />{/if}</CommandView.ItemIcon
              >
              <CommandView.ItemLabel>{option.label}</CommandView.ItemLabel>
              <CommandView.ItemDescription
                >{option.detail}</CommandView.ItemDescription
              >
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>

<style>
  .stack-conflict-version-picker__search {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
  .stack-conflict-version-picker__search
    :global([data-ui-part="input-wrapper"]) {
    flex: 1;
    min-width: 0;
  }
</style>

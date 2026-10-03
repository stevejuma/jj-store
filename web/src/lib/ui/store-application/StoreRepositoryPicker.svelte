<script lang="ts">
  // CommunityProjectFilesRepositoryCommand: native repository identities.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import XIcon from "@lucide/svelte/icons/x";
  let {
    id,
    label = "Repository",
    repositories,
    value = $bindable(""),
    emptyLabel = "Choose repository",
    allowEmpty = false,
    disabled = false,
    variant = "outline",
    onChange,
  }: {
    id?: string;
    label?: string;
    repositories: readonly {
      id: string;
      name: string;
      permission?: string | null;
    }[];
    value?: string;
    emptyLabel?: string;
    allowEmpty?: boolean;
    disabled?: boolean;
    variant?: "outline" | "ghost";
    onChange?: (value: string) => void;
  } = $props();
  let open = $state(false),
    query = $state("");
  const selected = $derived(repositories.find((repo) => repo.id === value));
  const visible = $derived(
    repositories.filter((repo) =>
      `${repo.name} ${repo.id}`
        .toLocaleLowerCase()
        .includes(query.trim().toLocaleLowerCase()),
    ),
  );
  function handleOpen(next: boolean) {
    open = next;
    if (next) query = "";
  }
  function choose(next: string) {
    value = next;
    open = false;
    onChange?.(next);
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        {id}
        type="button"
        {variant}
        role="combobox"
        class={[
          "store-repository-picker__trigger",
          variant === "ghost" && "store-repository-picker__trigger--ghost",
          props.class,
        ]}
        aria-label={label}
        aria-expanded={open}
        {disabled}
        title={value || emptyLabel}
      >
        <FolderGitIcon aria-hidden="true" />
        <span
          >{selected?.name ??
            (value
              ? `Unavailable repository · ${value.slice(0, 8)}`
              : emptyLabel)}</span
        >
        <ChevronDownIcon aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="start"
    class="store-repository-picker__command"
    style="--ui-popover-width: min(26rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Title class="sr-only">Select {label.toLowerCase()}</Popover.Title>
    <CommandView.Root
      label={`Select ${label.toLowerCase()}`}
      shouldFilter={false}
      loop
    >
      <div class="store-repository-picker__search">
        <CommandView.Input
          bind:value={query}
          placeholder="Search repositories"
          aria-label="Search repositories"
        />
        <Popover.Close
          >{#snippet child({ props })}<Button
              {...props}
              type="button"
              variant="ghost"
              size="icon-sm"
              aria-label="Close repository selector"
              ><XIcon aria-hidden="true" /></Button
            >{/snippet}</Popover.Close
        >
      </div>
      <CommandView.List>
        {#if !visible.length && !(allowEmpty && !query)}<CommandView.Empty
            >No matching repositories.</CommandView.Empty
          >{/if}
        <CommandView.Group heading="Repositories">
          {#if allowEmpty && !query}<CommandView.Item
              value="__no_repository__"
              onSelect={() => choose("")}
            >
              <CommandView.ItemIcon
                >{#if !value}<CheckIcon
                    aria-hidden="true"
                  />{/if}</CommandView.ItemIcon
              >
              <CommandView.ItemLabel>{emptyLabel}</CommandView.ItemLabel>
            </CommandView.Item>{/if}
          {#each visible as repo (repo.id)}
            <CommandView.Item
              value={repo.id}
              keywords={[repo.name, repo.id]}
              onSelect={() => choose(repo.id)}
            >
              <CommandView.ItemIcon
                >{#if repo.id === value}<CheckIcon
                    aria-hidden="true"
                  />{:else}<FolderGitIcon
                    aria-hidden="true"
                  />{/if}</CommandView.ItemIcon
              >
              <CommandView.ItemLabel>{repo.name}</CommandView.ItemLabel>
              <CommandView.ItemDescription
                >{repo.id.slice(0, 8)}{repo.permission
                  ? ` · ${repo.permission} access`
                  : ""}</CommandView.ItemDescription
              >
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>

<style>
  :global(.store-repository-picker__trigger) {
    width: 100%;
    min-width: 0;
    justify-content: flex-start;
  }
  :global(.store-repository-picker__trigger--ghost) {
    border: 0;
    color: var(--muted-foreground);
  }
  :global(.store-repository-picker__trigger > span) {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: start;
  }
  .store-repository-picker__search {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
  .store-repository-picker__search :global([data-ui-part="input-wrapper"]) {
    min-width: 0;
    flex: 1;
  }
  :global(.store-repository-picker__command[data-ui-part="popover-content"]) {
    --ui-command-view-list-max-height: min(18rem, 50dvh);
    max-width: calc(100vw - 2rem);
  }
</style>

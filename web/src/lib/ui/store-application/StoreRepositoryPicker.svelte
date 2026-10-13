<script lang="ts">
  import "./repository-picker.css";
  // CommunityProjectFilesRepositoryCommand: native repository identities.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import XIcon from "@lucide/svelte/icons/x";
  import type { ScrollAreaStrategy } from "@lapismd/design-core/shadcn/scroll-area";
  import { untrack, type Snippet } from "svelte";
  let {
    id,
    label = "Repository",
    repositories,
    value = $bindable(""),
    values = $bindable<string[]>([]),
    multiple = false,
    inline = false,
    limit = 8,
    emptyLabel = "Choose repository",
    allowEmpty = false,
    disabled = false,
    variant = "outline",
    iconOnly = false,
    triggerIcon,
    itemIcon,
    notice,
    noticeAction,
    onSearch,
    onOpen,
    searchLabel = "Search repositories",
    groupLabel = "Repositories",
    emptyResult = "No matching repositories.",
    controlled = false,
    scrollStrategy = "auto",
    hasMore = false,
    loading = false,
    onLoadMore,
    onChange,
    keepItemIcon = false,
    selectionLabel = "repositories",
    unavailableLabel = "Unavailable repository",
    moreLabel = "branches",
  }: {
    id?: string;
    label?: string;
    repositories: readonly {
      id: string;
      name: string;
      permission?: string | null;
      description?: string;
      disabled?: boolean;
    }[];
    value?: string;
    values?: string[];
    multiple?: boolean;
    inline?: boolean;
    limit?: number;
    emptyLabel?: string;
    allowEmpty?: boolean;
    disabled?: boolean;
    variant?: "outline" | "ghost";
    iconOnly?: boolean;
    triggerIcon?: Snippet;
    itemIcon?: Snippet<[string]>;
    notice?: string;
    noticeAction?: Snippet;
    onSearch?: (query: string) => void;
    onOpen?: () => void;
    searchLabel?: string;
    groupLabel?: string;
    emptyResult?: string;
    controlled?: boolean;
    scrollStrategy?: ScrollAreaStrategy;
    hasMore?: boolean;
    loading?: boolean;
    onLoadMore?: () => void;
    onChange?: (value: string) => void;
    keepItemIcon?: boolean;
    selectionLabel?: string;
    unavailableLabel?: string;
    moreLabel?: string;
  } = $props();
  let open = $state(false),
    query = $state("");
  const selected = $derived(repositories.find((repo) => repo.id === value));
  const visible = $derived(
    repositories.filter((repo) =>
      `${repo.name} ${repo.id} ${repo.description ?? ""}`
        .toLocaleLowerCase()
        .includes(query.trim().toLocaleLowerCase()),
    ),
  );
  $effect(() => {
    const search = query;
    untrack(() => onSearch?.(search));
  });
  function handleOpen(next: boolean) {
    open = next;
    if (next) {
      query = "";
      onOpen?.();
    }
  }
  function choose(next: string) {
    if (disabled) return;
    if (multiple) {
      if (!next) values = [];
      else if (values.includes(next))
        values = values.filter((id) => id !== next);
      else if (values.length < limit) values = [...values, next];
      return;
    }
    if (!controlled) value = next;
    open = false;
    onChange?.(next);
  }
</script>

{#snippet command()}
  <CommandView.Root
    label={`Select ${label.toLowerCase()}`}
    shouldFilter={false}
    loop
  >
    <div class="store-repository-picker__search">
      <CommandView.Input
        bind:value={query}
        placeholder={searchLabel}
        aria-label={searchLabel}
      />
      {#if !inline}<Popover.Close
          >{#snippet child({ props })}<Button
              {...props}
              type="button"
              variant="ghost"
              size="icon-sm"
              aria-label={`Close ${label.toLowerCase()} selector`}
              ><XIcon aria-hidden="true" /></Button
            >{/snippet}</Popover.Close
        >{/if}
    </div>
    {#if notice}<p role="status">{notice}</p>{/if}
    {#if noticeAction}{@render noticeAction()}{/if}
    <CommandView.List {scrollStrategy}>
      {#if !visible.length && !(allowEmpty && !query)}<CommandView.Empty
          >{emptyResult}</CommandView.Empty
        >{/if}
      <CommandView.Group heading={groupLabel}>
        {#if allowEmpty && !query}<CommandView.Item
            value="__no_repository__"
            {disabled}
            onSelect={() => choose("")}
          >
            <CommandView.ItemIcon
              >{#if multiple ? !values.length : !value}<CheckIcon
                  aria-hidden="true"
                />{/if}</CommandView.ItemIcon
            >
            <CommandView.ItemLabel>{emptyLabel}</CommandView.ItemLabel>
          </CommandView.Item>{/if}
        {#each visible as repo (repo.id)}
          <CommandView.Item
            value={repo.id}
            disabled={disabled ||
              repo.disabled ||
              (multiple && !values.includes(repo.id) && values.length >= limit)}
            data-repository-selected={multiple
              ? values.includes(repo.id)
              : repo.id === value}
            keywords={[repo.name, repo.id]}
            onSelect={() => choose(repo.id)}
          >
            <CommandView.ItemIcon
              >{#if keepItemIcon && itemIcon}{@render itemIcon(
                  repo.id,
                )}{:else if multiple ? values.includes(repo.id) : repo.id === value}<CheckIcon
                  aria-hidden="true"
                />{:else if itemIcon}{@render itemIcon(
                  repo.id,
                )}{:else}<FolderGitIcon
                  aria-hidden="true"
                />{/if}</CommandView.ItemIcon
            >
            <CommandView.ItemLabel class="truncate" title={repo.name}
              >{repo.name}</CommandView.ItemLabel
            >
            <CommandView.ItemDescription
              >{repo.description ??
                `${repo.id.slice(0, 8)}${repo.permission ? ` · ${repo.permission} access` : ""}`}</CommandView.ItemDescription
            >
            {#if keepItemIcon && (multiple ? values.includes(repo.id) : repo.id === value)}<CheckIcon
                aria-hidden="true"
              />{/if}
          </CommandView.Item>
        {/each}
      </CommandView.Group>
      {#if hasMore}<CommandView.Group heading={`More ${moreLabel}`}
          ><CommandView.Item
            value="__load_more__"
            disabled={disabled || loading}
            onSelect={() => onLoadMore?.()}
            ><CommandView.ItemLabel
              >{loading
                ? `Loading ${moreLabel}…`
                : `Load more ${moreLabel}`}</CommandView.ItemLabel
            ></CommandView.Item
          ></CommandView.Group
        >{/if}
    </CommandView.List>
  </CommandView.Root>
{/snippet}

{#if inline}
  <div class="store-repository-picker__command">{@render command()}</div>
{:else}
  <Popover.Root {open} onOpenChange={handleOpen}>
    <Popover.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          {id}
          type="button"
          {variant}
          size={iconOnly ? "icon-sm" : "default"}
          role="combobox"
          class={[
            "store-repository-picker__trigger",
            variant === "ghost" && "store-repository-picker__trigger--ghost",
            iconOnly && "store-repository-picker__trigger--icon",
            props.class,
          ]}
          aria-label={label}
          aria-expanded={open}
          {disabled}
          title={iconOnly && selected
            ? `${selected.name}${selected.description ? ` · ${selected.description}` : ""}`
            : value || emptyLabel}
        >
          {#if triggerIcon}{@render triggerIcon()}{:else}<FolderGitIcon
              aria-hidden="true"
            />{/if}
          {#if !iconOnly}<span class="store-repository-picker__value"
              >{(multiple
                ? values.length
                  ? `${values.length} ${selectionLabel} selected`
                  : emptyLabel
                : selected?.name) ??
                (value
                  ? `${unavailableLabel} · ${value.slice(0, 8)}`
                  : emptyLabel)}</span
            >
            <ChevronDownIcon aria-hidden="true" />{/if}
        </Button>
      {/snippet}
    </Popover.Trigger>
    <Popover.Content
      align="start"
      class="store-repository-picker__command"
      style="--ui-popover-width: min(26rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
    >
      <Popover.Title class="sr-only">Select {label.toLowerCase()}</Popover.Title
      >
      {@render command()}
    </Popover.Content>
  </Popover.Root>
{/if}

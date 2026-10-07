<script lang="ts">
  import "./search-command.css";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import Search from "@lucide/svelte/icons/search";
  import Hash from "@lucide/svelte/icons/hash";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import CircleDot from "@lucide/svelte/icons/circle-dot";
  import Message from "@lucide/svelte/icons/message-circle";
  import History from "@lucide/svelte/icons/history";
  import File from "@lucide/svelte/icons/file-code-2";
  import Commit from "@lucide/svelte/icons/git-commit-horizontal";
  import Plus from "@lucide/svelte/icons/plus";
  import { tick, type Snippet } from "svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { CommunitySearchRow, CommunitySearchAction } from "./model.js";
  let {
    open = $bindable(false),
    query = "",
    results,
    status = "idle",
    errors = [],
    now,
    scopeLabel,
    scopeKind,
    activeScopeLabel,
    onScope,
    onClearScope,
    onQueryChange,
    onSelect,
    actions = [],
    onOpenChangeComplete,
    footer,
    sections = [],
  }: {
    open?: boolean;
    query?: string;
    results: readonly CommunitySearchRow[];
    status?: string;
    errors?: readonly string[];
    now: number;
    scopeLabel?: string;
    scopeKind?: CommunitySearchRow["kind"];
    activeScopeLabel?: string;
    onScope?: () => void;
    onClearScope?: () => void;
    onQueryChange: (value: string) => void;
    onSelect: (id: string) => void;
    actions?: readonly CommunitySearchAction[];
    sections?: readonly CommunitySearchAction[];
    onOpenChangeComplete?: (open: boolean) => void;
    footer?: Snippet;
  } = $props();
  let selected = $state(""),
    input = $state<HTMLInputElement | null>(null);
  const icons = {
    thread: Hash,
    message: Message,
    project: FolderKanban,
    task: CircleDot,
    comment: Message,
    repository: FolderGit,
    operation: History,
    commit: Commit,
    file: File,
  };
  const groups = {
    thread: "Threads",
    project: "Projects",
    task: "Tasks",
    message: "Messages",
    comment: "Comments",
    repository: "Repositories",
    operation: "Operations",
    commit: "Commits",
    file: "Files",
  };
  const visible = $derived(query.trim() ? results : results.slice(0, 4));
  async function focusAfter(action: (() => void) | undefined) {
    selected = "";
    action?.();
    await tick();
    input?.focus();
  }
</script>

<Dialog.Root bind:open {onOpenChangeComplete}
  ><Dialog.Content
    preventScroll={false}
    class="community-search-dialog"
    showCloseButton={false}
    aria-describedby={undefined}
  >
    <Dialog.Title class="sr-only"
      >Search {activeScopeLabel ?? "workspace"}</Dialog.Title
    >
    <CommandView.Root
      class="community-search-command"
      label="Search workspace"
      bind:value={selected}
      shouldFilter={false}
    >
      <CommandView.Input
        bind:ref={input}
        class="community-search-command__input"
        value={query}
        placeholder="Search threads, projects, tasks, repositories and code"
        aria-label="Search workspace"
        aria-controls="community-search-command-results"
        oninput={(e) => onQueryChange(e.currentTarget.value)}
      >
        {#snippet start()}<Search
            aria-hidden="true"
          />{#if activeScopeLabel}<CommunityChip
              variant="muted"
              tone="accent"
              ariaLabel={`Remove ${activeScopeLabel} search scope`}
              onclick={() => focusAfter(onClearScope)}
              >{activeScopeLabel}<span aria-hidden="true">×</span
              ></CommunityChip
            >{/if}{/snippet}
      </CommandView.Input>
      <Dialog.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="outline"
            size="sm"
            class="community-search-command__escape"
            aria-label="Close search">Esc</Button
          >{/snippet}</Dialog.Close
      >
      <CommandView.List id="community-search-command-results">
        {#if !activeScopeLabel && scopeLabel && onScope}<CommandView.Group
            ><CommandView.Item
              class="community-search-command__scope-action"
              value="scope:context"
              onSelect={() => focusAfter(onScope)}
              ><CommandView.ItemIcon
                >{@render actionIcon(scopeKind)}</CommandView.ItemIcon
              ><CommandView.ItemLabel
                >Search in {scopeLabel}</CommandView.ItemLabel
              ><CommandView.ItemDescription
                >Search within this section</CommandView.ItemDescription
              ></CommandView.Item
            ></CommandView.Group
          >{/if}
        {#if status === "loading"}<CommandView.Group
            ><CommandView.Item value="loading" disabled
              ><CommandView.ItemLabel
                ><span role="status">Searching…</span></CommandView.ItemLabel
              ></CommandView.Item
            ></CommandView.Group
          >{/if}
        {#if !query.trim() && visible.length}<CommandView.Group
            heading="Recent activity"
            >{#each visible as result (result.id)}{@render resultItem(
                result,
              )}{/each}</CommandView.Group
          >
        {:else if query.trim()}{#each Object.entries(groups) as [kind, heading]}{@const rows =
              visible.filter(
                (r) => r.kind === kind,
              )}{#if rows.length}<CommandView.Group {heading}
                >{#each rows as result (result.id)}{@render resultItem(
                    result,
                  )}{/each}</CommandView.Group
              >{/if}{/each}{/if}
        {#if (status === "empty" || status === "ready") && query.trim() && !visible.length}<CommandView.Empty
            >No matching content</CommandView.Empty
          >{/if}
        {#if errors.length}<CommandView.Group heading="Unavailable sources"
            >{#each errors as error}<CommandView.Item
                value={`error:${error}`}
                disabled
                ><CommandView.ItemDescription
                  ><span role="alert">{error}</span
                  ></CommandView.ItemDescription
                ></CommandView.Item
              >{/each}</CommandView.Group
          >{/if}
        {#if sections.length}<CommandView.Group heading="Search sections"
            >{#each sections as section}<CommandView.Item
                value={section.id}
                onSelect={() => focusAfter(section.run)}
                ><CommandView.ItemIcon
                  >{@render actionIcon(section.kind)}</CommandView.ItemIcon
                ><CommandView.ItemLabel>{section.label}</CommandView.ItemLabel
                ></CommandView.Item
              >{/each}</CommandView.Group
          >{/if}
        {#if actions.length}<CommandView.Group heading="Actions"
            >{#each actions as action}<CommandView.Item
                value={action.id}
                onSelect={action.run}
                ><CommandView.ItemIcon
                  >{@render actionIcon(
                    action.kind,
                    action.create,
                  )}</CommandView.ItemIcon
                ><CommandView.ItemLabel>{action.label}</CommandView.ItemLabel
                ></CommandView.Item
              >{/each}</CommandView.Group
          >{/if}
      </CommandView.List>
      {#if footer}<CommandView.Footer>{@render footer()}</CommandView.Footer
        >{/if}
    </CommandView.Root>
  </Dialog.Content></Dialog.Root
>
{#snippet actionIcon(
  kind: CommunitySearchRow["kind"] | undefined,
  create = false,
)}
  {#if kind}
    {@const Icon = icons[kind]}
    <span class="community-search-command__browse-icon" aria-hidden="true">
      <Icon />
      {#if create}<Plus />{:else}<Search />{/if}
    </span>
  {:else if create}<Plus aria-hidden="true" />{:else}<Search
      aria-hidden="true"
    />{/if}
{/snippet}
{#snippet resultItem(result: CommunitySearchRow)}
  {@const Icon = icons[result.kind]}
  <CommandView.Item
    class="community-search-command__message-result"
    value={result.id}
    keywords={[result.title, result.description]}
    onSelect={() => onSelect(result.id)}
  >
    <CommandView.ItemIcon><Icon aria-hidden="true" /></CommandView.ItemIcon
    ><CommandView.ItemLabel>{result.title}</CommandView.ItemLabel
    ><CommandView.ItemDescription
      >{result.source || groups[result.kind]}</CommandView.ItemDescription
    >
    {#if result.timestamp}<span
        data-ui-component="command-view"
        data-ui-part="trailing"
        ><CommunityRelativeTime timestamp={result.timestamp} {now} /></span
      >{/if}
    {#if result.description}<span
        class="community-search-command__message-excerpt"
        >{result.description}</span
      >{/if}
  </CommandView.Item>
{/snippet}

<script lang="ts">
  // Community repository command composition, native project/task choices.
  import StoreWorkField from "./StoreWorkField.svelte";
  import { untrack } from "svelte";
  import { pretty, type Model } from "../../api/client.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import XIcon from "@lucide/svelte/icons/x";
  import StoreWorkTaskStateIcon from "./StoreWorkTaskStateIcon.svelte";
  import { taskStateLabel } from "./work-presentation.js";
  import type { WorkBrowseTransport } from "../../controller/projects-hub-types.js";
  type Choice = {
    id: string;
    title: string;
    state?: Model<"TaskV1">["state"];
    repository?: string | null;
    task_counts?: Model<"WorkProjectSummary">["task_counts"];
  };
  let {
    transport,
    project,
    kind = "tasks",
    label,
    value = $bindable(""),
    selectedLabel = "",
    excluded = "",
    disabled = false,
    rows: localRows = [],
    repositoryName = (id: string) => id.slice(0, 8),
  }: {
    transport?: WorkBrowseTransport | undefined;
    project?: string;
    kind?: "projects" | "tasks";
    label: string;
    value?: string;
    selectedLabel?: string;
    excluded?: string;
    disabled?: boolean;
    rows?: readonly Choice[];
    repositoryName?: (id: string) => string;
  } = $props();
  const fieldId = $props.id();
  const emptyLabel = $derived(
    kind === "tasks" ? "No parent" : "Choose project",
  );
  let open = $state(false),
    search = $state(""),
    rows = $state<Choice[]>([]),
    chosen = $state<Choice>(),
    cursor = $state<string | null>(null),
    loading = $state(false),
    error = $state("");
  const visible = $derived(
    transport
      ? rows
      : localRows.filter(
          (row) =>
            row.id !== excluded &&
            `${row.title} ${row.id}`
              .toLocaleLowerCase()
              .includes(search.toLocaleLowerCase()),
        ),
  );
  const selected = $derived(
    visible.find((row) => row.id === value) ??
      localRows.find((row) => row.id === value) ??
      (chosen?.id === value ? chosen : undefined),
  );
  let generation = 0,
    reads = new AbortController(),
    pin = "";
  async function load(more = false) {
    if (!transport) return;
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation;
    loading = true;
    error = "";
    try {
      const page = await transport.browse(
        {
          kind,
          ...(project ? { project } : {}),
          q: search,
          limit: 50,
          ...(more && cursor ? { cursor } : {}),
        },
        reads.signal,
      );
      if (ticket !== generation) return;
      if (more && pin !== pretty(page.snapshot))
        throw new Error("Snapshot changed. Refresh the list.");
      pin = pretty(page.snapshot);
      const items = (kind === "tasks" ? page.tasks : page.projects).filter(
        (row) => row.id !== excluded,
      );
      rows = more ? [...rows, ...items] : items;
      cursor = page.next_cursor;
    } catch (cause) {
      if (ticket === generation)
        error = cause instanceof Error ? cause.message : "Choices unavailable";
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  $effect(() => {
    transport;
    project;
    kind;
    search;
    excluded;
    if (!open) return;
    untrack(() => void load());
    return () => {
      generation++;
      reads.abort();
    };
  });
  function choose(row?: Choice) {
    value = row?.id ?? "";
    chosen = row;
    open = false;
  }
  function details(row: Choice) {
    return row.state
      ? `${taskStateLabel(row.state)} · ${row.repository ? repositoryName(row.repository) : "Project only"} · ${row.id.slice(0, 8)}`
      : `${row.task_counts?.total ?? 0} tasks · ${row.id.slice(0, 8)}`;
  }
</script>

<StoreWorkField {label} id={fieldId} {disabled}>
  <Popover.Root
    {open}
    onOpenChange={(next) => {
      open = next;
      if (next) search = "";
    }}
  >
    <Popover.Trigger>
      {#snippet child({ props })}<Button
          {...props}
          id={fieldId}
          type="button"
          variant="outline"
          role="combobox"
          aria-label={label}
          aria-expanded={open}
          class={["store-work-picker__trigger", props.class]}
          title={value || emptyLabel}
          {disabled}
        >
          {#if kind === "tasks"}<ListTodoIcon
              aria-hidden="true"
            />{:else}<FolderKanbanIcon aria-hidden="true" />{/if}
          <span
            >{selected?.title ??
              (value
                ? selectedLabel ||
                  `Unavailable ${kind === "tasks" ? "task" : "project"} · ${value.slice(0, 8)}`
                : emptyLabel)}</span
          >
          <ChevronDownIcon aria-hidden="true" />
        </Button>{/snippet}
    </Popover.Trigger>
    <Popover.Content
      align="start"
      class="store-work-picker__command"
      style="--ui-popover-width: min(36rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
    >
      <Popover.Header class="store-work-picker__header">
        <Popover.Title>Select {label.toLowerCase()}</Popover.Title>
        <Popover.Close
          >{#snippet child({ props })}<Button
              {...props}
              type="button"
              variant="ghost"
              size="icon-sm"
              aria-label="Close work selector"
              ><XIcon aria-hidden="true" /></Button
            >{/snippet}</Popover.Close
        >
      </Popover.Header>
      <CommandView.Root
        label={`Select ${label.toLowerCase()}`}
        shouldFilter={false}
        loop
      >
        <CommandView.Input
          bind:value={search}
          placeholder={`Search ${kind}`}
          aria-label={`Search ${kind}`}
        />
        <CommandView.List aria-busy={loading}>
          {#if !visible.length && search && !loading && !error}<CommandView.Empty
              >No matching {kind}.</CommandView.Empty
            >{/if}
          <CommandView.Group heading={kind === "tasks" ? "Tasks" : "Projects"}>
            {#if !search}<CommandView.Item
                value="__no_work__"
                onSelect={() => choose()}
              >
                <CommandView.ItemIcon
                  >{#if !value}<CheckIcon
                      aria-hidden="true"
                    />{/if}</CommandView.ItemIcon
                >
                <CommandView.ItemLabel>{emptyLabel}</CommandView.ItemLabel>
              </CommandView.Item>{/if}
            {#each visible as row (row.id)}<CommandView.Item
                value={row.id}
                keywords={[row.title, row.id]}
                disabled={loading}
                title={row.id}
                onSelect={() => choose(row)}
              >
                <CommandView.ItemIcon
                  >{#if row.state}<StoreWorkTaskStateIcon
                      state={row.state}
                    />{:else}<FolderKanbanIcon
                      aria-hidden="true"
                    />{/if}</CommandView.ItemIcon
                >
                <CommandView.ItemLabel>{row.title}</CommandView.ItemLabel>
                <CommandView.ItemDescription
                  >{details(row)}</CommandView.ItemDescription
                >
                {#if row.id === value}<CommandView.Shortcut
                    ><CheckIcon aria-label="Selected" /></CommandView.Shortcut
                  >{/if}
              </CommandView.Item>{/each}
          </CommandView.Group>
        </CommandView.List>
        {#if loading || error || cursor}<CommandView.Footer>
            {#if loading}<span role="status">Loading choices…</span>{/if}
            {#if error}<p role="alert">{error}</p>
              <Button variant="outline" onclick={() => load()}
                >Retry choices</Button
              >{/if}
            {#if cursor}<Button
                variant="outline"
                disabled={loading}
                onclick={() => load(true)}>More choices</Button
              >{/if}
          </CommandView.Footer>{/if}
      </CommandView.Root>
    </Popover.Content>
  </Popover.Root>
</StoreWorkField>

<style>
  :global(.store-work-picker__trigger) {
    width: 100%;
    min-width: 0;
    justify-content: flex-start;
  }
  :global(.store-work-picker__trigger > span) {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: start;
  }
  :global(.store-work-picker__header[data-ui-part="popover-header"]) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    margin: 0;
    padding: 0;
  }
  :global(.store-work-picker__command[data-ui-part="popover-content"]) {
    --ui-command-view-list-max-height: min(18rem, 50dvh);
    max-width: calc(100vw - 2rem);
  }
</style>

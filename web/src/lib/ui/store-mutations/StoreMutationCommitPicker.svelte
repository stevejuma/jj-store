<script lang="ts">
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import XIcon from "@lucide/svelte/icons/x";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import { TagList } from "@lapismd/design-core/shadcn/tag-list";
  import { commitTags } from "./commit-tags.js";
  import StoreMutationCommitTarget from "./StoreMutationCommitTarget.svelte";

  let {
    id = "store-mutation-target",
    label,
    target = $bindable(),
    revisions,
    now,
    onChange,
  }: {
    id?: string;
    label: string;
    target: string;
    revisions: readonly RepositoryLogRevision[];
    now?: number;
    onChange: () => void;
  } = $props();

  let open = $state(false);
  let query = $state("");
  let highlighted = $state("");
  const selected = $derived(
    revisions.find((revision) => revision.eventId === target),
  );
  const preview = $derived(
    revisions.find((revision) => revision.eventId === highlighted) ?? selected,
  );
  const visible = $derived.by(() => {
    const search = query.trim().toLocaleLowerCase();
    if (!search) return revisions;
    return revisions.filter((revision) =>
      searchableRevision(revision).includes(search),
    );
  });

  function searchableRevision(revision: RepositoryLogRevision): string {
    return [
      revision.message,
      revision.author,
      revision.authorEmail,
      revision.eventId,
      revision.changeId,
      ...revision.bookmarks.flatMap((bookmark) => [
        bookmark.name,
        bookmark.remote,
      ]),
      ...revision.workspaces,
    ]
      .filter(Boolean)
      .join(" ")
      .toLocaleLowerCase();
  }

  function handleOpen(next: boolean) {
    open = next;
    if (next) {
      query = "";
      highlighted = target;
    }
  }

  function choose(revision: RepositoryLogRevision) {
    target = revision.eventId;
    onChange();
    open = false;
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <div class="store-mutation-commit-picker__control">
    <Popover.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          {id}
          variant="outline"
          class={["store-mutation-commit-picker__trigger", props.class]}
          role="combobox"
          aria-label={label}
          aria-describedby={`${id}-value`}
        >
          <ChevronDownIcon
            class="store-mutation-commit-picker__chevron"
            aria-hidden="true"
          />
        </Button>
      {/snippet}
    </Popover.Trigger>
    <div id={`${id}-value`} class="store-mutation-commit-picker__value">
      <StoreMutationCommitTarget
        {target}
        revision={selected}
        {now}
        copyableIds={false}
        framed={false}
        ariaLabel=""
      />
    </div>
  </div>
  <Popover.Content
    class="store-mutation-commit-picker"
    align="start"
    sideOffset={6}
    style="--ui-popover-width: min(42rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Header class="store-mutation-commit-picker__header">
      <Popover.Title class="store-mutation-commit-picker__title"
        >Select {label.toLocaleLowerCase()}</Popover.Title
      >
      <Popover.Close>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close commit selector"
          >
            <XIcon aria-hidden="true" />
          </Button>
        {/snippet}
      </Popover.Close>
    </Popover.Header>
    <CommandView.Root
      bind:value={highlighted}
      label={`Select ${label.toLocaleLowerCase()}`}
      shouldFilter={false}
    >
      <CommandView.Input
        bind:value={query}
        placeholder="Find commit"
        aria-label="Find commit"
      />
      <CommandView.List>
        {#if visible.length === 0}
          <CommandView.Empty>No matching commits</CommandView.Empty>
        {:else}
          <CommandView.Group heading="Commits">
            {#each visible as revision (revision.eventId)}
              <CommandView.Item
                value={revision.eventId}
                keywords={[searchableRevision(revision)]}
                class="store-mutation-commit-picker__option"
                data-current={revision.eventId === target || undefined}
                onSelect={() => choose(revision)}
              >
                <span class="store-mutation-commit-picker__selection">
                  {#if revision.eventId === target}<CheckIcon
                      aria-hidden="true"
                    />{/if}
                </span>
                <StoreMutationCommitTarget
                  target={revision.eventId}
                  {revision}
                  {now}
                  copyableIds={false}
                  showTags={false}
                  framed={false}
                  ariaLabel=""
                />
              </CommandView.Item>
            {/each}
          </CommandView.Group>
        {/if}
      </CommandView.List>
      <CommandView.Footer>
        <section aria-label="Commit tag preview" data-commit={preview?.eventId}>
          <TagList tags={commitTags(preview)} label="Preview commit tags" />
        </section>
      </CommandView.Footer>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>

<style>
  :global(.store-mutation-commit-picker__trigger[data-ui-component="button"]) {
    width: 100%;
    height: auto;
    min-height: 3.25rem;
    padding: 0.5rem;
    position: absolute;
    inset: 0;
    justify-content: flex-end;
    color: var(--foreground);
    text-align: start;
  }

  .store-mutation-commit-picker__control {
    position: relative;
    min-width: 0;
  }
  .store-mutation-commit-picker__value {
    position: relative;
    padding: 0.5rem 2rem 0.5rem 0.5rem;
    pointer-events: none;
  }
  .store-mutation-commit-picker__value :global([data-ui-component="tag-list"]) {
    pointer-events: auto;
  }

  :global(.store-mutation-commit-picker__chevron) {
    width: 1rem;
    height: 1rem;
    flex: none;
    color: var(--muted-foreground);
  }

  :global(
    .store-mutation-commit-picker[data-ui-part="popover-content"]
      [data-ui-component="command-view"][data-ui-part="root"]
  ) {
    --ui-command-view-list-max-height: min(24rem, 54dvh);
    overflow: visible;
  }

  :global(
    .store-mutation-commit-picker__header[data-ui-part="popover-header"]
  ) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    min-height: 0;
    padding: 0;
    margin: 0;
  }

  :global(.store-mutation-commit-picker__title[data-ui-part="popover-title"]) {
    margin: 0;
    line-height: 1.2;
  }

  :global(
    .store-mutation-commit-picker__option[data-ui-component="command-view"]
  ) {
    grid-template-columns: auto minmax(0, 1fr);
    gap: 0.5rem;
    cursor: pointer;
  }

  .store-mutation-commit-picker__selection {
    display: inline-flex;
    width: 1rem;
    height: 1rem;
    align-items: center;
    justify-content: center;
  }

  .store-mutation-commit-picker__selection :global(svg) {
    width: 0.9rem;
    height: 0.9rem;
  }
</style>

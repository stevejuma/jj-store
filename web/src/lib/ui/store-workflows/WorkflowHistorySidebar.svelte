<script lang="ts">
  import "./workflow-history-tree.css";
  import WorkflowActionsMenu from "./WorkflowActionsMenu.svelte";
  import type { WorkflowEntryActions } from "./workflow-actions.js";
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Layers from "@lucide/svelte/icons/layers";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import type { WorkflowHistoryEntry } from "./workflow-history.svelte.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import { workflowRepositoryGroups } from "./workflow-history-presentation.js";
  let {
    entries,
    selection,
    repositoryName,
    onSelect,
    sourceRepository,
    workflowActions,
    canWrite,
  }: {
    entries: readonly WorkflowHistoryEntry[];
    selection: WorkflowSelection;
    sourceRepository?: string;
    repositoryName(id: string): string;
    onSelect(fields: WorkflowSelection): void;
    workflowActions: WorkflowEntryActions;
    canWrite(home: string): boolean;
  } = $props();
  const groups = $derived(workflowRepositoryGroups(entries, repositoryName));
  let expanded = $state<Record<string, boolean>>({});
  $effect(() => {
    const selected = groups.find(
      (group) =>
        group.id === selection.source ||
        group.entries.some(
          (entry) =>
            entry.home === selection.scope &&
            entry.workflow.id === selection.workflow,
        ),
    );
    if (selected)
      untrack(() => {
        expanded = { ...expanded, [selected.id]: true };
      });
  });
</script>

<nav class="workflow-history-tree" aria-label="Workflows by repository">
  <h2>Workflows</h2>
  <Button
    variant="ghost"
    class="workflow-history-tree__item"
    aria-current={!selection.workflow &&
    !selection.scope &&
    (!selection.source || !!sourceRepository)
      ? "page"
      : undefined}
    onclick={() =>
      onSelect({ scope: undefined, workflow: undefined, source: undefined })}
  >
    <Layers data-icon="inline-start" aria-hidden="true" /><span
      >All workflows</span
    >
  </Button>
  {#each groups as group (group.id)}
    {@const open = expanded[group.id] ?? true}
    <Collapsible.Root
      {open}
      onOpenChange={(value) => (expanded = { ...expanded, [group.id]: value })}
      role="group"
      aria-label={`${group.name} workflows`}
    >
      <div class="workflow-history-tree__header">
        <Button
          variant="ghost"
          class="workflow-history-tree__repository"
          aria-label={`Show runs for ${group.name}`}
          title={group.name}
          aria-current={!sourceRepository &&
          selection.source === group.id &&
          !selection.workflow
            ? "page"
            : undefined}
          onclick={() =>
            onSelect({
              source: group.id,
              scope: undefined,
              workflow: undefined,
            })}
        >
          <FolderGit data-icon="inline-start" aria-hidden="true" /><span
            >{group.name}</span
          >
        </Button>
        <Collapsible.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-xs"
              class="workflow-history-tree__toggle"
              aria-label={`${open ? "Collapse" : "Expand"} ${group.name} workflows`}
              ><ChevronRight
                class="workflow-history-tree__chevron"
                aria-hidden="true"
              /></Button
            >{/snippet}</Collapsible.Trigger
        >
      </div>
      <Collapsible.Content>
        <div class="workflow-history-tree__children">
          {#each group.entries as entry (`${entry.home}:${entry.workflow.id}`)}
            <div class="workflow-history-tree__workflow">
              <Button
                variant="ghost"
                class="workflow-history-tree__item"
                title={`${entry.workflow.definition.name} · ${repositoryName(entry.home)}`}
                aria-current={selection.scope === entry.home &&
                selection.workflow === entry.workflow.id
                  ? "page"
                  : undefined}
                onclick={() =>
                  onSelect({
                    scope: entry.home,
                    workflow: entry.workflow.id,
                    source: undefined,
                  })}
              >
                <Workflow data-icon="inline-start" aria-hidden="true" /><span
                  >{entry.workflow.definition.name}</span
                >
              </Button>
              <WorkflowActionsMenu
                workflow={entry.workflow}
                writable={canWrite(entry.home)}
                busy={entry.work.state.writing || !!entry.work.state.pending}
                onOpen={() => workflowActions.onOpen(entry)}
                onToggle={() => workflowActions.onToggle(entry)}
                onDuplicate={() => workflowActions.onOpen(entry, true)}
                onRemove={() => workflowActions.onRemove(entry)}
                onRun={() => workflowActions.onRun(entry)}
              />
            </div>
          {/each}
        </div>
      </Collapsible.Content>
    </Collapsible.Root>
  {/each}
</nav>

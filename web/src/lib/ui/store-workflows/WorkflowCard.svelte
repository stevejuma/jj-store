<script lang="ts">
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import WorkflowActionsMenu from "./WorkflowActionsMenu.svelte";
  import WorkflowIcon from "@lucide/svelte/icons/workflow";
  import Bot from "@lucide/svelte/icons/bot";
  import Terminal from "@lucide/svelte/icons/terminal";
  import ArrowRight from "@lucide/svelte/icons/arrow-right";
  import type { WorkflowVersion } from "../../controller/workflow-types.js";
  import {
    workflowCardSummary,
    workflowTriggerIcons,
  } from "./workflow-card-presentation.js";
  import "./workflow-card.css";
  let {
    workflow,
    repositoryName,
    busy = false,
    writable = true,
    onOpen,
    onToggle,
    onDuplicate,
    onRemove,
    onRun,
  }: {
    workflow: WorkflowVersion;
    repositoryName: string;
    busy?: boolean;
    writable?: boolean;
    onOpen(): void;
    onToggle(): void;
    onDuplicate(): void;
    onRemove(): void;
    onRun(): void;
  } = $props();
  const summary = $derived(workflowCardSummary(workflow.definition));
  const TriggerIcon = $derived(
    workflowTriggerIcons[workflow.definition.trigger.on],
  );
</script>

<article class="workflow-card" aria-label={workflow.definition.name}>
  <div class="workflow-card__top">
    <div class="workflow-card__tiles" aria-hidden="true">
      <span
        class="workflow-card__tile workflow-card__trigger"
        data-trigger={workflow.definition.trigger.on}><TriggerIcon /></span
      ><ArrowRight /><span
        class="workflow-card__action-stack"
        data-count={Math.min(workflow.definition.actions.length, 3)}
        >{#each workflow.definition.actions.slice(0, 3) as action, index (action.id)}<span
            class="workflow-card__tile"
            data-layer={index}
            data-executor={action.executor.type}
            >{#if action.executor.type === "agent"}<Bot />{:else}<Terminal
              />{/if}</span
          >{/each}</span
      >
    </div>
    <div class="workflow-card__controls">
      <Switch
        checked={workflow.definition.enabled}
        disabled={busy || !writable}
        aria-label={`${workflow.definition.enabled ? "Disable" : "Enable"} ${workflow.definition.name}`}
        onCheckedChange={onToggle}
      />
      <WorkflowActionsMenu
        {workflow}
        {busy}
        {writable}
        {onOpen}
        {onToggle}
        {onDuplicate}
        {onRemove}
        {onRun}
      />
    </div>
  </div>
  <button
    class="workflow-card__open"
    onclick={onOpen}
    aria-label={`View ${workflow.definition.name}`}
    title={summary}><h2>{summary}</h2></button
  >
  <footer>
    <div class="workflow-card__context">
      <span
        ><WorkflowIcon aria-hidden="true" /><span title={repositoryName}
          >{repositoryName}</span
        ></span
      >
      <p title={workflow.definition.name}>{workflow.definition.name}</p>
    </div>
    <time datetime={new Date(Number(workflow.created_at)).toISOString()}
      >{new Date(Number(workflow.created_at)).toLocaleDateString()}</time
    >
  </footer>
</article>

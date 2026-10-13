<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import WorkflowIcon from "@lucide/svelte/icons/workflow";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Bot from "@lucide/svelte/icons/bot";
  import Terminal from "@lucide/svelte/icons/terminal";
  import ArrowRight from "@lucide/svelte/icons/arrow-right";
  import More from "@lucide/svelte/icons/ellipsis";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Play from "@lucide/svelte/icons/play";
  import Copy from "@lucide/svelte/icons/copy";
  import Trash from "@lucide/svelte/icons/trash-2";
  import type { WorkflowVersion } from "../../controller/workflow-types.js";
  import { triggerLabel } from "./workflow-presentation.js";
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
</script>

<article class="workflow-card" aria-label={workflow.definition.name}>
  <div class="workflow-card__top">
    <div class="workflow-card__tiles" aria-hidden="true">
      <span class="workflow-card__tile workflow-card__trigger"
        ><GitBranch /></span
      ><ArrowRight
      />{#each workflow.definition.actions.slice(0, 3) as action (action.id)}<span
          class="workflow-card__tile"
          >{#if action.executor.type === "agent"}<Bot />{:else}<Terminal
            />{/if}</span
        >{/each}
    </div>
    <div class="workflow-card__controls">
      <Switch
        checked={workflow.definition.enabled}
        disabled={busy || !writable}
        aria-label={`${workflow.definition.enabled ? "Disable" : "Enable"} ${workflow.definition.name}`}
        onCheckedChange={onToggle}
      />
      <Menu.Root
        ><Menu.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label={`Actions for ${workflow.definition.name}`}
              ><More /></Button
            >{/snippet}</Menu.Trigger
        ><Menu.Content align="end" class="workflow-card-menu"
          ><Menu.Group
            ><Menu.Item onclick={onOpen}
              ><Pencil aria-hidden="true" />Edit workflow</Menu.Item
            ><Menu.Item disabled={!writable || busy} onclick={onRun}
              ><Play aria-hidden="true" />Run workflow</Menu.Item
            ><Menu.Item disabled={!writable || busy} onclick={onDuplicate}
              ><Copy aria-hidden="true" />Duplicate</Menu.Item
            ><Menu.Separator /><Menu.Item
              disabled={!writable || busy}
              variant="destructive"
              onclick={onRemove}
              ><Trash aria-hidden="true" />Remove workflow</Menu.Item
            ></Menu.Group
          ></Menu.Content
        ></Menu.Root
      >
    </div>
  </div>
  <button class="workflow-card__open" onclick={onOpen}
    ><h2>{workflow.definition.name}</h2>
    <p>{triggerLabel(workflow.definition.trigger)}</p>
    {#if workflow.definition.description}<p>
        {workflow.definition.description}
      </p>{/if}</button
  >
  <footer>
    <span><WorkflowIcon aria-hidden="true" />{repositoryName}</span
    ><CommunityChip
      size="compact"
      tone={workflow.definition.enabled ? "success" : "neutral"}
      >{workflow.definition.enabled ? "Enabled" : "Disabled"}</CommunityChip
    >
  </footer>
</article>

<style>
  :global(.workflow-card-menu[data-slot="dropdown-menu-content"]) {
    min-width: min(15rem, calc(100vw - 2rem));
    max-width: calc(100vw - 2rem);
  }
  :global(.workflow-card-menu [data-slot="dropdown-menu-item"]) {
    white-space: nowrap;
  }
</style>

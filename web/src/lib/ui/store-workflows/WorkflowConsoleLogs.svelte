<script lang="ts">
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import type { WorkflowEvidence } from "../../controller/workflow-types.js";
  import { WorkflowLogLines, consoleLogRows } from "./workflow-log-lines.js";
  let {
    chunks,
    seconds = false,
    timestamps = false,
    started,
  }: {
    chunks: readonly WorkflowEvidence[];
    seconds?: boolean;
    timestamps?: boolean;
    started?: number;
  } = $props();
  const buffer = new WorkflowLogLines();
  const lines = $derived(consoleLogRows(buffer.read(chunks)));
  let expanded = $state<Set<string>>(new Set());
  function toggle(id: string) {
    expanded = new Set(expanded);
    expanded.has(id) ? expanded.delete(id) : expanded.add(id);
  }
</script>

<div class="workflow-console-logs" aria-label="Step logs">
  {#each lines as line, index (line.id)}
    <div
      class="workflow-console-line"
      aria-label={`${line.stream} log`}
      hidden={line.parents.some((id) => !expanded.has(id))}
    >
      <span class="workflow-console-line__number" aria-hidden="true"
        >{index + 1}</span
      >
      {#if timestamps}<time
          class="workflow-console-line__time"
          datetime={new Date(line.at).toISOString()}
          >{new Date(line.at).toLocaleTimeString()}</time
        >{/if}
      <span
        class="workflow-console-line__text"
        style:padding-left={`${line.parents.length}em`}
      >
        {#if line.group}<button
            class="workflow-console-group"
            aria-expanded={expanded.has(line.id)}
            onclick={() => toggle(line.id)}
          >
            {#if expanded.has(line.id)}<ChevronDown
                aria-hidden="true"
              />{:else}<ChevronRight aria-hidden="true" />{/if}{@html line.html}
          </button>{:else}{@html line.html}{/if}
      </span>
      {#if seconds}<span class="workflow-console-line__seconds"
          >{started === undefined
            ? "—"
            : `${Math.max(0, Math.floor((line.at - started) / 1000))}s`}</span
        >{/if}
    </div>
  {/each}
</div>

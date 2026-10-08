<script lang="ts">
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  let { preferences }: { preferences: AgentSetup["preferences"] } = $props();
</script>

{#if preferences.model || preferences.effort || preferences.mode}
  <dl class="agent-editor__runtime-defaults">
    {#each Object.entries(preferences) as [name, value] (name)}
      <div>
        <dt>
          {name === "effort"
            ? "Thinking"
            : name === "mode"
              ? "Execution mode"
              : "Model"}
        </dt>
        <dd>{value || "Harness default"}</dd>
      </div>
    {/each}
  </dl>
{/if}

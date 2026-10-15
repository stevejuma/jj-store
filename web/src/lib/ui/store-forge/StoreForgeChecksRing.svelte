<script lang="ts">
  import type { ForgeCheck } from "../../controller/forge/types.js";
  import { checkGroups } from "../../controller/forge/checks-presentation.svelte.js";
  let { checks, state }: { checks: readonly ForgeCheck[]; state: string } =
    $props();
  const groups = $derived(checkGroups(checks));
  const total = $derived(Math.max(1, checks.length));
</script>

<svg
  viewBox="0 0 36 36"
  width="30"
  height="30"
  class="forge-checks-ring"
  data-state={state}
  aria-hidden="true"
>
  <circle
    cx="18"
    cy="18"
    r="15.9"
    fill="none"
    stroke="var(--border)"
    stroke-width="3.5"
  />
  {#each groups as group, index}
    <circle
      class="forge-checks-ring__segment"
      data-state={state === "unknown" ? "unknown" : group.id}
      cx="18"
      cy="18"
      r="15.9"
      fill="none"
      stroke-width="3.5"
      pathLength="100"
      stroke-dasharray={`${Math.max(0, (group.checks.length / total) * 100 - 2)} 100`}
      stroke-dashoffset={-groups
        .slice(0, index)
        .reduce((n, g) => n + (g.checks.length / total) * 100, 0)}
    />
  {/each}
</svg>

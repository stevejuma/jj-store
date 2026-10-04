<script lang="ts">
  // CommunityProjects.svelte taskStatusIcon, adapted to native lifecycle values.
  import CircleIcon from "@lucide/svelte/icons/circle";
  import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
  import CircleXIcon from "@lucide/svelte/icons/circle-x";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import type { Model } from "../../api/client.js";
  import { taskStateLabel } from "./work-presentation.js";
  let { state }: { state: Model<"TaskStateV1"> } = $props();
</script>

<span
  class="community-projects__task-status-icon"
  data-status={state}
  title={taskStateLabel(state)}
>
  {#if state === "blocked"}<CircleXIcon aria-hidden="true" />
  {:else if state === "done"}<CircleCheckIcon aria-hidden="true" />
  {:else if state === "in_progress"}<CircleDotIcon aria-hidden="true" />
  {:else}<CircleIcon aria-hidden="true" />{/if}
  <span class="sr-only">{taskStateLabel(state)}</span>
</span>

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  let {
    owners,
    repositoryName,
  }: {
    owners: readonly (readonly [string, WorkflowController])[];
    repositoryName(id: string): string;
  } = $props();
</script>

{#each owners.filter(([, owner]) => owner.state.page?.next) as [home, owner] (home)}
  <Button
    variant="outline"
    disabled={owner.state.loading}
    onclick={() => owner.more()}
    >More workflows in {repositoryName(home)}</Button
  >
{/each}

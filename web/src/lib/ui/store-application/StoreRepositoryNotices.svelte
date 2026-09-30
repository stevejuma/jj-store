<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const error = $derived(
    state.route.workspace && state.status.navigation === "failed"
      ? ""
      : state.status.error,
  );
</script>

{#if state.comparisonParents.length || error}
  <section class="store-repository-notices" aria-label="Repository notices">
    {#if state.comparisonParents.length}<Alert.Root>
        <Alert.Title>Choose comparison parent</Alert.Title>
        <Alert.Description
          >This merge operation has multiple recorded predecessors.</Alert.Description
        >
        {#each state.comparisonParents as parent}<Button
            variant="outline"
            onclick={() => controller.actions.selectComparisonParent(parent)}
            >{parent.slice(0, 12)}</Button
          >{/each}
      </Alert.Root>{/if}
    {#if error}<Alert.Root>
        <TriangleAlertIcon aria-hidden="true" />
        <Alert.Title>Repository request failed</Alert.Title>
        <Alert.Description>{error}</Alert.Description>
        {#if error.includes("index rebuild required")}<Button
            variant="outline"
            onclick={() => controller.actions.prepareIndex()}
            >Prepare index</Button
          >{/if}
      </Alert.Root>{/if}
  </section>
{/if}

<style>
  .store-repository-notices {
    display: grid;
    gap: 0.5rem;
    flex: none;
    padding: 0.75rem 1rem;
  }
</style>

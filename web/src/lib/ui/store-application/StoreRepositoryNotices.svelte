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

{#if error}
  <section class="store-repository-notices" aria-label="Repository notices">
    {#if error}<Alert.Root>
        <TriangleAlertIcon aria-hidden="true" />
        <Alert.Title>Repository request failed</Alert.Title>
        <Alert.Description>{error}</Alert.Description>
        {#if error.includes("index rebuild required")}<Button
            variant="outline"
            onclick={() => controller.actions.prepareIndex()}
            >Prepare graph index</Button
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

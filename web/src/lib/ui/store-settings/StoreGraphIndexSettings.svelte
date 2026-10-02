<script lang="ts">
  import NetworkIcon from "@lucide/svelte/icons/network";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const supported = $derived(settings.capabilities?.index_tasks ?? false);
  const preparing = $derived(settings.indexPreparation === "loading");
</script>

<section
  class="store-settings__section"
  aria-labelledby="repository-graph-index"
>
  <header class="store-settings__section-heading">
    <div>
      <h2 id="repository-graph-index">Graph index</h2>
      <p>Prepare the selected operation for repository history browsing.</p>
    </div>
    {#if supported}
      <Button
        variant="outline"
        disabled={!settings.indexOperation || preparing}
        onclick={() => void controller.actions.prepareIndex()}
        ><NetworkIcon data-icon="inline-start" aria-hidden="true" />{preparing
          ? "Preparing graph index…"
          : "Prepare graph index"}</Button
      >
    {/if}
  </header>
  <p>Text search has a separate index, managed in Search settings.</p>
  {#if !supported}
    <StoreSettingsEmpty
      Icon={NetworkIcon}
      title="Graph preparation unavailable"
      description="Graph preparation is not available."
    />
  {:else if !settings.indexOperation}
    <p class="store-settings__empty">
      Choose one operation head before preparing the graph index.
    </p>
  {/if}
  {#if preparing}
    <p role="status">Preparing the graph index…</p>
  {:else if settings.indexPreparation === "ready"}
    <p role="status">Graph index preparation completed.</p>
  {/if}
  {#if settings.indexPreparationError}
    <Alert.Root variant="destructive">
      <Alert.Title>Graph index preparation failed</Alert.Title>
      <Alert.Description>{settings.indexPreparationError}</Alert.Description>
    </Alert.Root>
  {/if}
</section>

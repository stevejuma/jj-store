<script lang="ts">
  import NetworkIcon from "@lucide/svelte/icons/network";
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
    <h2 id="repository-graph-index">Graph index</h2>
  </header>
  <dl class="store-settings__rows">
    <div>
      <dt>
        Repository history
        <p class="store-settings__row-description">
          Prepare the selected operation for repository history browsing.
        </p>
      </dt>
      <dd>
        {#if supported}
          <div class="store-settings__actions">
            <Button
              variant="outline"
              disabled={!settings.indexOperation || preparing}
              onclick={() => void controller.actions.prepareIndex()}
              ><NetworkIcon
                data-icon="inline-start"
                aria-hidden="true"
              />{preparing
                ? "Preparing graph index…"
                : "Prepare graph index"}</Button
            >
          </div>
        {:else}<span>Graph preparation is not available.</span>{/if}
      </dd>
    </div>
    <div>
      <dt>Text-search index</dt>
      <dd>Text search has a separate index, managed in Search settings.</dd>
    </div>
    {#if supported && !settings.indexOperation}<div>
        <dt>Selected operation</dt>
        <dd>Choose one operation head before preparing the graph index.</dd>
      </div>{/if}
    {#if preparing}<div>
        <dt>Status</dt>
        <dd><span role="status">Preparing the graph index…</span></dd>
      </div>
    {:else if settings.indexPreparation === "ready"}<div>
        <dt>Status</dt>
        <dd><span role="status">Graph index preparation completed.</span></dd>
      </div>{/if}
  </dl>
  {#if settings.indexPreparationError}
    <Alert.Root variant="destructive">
      <Alert.Title>Graph index preparation failed</Alert.Title>
      <Alert.Description>{settings.indexPreparationError}</Alert.Description>
    </Alert.Root>
  {/if}
</section>

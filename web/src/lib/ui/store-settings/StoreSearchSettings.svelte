<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const capabilities = $derived(settings.capabilities);
  const search = $derived(settings.searchStatus);
  const needsIndex = $derived(
    Boolean(
      search &&
      (search.is_stale || search.degraded || search.indexed_position === null),
    ),
  );
  const canPrepareIndex = $derived(
    Boolean(
      capabilities?.search &&
      capabilities.index_tasks &&
      settings.indexOperation &&
      settings.indexPreparation !== "loading",
    ),
  );
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
</script>

<section class="store-settings__section" aria-labelledby="search-health">
  <header class="store-settings__section-heading">
    <div>
      <h2 id="search-health">Projection health</h2>
      <p>Positions update only when status is explicitly refreshed.</p>
    </div>
    {#if capabilities?.search}
      <Button
        variant="outline"
        disabled={settings.searchStatusState === "loading" ||
          settings.searchStatusState === "refreshing"}
        onclick={() => void controller.actions.refreshSettings()}
        >Refresh status</Button
      >
    {/if}
  </header>
  {#if !capabilities?.search}
    <p class="store-settings__empty">
      Search is not configured for this repository.
    </p>
  {:else if settings.searchStatusState === "loading" && !search}
    <p class="store-settings__empty" role="status">Loading search status…</p>
  {:else}
    <dl class="store-settings__rows">
      <div>
        <dt>Configured</dt>
        <dd>Yes</dd>
      </div>
      <div>
        <dt>Health</dt>
        <dd>
          <span
            class="store-settings__status"
            data-tone={search?.degraded
              ? "error"
              : search?.is_stale
                ? "warning"
                : "ready"}
            >{search?.degraded
              ? "Degraded"
              : search?.is_stale
                ? "Stale"
                : search
                  ? "Fresh"
                  : "Unavailable"}</span
          >
        </dd>
      </div>
      <div>
        <dt>Repository position</dt>
        <dd>{number(search?.repository_position)}</dd>
      </div>
      <div>
        <dt>Durable position</dt>
        <dd>{number(search?.durable_position)}</dd>
      </div>
      <div>
        <dt>Indexed position</dt>
        <dd>{number(search?.indexed_position)}</dd>
      </div>
      <div>
        <dt>Index generation</dt>
        <dd><code>{search?.index_generation ?? "Not available"}</code></dd>
      </div>
    </dl>
  {/if}
  {#if settings.searchStatusError}
    <Alert.Root variant="destructive">
      <Alert.Title>Search status unavailable</Alert.Title>
      <Alert.Description>{settings.searchStatusError}</Alert.Description>
    </Alert.Root>
  {/if}
  {#if settings.indexPreparationError}
    <Alert.Root variant="destructive">
      <Alert.Title>Index preparation failed</Alert.Title>
      <Alert.Description>{settings.indexPreparationError}</Alert.Description>
    </Alert.Root>
  {/if}
  {#if needsIndex || settings.indexPreparation === "loading"}
    <div class="store-settings__action-row">
      <div>
        <strong>Prepare search index</strong>
        <p>
          {settings.indexOperation
            ? "Prepare the selected immutable operation for search."
            : "Choose one operation head before preparing the index."}
        </p>
      </div>
      <Button
        disabled={!canPrepareIndex}
        onclick={() => void controller.actions.prepareIndex()}
        >{settings.indexPreparation === "loading"
          ? "Preparing…"
          : "Prepare index"}</Button
      >
    </div>
  {/if}
</section>

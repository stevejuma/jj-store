<script lang="ts">
  import { indexPreparationLabels } from "../../controller/index-preparation-labels.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const supported = $derived(settings.capabilities?.index_tasks ?? false);
  const label = $derived(
    indexPreparationLabels[settings.indexPreparationKind ?? "graph"],
  );
  const kindSupported = $derived(
    supported &&
      !!controller.actions.prepareKindIndex &&
      controller.state.repository.profile !== "git_sha1_jj045",
  );
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
              />{preparing &&
              (settings.indexPreparationKind ?? "graph") === "graph"
                ? "Preparing graph index…"
                : "Prepare graph index"}</Button
            >
            {#if preparing && (settings.indexPreparationKind ?? "graph") === "graph"}
              <Button
                variant="outline"
                disabled={settings.indexTaskStatus !== "running"}
                onclick={() => void controller.actions.cancelIndex()}
                >{settings.indexTaskStatus === "canceling"
                  ? "Canceling…"
                  : "Cancel preparation"}</Button
              >
            {/if}
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
    {#if settings.indexProgress}<div>
        <dt>Preparation progress</dt>
        <dd aria-live="polite">
          {settings.indexProgress.matching_ids.toLocaleString()} matching identities
          in
          {settings.indexProgress.stats.windows.toLocaleString()} windows;
          {settings.indexProgress.stats.scanned.toLocaleString()} descriptors inspected.
        </dd>
      </div>{/if}
    {#if settings.indexCancellationUnconfirmed}<div>
        <dt>Cancellation</dt>
        <dd role="status">
          {settings.indexPreparationError} Retry cancellation below.
        </dd>
      </div>{/if}
    {#if preparing}<div>
        <dt>Status</dt>
        <dd>
          <span role="status"
            >{settings.indexTaskStatus === "canceling"
              ? `Canceling ${label.toLowerCase()} preparation…`
              : `Preparing the ${label.toLowerCase()}…`}</span
          >
        </dd>
      </div>
    {:else if settings.indexTaskStatus === "canceled"}<div>
        <dt>Status</dt>
        <dd><span role="status">{label} preparation canceled.</span></dd>
      </div>
    {:else if settings.indexPreparation === "ready"}<div>
        <dt>Status</dt>
        <dd><span role="status">{label} preparation completed.</span></dd>
      </div>{/if}
  </dl>
  {#if kindSupported}
    {#each ["operations", "copies"] as kind}
      <div class="store-settings__action-row">
        <div>
          <strong
            >{kind === "operations"
              ? "Operation inventory"
              : "Copy inventory"}</strong
          >
          <p>
            Prepare the selected operation's {kind === "operations"
              ? "operation"
              : "copy"} identities for bounded repository reads.
          </p>
        </div>
        <div class="store-settings__actions">
          <Button
            variant="outline"
            disabled={!settings.indexOperation || preparing}
            onclick={() =>
              void controller.actions.prepareKindIndex?.(
                kind as "operations" | "copies",
              )}
            >{preparing && settings.indexPreparationKind === kind
              ? "Preparing"
              : "Prepare"}
            {kind === "operations" ? "operation" : "copy"} inventory{preparing &&
            settings.indexPreparationKind === kind
              ? "…"
              : ""}</Button
          >
          {#if preparing && settings.indexPreparationKind === kind}
            <Button
              variant="outline"
              disabled={settings.indexTaskStatus !== "running"}
              onclick={() => void controller.actions.cancelIndex()}
              >{settings.indexTaskStatus === "canceling"
                ? "Canceling…"
                : "Cancel preparation"}</Button
            >
          {/if}
        </div>
      </div>
    {/each}
  {/if}
  <StoreNotice
    noticeKey="graph-index"
    title={`${label} preparation failed`}
    message={settings.indexCancellationUnconfirmed
      ? ""
      : settings.indexPreparationError}
    pending={false}
  />
</section>

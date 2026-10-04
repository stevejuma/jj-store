<script lang="ts">
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  const notifications = ownNotifications();
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreIndexPreparationNotice from "../store-application/StoreIndexPreparationNotice.svelte";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const supported = $derived(settings.capabilities?.index_tasks ?? false);
  const automatic = $derived(
    settings.capabilities?.automatic_code_index === true,
  );
  const kindSupported = $derived(
    supported &&
      !automatic &&
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
          {automatic
            ? "Repository history is prepared automatically when opened and kept current as it changes."
            : "Prepare the selected operation for repository history browsing."}
        </p>
      </dt>
      <dd>
        {#if automatic && settings.indexPreparation !== "failed" && !preparing}
          <span>Automatic</span>
        {:else if supported}
          <div class="store-settings__actions">
            <Button
              variant="outline"
              disabled={!settings.indexOperation || preparing}
              onclick={() => void controller.actions.prepareIndex()}
              ><NetworkIcon data-icon="inline-start" aria-hidden="true" />
              {automatic
                ? "Retry history preparation"
                : "Prepare graph index"}</Button
            >
          </div>
        {/if}
      </dd>
    </div>
    <div>
      <dt>Text-search index</dt>
      <dd>Text search has a separate index, managed in Search settings.</dd>
    </div>
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
            >Prepare {kind === "operations" ? "operation" : "copy"} inventory</Button
          >
        </div>
      </div>
    {/each}
  {/if}
  {#if notifications.owned}<StoreIndexPreparationNotice {controller} />{/if}
  <StoreNotice
    noticeKey="index-operation-selection"
    title="Selected operation"
    severity="warning"
    message={supported && !settings.indexOperation
      ? "Choose one operation head before preparing the graph index."
      : ""}
  />
  <StoreNotice
    noticeKey="index-capability"
    title="Graph preparation unavailable"
    severity="info"
    message={!supported ? "Graph preparation is not available." : ""}
  />
</section>

<StoreNotificationHost owner={notifications} />

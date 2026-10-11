<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import SearchIcon from "@lucide/svelte/icons/search";
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  const notifications = ownNotifications();
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreIndexPreparationNotice from "../store-application/StoreIndexPreparationNotice.svelte";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const presentation = settingsPresentation();
  const repository = $derived(presentation.value === "repository");
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

{#snippet prepareIndex()}<Button
    variant="outline"
    disabled={!settings.indexOperation || preparing}
    onclick={() => void controller.actions.prepareIndex()}
    ><NetworkIcon data-icon="inline-start" aria-hidden="true" />
    {automatic ? "Retry history preparation" : "Prepare graph index"}</Button
  >{/snippet}

<StoreSettingsCard
  id="repository-graph-index"
  title="Graph index"
  description="Repository history and text-search indexing."
  Icon={DatabaseIcon}
>
  <dl class="store-settings__rows">
    {#if repository}<StoreReadOnlyRow
        label="Repository history"
        Icon={NetworkIcon}
        description={automatic
          ? "Repository history is prepared automatically when opened and kept current as it changes."
          : "Prepare the selected operation for repository history browsing."}
        value={automatic
          ? "Automatic"
          : supported
            ? "On demand"
            : "Unavailable"}
      />
      <StoreReadOnlyRow
        label="Text-search index"
        Icon={SearchIcon}
        value="Text search has a separate index, managed in Search settings."
      />
    {:else}
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
            <CommunityChip variant="subtle" tone="success" size="compact"
              >Automatic</CommunityChip
            >
          {:else if supported && !repository}
            <div class="store-settings__actions">
              {@render prepareIndex()}
            </div>
          {/if}
        </dd>
      </div>
      <div>
        <dt>Text-search index</dt>
        <dd>Text search has a separate index, managed in Search settings.</dd>
      </div>
    {/if}
  </dl>
  {#if repository && (kindSupported || (supported && (!automatic || settings.indexPreparation === "failed" || preparing)))}
    <StoreSettingsFooter>
      {#snippet description()}<div>
          <strong>Index maintenance</strong>
          <p>
            Prepare the selected operation for repository history{kindSupported
              ? ", operation inventory or copy inventory"
              : ""}. Repository text search has its own maintenance controls.
          </p>
        </div>{/snippet}
      {#if kindSupported}{#each ["operations", "copies"] as kind}<Button
            variant="outline"
            disabled={!settings.indexOperation || preparing}
            onclick={() =>
              void controller.actions.prepareKindIndex?.(
                kind as "operations" | "copies",
              )}
            >Prepare {kind === "operations" ? "operation" : "copy"} inventory</Button
          >{/each}{/if}
      {#if supported && (!automatic || settings.indexPreparation === "failed" || preparing)}{@render prepareIndex()}{/if}
    </StoreSettingsFooter>
  {/if}
  {#if kindSupported && !repository}
    {#each ["operations", "copies"] as kind}
      <StoreSettingsFooter>
        {#snippet description()}<div>
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
          </div>{/snippet}

        <Button
          variant="outline"
          disabled={!settings.indexOperation || preparing}
          onclick={() =>
            void controller.actions.prepareKindIndex?.(
              kind as "operations" | "copies",
            )}
          >Prepare {kind === "operations" ? "operation" : "copy"} inventory</Button
        >
      </StoreSettingsFooter>
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
</StoreSettingsCard>

<StoreNotificationHost owner={notifications} />

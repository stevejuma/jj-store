<script lang="ts">
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import XIcon from "@lucide/svelte/icons/x";
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreConfigurationFields from "./StoreConfigurationFields.svelte";
  import StoreSyncConfiguration from "./StoreSyncConfiguration.svelte";
  import {
    equal,
    type Section,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    section,
    repository = "",
    repositoryName = "",
  }: {
    management?: ServerSettingsManagement;
    section: Exclude<Section, "workflow">;
    repository?: string;
    repositoryName?: string;
  } = $props();
  const presentation = settingsPresentation();
  const modernPresentation = $derived(presentation.value !== "default");
  const edit = $derived(management?.state.drafts[section]);
  const busy = $derived(management?.state.saving ?? false);
  const pending = $derived(
    management?.state.value &&
      !equal(
        management.state.value.desired[section],
        management.state.value.effective[section],
      ),
  );
  const title = $derived(
    section === "service"
      ? "Service resource limits"
      : section === "protection"
        ? "Installation protection rules"
        : "Sync configuration",
  );
</script>

<StoreSettingsCard
  id={`config-${section}`}
  layout="page"
  {title}
  description="Local administrator settings. Save changes, then use Reload saved settings to apply them while the server stays online."
  Icon={SlidersHorizontalIcon}
>
  {#if !management?.available()}<p>
      Configuration editing requires a local service with administrator access.
    </p>
  {:else}
    <StoreNotice
      scope="installation-server-settings"
      noticeKey={`server-config-${section}`}
      title="Configuration needs attention"
      message={management.state.error || edit?.error}
      pending={management.state.loading || busy}
    />
    <StoreNotice
      scope="installation-server-settings"
      noticeKey={`server-config-${section}-saved`}
      title="Settings saved"
      severity="info"
      lifetime="timed"
      message={edit?.message}
    />
    {#if edit?.draft}
      <form
        onsubmit={(event) => {
          event.preventDefault();
          void management.save(section);
        }}
      >
        <div class={modernPresentation ? undefined : "store-settings__rows"}>
          {#if !modernPresentation}<dl class="store-settings__summary-rows">
              <div>
                <dt>Saved and running</dt>
                <dd>
                  <span role="status">
                    {pending
                      ? "Saved changes need a configuration reload."
                      : "Saved settings match the running configuration."}
                  </span>
                </dd>
              </div>
            </dl>{/if}
          {#if section === "sync"}<StoreSyncConfiguration
              {management}
              {repository}
              {repositoryName}
              pending={Boolean(pending)}
            />{:else}
            <StoreSettingsCard
              id={`config-${section}-status`}
              title="Configuration status"
              Icon={SlidersHorizontalIcon}
              repositoryOnly
            >
              <dl class="store-settings__rows">
                <StoreReadOnlyRow
                  label="Saved and running"
                  live="status"
                  value={pending
                    ? "Saved changes need a configuration reload."
                    : "Saved settings match the running configuration."}
                />
              </dl>
            </StoreSettingsCard>
            <StoreSettingsCard
              id={`config-${section}-fields`}
              title={section === "service"
                ? "Resource limits"
                : "Protection rules"}
              Icon={SlidersHorizontalIcon}
              repositoryOnly
              ><StoreConfigurationFields
                {management}
                {section}
              /></StoreSettingsCard
            >
          {/if}
          {#if edit.validation}<p role="alert">{edit.validation}</p>{/if}
          {#if edit.conflict}<p role="status">
              The saved file changed while you were editing. Your draft is
              retained. Review the latest saved values before replacing the
              draft.
            </p>{/if}
          {#if edit.unknown}<p role="status">
              The save response was not confirmed. Refresh to inspect the saved
              configuration before another save.
            </p>{/if}
          {#snippet saveButton()}<Button
              type="submit"
              disabled={busy ||
                !edit.dirty ||
                edit.conflict ||
                Boolean(edit.unknown)}
              >{#if modernPresentation}<SaveIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{/if}Save settings</Button
            >{/snippet}
          {#snippet cancelButton()}<Button
              type="button"
              variant="outline"
              disabled={busy ||
                !edit.dirty ||
                (Boolean(edit.unknown) && !edit.unknownObserved)}
              onclick={() => management.cancel(section)}
              >{#if modernPresentation}<XIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{/if}{edit.conflict || edit.unknown
                ? "Use latest saved settings"
                : "Cancel changes"}</Button
            >{/snippet}
          <StoreSettingsFooter joined standalone>
            {#snippet description()}<div>
                <strong>Save configuration</strong>
                <p>
                  Current workers retain their settings until configuration
                  reload.
                </p>
              </div>{/snippet}

            {#if modernPresentation}{@render cancelButton()}{@render saveButton()}{:else}{@render saveButton()}{@render cancelButton()}{/if}
          </StoreSettingsFooter>
        </div>
      </form>
      {#if edit.conflict || edit.unknown}<details>
          <summary>Latest saved values</summary>
          <pre class="store-settings__configuration-preview">{JSON.stringify(
              management.state.value?.desired[section],
              null,
              2,
            )}</pre>
        </details>{/if}
    {:else if modernPresentation && management.state.loading}
      <StoreSettingsSkeleton
        section="sync"
        cards={section === "sync"
          ? undefined
          : globalSettingsSkeletonCards(
              "service",
              section === "service" ? "limits" : "protection",
            )}
        tab="configuration"
        heading={false}
        label={`Loading ${section === "sync" ? "sync" : section === "service" ? "service" : "protection"} configuration`}
      />
    {:else}<p role="status">
        {management.state.loading
          ? "Loading configuration…"
          : "Refresh settings to load configuration."}
      </p>{/if}
  {/if}
</StoreSettingsCard>

<style>
  .store-settings__configuration-preview {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    font-size: 0.8rem;
  }
</style>

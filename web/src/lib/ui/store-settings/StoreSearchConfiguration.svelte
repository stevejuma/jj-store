<script lang="ts">
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import StoreSearchFields from "./StoreSearchFields.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  import WrenchIcon from "@lucide/svelte/icons/wrench";
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import SearchIcon from "@lucide/svelte/icons/search";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import SearchXIcon from "@lucide/svelte/icons/search-x";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import XIcon from "@lucide/svelte/icons/x";
  import CheckIcon from "@lucide/svelte/icons/check";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { searchOperating } from "../../controller/production-search-settings.js";
  import type {
    SearchSettings,
    SearchDraft,
    SearchAction,
  } from "../../controller/search-settings.js";
  let {
    settings,
    supported,
    section = "configuration",
    onDraft,
    onCancel,
    onSave,
    onControl,
  }: {
    settings: SearchSettings;
    supported: boolean;
    section?: "configuration" | "scopes" | "advanced";
    onDraft: (patch: Partial<SearchDraft>) => void;
    onCancel: () => void;
    onSave: () => Promise<void>;
    onControl: (action: SearchAction) => Promise<void>;
  } = $props();
  const busy = $derived(
    settings.changing || searchOperating(settings.value?.operation),
  );
  const ready = $derived(settings.state === "ready" && !busy);
  const pending = $derived(
    settings.value &&
      JSON.stringify(settings.value.desired) !==
        JSON.stringify(settings.value.effective),
  );
  const presentation = settingsPresentation();
  const repository = $derived(presentation.value === "repository");
</script>

{#snippet saveButton()}
  <Button type="submit" disabled={!ready || !settings.dirty}
    ><SaveIcon data-icon="inline-start" aria-hidden="true" />Save settings</Button
  >{/snippet}
{#snippet cancelButton()}<Button
    type="button"
    variant="outline"
    disabled={busy || !settings.dirty}
    onclick={onCancel}
    ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel changes</Button
  >{/snippet}
{#snippet maintenanceDescription()}<div>
    <strong>Index maintenance</strong>
    <p>
      Apply verifies a replacement before activation. Rebuild recreates the text
      index from committed data. Repository work stays available.
    </p>
  </div>{/snippet}

<StoreSettingsCard
  id="search-configuration"
  layout="page"
  title={section === "configuration"
    ? "Text-search configuration"
    : section === "scopes"
      ? "File search scopes"
      : "Search worker limits"}
  description="Save your settings, then apply them to this repository."
  Icon={SearchIcon}
>
  {#if !supported}<StoreSettingsEmpty
      Icon={SearchXIcon}
      title="Search editing unavailable"
      description="Search editing is not available for this connection."
    />
  {:else}
    {#if settings.draft}
      <form
        onsubmit={(event) => {
          event.preventDefault();
          void onSave();
        }}
      >
        <div class={repository ? undefined : "store-settings__rows"}>
          {#if settings.value}
            <StoreSettingsCard
              repositoryOnly
              id="search-index-status"
              title="Index status"
              description="Saved settings, active configuration and the current operation."
              Icon={ActivityIcon}
            >
              <dl class="store-settings__summary-rows">
                {#each [{ label: "Saved", value: settings.value.desired.enabled ? "Enabled" : "Disabled" }, { label: "Currently active", value: `${settings.value.effective.enabled ? "Enabled" : "Disabled"}${pending ? " · Saved changes need applying" : " · Matches saved settings"}` }, { label: "Operation", value: settings.value.operation }] as row}
                  {#if repository}<StoreReadOnlyRow
                      label={row.label}
                      value={row.value}
                      live={row.label === "Operation" ? "status" : undefined}
                    />
                  {:else}<div>
                      <dt>{row.label}</dt>
                      <dd>
                        {#if row.label === "Operation"}<span role="status"
                            >{row.value}</span
                          >{:else}{row.value}{/if}
                      </dd>
                    </div>{/if}
                {/each}
              </dl>
            </StoreSettingsCard>
          {/if}
          <StoreSettingsCard
            repositoryOnly
            id="search-options"
            title={section === "configuration"
              ? "Search options"
              : section === "scopes"
                ? "File scopes"
                : "Worker limits"}
            Icon={SearchIcon}
          >
            <StoreSearchFields
              draft={settings.draft}
              effective={settings.value?.effective}
              {busy}
              {section}
              {onDraft}
            />
            <StoreSettingsFooter>
              {#snippet description()}<div>
                  <strong>Save configuration</strong>
                  <p>
                    Saving preserves the running configuration until Apply or
                    service restart.
                  </p>
                </div>{/snippet}
              {#if repository}{@render cancelButton()}{@render saveButton()}{:else}{@render saveButton()}{@render cancelButton()}{/if}
            </StoreSettingsFooter>
          </StoreSettingsCard>
        </div>
      </form>
      <StoreSettingsCard
        repositoryOnly
        id="search-index-maintenance"
        title="Index maintenance"
        description={repository
          ? "Apply verifies a replacement before activation. Rebuild recreates the text index from committed data. Repository work stays available."
          : undefined}
        Icon={WrenchIcon}
      >
        <StoreSettingsFooter
          description={repository ? undefined : maintenanceDescription}
        >
          <Button
            variant="outline"
            disabled={!ready || settings.dirty || !pending}
            onclick={() => void onControl("apply")}
            ><CheckIcon data-icon="inline-start" aria-hidden="true" />Apply
            saved settings</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("retry")}
            ><RotateCwIcon data-icon="inline-start" aria-hidden="true" />Retry
            indexing</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("rebuild")}
            ><DatabaseIcon data-icon="inline-start" aria-hidden="true" />Rebuild
            text index</Button
          >
        </StoreSettingsFooter>
      </StoreSettingsCard>
    {:else if repository && ["idle", "loading", "refreshing"].includes(settings.state)}
      <StoreSettingsSkeleton
        section="search"
        tab={section}
        heading={false}
        label="Loading search settings"
      />
    {:else if settings.changing}<StoreSettingsNotice
        >Loading search settings…</StoreSettingsNotice
      >{/if}
    {#if settings.validation}<p role="alert">{settings.validation}</p>{/if}
    <StoreNotice
      noticeKey="search-settings"
      title="Search settings need attention"
      message={settings.error || settings.value?.error}
      pending={settings.changing}
    />
  {/if}
</StoreSettingsCard>

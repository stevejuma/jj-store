<script lang="ts">
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
    section: Section;
    repository?: string;
    repositoryName?: string;
  } = $props();
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

<section class="store-settings__section" aria-labelledby={`config-${section}`}>
  <header class="store-settings__section-heading">
    <div>
      <h2 id={`config-${section}`}>{title}</h2>
      <p>
        Local administrator settings. Changes apply after server restart; in the
        desktop app, Quit and reopen.
      </p>
    </div>
  </header>
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
        <div class="store-settings__rows">
          <dl class="store-settings__summary-rows">
            <div>
              <dt>Saved and running</dt>
              <dd>
                <span role="status">
                  {pending
                    ? "Saved changes need a server restart."
                    : "Saved settings match the running configuration."}
                </span>
              </dd>
            </div>
          </dl>
          {#if section === "sync"}<StoreSyncConfiguration
              {management}
              {repository}
              {repositoryName}
            />{:else}<StoreConfigurationFields {management} {section} />{/if}
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
          <div
            class="store-settings__action-row store-settings__action-row--joined"
          >
            <div>
              <strong>Save configuration</strong>
              <p>Current workers keep their running settings until restart.</p>
            </div>
            <div class="store-settings__actions">
              <Button
                type="submit"
                disabled={busy ||
                  !edit.dirty ||
                  edit.conflict ||
                  Boolean(edit.unknown)}>Save settings</Button
              >
              <Button
                type="button"
                variant="outline"
                disabled={busy ||
                  !edit.dirty ||
                  (Boolean(edit.unknown) && !edit.unknownObserved)}
                onclick={() => management.cancel(section)}
                >{edit.conflict || edit.unknown
                  ? "Use latest saved settings"
                  : "Cancel changes"}</Button
              >
            </div>
          </div>
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
    {:else}<p role="status">
        {management.state.loading
          ? "Loading configuration…"
          : "Refresh settings to load configuration."}
      </p>{/if}
  {/if}
</section>

<style>
  .store-settings__configuration-preview {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    font-size: 0.8rem;
  }
</style>

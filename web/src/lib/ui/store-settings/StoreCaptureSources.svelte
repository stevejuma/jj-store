<script lang="ts">
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    captureProfiles,
    captureHistorySupported,
    type CaptureManagement,
  } from "../../controller/capture-types.js";
  let {
    management,
    repositoryName,
  }: { management: CaptureManagement; repositoryName: (id: string) => string } =
    $props();
  const state = $derived(management.state);
  const sources = $derived(state.sources);
  const allowed = $derived(
    management.supported &&
      !state.busy &&
      !state.pending &&
      state.status === "ready" &&
      state.sourceState === "ready" &&
      state.value?.enabled &&
      !state.value.policy?.paused,
  );
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-source-heading"
>
  <header><h3 id="capture-source-heading">Source coverage</h3></header>
  <div class="store-settings__rows store-settings__rows--controls">
    <div>
      <div>
        <strong>Observed sources</strong>
        <p class="store-settings__row-description">
          Source observations update automatically. Import history explicitly
          enrolls an inactive source; repository access is checked by the
          server.
        </p>
      </div>
      <div class="store-settings__stack">
        {#if sources}<span
            >{sources.sources.length} sources shown · {sources.total.toLocaleString(
              "en-GB",
            )} currently observed</span
          >{/if}
        {#if sources?.next_cursor}
          <div class="store-settings__actions">
            <Button
              variant="outline"
              disabled={state.busy ||
                !management.supported ||
                state.sourceState !== "ready"}
              onclick={() => management.moreSources()}
              >More capture sources</Button
            >
          </div>
        {/if}
      </div>
    </div>
  </div>
  {#if state.sourceState === "failed" || state.sourceState === "idle"}
    <StoreSettingsNotice
      >Refresh capture to select current source coverage.</StoreSettingsNotice
    >
  {/if}
  {#if sources}
    {#if sources.sources.length}
      <dl class="store-settings__rows">
        {#each sources.sources as source, index (source.selector)}
          {@const label = `${captureProfiles[source.profile]} source ${index + 1}`}
          <div>
            <dt>
              {label}
              <p class="store-settings__row-description">
                {source.product_version} · {source.phase}
              </p>
              <p class="store-settings__row-description">
                {source.last_bound_repository
                  ? repositoryName(source.last_bound_repository)
                  : "No recorded repository binding"}
              </p>
            </dt>
            <dd class="store-settings__stack">
              <span
                >{source.observed
                  ? "Observed"
                  : "Unobserved"}{source.awaiting_activity
                  ? " · Awaiting activity"
                  : ""}{source.caught_up ? " · Caught up" : ""}{source.running
                  ? " · Running"
                  : ""}{source.degraded ? " · Degraded" : ""}</span
              >
              {#if source.ingress_required_capacity_bytes !== null}<span
                  >Requires {source.ingress_required_capacity_bytes.toLocaleString(
                    "en-GB",
                  )} bytes</span
                >{/if}
              <div class="store-settings__actions">
                <Button
                  variant="outline"
                  size="sm"
                  aria-label={`Import history for ${label}`}
                  disabled={!allowed ||
                    !captureHistorySupported(source.profile) ||
                    !source.awaiting_activity ||
                    source.running}
                  onclick={() => management.importHistory(source.selector)}
                >
                  <DownloadIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />{captureHistorySupported(source.profile)
                    ? "Import history"
                    : "Import unavailable"}
                </Button>
              </div>
            </dd>
          </div>
        {/each}
      </dl>
    {:else}<StoreSettingsEmpty
        Icon={BotIcon}
        title="No capture sources"
        description="No sources are observed in this snapshot."
      />{/if}
  {/if}
</section>

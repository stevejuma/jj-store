<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
  import SaveIcon from "@lucide/svelte/icons/save";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type {
    StorageSettings,
    StorageAction,
  } from "../../controller/storage-settings.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import StoreStoragePosition from "./StoreStoragePosition.svelte";
  import StoreStorageOperation from "./StoreStorageOperation.svelte";
  let {
    settings,
    supported,
    onRefresh,
    onStart,
    onRetry,
    configuration,
    onRefreshConfiguration,
    configurationBusy = false,
  }: {
    settings: StorageSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onStart: (action: StorageAction) => Promise<void>;
    onRetry: () => Promise<void>;
    configuration?: import("svelte").Snippet;
    onRefreshConfiguration?: () => Promise<void>;
    configurationBusy?: boolean;
  } = $props();
  let active = $state("overview");
  const report = $derived(settings.operation);
  const blocked = $derived(
    settings.changing ||
      Boolean(settings.pending) ||
      Boolean(
        report && ["prepared", "running", "unknown"].includes(report.phase),
      ),
  );
</script>

<StoreSettingsPage
  section="storage"
  bind:value={active}
  onRefresh={active === "configuration"
    ? onRefreshConfiguration
    : supported
      ? onRefresh
      : undefined}
  refreshLabel={active === "maintenance" ? "Check status" : "Refresh storage"}
  refreshDisabled={active === "configuration"
    ? configurationBusy
    : settings.changing}
  tabs={[
    { value: "overview", label: "Overview" },
    ...(supported ? [{ value: "maintenance", label: "Maintenance" }] : []),
    ...(configuration
      ? [{ value: "configuration", label: "Configuration" }]
      : []),
  ]}
>
  {#if configuration}<Tabs.Content value="configuration"
      >{@render configuration()}</Tabs.Content
    >{/if}
  {#if !supported}
    <Tabs.Content value="overview"
      ><StoreSettingsEmpty
        Icon={DatabaseIcon}
        title="Storage unavailable"
        description="Open a native repository on a local service with administrator access to use Storage maintenance."
      /></Tabs.Content
    >
  {:else}
    <StoreNotice
      noticeKey="storage"
      title="Storage request failed"
      message={settings.pending ? "" : settings.error}
      pending={settings.changing ||
        ["loading", "refreshing"].includes(settings.state)}
    />
    {#if settings.pending && settings.error}<p role="status">
        {settings.error}
      </p>{/if}
    <Tabs.Content value="overview">
      <section
        class="store-settings__section"
        aria-labelledby="storage-overview"
      >
        <header class="store-settings__section-heading">
          <h2 id="storage-overview">Storage readiness</h2>
        </header>
        <dl class="store-settings__rows">
          <StoreReadOnlyRow
            label="Status"
            description="Status updates automatically. Integrity checks run only when requested."
            live="status"
            value={settings.value
              ? settings.value.ready
                ? "Selected manifest is readable"
                : "Storage unavailable"
              : settings.state === "failed"
                ? "Storage status could not be read."
                : "Loading storage status…"}
          />
          {#if settings.value}
            <div>
              <dt>Checkpoint coverage</dt>
              <dd>
                {settings.value.checkpoint
                  ? `Selected checkpoint covers position ${settings.value.checkpoint.position.toLocaleString("en-GB")}. Current position is ${settings.value.current.position.toLocaleString("en-GB")}.`
                  : "No checkpoint is selected."}
              </dd>
            </div>
            <div>
              <dt>Integrity and retention</dt>
              <dd>
                A readable manifest does not establish full data integrity.
                Checkpoints retain repository data and publication history.
              </dd>
            </div>
          {/if}
        </dl>
        {#if settings.value}<StoreStoragePosition
            position={settings.value.current}
          />{/if}
      </section>
    </Tabs.Content>
    <Tabs.Content value="maintenance">
      <section
        class="store-settings__section"
        aria-labelledby="storage-maintenance"
      >
        <header class="store-settings__section-heading">
          <h2 id="storage-maintenance">Repository maintenance</h2>
        </header>
        <div class="store-settings__action-row">
          <div>
            <strong>Integrity and checkpoints</strong>
            <p>
              Operations continue on the local service when Settings closes.
            </p>
          </div>
          <div class="store-settings__actions">
            <Button
              variant="outline"
              disabled={blocked || !settings.value}
              onclick={() => void onStart("check")}
              ><ShieldCheckIcon
                data-icon="inline-start"
                aria-hidden="true"
              />Check integrity</Button
            >
            <Button
              variant="outline"
              disabled={blocked || !settings.value}
              onclick={() => void onStart("prepare_checkpoint")}
              ><SaveIcon data-icon="inline-start" aria-hidden="true" />Prepare
              checkpoint</Button
            >
          </div>
        </div>
        {#if settings.value}<dl class="store-settings__rows">
            <div>
              <dt>Check limits</dt>
              <dd>
                Check limits: {settings.value.limits.entries.toLocaleString(
                  "en-GB",
                )} entries, {settings.value.limits.bytes.toLocaleString(
                  "en-GB",
                )} bytes, {Number(settings.value.limits.timeout_millis) / 1000} seconds.
              </dd>
            </div>
          </dl>{/if}
        {#if settings.pending && !settings.changing}
          <div class="store-settings__action-row">
            <div>
              <strong>Pending request</strong>
              <p role="status">
                The response was not confirmed. Retry uses the same request and
                target.
              </p>
            </div>
            <div class="store-settings__actions">
              <Button variant="outline" onclick={() => void onRetry()}
                ><RefreshCwIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />Retry original request</Button
              >
            </div>
          </div>
        {/if}
        {#if report}<StoreStorageOperation
            {report}
            disabled={settings.changing || Boolean(settings.pending)}
            {onStart}
          />
        {:else}<dl class="store-settings__rows">
            <div>
              <dt>Operation</dt>
              <dd>No maintenance operation has been requested.</dd>
            </div>
          </dl>{/if}
      </section>
    </Tabs.Content>
  {/if}
</StoreSettingsPage>

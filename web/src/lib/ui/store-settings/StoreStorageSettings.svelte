<script lang="ts">
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
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
    value,
    embedded = false,
    onSelect,
    settings,
    supported,
    onRefresh,
    onStart,
    onRetry,
    configuration,
    onRefreshConfiguration,
    configurationBusy = false,
  }: {
    value?: string;
    embedded?: boolean;
    onSelect?: (value: string) => void;
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
  const selected = $derived(value ?? active);
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
  {embedded}
  section="storage"
  value={selected}
  onSelect={onSelect ?? ((next) => (active = next))}
  onRefresh={selected === "configuration"
    ? onRefreshConfiguration
    : supported
      ? onRefresh
      : undefined}
  refreshLabel={selected === "maintenance" ? "Check status" : "Refresh storage"}
  refreshDisabled={selected === "configuration"
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
      <StoreSettingsCard
        id="storage-overview"
        title="Storage readiness"
        Icon={DatabaseIcon}
      >
        <dl class="store-settings__rows">
          <StoreReadOnlyRow
            label="Status"
            tone={settings.value?.ready ? "ready" : undefined}
            Icon={ShieldCheckIcon}
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
            <StoreReadOnlyRow
              label="Checkpoint coverage"
              Icon={DatabaseIcon}
              value={settings.value.checkpoint
                ? `Selected checkpoint covers position ${settings.value.checkpoint.position.toLocaleString("en-GB")}. Current position is ${settings.value.current.position.toLocaleString("en-GB")}.`
                : "No checkpoint is selected."}
            />
            <StoreReadOnlyRow
              label="Integrity and retention"
              Icon={ShieldCheckIcon}
              value="A readable manifest does not establish full data integrity. Checkpoints retain repository data and publication history."
            />
          {/if}
        </dl>
        {#if !embedded && settings.value}<StoreStoragePosition
            position={settings.value.current}
          />{/if}
      </StoreSettingsCard>
      {#if embedded && settings.value}<StoreSettingsCard
          id="storage-position"
          title="Published position"
          Icon={DatabaseIcon}
        >
          <StoreStoragePosition position={settings.value.current} />
        </StoreSettingsCard>{/if}
    </Tabs.Content>
    <Tabs.Content value="maintenance">
      <StoreSettingsCard
        id="storage-maintenance"
        title="Repository maintenance"
        Icon={DatabaseIcon}
      >
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
        <StoreSettingsFooter>
          {#snippet description()}<div>
              <strong>Integrity and checkpoints</strong>
              <p>
                Operations continue on the local service when Settings closes.
              </p>
            </div>{/snippet}

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
        </StoreSettingsFooter>
        {#if settings.pending && !settings.changing}
          <StoreSettingsFooter>
            {#snippet description()}<div>
                <strong>Pending request</strong>
                <p role="status">
                  The response was not confirmed. Retry uses the same request
                  and target.
                </p>
              </div>{/snippet}

            <Button variant="outline" onclick={() => void onRetry()}
              ><RefreshCwIcon
                data-icon="inline-start"
                aria-hidden="true"
              />Retry original request</Button
            >
          </StoreSettingsFooter>
        {/if}
      </StoreSettingsCard>
      <StoreSettingsCard
        id="storage-report"
        title="Maintenance operation"
        Icon={ShieldCheckIcon}
      >
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
      </StoreSettingsCard>
    </Tabs.Content>
  {/if}
</StoreSettingsPage>

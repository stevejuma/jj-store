<script lang="ts">
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
  import SaveIcon from "@lucide/svelte/icons/save";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
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
  }: {
    settings: StorageSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onStart: (action: StorageAction) => Promise<void>;
    onRetry: () => Promise<void>;
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
  onRefresh={supported ? onRefresh : undefined}
  refreshLabel={active === "maintenance" ? "Check status" : "Refresh storage"}
  refreshDisabled={settings.changing}
  tabs={supported
    ? [
        { value: "overview", label: "Overview" },
        { value: "maintenance", label: "Maintenance" },
      ]
    : []}
>
  {#if !supported}
    <StoreSettingsEmpty
      Icon={DatabaseIcon}
      title="Storage unavailable"
      description="Open a native repository on a local service with administrator access to use Storage maintenance."
    />
  {:else}
    {#if settings.error}<Alert.Root variant="destructive"
        ><Alert.Description>{settings.error}</Alert.Description></Alert.Root
      >{/if}
    <Tabs.Content value="overview">
      <section
        class="store-settings__section"
        aria-labelledby="storage-overview"
      >
        <header class="store-settings__section-heading">
          <h2 id="storage-overview">Storage readiness</h2>
        </header>
        <dl class="store-settings__rows">
          <div>
            <dt>
              Status
              <p class="store-settings__row-description">
                Status updates automatically. Integrity checks run only when
                requested.
              </p>
            </dt>
            <dd>
              <span role="status">
                {settings.value
                  ? settings.value.ready
                    ? "Selected manifest is readable"
                    : "Storage unavailable"
                  : settings.state === "failed"
                    ? "Storage status could not be read."
                    : "Loading storage status…"}
              </span>
            </dd>
          </div>
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

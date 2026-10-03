<script lang="ts">
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
  import SaveIcon from "@lucide/svelte/icons/save";
  import XIcon from "@lucide/svelte/icons/x";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type {
    StorageSettings,
    StorageAction,
  } from "../../controller/storage-settings.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import StoreStoragePosition from "./StoreStoragePosition.svelte";
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
  const report = $derived(settings.operation);
  const blocked = $derived(
    settings.changing ||
      Boolean(settings.pending) ||
      Boolean(
        report && ["prepared", "running", "unknown"].includes(report.phase),
      ),
  );
  const words = (value: string) => value.replaceAll("_", " ");
</script>

<StoreSettingsPage
  section="storage"
  value="overview"
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
          <div>
            <h2 id="storage-overview">Storage readiness</h2>
            <p>
              Status updates automatically. Integrity checks run only when
              requested.
            </p>
          </div>
          <Button
            variant="outline"
            disabled={settings.changing}
            onclick={() => void onRefresh()}
            ><RefreshCwIcon
              data-icon="inline-start"
              aria-hidden="true"
            />Refresh storage</Button
          >
        </header>
        {#if settings.value}
          <p role="status">
            {settings.value.ready
              ? "Selected manifest is readable"
              : "Storage unavailable"}
          </p>
          <StoreStoragePosition position={settings.value.current} />
          <h3>Checkpoint coverage</h3>
          <p>
            {settings.value.checkpoint
              ? `Selected checkpoint covers position ${settings.value.checkpoint.position.toLocaleString("en-GB")}. Current position is ${settings.value.current.position.toLocaleString("en-GB")}.`
              : "No checkpoint is selected."}
          </p>
          <p>
            A readable manifest does not establish full data integrity.
            Checkpoints retain repository data and publication history.
          </p>
        {:else}<p role="status">
            {settings.state === "failed"
              ? "Storage status could not be read."
              : "Loading storage status…"}
          </p>{/if}
      </section>
    </Tabs.Content>
    <Tabs.Content value="maintenance">
      <section
        class="store-settings__section"
        aria-labelledby="storage-maintenance"
      >
        <header class="store-settings__section-heading">
          <div>
            <h2 id="storage-maintenance">Repository maintenance</h2>
            <p>
              Operations continue on the local service when Settings closes.
            </p>
          </div>
        </header>
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
          <Button
            variant="outline"
            disabled={settings.changing}
            onclick={() => void onRefresh()}
            ><RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Check
            status</Button
          >
        </div>
        {#if settings.value}<p>
            Check limits: {settings.value.limits.entries.toLocaleString(
              "en-GB",
            )} entries, {settings.value.limits.bytes.toLocaleString("en-GB")} bytes,
            {Number(settings.value.limits.timeout_millis) / 1000} seconds.
          </p>{/if}
        {#if settings.pending && !settings.changing}
          <p role="status">
            The response was not confirmed. Retry uses the same request and
            target.
          </p>
          <Button variant="outline" onclick={() => void onRetry()}
            ><RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Retry
            original request</Button
          >
        {/if}
        {#if report}
          <h3>{report.kind === "check" ? "Integrity check" : "Checkpoint"}</h3>
          <p role="status">
            {report.kind === "check" && report.phase === "complete"
              ? "Captured data verified"
              : `Operation ${words(report.phase)}`}
          </p>
          {#if report.message}<p role="alert">{report.message}</p>{/if}
          {#if report.phase === "incomplete"}<p>
              The check did not verify all captured data within its limits.
            </p>{/if}
          {#if report.phase === "interrupted"}<p>
              The service stopped during this check. Start a new integrity check
              to verify current data.
            </p>{/if}
          {#if report.selection === "selected"}<p>
              The exact captured checkpoint is selected.
            </p>
          {:else if report.selection === "covered"}<p>
              A newer selected checkpoint covers this target. The original
              publication is not proven.
            </p>{/if}
          <CopyableValue
            value={report.request_id}
            label="Maintenance request"
          />
          <h3>Captured target</h3>
          <StoreStoragePosition position={report.target} />
          {#if report.kind === "checkpoint" && ["prepared", "unknown"].includes(report.phase)}
            <p>
              Publishing keeps newer changes and retained history. This target
              stays fixed across retries.
            </p>
            <div class="store-settings__actions">
              <Button
                disabled={settings.changing || Boolean(settings.pending)}
                onclick={() => void onStart("publish_checkpoint")}
                ><SaveIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{report.phase === "prepared"
                  ? "Publish checkpoint"
                  : "Retry checkpoint"}</Button
              >
              {#if report.phase === "prepared"}<Button
                  variant="outline"
                  disabled={settings.changing || Boolean(settings.pending)}
                  onclick={() => void onStart("cancel_checkpoint")}
                  ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel
                  checkpoint</Button
                >{/if}
            </div>
          {/if}
        {:else}<p>No maintenance operation has been requested.</p>{/if}
      </section>
    </Tabs.Content>
  {/if}
</StoreSettingsPage>

<script lang="ts">
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import PauseIcon from "@lucide/svelte/icons/pause";
  import PlayIcon from "@lucide/svelte/icons/play";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import CloudOffIcon from "@lucide/svelte/icons/cloud-off";
  import StoreSyncWait from "./StoreSyncWait.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type {
    SyncSettings,
    SyncAction,
  } from "../../controller/sync-settings.js";
  let {
    value,
    embedded = false,
    onSelect,
    settings,
    supported,
    onRefresh,
    onControl,
    onWait,
    onStopWait,
    reconciliation,
    configuration,
    configurationSupported,
    onRefreshConfiguration,
    configurationBusy = false,
  }: {
    value?: string;
    embedded?: boolean;
    onSelect?: (value: string) => void;
    settings: SyncSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onControl: (action: SyncAction) => Promise<void>;
    onWait: (capture: boolean) => Promise<void>;
    onStopWait: () => void;
    reconciliation?: import("svelte").Snippet;
    configuration?: import("svelte").Snippet;
    configurationSupported?: boolean;
    onRefreshConfiguration?: () => Promise<void>;
    configurationBusy?: boolean;
  } = $props();
  let active = $state("overview");
  const selected = $derived(value ?? active);
  const worker = $derived(settings.value?.worker);
  const busy = $derived(
    settings.changing || ["loading", "refreshing"].includes(settings.state),
  );
  const number = (value: number | bigint | null | undefined) =>
    value == null ? "Unavailable" : value.toLocaleString("en-GB");
  const words = (value: string | null | undefined) =>
    value?.replaceAll("_", " ") ?? "None";
</script>

<StoreSettingsPage
  {embedded}
  section="sync"
  onRefresh={selected === "configuration"
    ? onRefreshConfiguration
    : supported
      ? onRefresh
      : undefined}
  refreshLabel="Refresh sync"
  refreshDisabled={selected === "configuration" ? configurationBusy : busy}
  value={selected}
  onSelect={onSelect ?? ((next) => (active = next))}
  tabs={[
    { value: "overview", label: "Overview" },
    ...(supported && worker ? [{ value: "progress", label: "Progress" }] : []),
    ...(configuration && configurationSupported !== false
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
        Icon={CloudOffIcon}
        title="Sync unavailable"
        description="Sync management is not available for this connection."
      /></Tabs.Content
    >
  {:else}
    <Tabs.Content value="overview">
      <StoreSettingsCard
        id="sync-overview"
        title="Upstream sync"
        description="Status updates automatically."
        Icon={RefreshCwIcon}
      >
        {#if busy}<StoreSettingsNotice>
            {settings.changing ? "Updating sync…" : "Loading sync status…"}
          </StoreSettingsNotice>{/if}
        {#if settings.value && !settings.value.configured}
          <dl class="store-settings__rows">
            <StoreReadOnlyRow
              label="Upstream"
              value="No upstream is configured. Repository work remains local."
            />
          </dl>
        {:else if worker}
          <dl class="store-settings__rows">
            <StoreReadOnlyRow label="Worker" value={words(worker.phase)} />
            <StoreReadOnlyRow
              label="Local repository"
              value={worker.local_ready ? "Ready" : "Unavailable"}
            />
            <StoreReadOnlyRow
              label="Offline access"
              value={worker.offline_ready ? "Ready" : "Unavailable"}
            />
            <StoreReadOnlyRow
              label="Operation heads"
              value={worker.operation_heads}
              display={number(worker.operation_heads)}
            />
            <div>
              <dt>Reconciliation</dt>
              <dd>
                {worker.reconciliation_required
                  ? "Required — review the operation heads and use Reconcile operations in repository actions."
                  : "Not required"}
              </dd>
            </div>
          </dl>
          {#if reconciliation}{@render reconciliation()}{/if}
          <StoreSettingsFooter>
            {#snippet description()}<div>
                <strong>Sync controls</strong>
                <p>
                  Pause lasts until resumed or the service restarts. Local
                  repository work stays available.
                </p>
              </div>{/snippet}

            <Button
              variant="outline"
              disabled={busy || settings.state !== "ready"}
              onclick={() =>
                void onControl(worker.phase === "paused" ? "resume" : "pause")}
              >{#if worker.phase === "paused"}<PlayIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{:else}<PauseIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{/if}{worker.phase === "paused"
                ? "Resume sync"
                : "Pause sync"}</Button
            >
            <Button
              variant="outline"
              disabled={busy ||
                settings.state !== "ready" ||
                worker.phase === "paused"}
              onclick={() => void onControl("retry")}
              ><RotateCwIcon data-icon="inline-start" aria-hidden="true" />Retry
              sync</Button
            >
          </StoreSettingsFooter>
        {/if}
      </StoreSettingsCard>
    </Tabs.Content>
    <Tabs.Content value="progress">
      {#if worker}
        <StoreSettingsCard
          id="sync-progress"
          title="Replication progress"
          description="Local and peer positions belong to separate histories."
          Icon={RefreshCwIcon}
        >
          <StoreSyncWait wait={settings.wait} {onWait} {onStopWait} />
          <dl class="store-settings__rows">
            <StoreReadOnlyRow
              label="Local pending positions"
              value={worker.local_pending_positions}
              display={number(worker.local_pending_positions)}
            />
            <StoreReadOnlyRow
              label="Peer pending positions"
              value={worker.peer_pending_positions}
              display={number(worker.peer_pending_positions)}
            />
            <StoreReadOnlyRow
              label="Checkpoint revision"
              value={worker.checkpoint_revision}
              display={number(worker.checkpoint_revision)}
            />
            {#each [{ label: "Push", direction: worker.push }, { label: "Pull", direction: worker.pull }] as { label, direction }}
              <div>
                <dt>{label} health</dt>
                <dd>
                  {direction.circuit_open ? "Circuit open" : "Circuit closed"} · failures
                  {number(direction.consecutive_failures)} · {words(
                    direction.last_fault,
                  )}
                </dd>
              </div>
            {/each}
            <div>
              <dt>Search projection</dt>
              <dd>
                {worker.search.configured
                  ? worker.search.degraded
                    ? "Degraded"
                    : "Configured"
                  : "Not configured"}
              </dd>
            </div>
            <StoreReadOnlyRow
              label="Search repository / indexed position"
              value={`${number(worker.search.repository_position)} / ${number(worker.search.indexed_position)}`}
            />
          </dl>
          {#each [{ label: "Local frontier", frontier: worker.local_frontier }, { label: "Peer frontier", frontier: worker.peer_frontier }, { label: "Peer acknowledgement", frontier: worker.pushed }, { label: "Pulled frontier", frontier: worker.pulled }] as { label, frontier }}
            <h3>{label}</h3>
            {#if frontier}
              <dl class="store-settings__rows">
                <StoreReadOnlyRow
                  copyable
                  label="Replica"
                  copyLabel={`${label} replica`}
                  value={frontier.replica}
                />
                <StoreReadOnlyRow
                  copyable
                  label="Log epoch"
                  copyLabel={`${label} epoch`}
                  value={frontier.log_epoch}
                />
                <StoreReadOnlyRow
                  label="Through / observed upper bound"
                  value={`${number(frontier.through)} / ${number(frontier.observed_upper_bound)}`}
                />
              </dl>
            {:else}<p>Unavailable</p>{/if}
          {/each}
        </StoreSettingsCard>
      {/if}
    </Tabs.Content>
    <StoreNotice
      noticeKey="sync"
      title="Sync status unavailable"
      message={settings.error}
      pending={settings.changing ||
        ["loading", "refreshing"].includes(settings.state)}
    />
  {/if}
</StoreSettingsPage>

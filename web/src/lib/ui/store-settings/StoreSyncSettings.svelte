<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreSettingsIdentifier from "./StoreSettingsIdentifier.svelte";
  import type {
    SyncSettings,
    SyncAction,
  } from "../../controller/sync-settings.js";
  let {
    settings,
    supported,
    onRefresh,
    onControl,
  }: {
    settings: SyncSettings;
    supported: boolean;
    onRefresh: () => Promise<void>;
    onControl: (action: SyncAction) => Promise<void>;
  } = $props();
  const worker = $derived(settings.value?.worker);
  const busy = $derived(
    settings.changing || ["loading", "refreshing"].includes(settings.state),
  );
  const number = (value: number | bigint | null | undefined) =>
    value == null ? "Unavailable" : value.toLocaleString("en-GB");
  const words = (value: string | null | undefined) =>
    value?.replaceAll("_", " ") ?? "None";
</script>

{#if !supported}
  <p class="store-settings__empty">
    Sync management is not available for this connection.
  </p>
{:else}
  <Tabs.Root value="overview" class="store-settings__tabs">
    <Tabs.List
      aria-label="Sync settings sections"
      class="store-settings__tab-list"
    >
      <Tabs.Trigger value="overview">Overview</Tabs.Trigger>
      <Tabs.Trigger value="progress" disabled={!worker}>Progress</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="overview">
      <section class="store-settings__section" aria-labelledby="sync-overview">
        <header class="store-settings__section-heading">
          <div>
            <h2 id="sync-overview">Upstream sync</h2>
            <p>Status changes only when refreshed or a control completes.</p>
          </div>
          <Button
            variant="outline"
            disabled={busy}
            onclick={() => void onRefresh()}>Refresh sync</Button
          >
        </header>
        {#if busy}<p role="status">
            {settings.changing ? "Updating sync…" : "Loading sync status…"}
          </p>{/if}
        {#if settings.value && !settings.value.configured}
          <p>No upstream is configured. Repository work remains local.</p>
          <p class="store-settings__empty">WorkStore sync is not supported.</p>
        {:else if worker}
          <dl class="store-settings__rows">
            <div>
              <dt>Worker</dt>
              <dd>{words(worker.phase)}</dd>
            </div>
            <div>
              <dt>Local repository</dt>
              <dd>{worker.local_ready ? "Ready" : "Unavailable"}</dd>
            </div>
            <div>
              <dt>Offline access</dt>
              <dd>{worker.offline_ready ? "Ready" : "Unavailable"}</dd>
            </div>
            <div>
              <dt>Operation heads</dt>
              <dd>{number(worker.operation_heads)}</dd>
            </div>
            <div>
              <dt>Reconciliation</dt>
              <dd>
                {worker.reconciliation_required
                  ? "Required — review the operation heads and use Reconcile operations in repository actions."
                  : "Not required"}
              </dd>
            </div>
          </dl>
          <div class="store-settings__action-row">
            <div>
              <Button
                variant="outline"
                disabled={busy || settings.state !== "ready"}
                onclick={() =>
                  void onControl(
                    worker.phase === "paused" ? "resume" : "pause",
                  )}
                >{worker.phase === "paused"
                  ? "Resume sync"
                  : "Pause sync"}</Button
              >
              <Button
                variant="outline"
                disabled={busy ||
                  settings.state !== "ready" ||
                  worker.phase === "paused"}
                onclick={() => void onControl("retry")}>Retry sync</Button
              >
            </div>
            <p>
              Pause lasts until resumed or the service restarts. Local
              repository work stays available.
            </p>
          </div>
        {/if}
      </section>
    </Tabs.Content>
    <Tabs.Content value="progress">
      {#if worker}
        <section
          class="store-settings__section"
          aria-labelledby="sync-progress"
        >
          <header>
            <h2 id="sync-progress">Replication progress</h2>
            <p>Local and peer positions belong to separate histories.</p>
          </header>
          <dl class="store-settings__rows">
            <div>
              <dt>Local pending positions</dt>
              <dd>{number(worker.local_pending_positions)}</dd>
            </div>
            <div>
              <dt>Peer pending positions</dt>
              <dd>{number(worker.peer_pending_positions)}</dd>
            </div>
            <div>
              <dt>Checkpoint revision</dt>
              <dd>{number(worker.checkpoint_revision)}</dd>
            </div>
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
            <div>
              <dt>Search repository / indexed position</dt>
              <dd>
                {number(worker.search.repository_position)} / {number(
                  worker.search.indexed_position,
                )}
              </dd>
            </div>
          </dl>
          {#each [{ label: "Local frontier", frontier: worker.local_frontier }, { label: "Peer frontier", frontier: worker.peer_frontier }, { label: "Peer acknowledgement", frontier: worker.pushed }, { label: "Pulled frontier", frontier: worker.pulled }] as { label, frontier }}
            <h3>{label}</h3>
            {#if frontier}
              <dl class="store-settings__rows">
                <div>
                  <dt>Replica</dt>
                  <dd>
                    <StoreSettingsIdentifier
                      label={`${label} replica`}
                      value={frontier.replica}
                    />
                  </dd>
                </div>
                <div>
                  <dt>Log epoch</dt>
                  <dd>
                    <StoreSettingsIdentifier
                      label={`${label} epoch`}
                      value={frontier.log_epoch}
                    />
                  </dd>
                </div>
                <div>
                  <dt>Through / observed upper bound</dt>
                  <dd>
                    {number(frontier.through)} / {number(
                      frontier.observed_upper_bound,
                    )}
                  </dd>
                </div>
              </dl>
            {:else}<p>Unavailable</p>{/if}
          {/each}
        </section>
      {/if}
    </Tabs.Content>
  </Tabs.Root>
  {#if settings.error}<Alert.Root variant="destructive"
      ><Alert.Title>Sync status unavailable</Alert.Title><Alert.Description
        >{settings.error}</Alert.Description
      ></Alert.Root
    >{/if}
{/if}

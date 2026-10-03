<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SquareIcon from "@lucide/svelte/icons/square";
  import type { SyncSettings } from "../../controller/sync-settings.js";
  let {
    wait,
    onWait,
    onStopWait,
  }: {
    wait: SyncSettings["wait"];
    onWait: (capture: boolean) => Promise<void>;
    onStopWait: () => void;
  } = $props();
  const busy = $derived(wait.state === "capturing" || wait.state === "waiting");
  const labels = {
    idle: "No target captured",
    capturing: "Capturing current sync target…",
    waiting: "Waiting for captured target…",
    pending: "Captured target is still pending",
    reached: "Captured target reached",
    stopped: "Waiting stopped; sync continues",
    invalid: "Captured target is no longer valid",
    failed: "Sync wait unavailable",
  };
</script>

<div class="store-settings__action-row">
  <div>
    <strong>Captured sync target</strong>
    <p role="status">{labels[wait.state]}</p>
    <p>New repository writes do not move this target.</p>
  </div>
  <div class="store-settings__actions">
    <Button variant="outline" disabled={busy} onclick={() => void onWait(true)}
      ><RefreshCwIcon
        data-icon="inline-start"
        aria-hidden="true"
      />{wait.barrier ? "Capture new target" : "Wait for current sync"}</Button
    >
    {#if busy}<Button variant="outline" onclick={onStopWait}
        ><SquareIcon data-icon="inline-start" aria-hidden="true" />Stop waiting</Button
      >
    {:else if wait.barrier && !["reached", "invalid"].includes(wait.state)}<Button
        variant="outline"
        onclick={() => void onWait(false)}>Continue waiting</Button
      >{/if}
  </div>
</div>
{#if wait.barrier}
  <dl class="store-settings__rows">
    {#each [{ label: "Push target", replica: wait.barrier.local_replica, epoch: wait.barrier.push_epoch, through: wait.barrier.push_through }, { label: "Pull target", replica: wait.barrier.peer_replica, epoch: wait.barrier.pull_epoch, through: wait.barrier.pull_through }] as target}
      <StoreReadOnlyRow
        label={`${target.label} replica`}
        value={target.replica}
      />
      <StoreReadOnlyRow label={`${target.label} epoch`} value={target.epoch} />
      <StoreReadOnlyRow
        label={`${target.label} through`}
        value={target.through}
      />
    {/each}
  </dl>
{/if}
<StoreNotice
  noticeKey="sync-wait"
  title="Sync wait failed"
  message={wait.error}
  pending={wait.state === "capturing" || wait.state === "waiting"}
/>

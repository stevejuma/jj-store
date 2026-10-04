<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Badge } from "@lapismd/design-core/shadcn/badge";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  let { management, shown }: { management: CaptureManagement; shown: number } =
    $props();
  const state = $derived(management.state);
  const sources = $derived(state.sources);
  const status = $derived(
    state.sourceState === "failed" || state.status === "failed"
      ? "Refresh needed"
      : state.sourceState === "refreshing"
        ? "Refreshing sources…"
        : state.sourceState === "ready"
          ? "Coverage current"
          : "Waiting for source coverage",
  );
</script>

<div class="store-capture-sources__footer">
  <div
    class="store-capture-sources__summary"
    role="status"
    aria-label="Source coverage status"
  >
    <span>{status}</span>
    <div class="store-capture-sources__counts">
      <Badge variant="secondary">{shown.toLocaleString("en-GB")} shown</Badge>
      <Badge variant="outline"
        >{sources?.sources.length.toLocaleString("en-GB")} loaded</Badge
      >
      <Badge variant="outline"
        >{sources?.total.toLocaleString("en-GB")} observed</Badge
      >
    </div>
    {#if state.sourcesObservedAt !== null}<span
        class="store-settings__row-description"
        >Updated <CommunityRelativeTime
          timestamp={state.sourcesObservedAt / 1_000}
          ariaLabel="Source coverage updated"
        /></span
      >{/if}
  </div>
  {#if sources?.next_cursor}
    <Button
      variant="outline"
      size="sm"
      disabled={state.busy ||
        !management.supported ||
        state.sourceState !== "ready"}
      onclick={() => management.moreSources()}>More capture sources</Button
    >
  {/if}
</div>

<style>
  .store-capture-sources__footer,
  .store-capture-sources__summary,
  .store-capture-sources__counts {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  .store-capture-sources__footer {
    justify-content: space-between;
  }
  .store-capture-sources__summary {
    gap: 0.75rem;
  }
  .store-capture-sources__counts {
    gap: 0.35rem;
  }
</style>

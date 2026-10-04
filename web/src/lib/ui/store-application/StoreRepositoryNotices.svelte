<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const preparing = $derived(state.settings.indexPreparation === "loading");
  const error = $derived(
    state.route.workspace && state.status.navigation === "failed"
      ? ""
      : state.status.error,
  );
</script>

<StoreNotice
  noticeKey="repository-request"
  title="Repository request failed"
  message={error}
  pending={state.status.navigation === "loading" ||
    state.status.navigation === "refreshing"}
  action={error.includes("index rebuild required")
    ? {
        id: "prepare-index",
        label: "Prepare graph index",
        run: () => void controller.actions.prepareIndex(),
      }
    : undefined}
/>

<StoreNotice
  noticeKey="graph-index-recovery"
  title="Graph index preparation"
  message={state.settings.indexPreparationError}
/>

{#if preparing}
  <div aria-label="Graph index preparation">
    <p role="status">
      {state.settings.indexTaskStatus === "canceling"
        ? "Canceling graph index preparation…"
        : "Preparing the graph index…"}
    </p>
    <Button
      variant="outline"
      disabled={state.settings.indexTaskStatus !== "running"}
      onclick={() => void controller.actions.cancelIndex()}
      >Cancel preparation</Button
    >
  </div>
{/if}

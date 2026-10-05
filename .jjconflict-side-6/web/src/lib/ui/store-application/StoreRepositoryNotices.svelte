<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const error = $derived(
    state.settings.indexPreparation === "loading" &&
      state.status.error.includes("index rebuild required")
      ? ""
      : state.status.error,
  );
  const indexRequired = $derived(
    state.settings.indexRequired || error.includes("index rebuild required"),
  );
</script>

<StoreNotice
  noticeKey="repository-request"
  title="Repository request failed"
  message={error || (indexRequired ? "index rebuild required" : "")}
  pending={state.status.navigation === "loading" ||
    state.status.navigation === "refreshing"}
  action={indexRequired
    ? {
        id: "prepare-index",
        label: "Prepare graph index",
        run: () => void controller.actions.prepareIndex(),
      }
    : undefined}
/>

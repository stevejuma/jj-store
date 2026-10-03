<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
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

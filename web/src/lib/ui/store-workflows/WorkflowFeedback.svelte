<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  let {
    owners,
  }: { owners: readonly (readonly [string, WorkflowController])[] } = $props();
</script>

{#each owners as [repository, owner] (repository)}<StoreNotice
    noticeKey={`workflows-read-${repository}`}
    title="Workflows unavailable"
    message={owner.state.readError}
  /><StoreNotice
    noticeKey={`workflow-result-${repository}`}
    title={owner.state.pending
      ? "Workflow outcome unknown"
      : owner.state.error
        ? "Workflow needs attention"
        : "Workflow saved"}
    severity={owner.state.error || owner.state.pending ? "error" : "info"}
    lifetime={owner.state.pending
      ? "condition"
      : owner.state.error
        ? "dismiss"
        : "timed"}
    pending={owner.state.writing}
    message={owner.state.error ||
      owner.state.message ||
      (owner.state.pending && !owner.state.writing
        ? "The save outcome is unknown. Your draft and original request are retained."
        : "")}
    action={owner.state.pending
      ? {
          id: `workflow-retry-${owner.state.pending.request_id}`,
          label: "Check original request",
          run: () => void owner.retry(),
        }
      : undefined}
  />{/each}

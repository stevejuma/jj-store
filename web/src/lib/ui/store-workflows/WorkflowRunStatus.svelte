<script lang="ts">
  import Check from "@lucide/svelte/icons/circle-check";
  import Failed from "@lucide/svelte/icons/circle-x";
  import Cancelled from "@lucide/svelte/icons/ban";
  import Unknown from "@lucide/svelte/icons/circle-dot";
  import Running from "@lucide/svelte/icons/play";
  import Clock from "@lucide/svelte/icons/clock";
  let { status, label = false }: { status: string; label?: boolean } = $props();
  const Icon = $derived(
    status === "completed"
      ? Check
      : status === "failed"
        ? Failed
        : status === "cancelled"
          ? Cancelled
          : status === "unknown"
            ? Unknown
            : ["running", "claimed", "cancel_requested"].includes(status)
              ? Running
              : Clock,
  );
</script>

<span
  class="workflow-run-status"
  data-status={status}
  title={status.replaceAll("_", " ")}
>
  <Icon aria-hidden="true" />{#if label}<span
      >{status.replaceAll("_", " ")}</span
    >{:else}<span class="sr-only">{status.replaceAll("_", " ")}</span>{/if}
</span>

<style>
  .workflow-run-status {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
  }
  .workflow-run-status :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: none;
  }
  .workflow-run-status[data-status="completed"] :global(svg) {
    color: var(--success, #16a34a);
  }
  .workflow-run-status[data-status="failed"] :global(svg) {
    color: var(--destructive);
  }
</style>

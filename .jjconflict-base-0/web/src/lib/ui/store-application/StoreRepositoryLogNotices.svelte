<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let {
    error,
    page,
    targetCount,
    headCount,
    unavailableComparisonCount,
  }: {
    error: string;
    page: string;
    targetCount: number;
    headCount: number;
    unavailableComparisonCount: number;
  } = $props();
</script>

<StoreNotice
  noticeKey="history-request"
  title="History unavailable"
  message={error}
/>
<StoreNotice
  noticeKey="history-bookmark-conflict"
  severity="warning"
  title="Conflicted bookmark"
  message={targetCount > 1
    ? `All ${targetCount} targets are shown. No target is implicitly selected.`
    : ""}
/>
<StoreNotice
  noticeKey="operation-heads"
  severity="info"
  title="Operation heads"
  message={page === "oplog" && headCount > 0
    ? `${headCount} operation heads · selecting one does not reconcile history`
    : ""}
/>
<StoreNotice
  noticeKey="operation-comparison-target"
  severity="warning"
  title="Comparison needs a target"
  message={unavailableComparisonCount > 0
    ? `${unavailableComparisonCount === 1 ? "One affected change has" : `${unavailableComparisonCount} affected changes have`} ambiguous lineage and cannot be compared until an explicit target is selected.`
    : ""}
/>

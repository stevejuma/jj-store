<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { WorkController } from "../../controller/work-types.js";
  let { work }: { work: WorkController } = $props();
</script>

<StoreNotice
  noticeKey={`work:${work.state.project?.id ?? "directory"}:${work.state.task?.id ?? ""}`}
  title="Task command failed"
  message={work.state.pending ? "" : work.state.error}
  pending={work.state.loading || work.state.writing}
/>
<StoreNotice
  noticeKey="work-result"
  title="Task command completed"
  message={work.state.pending ? "" : work.state.message}
  severity="info"
  lifetime="timed"
/>
{#if work.state.pending && (work.state.error || work.state.message)}<p
    role="status"
    aria-label="Work command status"
  >
    {work.state.error || work.state.message}
  </p>{/if}
{#if work.state.pending}<div class="store-work__notice">
    <p>
      Pending request {work.state.pending.request_id.slice(0, 8)}. Check status
      or retry the same command.
    </p>
    <Button
      variant="outline"
      disabled={work.state.writing}
      onclick={() => work.check()}>Check task command</Button
    >
    <Button
      variant="outline"
      disabled={work.state.writing}
      onclick={() => work.retry()}>Retry task command</Button
    >
  </div>{/if}

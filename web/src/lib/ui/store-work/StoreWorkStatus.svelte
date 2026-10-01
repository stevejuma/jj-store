<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { WorkController } from "../../controller/work-types.js";
  let { work }: { work: WorkController } = $props();
</script>

{#if work.state.error}<p role="alert">{work.state.error}</p>{/if}
{#if work.state.message}<p role="status" aria-label="Work command status">
    {work.state.message}
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

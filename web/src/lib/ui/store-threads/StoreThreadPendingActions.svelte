<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ThreadsController } from "../../controller/thread-types.js";
  let { threads }: { threads: ThreadsController } = $props();
</script>

{#if threads.pending}
  <Button
    variant="outline"
    disabled={threads.busy}
    onclick={() => threads.checkPending()}>Check action status</Button
  >
  <Button
    variant="ghost"
    disabled={threads.busy}
    onclick={() => threads.retryPending().catch(() => {})}
    >Retry same action</Button
  >
{/if}

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
</script>

{#if manager.pending}<p role="status">
    Request {String(manager.pending.body.request_id)} · {manager.outcome
      ?.state ?? "checking outcome"}. Draft and request identity retained.
  </p>{/if}
{#if manager.pending && manager.outcome?.state === "not_known"}<Button
    variant="outline"
    disabled={manager.busy}
    onclick={() => manager.retryRetainedRequest()}
    >Submit retained request</Button
  >{/if}
{#if manager.pending && manager.outcome?.state === "outcome_unknown"}<Button
    variant="outline"
    onclick={() => manager.retainAsUnresolved()}
    >Keep unresolved request and review workspace</Button
  >{/if}

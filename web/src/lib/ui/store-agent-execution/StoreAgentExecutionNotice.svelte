<script lang="ts">
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let {
    manager,
    activeSession,
  }: { manager: AgentExecution; activeSession?: string | null } = $props();
</script>

<StoreNotice
  scope="agent-execution"
  noticeKey={String(manager.pending?.body.request_id ?? "runtime")}
  title="Agent execution"
  message={(manager.setupConflict ||
  (!!activeSession && manager.errorSessionId === activeSession)
    ? ""
    : manager.error) ||
    manager.connectionError ||
    manager.targetError ||
    (!activeSession ? manager.activityError : "")}
  pending={manager.busy}
  action={{
    id: "refresh-agent-execution",
    label: manager.pending ? "Check outcome" : "Retry",
    run: () => void manager.refresh(),
  }}
/>

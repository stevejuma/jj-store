<script lang="ts">
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { requestRecoveryNotice } from "./request-recovery-notice.js";
  let {
    manager,
    activeSession,
  }: { manager: AgentExecution; activeSession?: string | null } = $props();
  const recovery = $derived(requestRecoveryNotice(manager));
</script>

<StoreNotice
  scope="agent-execution"
  noticeKey={String(manager.pending?.body.request_id ?? "runtime")}
  title={recovery?.title ?? "Agent execution"}
  severity={recovery?.severity ?? "error"}
  message={manager.busy && recovery
    ? ""
    : recovery?.message ||
      (manager.setupConflict ||
      (!!activeSession && manager.errorSessionId === activeSession)
        ? ""
        : manager.error) ||
      manager.connectionError ||
      manager.targetError ||
      (!activeSession ? manager.activityError : "")}
  pending={manager.busy}
  action={recovery?.action ?? {
    id: "refresh-agent-execution",
    label: manager.pending ? "Check outcome" : "Retry",
    run: () => void manager.refresh(),
  }}
/>

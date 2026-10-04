<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  let { controller }: { controller: StoreController } = $props();
  const selected = $derived(controller.state.agentSession);
</script>

{#if selected}
  <StoreRecordMenu
    label="Agent details actions"
    items={[
      { label: "Copy session ID", value: selected.session.session_id },
      { label: "Copy snapshot ID", value: selected.session.snapshot_id },
      { label: "Copy agent root", value: selected.snapshot.agent_root },
      { label: "Copy repository ID", value: selected.snapshot.repo_id },
      {
        label: "Copy epoch ID",
        value:
          "epoch" in selected.snapshot
            ? selected.snapshot.epoch
            : selected.snapshot.metadata_commit,
      },
    ]}
  />
{/if}

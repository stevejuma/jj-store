<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import InfoIcon from "@lucide/svelte/icons/info";
  import { metadataKey } from "../../controller/conversation-metadata.svelte.js";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  let { controller }: { controller: StoreController } = $props();
  const selected = $derived(controller.state.agentSession);
</script>

{#if selected}
  {#if controller.execution}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Conversation details"
      aria-pressed={controller.execution.metadata.details[
        metadataKey({
          session: selected.session.session_id,
          repository: controller.state.repository.id,
        })
      ] ?? false}
      onclick={() => {
        const key = metadataKey({
          session: selected.session.session_id,
          repository: controller.state.repository.id,
        });
        controller.execution!.metadata.details[key] =
          !controller.execution!.metadata.details[key];
      }}><InfoIcon aria-hidden="true" /></Button
    >{/if}
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

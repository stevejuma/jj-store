<script lang="ts">
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  import type { Model } from "../../api/client.js";
  let {
    id,
    sessionId,
    activity,
    legacy,
    transcriptId,
  }: {
    id: string;
    sessionId: string;
    activity: readonly Model<"AgentActivity">[];
    legacy: readonly Model<"AgentTranscriptChunk">[];
    transcriptId?: string;
  } = $props();
  function capturedMessage(id: string) {
    return activity.find((entry) => {
      if (entry.type !== "message") return false;
      const capturedId = `${sessionId}:${entry.message_id}`;
      return (
        id === capturedId ||
        id.startsWith(`${capturedId}/`) ||
        id.startsWith(`${capturedId}:`)
      );
    });
  }

  const entry = $derived(capturedMessage(id));
  const chunk = $derived(
    legacy.find((c) => `${sessionId}:legacy:${c.sequence}` === id),
  );
</script>

{#if entry?.type === "message" || chunk}<StoreRecordMenu
    items={[
      {
        label: "Copy message",
        value:
          entry?.type === "message" ? entry.content.content : chunk?.content,
      },
      {
        label: "Copy message ID",
        value: entry?.type === "message" ? entry.message_id : undefined,
      },
      {
        label: "Copy turn ID",
        value: entry?.type === "message" ? entry.turn_id : undefined,
      },
      { label: "Copy session ID", value: sessionId },
      {
        label: "Copy transcript ID",
        value: chunk ? transcriptId : undefined,
      },
      {
        label: "Copy sequence",
        value: String(entry?.sequence ?? chunk?.sequence),
      },
    ]}
  />{/if}

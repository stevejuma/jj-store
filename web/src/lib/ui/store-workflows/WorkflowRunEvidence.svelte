<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Download from "@lucide/svelte/icons/download";
  import Bot from "@lucide/svelte/icons/bot";
  import type { WorkflowEvidence } from "../../controller/workflow-types.js";
  let { rows }: { rows: readonly WorkflowEvidence[] } = $props();
  function download(
    name: string,
    mediaType: string,
    bytes: readonly (number | bigint)[],
  ) {
    const url = URL.createObjectURL(
      new Blob([new Uint8Array(bytes.map(Number))], { type: mediaType }),
    );
    const link = document.createElement("a");
    link.href = url;
    link.download = name.split("/").at(-1) ?? "artifact";
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }
</script>

{#each rows as row (row.id)}
  {#if row.data.type === "artifact"}{@const artifact = row.data}<Menu.Item
      title={`Attempt ${row.attempt} · sequence ${row.sequence}`}
      onclick={() =>
        download(artifact.name, artifact.media_type, artifact.bytes)}
      ><Download aria-hidden="true" />Download {artifact.name}</Menu.Item
    >
  {:else if row.data.type === "agent"}{@const agent = row.data}<Menu.Item
      >{#snippet child({ props })}<a
          {...props}
          href={`#/agents/sessions/${encodeURIComponent(agent.session)}`}
          ><Bot aria-hidden="true" />Agent transcript · {agent.session.slice(
            0,
            8,
          )}</a
        >{/snippet}</Menu.Item
    >{/if}
{/each}

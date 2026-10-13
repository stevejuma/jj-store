<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
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

<div class="workflow-run-evidence">
  {#each rows as row (row.id)}
    <div
      class="workflow-run-evidence__item"
      title={`Attempt ${row.attempt} · sequence ${row.sequence}`}
    >
      {#if row.data.type === "step"}<strong>{row.data.step}</strong>
        <WorkflowRunStatus status={row.data.status} label />
      {:else if row.data.type === "log"}<pre
          aria-label={`${row.data.stream} log`}>{row.data.text}</pre>
      {:else if row.data.type === "artifact"}{@const artifact = row.data}<Button
          variant="outline"
          onclick={() =>
            download(artifact.name, artifact.media_type, artifact.bytes)}
          >Download {artifact.name}</Button
        >
      {:else if row.data.type === "agent"}<a
          href={`#/agents/sessions/${encodeURIComponent(row.data.session)}`}
          >Agent transcript · {row.data.session.slice(0, 8)}</a
        >{/if}
    </div>
  {/each}
</div>

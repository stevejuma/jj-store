<script lang="ts">
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import "./transcript-plan.css";
  let { entries }: { entries: readonly { content: string; status: string }[] } =
    $props();
</script>

<div class="agent-transcript-plan" role="group" aria-label="Plan">
  <h3>Plan</h3>
  <ol>
    {#each entries as entry, index (`${index}:${entry.content}`)}<li
        data-plan-status={entry.status || "pending"}
      >
        <div class="agent-transcript-plan__step">
          <CommunityChip
            size="compact"
            tone={entry.status === "completed"
              ? "success"
              : entry.status === "in_progress"
                ? "accent"
                : "warning"}
            >{entry.status.replaceAll("_", " ") || "Pending"}</CommunityChip
          ><span>{entry.content}</span>
        </div>
      </li>{/each}
  </ol>
</div>

<script lang="ts">
  import TranscriptToolDetail from "../community-agent-activity/TranscriptToolDetail.svelte";
  import { recordedTranscriptTool } from "../community-agent-activity/transcript-tool-adapters.js";
  import type { Model } from "../../api/client.js";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";

  let {
    tool,
    patches = {},
    raw = $bindable(false),
  }: {
    tool: Model<"AgentToolActivity">;
    patches?: Readonly<Record<string, string>>;
    raw?: boolean;
  } = $props();

  const payload = $derived(recordedTranscriptTool(tool, patches));
</script>

<div class="store-agent-tool-detail">
  <div class="store-agent-tool-detail__identities">
    {#each tool.code as association (association.association_id)}<CommunityProjectsRevisionGraphIds
        commitId={association.after_commits[0] ??
          association.before_commits[0] ??
          ""}
        changeId={association.changes[0]}
        placement="inline"
      />{/each}
  </div>
  <TranscriptToolDetail tool={payload} value={tool} bind:raw />
</div>

<style>
  .store-agent-tool-detail {
    display: grid;
    min-width: 0;
    gap: 0.75rem;
  }

  .store-agent-tool-detail__identities {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    gap: 0.35rem;
  }
</style>

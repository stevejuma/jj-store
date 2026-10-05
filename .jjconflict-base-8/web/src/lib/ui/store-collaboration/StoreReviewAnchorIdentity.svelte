<script lang="ts">
  import type { Model } from "../../api/client.js";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";

  let { anchor }: { anchor: Model<"ArtifactAnchorV1"> } = $props();
  const target = $derived(anchor.target);
</script>

{#if target.target === "operation"}
  <StoreIdentityCopy value={target.value.operation_id} label="operation ID" />
{:else if target.target === "revision" || target.target === "code_text" || target.target === "code_artifact"}
  <CommunityProjectsRevisionGraphIds
    commitId={target.value.commit_id}
    changeId={target.value.change_id ?? ""}
    placement="inline"
  />
{:else if target.target === "agent_text" || target.target === "agent_artifact"}
  <StoreIdentityCopy value={target.value.session_id} label="agent session ID" />
{/if}

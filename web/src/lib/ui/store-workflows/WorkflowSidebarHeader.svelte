<script lang="ts">
  import StoreToolbarSearch from "../StoreToolbarSearch.svelte";
  import WorkflowNavigation from "./WorkflowNavigation.svelte";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    sourceRepository,
    selection,
    onChange,
  }: {
    sourceRepository?: string;
    selection: WorkflowSelection;
    onChange(fields: WorkflowSelection): void;
  } = $props();
  let searchOpen = $state(false);
</script>

{#if sourceRepository}<div class="community-projects__tabs workflow-navigation">
    <StoreToolbarSearch
      label="Search runs"
      scope={`repository-actions:${sourceRepository}`}
      query={selection.q ?? ""}
      bind:open={searchOpen}
      onQuery={(q) => onChange({ q: q || undefined })}
    />
  </div>
{:else}<WorkflowNavigation {selection} {onChange} />{/if}

<script lang="ts">
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";

  let { value, label }: { value: string; label: string } = $props();
</script>

<CommunityProjectsRevisionGraphIds
  commitId={value}
  commitLabel={label}
  placement="inline"
/>

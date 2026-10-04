<script lang="ts">
  import CommunityProjectsRepositoryLogFiles from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import { sampleDetail } from "./repository.js";
  let selectedPath = $state<string | null>(null);
</script>

<CommunityProjectsRepositoryLogFiles
  files={sampleDetail.files}
  {selectedPath}
  onSelectPath={(path) => (selectedPath = path)}
/>

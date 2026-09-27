<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { createRepositoryLogCanvas } from "../projects/repository-log-canvas.js";
  import CommunityProjectsRepositoryLogDiff from "../components/community-projects/CommunityProjectsRepositoryLogDiff.svelte";
  import { sampleDetail } from "./repository.js";
  let { path = "README.md" }: { path?: string } = $props();
  const canvas = createRepositoryLogCanvas();
  canvas.select(0, "revision");
  canvas.select(1, "README.md");
</script>

<div class="diff-fixture">
  <ColumnCanvas.Root controller={canvas}
    ><CommunityProjectsRepositoryLogDiff
      {path}
      patch={sampleDetail.patches[path]}
      files={sampleDetail.files}
    /></ColumnCanvas.Root
  >
</div>

<style>
  .diff-fixture {
    height: 28rem;
    display: flex;
  }
</style>

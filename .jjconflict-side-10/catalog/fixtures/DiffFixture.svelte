<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { createRepositoryLogCanvas } from "../../web/src/lib/projects/repository-log-canvas.js";
  import CommunityProjectsRepositoryLogDiff from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte";
  import { sampleDetail } from "./repository.js";
  import type { DiffUnavailableReason } from "../../web/src/lib/projects/repository-diff.js";
  let {
    path = "README.md",
    unavailableReason,
    loading = false,
    operation,
  }: {
    path?: string | null;
    unavailableReason?: DiffUnavailableReason;
    loading?: boolean;
    operation?: import("../../web/src/lib/projects/repository-log.js").RepositoryLogRevision;
  } = $props();
  const canvas = createRepositoryLogCanvas();
  canvas.select(0, "revision");
  canvas.select(1, "README.md");
</script>

<div class="diff-fixture">
  <ColumnCanvas.Root controller={canvas}
    ><CommunityProjectsRepositoryLogDiff
      {path}
      {unavailableReason}
      {loading}
      {operation}
      patch={path ? sampleDetail.patches[path] : undefined}
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

<script lang="ts">
  import CommunityProjectsRepositoryLog from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import {
    NOW,
    fixtureGraph,
    bookmarks,
    operations,
    sampleDetail,
  } from "./repository.js";
  let {
    page = "branches",
    state: fixtureState = "populated",
  }: {
    page?: StorePage;
    state?: "populated" | "empty" | "loading" | "error" | "refreshing";
  } = $props();
  let branch = $state("");
  let selectedId = $state<string | null>(null);
  let selectedPath = $state<string | null>(null);
  const graph = $derived(
    fixtureState === "empty" ||
      fixtureState === "loading" ||
      fixtureState === "error"
      ? { revisions: [], devices: [] }
      : page === "oplog"
        ? operations
        : fixtureGraph(page === "branches" ? branch : undefined),
  );
  function selectBranch(name: string) {
    branch = name;
    selectedId = null;
    selectedPath = null;
  }
  function selectRevision(id: string | null) {
    selectedId = id;
    selectedPath = null;
  }
</script>

<CommunityProjectsRepositoryLog
  {page}
  {graph}
  {bookmarks}
  selectedBranch={branch}
  {selectedId}
  {selectedPath}
  files={graph.revisions.find((revision) => revision.eventId === selectedId)
    ?.empty
    ? []
    : sampleDetail.files}
  patch={selectedPath ? sampleDetail.patches[selectedPath] : undefined}
  now={NOW}
  loading={fixtureState === "loading" || fixtureState === "refreshing"}
  error={fixtureState === "error" ? "Fixture history is unavailable." : ""}
  onSelectBranch={selectBranch}
  onSelectRevision={selectRevision}
  onSelectPath={(path) => (selectedPath = path)}
/>

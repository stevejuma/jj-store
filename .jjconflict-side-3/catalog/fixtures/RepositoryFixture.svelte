<script lang="ts">
  import CommunityProjectsRepositoryLog from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import {
    NOW,
    fixtureGraph,
    bookmarks,
    workingCopies,
    operations,
    sampleDetail,
  } from "./repository.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { untrack } from "svelte";
  let {
    page = "branches",
    state: fixtureState = "populated",
    unavailableComparisonCount = 0,
  }: {
    page?: StorePage;
    state?: "populated" | "empty" | "loading" | "error" | "refreshing";
    unavailableComparisonCount?: number;
  } = $props();
  let branch = $state("");
  let selectedId = $state<string | null>(null);
  let selectedPath = $state<string | null>(null);
  const layoutStorage = untrack(createMemoryStoryStorage);
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
  bookmarks={fixtureState === "loading" ? [] : bookmarks}
  workingCopies={fixtureState === "loading" ? [] : workingCopies}
  selectedBranch={branch}
  {selectedId}
  {selectedPath}
  files={graph.revisions.find((revision) => revision.eventId === selectedId)
    ?.empty
    ? []
    : sampleDetail.files}
  patch={selectedPath ? sampleDetail.patches[selectedPath] : undefined}
  now={NOW}
  {layoutStorage}
  loading={fixtureState === "loading" || fixtureState === "refreshing"}
  error={fixtureState === "error" ? "Fixture history is unavailable." : ""}
  {unavailableComparisonCount}
  onSelectBranch={selectBranch}
  onSelectWorkingCopy={(commit) => {
    branch = "";
    selectedId = commit;
    selectedPath = null;
  }}
  onSelectRevision={selectRevision}
  onSelectPath={(path) => (selectedPath = path)}
/>

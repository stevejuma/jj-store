<script lang="ts">
  import StoreMutationCommitPicker from "../../web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte";
  import { NOW, revisions } from "./repository.js";
  import CommunityProjectsRevisionGraphIds from "../../web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte";
  const choices = [
    { ...revisions[0]!, message: "", bookmarks: [], workspaces: [] },
    {
      ...revisions[1]!,
      message:
        "A much longer title for preserving native objects across the repository",
      createdAt: NOW - 86400 * 18,
    },
    {
      ...revisions[2]!,
      message: "Short title",
      changeId: "",
      createdAt: NOW - 60,
    },
  ];
  let target = $state(choices[0]!.eventId);
</script>

<div style="max-width: 44rem; padding: 1rem">
  <StoreMutationCommitPicker
    label="Parent commit"
    bind:target
    revisions={choices}
    now={NOW}
    onChange={() => {}}
  />
  <div data-testid="copy-id-reference">
    <CommunityProjectsRevisionGraphIds
      commitId={choices[0]!.eventId}
      changeId={choices[0]!.changeId}
      placement="inline"
    />
  </div>
</div>

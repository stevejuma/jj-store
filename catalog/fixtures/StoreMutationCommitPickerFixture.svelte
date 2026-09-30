<script lang="ts">
  import StoreMutationCommitPicker from "../../web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte";
  import { NOW, revisions } from "./repository.js";
  const choices = [
    { ...revisions[0]!, message: "", bookmarks: [], workspaces: [] },
    revisions[1]!,
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
</div>

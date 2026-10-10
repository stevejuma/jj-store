<script lang="ts">
  import CommunityInbox from "./CommunityInbox.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  let {
    kind = "session",
    selected = false,
    layoutStorage,
  }: {
    kind?: "session" | "thread";
    selected?: boolean;
    layoutStorage?: Storage;
  } = $props();
</script>

<!-- Use the live shell so saved widths, tabs and mobile regions match. -->
<CommunityInbox
  {kind}
  {layoutStorage}
  title={kind === "thread" ? "Threads" : "Sessions"}
  items={[]}
  loading
  activeId={selected ? "loading" : undefined}
  onSelect={() => {}}
  onNew={() => {}}
  onBack={() => {}}
>
  <CommunitySkeleton
    layout="messages"
    padded
    label={kind === "thread" ? "Loading Thread" : "Loading session"}
  />
</CommunityInbox>

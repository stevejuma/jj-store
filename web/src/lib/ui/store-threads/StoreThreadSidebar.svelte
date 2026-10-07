<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import CommunityDetails from "../community-details/CommunityDetails.svelte";
  import StoreThreadReplies from "./StoreThreadReplies.svelte";
  import StoreThreadSettings from "./StoreThreadSettings.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import { threadMembers, threadDisabled } from "./thread-view.js";
  let { controller }: { controller: StoreController } = $props();
  const panel = useThreadPanel();
  const threads = $derived(controller.threads);
  const members = $derived(threadMembers(controller));
</script>

{#if panel.kind === "settings"}<StoreThreadSettings {controller} />
{:else}<AppShell.Sidebar.Header
    ><strong>{panel.kind === "replies" ? "Replies" : "Contributors"}</strong
    ><AppShell.Sidebar.Close
      label="Close thread panel"
      onclick={() => {
        if (panel.kind === "replies") threads?.selectReply(null);
        panel.close();
      }}
    /></AppShell.Sidebar.Header
  >
  {#if panel.kind === "replies" && threads && controller.state.route.replyId}<StoreThreadReplies
      {controller}
      {threads}
      rootId={controller.state.route.replyId}
      {members}
      currentMemberId={controller.state.repository.principal}
      disabled={threadDisabled(controller)}
    />
  {:else if threads?.detail}<AppShell.Sidebar.Body
      ><CommunityDetails
        room={{
          id: threads.detail.thread.value.thread_id,
          name: threads.detail.thread.value.title,
          kind: "channel",
        }}
        {members}
        details={{
          description:
            "Contributors in the loaded conversation. Load older messages to see earlier participants.",
          memberIds: members.map((m) => m.id),
        }}
      /></AppShell.Sidebar.Body
    >{/if}
{/if}

<script lang="ts">
  import StoreDiscussionSheet from "../store-discussion/StoreDiscussionSheet.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreThreadReplies from "./StoreThreadReplies.svelte";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import { threadMembers, threadDisabled } from "./thread-view.js";
  let { controller, open }: { controller: StoreController; open: boolean } =
    $props();
  const panel = useThreadPanel();
  function close() {
    controller.threads?.selectReply(null);
    panel.close();
  }
</script>

<StoreDiscussionSheet
  open={open && !!controller.threads && !!controller.state.route.replyId}
  title="Replies"
  onClose={close}
>
  {#if controller.threads && controller.state.route.replyId}
    <StoreThreadReplies
      threads={controller.threads}
      rootId={controller.state.route.replyId}
      members={threadMembers(controller)}
      currentMemberId={controller.state.repository.principal}
      disabled={threadDisabled(controller)}
    />
  {/if}
</StoreDiscussionSheet>

<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreThreadReplies from "./StoreThreadReplies.svelte";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import { threadMembers, threadDisabled } from "./thread-view.js";
  let { controller }: { controller: StoreController } = $props();
  const panel = useThreadPanel();
  function close() {
    controller.threads?.selectReply(null);
    panel.close();
  }
</script>

{#if controller.threads && controller.state.route.replyId}<Dialog.Root
    open
    onOpenChange={(open) => {
      if (!open) close();
    }}
    ><Dialog.Content class="store-native-thread-focus"
      ><Dialog.Header
        ><Dialog.Title>Replies</Dialog.Title><Dialog.Description
          >Replies to the selected thread message.</Dialog.Description
        ></Dialog.Header
      ><StoreThreadReplies
        threads={controller.threads}
        rootId={controller.state.route.replyId}
        members={threadMembers(controller)}
        currentMemberId={controller.state.repository.principal}
        disabled={threadDisabled(controller)}
      /></Dialog.Content
    ></Dialog.Root
  >{/if}

<style>
  :global(.store-native-thread-focus[data-ui-part="dialog-content"]) {
    display: flex;
    flex-direction: column;
    width: min(58rem, calc(100vw - 2rem));
    max-width: none;
    height: min(48rem, calc(100dvh - 2rem));
    min-height: 0;
  }
  :global(.store-native-thread-focus .community-thread-panel) {
    flex: 1;
    min-height: 0;
  }
</style>

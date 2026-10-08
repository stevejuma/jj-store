<script lang="ts">
  import StoreThreadConversation from "./StoreThreadConversation.svelte";
  import { agentMember } from "../community-agents/model.js";

  import type { StoreController } from "../../controller/contracts.js";

  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../../community/workspace-model.js";

  import type { ThreadsController } from "../../controller/thread-types.js";
  import { replyRoot, threadDescendants } from "./thread-view.js";
  let {
    threads,
    controller,
    rootId,
    members,
    currentMemberId,
    disabled,
  }: {
    threads: ThreadsController;
    controller?: StoreController;
    rootId: string;
    members: CommunityMemberModel[];
    currentMemberId?: string;
    disabled: boolean;
  } = $props();
  const conversationRoot = $derived(
    replyRoot(threads.projection.messages, rootId),
  );
  const messages = $derived.by(() => {
    return threadDescendants(threads.projection.messages, conversationRoot).map(
      (m): CommunityMessageModel => ({
        ...m,
        reactions: threads.reactionModels(threads.projection.targets[m.id]!),
      }),
    );
  });
</script>

<StoreThreadConversation
  {threads}
  {controller}
  {messages}
  members={[
    ...members,
    ...(controller?.execution?.value?.setups ?? []).map(agentMember),
  ]}
  user={members.find((m) => m.id === currentMemberId) ?? {
    id: currentMemberId ?? "user",
    name: "You",
    handle: "",
    initials: "YO",
  }}
  {disabled}
  parent={rootId}
  onReply={threads.selectReply}
/>

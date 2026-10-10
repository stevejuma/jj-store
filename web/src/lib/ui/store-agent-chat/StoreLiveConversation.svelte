<script lang="ts">
  import { conversationNavigation } from "../../conversations/navigation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreAgentRequestRecovery from "../store-agent-execution/StoreAgentRequestRecovery.svelte";
  import StoreAgentReplyDraftRecovery from "../store-agent-execution/StoreAgentReplyDraftRecovery.svelte";
  import { openingReplayRowIds } from "../store-agent-execution/presentation.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import { untrack, type Snippet } from "svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import {
    agentConversationProjection,
    type AgentConversationRow,
  } from "../../conversations/agent-adapter.js";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import type {
    ChatBodyItem,
    ChatPromptSubmission,
  } from "../store-agent-chat-body/model.js";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import StoreAgentChatBody from "../store-agent-chat-body/StoreAgentChatBody.svelte";
  import StoreConversationAgentSupplement from "./StoreConversationAgentSupplement.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  let {
    controller,
    manager,
    rows,
    conversation,
    user = { id: "user", name: "You", handle: "", initials: "YO" },
    composer,
    footer,
    beforeItems,
    activity,
    loading = false,
    emptyState,
    onSend,
    restoreMessage,
  }: {
    controller?: StoreController;
    manager: AgentExecution;
    rows: readonly AgentConversationRow[];
    conversation: ConversationSession;
    user?: CommunityMemberModel;
    composer?: Snippet<["document" | "classic"]>;
    footer?: Snippet;
    beforeItems?: Snippet;
    activity?: Snippet;
    loading?: boolean;
    emptyState?: Snippet;
    onSend?: (
      submission: ChatPromptSubmission,
    ) => ReturnType<
      NonNullable<
        import("../store-agent-chat-body/model.js").ChatBodyProps["onSend"]
      >
    >;
    restoreMessage?: (submitted: string) => string;
  } = $props();
  const appearance = useAppearance();
  const inspection = useConversationInspection();
  const projection = $derived(
    agentConversationProjection(rows, manager.discussions.values, user),
  );
  $effect(() => {
    const next = projection;
    untrack(() => conversation.adopt(next));
  });
  const commentUnavailable = $derived(
    rows
      .map((r) => manager.discussions.values[r.session.id]?.write_unavailable)
      .find(Boolean),
  );
  $effect(() => {
    conversation.host = {
      ...conversationNavigation(controller, conversation),
      canComment: !commentUnavailable,
      commentUnavailable: commentUnavailable ?? undefined,
      author: user,
      onSend,
      restoreMessage,
      members: [
        user,
        ...projection.items.flatMap((i) => (i.author ? [i.author] : [])),
      ],
      communityEmojis: rows.length
        ? manager.discussions.reader(rows[0].session.repository).emojis
        : [],
      onInspectTool: inspection
        ? (tool, opener) => inspection.open(tool, conversation.comments, opener)
        : undefined,
      onReact: (id, emoji, custom) => {
        const target = conversation.projection.targets.get(id);
        const session = rows.find(
          (r) => r.session.id === target?.sessionId,
        )?.session;
        const feedback =
          target && manager.discussions.message(target.sessionId, target.rowId);
        if (session && feedback)
          void manager.discussions.toggle(
            session.repository,
            feedback.target,
            emoji,
            custom,
          );
      },
    };
  });
  const reactions = $derived(
    Object.fromEntries(
      projection.items.map((item) => {
        const target = projection.targets.get(item.id),
          session = rows.find(
            (r) => r.session.id === target?.sessionId,
          )?.session;
        const feedback =
          target && manager.discussions.message(target.sessionId, target.rowId);
        return [
          item.id,
          session && feedback
            ? manager.discussions
                .reader(session.repository)
                .reactionModels(feedback.target)
            : [],
        ];
      }),
    ),
  );
  function rowFor(item: ChatBodyItem) {
    const target = projection.targets.get(item.id);
    return rows.find(
      (r) =>
        r.key === item.id ||
        (r.session.id === target?.sessionId &&
          (r.sourceRowId ?? r.id) === target.rowId),
    );
  }
  const participations = $derived(
    [...new Map(rows.map((r) => [r.session.id, r.session])).values()].filter(
      (s) => ["running", "opening", "preparing"].includes(s.state),
    ),
  );
  const replay = $derived(
    openingReplayRowIds(manager.events).filter(
      (id) => !!manager.discussions.drafts[`${manager.session?.id}:${id}`],
    ),
  );
  function restore(rowId: string) {
    const selected = manager.session;
    if (!selected) return;
    const key = `${selected.id}:${rowId}`;
    const item = projection.items.find(
      (i) =>
        projection.targets.get(i.id)?.sessionId === selected.id &&
        projection.targets.get(i.id)?.rowId === rowId,
    );
    const block = item?.blocks.find((b) => b.commentable !== false);
    if (!block) return;
    conversation.comments.open(
      block,
      { from: 0, to: 0 },
      manager.discussions.drafts[key],
    );
    if (conversation.comments.active) manager.discussions.drafts[key] = "";
  }
</script>

<StoreAgentChatBody
  items={conversation.projection.items}
  commentState={conversation.comments}
  view={appearance?.live.conversationView ?? "document"}
  {composer}
  {footer}
  {beforeItems}
  {loading}
  {emptyState}
  {reactions}
  onReact={conversation.host.onReact}
  onInspectTool={conversation.host.onInspectTool}
>
  {#snippet afterItems()}
    {#each replay as source (source)}<StoreAgentReplyDraftRecovery
        {manager}
        {source}
        rows={rows.filter((r) => r.session.id === manager.selected)}
        onRestore={restore}
      />{/each}
    {#if activity}{@render activity()}{:else}{#each participations as session (session.id)}<StoreAgentRequestRecovery
          {manager}
          {session}
        />{/each}{/if}
  {/snippet}
  {#snippet supplement(item)}<StoreConversationAgentSupplement
      {conversation}
      {controller}
      {manager}
      {rows}
      {projection}
      {item}
      {user}
    />{/snippet}
  {#snippet metadataSupplement(item)}
    {@const row = rowFor(item)}
    {#if row}
      {@const captured = manager.discussions.message(
        row.session.id,
        row.sourceRowId ?? row.id,
      )}
      {#if captured}<StoreMessageReactions
          threads={manager.discussions.reader(row.session.repository)}
          target={captured.target}
          showControls={false}
        />{/if}
    {/if}
  {/snippet}
</StoreAgentChatBody>

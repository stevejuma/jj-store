<script lang="ts">
  import "./store-threads.css";
  import CommunityWorkspaceSurface from "../community-workspace/CommunityWorkspaceSurface.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import ForumTopicList from "../forum-topic-list/ForumTopicList.svelte";
  import type { CommunityWorkspaceModel } from "../../community/workspace-model.js";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { untrack } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMessageReactions from "./StoreMessageReactions.svelte";
  import StoreThreadHeader from "./StoreThreadHeader.svelte";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import {
    threadMembers,
    threadDisabled,
    threadWritable,
  } from "./thread-view.js";
  import { principalMember } from "../../collaboration/presentation.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  let { controller }: { controller: StoreController } = $props();
  const threads = $derived(controller.threads);
  const panel = useThreadPanel();
  const members = $derived(threadMembers(controller));
  const disabled = $derived(threadDisabled(controller));
  const projection = $derived(threads?.projection);
  const room = $derived({
    id: threads?.detail?.thread.value.thread_id ?? "",
    name: threads?.detail?.thread.value.title ?? "Threads",
    kind: "channel" as const,
    participantIds: members.map((m) => m.id),
    memberListCompleteness: "partial" as const,
  });
  const model = $derived<CommunityWorkspaceModel>({
    communities: [],
    activeCommunityId: controller.state.route.repository,
    navigationSections: [{ id: "threads", label: "Threads", rooms: [room] }],
    activeRoomId: room.id,
    view: "channel",
    messages:
      projection?.messages
        .filter((m) => !m.parentMessageId)
        .map((m) => ({
          ...m,
          reactions: threads?.reactionModels(projection.targets[m.id]!) ?? [],
        })) ?? [],
    topics: [],
    members,
    currentMemberId: controller.state.repository.principal,
  });
  let forumViewport = $state<HTMLElement | null>(null);
  const topics = $derived(
    threads?.threads.map((t) => ({
      id: t.value.thread_id,
      title: t.value.title,
      excerpt: t.directory?.excerpt ?? "",
      authorId: t.value.actor,
      replyCount: Number(t.directory?.reply_count ?? 0),
      replyCountAvailable: !!t.directory,
      participantIds: t.directory?.participants ?? [],
      lastActivityLabel: new Date(
        Number(
          t.directory?.last_activity_at_millis ?? t.value.accepted_at_millis,
        ),
      ).toLocaleDateString(undefined, { month: "short", day: "numeric" }),
    })) ?? [],
  );
  const directoryMembers = $derived([
    ...new Map(
      (threads?.threads ?? []).flatMap((t) =>
        [t.value.actor, ...(t.directory?.participants ?? [])].map(
          (id) => [id, principalMember(id)] as const,
        ),
      ),
    ).values(),
  ]);
  $effect(() => {
    const id = room.id;
    untrack(() => {
      if (id) void threads?.loadEmojis().catch(() => {});
    });
  });
  function reply(id: string) {
    threads?.selectReply(id);
    panel.open("replies");
  }
</script>

{#if threads}<section class="store-threads" aria-label="Threads">
    <StoreThreadHeader {controller} />
    {#if threads.error}<Alert.Root variant="destructive"
        ><Alert.Title>Thread action needs attention</Alert.Title
        ><Alert.Description>{threads.error}</Alert.Description>
        {#if threads.pending}<Button
            variant="outline"
            disabled={threads.busy}
            onclick={() => threads.checkPending()}>Check action status</Button
          ><Button
            variant="ghost"
            disabled={threads.busy}
            onclick={() => threads.retryPending().catch(() => {})}
            >Retry same action</Button
          >{/if}
      </Alert.Root>{/if}
    {#if !threads.detail && threads.status === "loading"}<CommunitySkeleton
        layout="messages"
        label="Loading threads"
        padded
      />
    {:else if threads.detail && projection}
      <div class="store-threads__conversation">
        <CommunityWorkspaceSurface
          {model}
          communityEmojis={threads.emojis}
          readOnly={disabled}
          readOnlyReason="This thread is read only or has an action awaiting confirmation."
          history={{
            hasOlder: threads.hasOlder,
            loading: threads.status === "refreshing",
          }}
          onLoadOlder={threads.older}
          handlers={{
            onAddReaction: (id, emoji, custom) =>
              threads.toggle(projection.targets[id]!, emoji, custom),
            onToggleReaction: (id, emoji, custom) =>
              threads.toggle(projection.targets[id]!, emoji, custom, true),
            onSendMessage: (_, text) => threads.send(text),
            onOpenThread: reply,
            onReply: reply,
            onSaveMessageEdit: (id, text) => threads.send(text, undefined, id),
            onCopyMessageLink: (id) => {
              const hash = storeRouteToHash({
                ...controller.state.route,
                replyId: id,
              });
              if (hash)
                void navigator.clipboard.writeText(
                  new URL(hash, window.location.href).href,
                );
            },
          }}
        >
          {#snippet messageSupplement(
            message,
          )}{#if projection.targets[message.id]}<StoreMessageReactions
                {threads}
                target={projection.targets[message.id]!}
                {disabled}
                showControls={false}
              />{/if}{/snippet}
        </CommunityWorkspaceSurface>
      </div>
    {:else}<ScrollArea
        bind:viewportRef={forumViewport}
        type="hover"
        orientation="vertical"
        class="store-threads__directory"
      >
        <ForumTopicList
          {topics}
          members={directoryMembers}
          viewport={forumViewport}
          disabled={threads.busy || !!threads.pending}
          composerStatus={threads.error
            ? { type: "error", message: threads.error }
            : undefined}
          onCreateTopic={threadWritable(controller)
            ? (draft) => threads.create(draft.title, draft.content)
            : undefined}
          onOpenTopic={(id) => threads.open(id)}
        />{#if threads.hasMore}<Button
            variant="ghost"
            class="store-threads__more"
            onclick={threads.more}>More threads</Button
          >{/if}</ScrollArea
      >{/if}
  </section>{/if}

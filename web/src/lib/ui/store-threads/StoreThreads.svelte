<script lang="ts">
  import "./store-threads.css";
  import {
    CommunityWorkspaceSurface,
    CommunityDetails,
    CommunityMemberAvatarGroup,
    CommunitySkeleton,
  } from "@lapismd/lapis-community/components";
  import type {
    CommunityWorkspaceModel,
    CommunityRoomModel,
  } from "@lapismd/lapis-community/components";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { untrack } from "svelte";
  import HashIcon from "@lucide/svelte/icons/hash";
  import XIcon from "@lucide/svelte/icons/x";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreThreadReplies from "./StoreThreadReplies.svelte";
  import StoreThreadCreate from "./StoreThreadCreate.svelte";
  import StoreMessageReactions from "./StoreMessageReactions.svelte";
  import { principalMember } from "../../collaboration/presentation.js";
  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state),
    threads = $derived(controller.threads);
  const replyId = $derived(view.route.replyId ?? null);
  let detailsOpen = $state(false);
  const writable = $derived(
    view.repository.permission === "write" ||
      view.repository.permission === "admin",
  );
  const projection = $derived(threads?.projection);
  const creator = $derived(threads?.detail?.thread.value.actor);
  const members = $derived(
    creator
      ? [
          ...new Map(
            [principalMember(creator), ...(projection?.members ?? [])].map(
              (m) => [m.id, m],
            ),
          ).values(),
        ]
      : (projection?.members ?? []),
  );
  const room = $derived<CommunityRoomModel>({
    id: threads?.detail?.thread.value.thread_id ?? "",
    name: threads?.detail?.thread.value.title ?? "Threads",
    kind: "channel",
    participantIds: members.map((m) => m.id),
    memberListCompleteness: "partial",
    archived: threads?.detail?.thread.value.status === "archived",
  });
  const disabled = $derived(
    !writable ||
      !!threads?.pending ||
      !!threads?.busy ||
      ["archived", "tombstoned"].includes(
        threads?.detail?.thread.value.status ?? "",
      ),
  );
  const model = $derived<CommunityWorkspaceModel>({
    communities: [],
    activeCommunityId: view.route.repository,
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
    currentMemberId: view.repository.principal,
  });
  $effect(() => {
    const id = room.id;
    untrack(() => {
      detailsOpen = false;
      if (id) void threads?.loadEmojis().catch(() => {});
    });
  });
</script>

{#if threads}
  <section class="store-threads" aria-label="Threads">
    <header class="store-threads__header">
      <h1>
        {#if threads.detail}<HashIcon
            aria-hidden="true"
          />{room.name}{:else}Threads{/if}
      </h1>
      {#if threads.detail}<CommunityMemberAvatarGroup
          {members}
          label="Thread participants"
        /><Button
          variant="ghost"
          size="sm"
          onclick={() => (detailsOpen = !detailsOpen)}>Members</Button
        ><Button variant="ghost" size="sm" onclick={() => threads.open(null)}
          >All threads</Button
        >{/if}
      <Button
        variant="ghost"
        size="sm"
        disabled={threads.busy}
        onclick={() => threads.refresh()}>Refresh threads</Button
      >
      <StoreThreadCreate {threads} disabled={!writable} />
    </header>
    {#if threads.error}<Alert.Root variant="destructive"
        ><Alert.Title>Thread action needs attention</Alert.Title
        ><Alert.Description>{threads.error}</Alert.Description
        >{#if threads.pending}<Button
            variant="outline"
            disabled={threads.busy}
            onclick={() => threads.checkPending()}>Check action status</Button
          ><Button
            variant="ghost"
            disabled={threads.busy}
            onclick={() => threads.retryPending().catch(() => {})}
            >Retry same action</Button
          >{/if}</Alert.Root
      >{/if}
    {#if !threads.detail && threads.status === "loading"}<CommunitySkeleton
        layout="messages"
      />
    {:else if threads.detail && projection}
      {#if projection.messages.some( (m) => m.parentMessageId?.startsWith("unloaded:") )}<p
          role="status"
        >
          Some replies refer to older messages. Load older messages to see their
          conversation.
        </p>
        <Button
          variant="ghost"
          disabled={!threads.hasOlder || threads.status === "refreshing"}
          onclick={threads.older}>Load older messages</Button
        >{/if}
      <div class="store-threads__panes">
        <div class="store-threads__conversation">
          <CommunityWorkspaceSurface
            {model}
            communityEmojis={threads.emojis}
            readOnly={disabled}
            readOnlyReason={!writable
              ? "You have read access to this repository."
              : "This thread is closed or has an action awaiting confirmation."}
            history={{
              key: room.id,
              complete: !threads.hasOlder,
              hasNewer: false,
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
              onOpenThread: (id) => {
                threads.selectReply(id);
                detailsOpen = false;
              },
              onReply: (id) => {
                threads.selectReply(id);
                detailsOpen = false;
              },
              onSaveMessageEdit: (id, text) =>
                threads.send(text, undefined, id),
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
        {#if replyId}<aside
            class="store-threads__side"
            aria-label="Thread replies"
          >
            <header>
              <h2>Replies</h2>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label="Close replies"
                onclick={() => threads.selectReply(null)}
                ><XIcon aria-hidden="true" /></Button
              >
            </header>
            <StoreThreadReplies
              {threads}
              rootId={replyId}
              {members}
              currentMemberId={view.repository.principal}
              {disabled}
            />
          </aside>{:else if detailsOpen}<aside
            class="store-threads__side"
            aria-label="Thread members"
          >
            <header>
              <h2>Members</h2>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label="Close members"
                onclick={() => (detailsOpen = false)}
                ><XIcon aria-hidden="true" /></Button
              >
            </header>
            <CommunityDetails
              {room}
              {members}
              details={{
                description:
                  "Participants visible in the loaded conversation. Load older messages to see earlier participants.",
                memberIds: members.map((m) => m.id),
              }}
            />
          </aside>{/if}
      </div>
    {:else}<div class="store-threads__directory">
        {#if !threads.threads.length}<p>
            No threads yet. Create a thread to start a conversation.
          </p>{/if}
        {#each threads.threads as thread (thread.value.thread_id)}<Button
            variant="ghost"
            onclick={() => threads.open(thread.value.thread_id)}
            ><HashIcon data-icon="inline-start" aria-hidden="true" />{thread
              .value.title}</Button
          >{/each}
        {#if threads.hasMore}<Button variant="outline" onclick={threads.more}
            >More threads</Button
          >{/if}
      </div>{/if}
  </section>
{/if}

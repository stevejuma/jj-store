<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import MoreIcon from "@lucide/svelte/icons/ellipsis";
  import UsersIcon from "@lucide/svelte/icons/users";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import MessagesIcon from "@lucide/svelte/icons/messages-square";
  import StoreContentEmpty from "../store-application/StoreContentEmpty.svelte";
  import CommunityInbox from "../community-inbox/CommunityInbox.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreThreadConversation from "./StoreThreadConversation.svelte";
  import StoreThreadSettings from "./StoreThreadSettings.svelte";
  import StoreThreadCreate from "./StoreThreadCreate.svelte";
  import { threadInboxRows } from "./thread-inbox.js";
  import { threadMembers, threadDisabled } from "./thread-view.js";
  import { agentMember } from "../community-agents/model.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const threads = $derived(controller.threads);
  const panel = useThreadPanel();
  let creating = $state(false);
  const rows = $derived(threadInboxRows(controller));
  const items = $derived(
    controller.state.route.threadRepository
      ? rows.filter(
          (r) => r.repository?.id === controller.state.route.threadRepository,
        )
      : rows,
  );
  const entries = $derived(
    Object.entries(controller.threadNavigation?.entries ?? {}),
  );
  const detail = $derived(threads?.detail);
  const id = $derived(controller.state.route.threadId);
  const missingDetail = $derived(
    !!id && (!detail || detail.thread.value.thread_id !== id),
  );
  const unavailable = $derived(missingDetail && threads?.status === "failed");
  const selected = $derived(
    id ? `${controller.state.route.repository}:${id}` : null,
  );
  const members = $derived([
    ...threadMembers(controller),
    ...(controller.execution?.value?.setups ?? []).map(agentMember),
  ]);
  const messages = $derived(
    threads?.projection.messages
      .filter((m) => !m.parentMessageId)
      .map((m) => ({
        ...m,
        reactions:
          threads?.reactionModels(threads.projection.targets[m.id]!) ?? [],
      })) ?? [],
  );
  $effect(() => {
    const key = detail?.thread.value.thread_id;
    untrack(() => {
      if (key) void threads?.loadEmojis().catch(() => {});
    });
  });
  $effect(() => {
    if (controller.execution?.available())
      untrack(() => void controller.execution?.ensure());
  });
  function reply(message: string) {
    threads?.selectReply(message);
    panel.open("replies");
  }
</script>

<CommunityInbox
  kind="thread"
  title="Threads"
  scopeLabel={controller.state.route.threadRepository
    ? (controller.state.repositories.find(
        (r) => r.id === controller.state.route.threadRepository,
      )?.name ?? "This repository")
    : undefined}
  onAll={controller.state.route.threadRepository
    ? () => void controller.actions.openThreads?.()
    : undefined}
  {items}
  activeId={selected}
  loading={!entries.length ||
    entries.some(([, e]) => e.loading && !e.threads.length)}
  threadTitle={detail?.thread.value.title ?? "Threads"}
  threadStatus={detail?.thread.value.status}
  onNew={() => (creating = true)}
  onBack={() => void controller.actions.openThreads?.()}
  onSelect={(target) => {
    const [home, thread] = target.split(":");
    void controller.actions.openThreads?.(thread, home);
  }}
  threadSidebar={detail ? settings : undefined}
>
  {#snippet threadActions({ openSidebar })}
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Refresh Threads"
      onclick={() => {
        controller.threadNavigation?.invalidate();
        void controller.threadNavigation?.load();
        if (id) void threads?.refresh();
      }}><RefreshIcon aria-hidden="true" /></Button
    >
    {#if detail}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="Thread contributors"
        onclick={() => panel.open("members")}
        ><UsersIcon aria-hidden="true" /></Button
      ><Button
        variant="ghost"
        size="icon-sm"
        aria-label="Conversation details"
        onclick={openSidebar}><MoreIcon aria-hidden="true" /></Button
      >{/if}
  {/snippet}
  {#if threads?.pending && threads.error}<Alert.Root variant="destructive"
      ><Alert.Title>Thread action needs attention</Alert.Title
      ><Alert.Description>{threads.error}</Alert.Description><Button
        disabled={threads.busy}
        onclick={() => void threads.checkPending()}>Check action status</Button
      ><Button
        variant="ghost"
        disabled={threads.busy}
        onclick={() => void threads.retryPending().catch(() => {})}
        >Retry same action</Button
      ></Alert.Root
    >{/if}
  <StoreNotice
    noticeKey="thread-inbox"
    title="Thread action needs attention"
    message={threads?.pending || unavailable
      ? ""
      : (threads?.error ?? controller.state.status.error)}
  />
  {#if missingDetail}
    {#if unavailable}<StoreContentEmpty
        Icon={MessagesIcon}
        title="Thread unavailable"
        description={threads?.error ||
          "This Thread is unavailable or no longer accessible."}
      />{:else}<CommunitySkeleton
        layout="messages"
        label="Loading Thread"
        padded
      />{/if}
  {:else if threads && detail}
    {#key `${threads.conversationScope}:${id}`}
      <StoreThreadConversation
        {controller}
        {threads}
        {messages}
        {members}
        user={members.find(
          (m) => m.id === controller.state.repository.principal,
        ) ?? controller.state.member}
        disabled={threadDisabled(controller)}
        onReply={reply}
      />
    {/key}
  {/if}
  {#snippet listFooter()}
    {#each entries.filter(([, e]) => e.error || e.cursor) as [home, entry] (home)}
      <Button
        variant="ghost"
        onclick={() =>
          void (entry.error
            ? controller.threadNavigation?.read(home)
            : controller.threadNavigation?.more(home))}
        >{entry.error ? "Retry Threads" : "More Threads"} · {controller.state.repositories.find(
          (r) => r.id === home,
        )?.name}</Button
      >
    {/each}
  {/snippet}
</CommunityInbox>
{#snippet settings({ close }: { close: () => void })}<StoreThreadSettings
    {controller}
    embedded
    onClose={close}
  />{/snippet}
{#if creating}<StoreThreadCreate
    {controller}
    onClose={() => (creating = false)}
  />{/if}

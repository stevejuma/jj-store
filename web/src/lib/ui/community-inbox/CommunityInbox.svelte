<!-- Community Inbox regions and conversation rows, adapted to owned session data. -->
<script lang="ts">
  import { AppShell, useAppShell } from "@lapismd/design-core/shell";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MailIcon from "@lucide/svelte/icons/mail";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import CloseIcon from "@lucide/svelte/icons/x";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { formatCommunityFullDate } from "../community-relative-time/model.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { Snippet } from "svelte";
  import { tick } from "svelte";
  import "./inbox.css";
  let {
    items,
    activeId,
    title = "Sessions",
    loading = false,
    onSelect,
    onNew,
    onBack,
    onAll,
    threadTitle,
    threadMember,
    threadStatus,
    threadActions,
    threadAvatar,
    threadSidebar,
    children,
  }: {
    items: readonly {
      id: string;
      author: CommunityMemberModel;
      context: string;
      excerpt?: string;
      lastActivityAt?: number;
      status: string;
    }[];
    activeId?: string | null;
    title?: string;
    loading?: boolean;
    onSelect: (id: string) => void;
    onNew: () => void;
    onBack: () => void;
    onAll?: () => void;
    threadTitle?: string;
    threadMember?: CommunityMemberModel;
    threadStatus?: string;
    threadActions?: Snippet<[{ openSidebar: () => void }]>;
    threadAvatar?: Snippet;
    threadSidebar?: Snippet;
    children: Snippet;
  } = $props();
  const shell = useAppShell();
  let listOpen = $state(true);
  let sidebarOpen = $state(false);
  let actionsElement = $state<HTMLDivElement>();
  let settingsHeading = $state<HTMLHeadingElement>();
  function openSidebar() {
    sidebarOpen = true;
    if (shell.mobile.resolvedMode === "mobile")
      shell.mobile.show("right", "agent-session-settings");
    void tick().then(() => settingsHeading?.focus({ preventScroll: true }));
  }
  function closeSidebar() {
    sidebarOpen = false;
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    actionsElement?.querySelector("button")?.focus({ preventScroll: true });
  }
  const expanded = $derived(
    shell.mobile.resolvedMode === "mobile"
      ? shell.mobile.stage === "left"
      : listOpen,
  );
</script>

<AppShell.Body
  layout="regions"
  label="Agent sessions inbox"
  class="lc-community-inbox"
>
  <AppShell.Body.Sidebar
    side="left"
    panelId="agent-sessions"
    label="Session chats"
    mobileLabel="Sessions"
    class="lc-community-inbox__list-region"
    bind:open={listOpen}
  >
    <div class="lc-community-inbox__list">
      <header class="lc-community-inbox__list-header">
        <strong>{title}</strong>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="New agent session"
          onclick={onNew}><PlusIcon aria-hidden="true" /></Button
        >
      </header>
      <div class="lc-community-inbox__list-header">
        <Button variant="ghost" size="sm" onclick={onBack}>Agents</Button>
        {#if onAll}<Button variant="ghost" size="sm" onclick={onAll}
            >All sessions</Button
          >{/if}
      </div>
      <ScrollArea class="lc-community-inbox__scroll">
        <div class="lc-community-inbox__items" aria-label="Session chats">
          {#each items as item (item.id)}
            <Button
              variant="ghost"
              class="lc-community-inbox__item"
              data-active={item.id === activeId ? "" : undefined}
              aria-pressed={item.id === activeId}
              data-ui-part="inbox-conversation"
              aria-label={`Open session ${item.author.name} · ${item.id}`}
              onclick={() => onSelect(item.id)}
            >
              <CommunityMemberAvatar
                member={item.author}
                size="small"
                shape="rounded"
                decorative
              />
              <span class="lc-community-inbox__item-body">
                <span class="lc-community-inbox__item-heading"
                  ><strong>{item.author.name}</strong><span>{item.status}</span
                  ></span
                >
                <span class="lc-community-inbox__context">{item.context}</span>
                {#if item.lastActivityAt !== undefined}<span
                    class="lc-community-inbox__context"
                  >
                    <CommunityRelativeTime
                      timestamp={item.lastActivityAt}
                      ariaLabel={`Last activity: ${formatCommunityFullDate(item.lastActivityAt)}`}
                    />
                  </span>
                {:else if item.excerpt}<span class="lc-community-inbox__excerpt"
                    >{item.excerpt}</span
                  >{/if}
              </span>
            </Button>
          {/each}
          {#if loading && !items.length}<CommunitySkeleton
              layout="messages"
              count={5}
              label="Loading sessions"
            />
          {:else if !items.length}<Empty.Root
              class="lc-community-inbox__empty"
              role="status"
              ><Empty.Header
                ><Empty.Title>No sessions yet</Empty.Title><Empty.Description
                  >Start a session to begin a chat with this agent.</Empty.Description
                ></Empty.Header
              ></Empty.Root
            >{/if}
        </div>
      </ScrollArea>
    </div>
  </AppShell.Body.Sidebar>
  <AppShell.Body.Content
    label="Selected agent session"
    class="lc-community-inbox__thread-region"
  >
    <section class="lc-community-inbox__thread">
      <header class="lc-community-inbox__thread-header">
        <div class="lc-community-inbox__thread-identity">
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Toggle session chats"
            title="Toggle session chats"
            aria-pressed={expanded}
            onclick={() => {
              if (shell.mobile.resolvedMode === "mobile") {
                if (expanded) shell.mobile.showMain();
                else shell.mobile.show("left", "agent-sessions");
              } else listOpen = !listOpen;
            }}><CommunitySidebarToggleIcon side="left" {expanded} /></Button
          >
          {#if threadAvatar}{@render threadAvatar()}{:else if threadMember}<CommunityMemberAvatar
              member={threadMember}
              size="small"
              shape="rounded"
              decorative
            />{/if}
          <div>
            <h1>{threadTitle ?? "Sessions"}</h1>
            {#if threadStatus}<p role="status">{threadStatus}</p>{/if}
          </div>
        </div>
        <div
          class="lc-community-inbox__thread-actions"
          bind:this={actionsElement}
        >
          {#if threadActions}{@render threadActions({ openSidebar })}{/if}
        </div>
      </header>
      {#if activeId}
        {@render children()}
      {:else}<Empty.Root
          class="lc-community-inbox__selection-empty"
          role="status"
          ><Empty.Header
            ><Empty.Media variant="icon"
              ><MailIcon aria-hidden="true" /></Empty.Media
            ><Empty.Title>Select a session</Empty.Title><Empty.Description
              >Choose a chat on the left, or start a new session.</Empty.Description
            ></Empty.Header
          ><Button onclick={onNew}>Start session</Button></Empty.Root
        >{/if}
    </section>
  </AppShell.Body.Content>
  {#if threadSidebar}
    <AppShell.Body.Sidebar
      side="right"
      panelId="agent-session-settings"
      label="Workspace and runtime"
      mobileLabel="Session settings"
      class="lc-community-inbox__settings"
      bind:open={sidebarOpen}
    >
      <header class="lc-community-inbox__thread-header">
        <h2 tabindex="-1" bind:this={settingsHeading}>Workspace and runtime</h2>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="Close workspace and runtime"
          onclick={closeSidebar}
        >
          <CloseIcon aria-hidden="true" />
        </Button>
      </header>
      {@render threadSidebar()}
    </AppShell.Body.Sidebar>
  {/if}
</AppShell.Body>

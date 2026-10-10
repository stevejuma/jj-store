<!-- Community Inbox regions and conversation rows, adapted to owned session data. -->
<script lang="ts">
  import { AppShell, useAppShell } from "@lapismd/design-core/shell";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import {
    readInboxPreferences,
    saveInboxPreferences,
    toggleInboxPreference,
    inboxPreferencesKey,
  } from "./inbox-preferences.js";
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import StoreToolbarSearch from "../StoreToolbarSearch.svelte";
  import CommunityInboxFilters from "./CommunityInboxFilters.svelte";
  import { emptyInboxFilters, filterInboxItems } from "./inbox-filters.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MailIcon from "@lucide/svelte/icons/mail";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import CloseIcon from "@lucide/svelte/icons/x";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityInboxSessions from "./CommunityInboxSessions.svelte";
  import { createInboxResize } from "./inbox-resize.svelte.js";
  import type { SessionInboxItem } from "./model.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { Snippet } from "svelte";
  import { tick, untrack, onMount } from "svelte";
  import "./inbox.css";
  let {
    items,
    activeId,
    title = "Sessions",
    kind = "session",
    loading = false,
    layoutStorage,
    onSelect,
    onNew,
    onBack,
    onAll,
    scopeLabel,
    threadTitle,
    threadMember,
    threadStatus,
    threadActions,
    threadAvatar,
    threadSidebar,
    listFooter,
    children,
  }: {
    items: readonly SessionInboxItem[];
    activeId?: string | null;
    title?: string;
    kind?: "session" | "thread";
    loading?: boolean;
    layoutStorage?: Storage;
    onSelect: (id: string) => void;
    onNew: () => void;
    onBack: () => void;
    onAll?: () => void;
    scopeLabel?: string;
    threadTitle?: string;
    threadMember?: CommunityMemberModel;
    threadStatus?: string;
    threadActions?: Snippet<[{ openSidebar: () => void }]>;
    threadAvatar?: Snippet;
    threadSidebar?: Snippet<[{ close: () => void }]>;
    listFooter?: Snippet;
    children: Snippet;
  } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  let preferences = $state(untrack(() => readInboxPreferences(storage, kind)));
  function togglePreference(field: "pinned" | "archived", id: string) {
    const restoreFocus = document.activeElement?.closest(".inbox-row-actions");
    preferences = toggleInboxPreference(preferences, field, id);
    saveInboxPreferences(storage, kind, preferences);
    if (restoreFocus)
      void tick().then(() => {
        const row = [
          ...(listElement?.querySelectorAll<HTMLElement>(
            "[data-inbox-identity]",
          ) ?? []),
        ].find((row) => row.dataset.inboxIdentity === id);
        const next = row?.querySelector<HTMLButtonElement>(
          `button[aria-label="${field === "pinned" ? (preferences.pinned.includes(id) ? "Unpin conversation" : "Pin conversation") : preferences.archived.includes(id) ? "Archive on this device" : "Restore on this device"}"]`,
        );
        (
          next ??
          listElement?.querySelector<HTMLButtonElement>(
            'button[aria-label="Search loaded conversations"],button[aria-label="Close search"]',
          )
        )?.focus({ preventScroll: true });
      });
  }
  onMount(() => {
    const update = (event: StorageEvent) => {
      if (
        event.storageArea === storage &&
        (event.key === inboxPreferencesKey(kind) || event.key === null)
      )
        preferences = readInboxPreferences(storage, kind);
    };
    window.addEventListener("storage", update);
    return () => window.removeEventListener("storage", update);
  });
  const plural = $derived(kind === "thread" ? "Threads" : "Sessions");
  const listLabel = $derived(
    kind === "thread" ? "Thread conversations" : "Session chats",
  );
  const panelId = $derived(
    kind === "thread" ? "thread-conversations" : "agent-sessions",
  );
  const settingsId = $derived(
    kind === "thread"
      ? "thread-conversation-settings"
      : "agent-session-settings",
  );
  let search = $state(""),
    searchOpen = $state(false),
    view = $state("all"),
    filters = $state(emptyInboxFilters());
  $effect(() => {
    view = scopeLabel ? "scope" : "all";
  });
  const sections = $derived([
    { value: "all", label: "All" },
    ...(scopeLabel ? [{ value: "scope", label: scopeLabel }] : []),
    ...(kind === "thread"
      ? [{ value: "active", label: "Active" }]
      : [{ value: "connected", label: "Connected" }]),
    { value: "archived", label: "Archived" },
    ...(kind === "thread" ? [] : [{ value: "agents", label: "Agents" }]),
  ]);
  function selectView(next: string) {
    if (next === "agents") {
      onBack();
      return;
    }
    view = next;
    if (next === "all") {
      if (onAll) onAll();
      else if (kind === "thread") onBack();
    }
  }
  const visibleItems = $derived(
    filterInboxItems(
      items,
      search,
      filters,
      view === "archived" ? "all" : view,
    ).filter(
      (item) =>
        preferences.archived.includes(item.targetId ?? item.id) ===
        (view === "archived"),
    ),
  );
  const shell = useAppShell();
  let bodyElement = $state<HTMLElement | null>(null);
  let listElement = $state<HTMLElement | null>(null);
  const resize = createInboxResize(
    () => bodyElement,
    () => listElement,
    storage,
  );
  let listOpen = $state(true);
  let sidebarOpen = $state(false);
  let actionsElement = $state<HTMLDivElement>();
  let settingsHeading = $state<HTMLHeadingElement>();
  function openSidebar() {
    sidebarOpen = true;
    if (shell.mobile.resolvedMode === "mobile")
      shell.mobile.show("right", settingsId);
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
  bind:ref={bodyElement}
  layout="regions"
  label={`${plural} inbox`}
  class={`lc-community-inbox${kind === "thread" ? " lc-community-inbox--thread" : ""}`}
>
  <AppShell.Body.Sidebar
    bind:ref={listElement}
    side="left"
    {panelId}
    label={listLabel}
    mobileLabel={plural}
    class="lc-community-inbox__list-region"
    style={resize.width === undefined
      ? undefined
      : `--ui-shell-body-sidebar-width: ${resize.width}px`}
    bind:open={listOpen}
  >
    <div class="lc-community-inbox__list">
      <header
        class="lc-community-inbox__list-header"
        aria-label={`${title} navigation`}
      >
        <StoreViewTabs
          value={view}
          {sections}
          onSelect={selectView}
          label={kind === "thread" ? "Thread views" : "Session views"}
          hideTabs={searchOpen}
        >
          {#snippet leading()}<StoreToolbarSearch
              label="Search loaded conversations"
              scope={kind}
              query={search}
              bind:open={searchOpen}
              live
              onQuery={(query) => {
                search = query;
              }}
            />{/snippet}
          {#snippet trailing()}
            <CommunityInboxFilters {items} bind:filters />
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={kind === "thread"
                ? "New Thread"
                : "New agent session"}
              onclick={onNew}><PlusIcon aria-hidden="true" /></Button
            >
          {/snippet}
        </StoreViewTabs>
      </header>
      <ScrollArea class="lc-community-inbox__scroll">
        <div class="lc-community-inbox__items" aria-label={listLabel}>
          <CommunityInboxSessions
            items={visibleItems}
            {activeId}
            {onSelect}
            {loading}
            {kind}
            pinned={preferences.pinned}
            archived={preferences.archived}
            onPin={(id) => togglePreference("pinned", id)}
            onArchive={(id) => togglePreference("archived", id)}
          />
          {@render listFooter?.()}
          {#if !loading && !visibleItems.length}<Empty.Root
              class="lc-community-inbox__empty"
              role="status"
              ><Empty.Header
                ><Empty.Media variant="icon"
                  ><MailIcon aria-hidden="true" /></Empty.Media
                ><Empty.Title
                  >{items.length
                    ? "No matching conversations"
                    : `No ${kind === "thread" ? "Threads" : "sessions"} yet`}</Empty.Title
                ><Empty.Description
                  >{items.length
                    ? "Try changing your search or filters."
                    : kind === "thread"
                      ? "Create a Thread to begin a conversation."
                      : "Start a session to begin a chat with this agent."}</Empty.Description
                ></Empty.Header
              ></Empty.Root
            >{/if}
        </div>
      </ScrollArea>
    </div>
  </AppShell.Body.Sidebar>
  {#if listOpen && shell.mobile.resolvedMode !== "mobile"}
    <button
      type="button"
      class="lc-community-inbox__resize-handle"
      data-resizing={resize.resizing ? "" : undefined}
      aria-label={`Resize ${listLabel.toLowerCase()}`}
      title="Drag to resize. Double-click to reset width."
      onpointerdown={resize.begin}
      onpointermove={resize.move}
      onpointerup={resize.finish}
      onpointercancel={resize.finish}
      ondblclick={resize.reset}
      onkeydown={resize.key}
    >
      <span aria-hidden="true"></span>
    </button>
  {/if}
  <AppShell.Body.Content
    label={`Selected ${kind}`}
    class="lc-community-inbox__thread-region"
  >
    <section class="lc-community-inbox__thread">
      <header class="lc-community-inbox__thread-header">
        <div class="lc-community-inbox__thread-identity">
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={`Toggle ${listLabel.toLowerCase()}`}
            title={`Toggle ${listLabel.toLowerCase()}`}
            aria-pressed={expanded}
            onclick={() => {
              if (shell.mobile.resolvedMode === "mobile") {
                if (expanded) shell.mobile.showMain();
                else shell.mobile.show("left", panelId);
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
            <h1 title={threadTitle ?? plural}>
              {threadTitle ?? plural}
            </h1>
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
            ><Empty.Title>Select a {kind}</Empty.Title><Empty.Description
              >{kind === "thread"
                ? "Choose a Thread on the left, or create one."
                : "Choose a chat on the left, or start a new session."}</Empty.Description
            ></Empty.Header
          ><Button onclick={onNew}
            >{kind === "thread" ? "Create Thread" : "Start session"}</Button
          ></Empty.Root
        >{/if}
    </section>
  </AppShell.Body.Content>
  {#if threadSidebar}
    <AppShell.Body.Sidebar
      side="right"
      panelId={settingsId}
      label="Conversation details"
      mobileLabel="Session settings"
      class="lc-community-inbox__settings"
      bind:open={sidebarOpen}
    >
      <header class="lc-community-inbox__thread-header">
        <h2 tabindex="-1" bind:this={settingsHeading}>Conversation details</h2>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="Close conversation details"
          onclick={closeSidebar}
        >
          <CloseIcon aria-hidden="true" />
        </Button>
      </header>
      {#if sidebarOpen}{@render threadSidebar({ close: closeSidebar })}{/if}
    </AppShell.Body.Sidebar>
  {/if}
</AppShell.Body>

<!-- Community Inbox regions and conversation rows, adapted to owned session data. -->
<script lang="ts">
  import CommunityInboxListEmpty from "./CommunityInboxListEmpty.svelte";
  import { inboxSections, inboxPanelIds } from "./inbox-sections.js";
  import { AppShell, useAppShell } from "@lapismd/design-core/shell";
  import CommunityInboxThreadHeader from "./CommunityInboxThreadHeader.svelte";
  import { createInboxPreferenceState } from "./inbox-preference-state.svelte.js";
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
  import CommunityInboxSessions from "./CommunityInboxSessions.svelte";
  import { createInboxResize } from "./inbox-resize.svelte.js";
  import type { SessionInboxItem } from "./model.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { Snippet } from "svelte";
  import { tick, untrack } from "svelte";
  import "./inbox.css";
  import "./inbox-thread.css";
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
  const preferences = createInboxPreferenceState(
    storage,
    () => kind,
    () => listElement,
  );
  const { plural, listLabel, panelId, settingsId } = $derived(
    inboxPanelIds(kind),
  );
  let search = $state(""),
    searchOpen = $state(false),
    view = $state("all"),
    filters = $state(emptyInboxFilters());
  $effect(() => {
    view = scopeLabel ? "scope" : "all";
  });
  const sections = $derived(inboxSections(kind, scopeLabel));
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
        preferences.value.archived.includes(item.targetId ?? item.id) ===
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
            pinned={preferences.value.pinned}
            archived={preferences.value.archived}
            onPin={(id) => preferences.toggle("pinned", id)}
            onArchive={(id) => preferences.toggle("archived", id)}
          />
          {@render listFooter?.()}
          {#if !loading && !visibleItems.length}<CommunityInboxListEmpty
              {kind}
              count={items.length}
            />{/if}
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
      <CommunityInboxThreadHeader
        {listLabel}
        {plural}
        {expanded}
        {threadTitle}
        {threadMember}
        {threadStatus}
        {threadActions}
        {threadAvatar}
        {openSidebar}
        bind:actionsElement
        onToggle={() => {
          if (shell.mobile.resolvedMode === "mobile") {
            if (expanded) shell.mobile.showMain();
            else shell.mobile.show("left", panelId);
          } else listOpen = !listOpen;
        }}
      />
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

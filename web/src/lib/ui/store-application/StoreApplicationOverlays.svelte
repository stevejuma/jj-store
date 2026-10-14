<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreApplicationMenu from "./StoreApplicationMenu.svelte";
  import StoreThreadFocus from "../store-threads/StoreThreadFocus.svelte";
  import StoreFocusedDiscussion from "../store-discussion/StoreFocusedDiscussion.svelte";
  import type { AppShellController } from "@lapismd/design-core/shell";
  import type { Snippet } from "svelte";
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  let {
    controller,
    stackWorkspace,
    current,
    settingsOpen,
    settingsNavigationNotice = "",
    conflictOpen = $bindable(false),
    searchOpen = $bindable(false),
    conflictReturnFocus,
    shell,
    editorOpen,
    threadFocused,
    menu = $bindable(),
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    current: StoreRoute;
    settingsOpen: boolean;
    settingsNavigationNotice?: string;
    conflictOpen?: boolean;
    searchOpen?: boolean;
    conflictReturnFocus: HTMLElement | null;
    shell: AppShellController;
    editorOpen: boolean;
    threadFocused: boolean;
    menu?: Snippet;
  } = $props();
  const focused = $derived(
    !settingsOpen && current.page !== "settings" && !editorOpen,
  );
</script>

<StoreApplicationMenu {controller} disabled={editorOpen} bind:menu />
{#if !settingsOpen && current.agentsHub && controller.execution && !controller.execution.chat.open}<StoreLazyView
    load={views.agentDetails}
    label="Agent details"
    active={Boolean(controller.execution.detailsId)}
    >{#snippet content(View)}<View
        manager={controller.execution!}
        {shell}
        route={current}
      />{/snippet}</StoreLazyView
  >{/if}
<StoreFocusedDiscussion open={focused && !threadFocused} />
<StoreThreadFocus open={focused && threadFocused} {controller} />
<StoreLazyView
  load={views.conflicts}
  label="Conflict editor"
  active={conflictOpen}
  >{#snippet content(View)}<View
      {controller}
      open={conflictOpen}
      returnFocus={conflictReturnFocus}
      onClose={() => (conflictOpen = false)}
    />{/snippet}</StoreLazyView
>
{#if stackWorkspace && isStacksRoute(current)}
  <StoreLazyView
    load={views.stackConflicts}
    label="Workspace conflicts"
    active={Boolean(stackWorkspace.conflictOpen)}
    >{#snippet content(View)}<View
        controller={stackWorkspace}
        bind:open={stackWorkspace.conflictOpen}
      />{/snippet}</StoreLazyView
  >
{/if}
{#if !settingsOpen}<StoreLazyView
    load={views.search}
    label="Search"
    active={searchOpen}
    >{#snippet content(View)}<View
        {controller}
        bind:open={searchOpen}
      />{/snippet}</StoreLazyView
  >{/if}

<StoreNotice
  noticeKey="repository-settings-navigation"
  title="Navigation paused"
  message={settingsNavigationNotice}
/>

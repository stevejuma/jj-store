<!-- @community-copy src/components/community-application/CommunityApplication.svelte -->
<script lang="ts">
  import StoreApplicationInspectionPanels from "./StoreApplicationInspectionPanels.svelte";
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { tick, untrack } from "svelte";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import { useThreadPanel } from "../store-threads/thread-panel.svelte.js";
  let {
    controller,
    stackWorkspace,
    shell,
    onSelectPage,
    threadFocused = false,
    visible = true,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    threadFocused?: boolean;
    visible?: boolean;
    shell: AppShellController;
    onSelectPage: StoreController["actions"]["selectPage"];
  } = $props();
  const inspection = useConversationInspection();
  const chatOpen = $derived(controller.execution?.chat.open === true);
  const appearance = useAppearance();
  const agentFocused = $derived(appearance?.live.threads === "focus");
  const current = $derived(controller.state.route);
  const threadPanel = useThreadPanel();
  const detailsRail = untrack(() => shell.getPanel(repositoryDetailsPanelId)!);
  const threadLabel = $derived(
    threadPanel.kind === "settings"
      ? "Thread settings"
      : threadPanel.kind === "members"
        ? "Thread contributors"
        : "Thread replies",
  );
  const projectAgent = $derived(
    current.page === "projects" && current.workTab === "agents",
  );
  const agentKey = $derived(
    !current.agentsHub &&
      (current.page === "agents" || projectAgent) &&
      current.agentSessionId
      ? `${current.repository}:${current.workAgentRepository ?? ""}:${current.agentSessionId}`
      : null,
  );
  const agentController = $derived(
    projectAgent ? controller.projects?.agents.response : controller,
  );
  let chatOpener: HTMLElement | null = null;
  $effect(() => {
    const open = chatOpen;
    untrack(() => {
      if (open) {
        chatOpener =
          document.activeElement instanceof HTMLElement
            ? document.activeElement
            : null;
        shell.right.open();
        if (shell.mobile.resolvedMode === "mobile")
          shell.mobile.show("right", "right", chatOpener);
      }
    });
  });
  async function closeChat() {
    controller.execution?.chat.close();
    shell.right.close();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    await tick();
    if (chatOpener?.isConnected) chatOpener.focus({ preventScroll: true });
  }
  let previousAgentKey: string | null = null;
  let agentOpener = $state<HTMLElement | null>(null);
  $effect(() => {
    const key = agentKey;
    const focused = agentFocused;
    untrack(() => {
      if (key) {
        if (key !== previousAgentKey)
          agentOpener =
            document.activeElement instanceof HTMLElement &&
            document.activeElement.matches(".store-collaboration__row-select")
              ? document.activeElement
              : null;
        if (focused) shell.right.close();
        else shell.right.open();
        void tick().then(() => {
          if (
            agentKey === key &&
            !agentFocused &&
            shell.mobile.resolvedMode === "mobile"
          )
            shell.mobile.show("right", "right", agentOpener);
        });
      } else if (previousAgentKey && current.page !== "threads") {
        shell.right.close();
        if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
      }
      previousAgentKey = key;
    });
  });
  const stackSettingsKey = $derived(
    isStacksRoute(current) &&
      stackWorkspace?.settingsOpen &&
      stackWorkspace.snapshot?.repository === current.repository
      ? stackWorkspace.snapshot.active
      : null,
  );
  let previousStackKey: string | null = null;
  $effect(() => {
    const key = stackSettingsKey;
    untrack(() => {
      if (previousStackKey && key !== previousStackKey) {
        stackWorkspace && (stackWorkspace.settingsOpen = false);
        if (!agentKey && current.page !== "threads") {
          shell.right.close();
          if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
        }
      } else if (key) {
        shell.right.open();
        if (shell.mobile.resolvedMode === "mobile")
          shell.mobile.show("right", "right");
      }
      previousStackKey = key;
    });
  });
  async function closeStackSettings() {
    if (stackWorkspace) stackWorkspace.settingsOpen = false;
    shell.right.close();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    await tick();
    shell.mobile
      .getRootElement()
      ?.querySelector<HTMLElement>(
        '.stack-workspace [aria-label="Workspace actions"]',
      )
      ?.focus({ preventScroll: true });
  }
  async function closeAgent() {
    const target = agentOpener?.isConnected
      ? agentOpener
      : shell.mobile
          .getRootElement()
          ?.querySelector<HTMLElement>(
            '.store-collaboration__row-select[aria-pressed="true"]',
          );
    if (projectAgent) await controller.projects?.agents.select(null, null);
    else await controller.actions.selectAgentSession(null);
    await tick();
    target?.focus({ preventScroll: true });
  }
</script>

{#if !inspection?.current && !inspection?.file && !chatOpen && current.page === "threads" && controller.threads?.detail && !threadFocused && threadPanel.kind !== "settings"}
  <AppShell.Sidebar
    hidden={!visible}
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label={threadLabel}
    resizeLabel="Resize thread details"
    closeable
  >
    <StoreLazyView load={views.threadSidebar} label="Thread details"
      >{#snippet content(View)}<View {controller} />{/snippet}</StoreLazyView
    >
  </AppShell.Sidebar>
{/if}
{#if !inspection?.current && !inspection?.file && !chatOpen && stackSettingsKey && stackWorkspace}
  <AppShell.Sidebar
    hidden={!visible}
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label="Workspace settings"
    resizeLabel="Resize workspace settings"
    closeable
  >
    {#key stackSettingsKey}<StoreLazyView
        load={views.stackSettings}
        label="Workspace settings"
        >{#snippet content(View)}<View
            controller={stackWorkspace}
            onClose={closeStackSettings}
          />{/snippet}</StoreLazyView
      >{/key}
  </AppShell.Sidebar>
{/if}
{#snippet agentResponse(focused = false)}
  <StoreLazyView
    load={views.agentSidebar}
    label="Agent response"
    active={Boolean(agentKey)}
    >{#snippet content(View)}<View
        controller={agentController}
        error={projectAgent
          ? (controller.projects?.agents.selectionError ?? "")
          : ""}
        {focused}
        open={visible && !!agentKey && focused === agentFocused}
        returnFocus={agentOpener}
        onClose={closeAgent}
      />{/snippet}</StoreLazyView
  >
{/snippet}
<!-- Controlled open clears the mobile sheet before its teleported content is removed. -->
{#if !current.agentsHub && !inspection?.current && !inspection?.file && !chatOpen}{@render agentResponse(
    true,
  )}{/if}
{#if !inspection?.current && !inspection?.file && !chatOpen && agentKey && !agentFocused}
  <AppShell.Sidebar
    hidden={!visible}
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label="Agent response"
    mobileLabel="Agent response"
    resizeLabel="Resize agent response sidebar"
    closeable
  >
    {@render agentResponse()}
  </AppShell.Sidebar>
{/if}

{#if !inspection?.current && !inspection?.file && chatOpen && controller.execution}<AppShell.Sidebar
    hidden={!visible}
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label="Chat"
    mobileLabel="Chat"
    resizeLabel="Resize chat sidebar"
    closeable
    ><StoreLazyView load={views.agentChatSidebar} label="Agent chat"
      >{#snippet content(View)}<View
          {controller}
          onClose={closeChat}
        />{/snippet}</StoreLazyView
    ></AppShell.Sidebar
  >{/if}

<StoreApplicationInspectionPanels
  {controller}
  {stackWorkspace}
  {shell}
  {onSelectPage}
  {visible}
/>

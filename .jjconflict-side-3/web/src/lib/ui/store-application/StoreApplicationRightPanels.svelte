<!-- @community-copy src/components/community-application/CommunityApplication.svelte -->
<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import { tick, untrack } from "svelte";
  import { repositoryDetailsPanelId } from "../layout-storage.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreProjectsRail from "../store-work/StoreProjectsRail.svelte";
  import StoreAgentSidebar from "../store-collaboration/StoreAgentSidebar.svelte";
  import StoreThreadSidebar from "../store-threads/StoreThreadSidebar.svelte";
  import { useThreadPanel } from "../store-threads/thread-panel.svelte.js";
  import StoreRepositoryDetails from "./StoreRepositoryDetails.svelte";
  import StoreRepositoryNavigation from "./StoreRepositoryNavigation.svelte";
  import StoreRepositoryContextActions from "./StoreRepositoryContextActions.svelte";
  let {
    controller,
    shell,
    onSelectPage,
    threadFocused = false,
  }: {
    controller: StoreController;
    threadFocused?: boolean;
    shell: AppShellController;
    onSelectPage: StoreController["actions"]["selectPage"];
  } = $props();
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
    (current.page === "agents" || projectAgent) && current.agentSessionId
      ? `${current.repository}:${current.workAgentRepository ?? ""}:${current.agentSessionId}`
      : null,
  );
  const agentController = $derived(
    projectAgent ? controller.projects?.agents.response : controller,
  );
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

{#if current.page === "threads" && controller.threads?.detail && !threadFocused}
  <AppShell.Sidebar
    side="right"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    class="community-application__standard-right-sidebar"
    label={threadLabel}
    resizeLabel="Resize thread details"
    closeable
  >
    <StoreThreadSidebar {controller} />
  </AppShell.Sidebar>
{/if}
{#snippet agentResponse(focused = false)}
  <StoreAgentSidebar
    controller={agentController}
    error={projectAgent
      ? (controller.projects?.agents.selectionError ?? "")
      : ""}
    {focused}
    open={!!agentKey && focused === agentFocused}
    returnFocus={agentOpener}
    onClose={closeAgent}
  />
{/snippet}
<!-- Controlled open clears the mobile sheet before its teleported content is removed. -->
{@render agentResponse(true)}
{#if agentKey && !agentFocused}
  <AppShell.Sidebar
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

<AppShell.Sidebar
  side="right"
  sidebarController={detailsRail}
  constraintPriority={100}
  variant="rail"
  class="community-application__project-overview-sidebar"
  label="Repository details"
  resizeLabel="Resize repository details"
  closeable
>
  <AppShell.Sidebar.Body>
    {#if current.page === "projects" && controller.projects && !isRepositoryProjects(current)}
      <StoreProjectsRail {controller} />
    {:else}
      <StoreRepositoryDetails {controller} />
      <StoreRepositoryNavigation page={current.page} onSelect={onSelectPage} />
      <StoreRepositoryContextActions {controller} />
    {/if}
  </AppShell.Sidebar.Body>
</AppShell.Sidebar>

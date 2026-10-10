<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import { ConfirmDialog } from "@lapismd/design-core/forms/dialog";
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  import CommunityAgents from "../community-agents/CommunityAgents.svelte";
  import CommunityInbox from "../community-inbox/CommunityInbox.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import StoreAgentLauncher from "./StoreAgentLauncher.svelte";
  import StoreAgentConversation from "./StoreAgentConversation.svelte";
  import StoreAgentChatActions from "./StoreAgentChatActions.svelte";
  import StoreConversationMetadata from "./StoreConversationMetadata.svelte";
  import StoreAgentRuntimeSettings from "./StoreAgentRuntimeSettings.svelte";
  import StoreAgentRequestRecovery from "./StoreAgentRequestRecovery.svelte";
  import StoreAgentSetupEditor from "./StoreAgentSetupEditor.svelte";
  import { agentMember } from "../community-agents/model.js";
  import { sessionItems } from "./session-presentation.js";
  import "./agent-execution.css";
  let {
    manager,
    controller,
    route,
    onReview,
    layoutStorage,
    repositories = [],
  }: {
    manager: AgentExecution;
    controller?: StoreController;
    route: StoreRoute;
    onReview?: (repository: string, commit: string, operation: string) => void;
    layoutStorage?: Storage;
    repositories?: readonly { id: string; name: string }[];
  } = $props();
  let editorOpen = $state(false),
    editing = $state<AgentSetup | undefined>(),
    deleting = $state<AgentSetup | undefined>();
  const directory = $derived((route.agentView ?? "directory") === "directory");
  const selectedId = $derived(
    route.agentView === "session" ? route.executionSessionId : undefined,
  );
  const agent = $derived(
    manager.value?.setups.find((a) => a.id === route.setupId),
  );
  const chat = $derived(
    manager.value?.sessions.find((s) => s.id === route.executionSessionId),
  );
  const inboxItems = $derived([
    ...(route.agentView === "new"
      ? [
          {
            id: "new-session",
            author: agent
              ? agentMember(agent)
              : {
                  id: "new-session",
                  name: "New chat",
                  initials: "AG",
                  handle: "",
                },
            context: "New chat",
            excerpt:
              manager.drafts[`new:${route.setupId || "unassigned"}`] ||
              "Choose a workspace and connect",
            status: "Draft",
          },
        ]
      : []),
    ...sessionItems(
      manager.value,
      route.setupId,
      !!manager.connectionError,
      repositories,
    ),
  ]);
  $effect(() => {
    if (route.agentView === "activity" && manager.value)
      untrack(() =>
        manager.viewActivity(
          route.setupId ?? undefined,
          route.executionSessionId ?? undefined,
        ),
      );
  });
  $effect(() => {
    const view = route.agentView,
      setup = route.setupId,
      available = manager.available();
    untrack(() => {
      manager.open = true;
      manager.launchOpen = view === "new";
      if (view === "new") manager.setupId = setup ?? "";
      if (available) void manager.ensure();
    });
  });
  $effect(() => {
    const id = selectedId,
      available = manager.available();
    if (available && id)
      untrack(() => {
        void manager.select(id);
      });
  });
  onDestroy(() => manager.pauseView());
  function edit(agent?: AgentSetup) {
    editing = agent;
    editorOpen = true;
  }
  function start() {
    manager.newChat(route.setupId ?? undefined);
  }
  $effect(() => {
    const agent = manager.editRequested;
    if (agent)
      untrack(() => {
        edit(agent);
        manager.editRequested = undefined;
      });
  });
</script>

{#snippet runtimeSidebar()}
  {#if chat}
    {#key chat.id}<StoreConversationMetadata
        {manager}
        query={{ session: chat.id }}
      /><StoreAgentRuntimeSettings {manager} session={chat} {onReview} />{/key}
  {/if}
{/snippet}

{#if route.agentView === "session" || route.agentView === "sessions" || route.agentView === "new" || route.agentView === "activity"}
  <CommunityInbox
    {layoutStorage}
    items={inboxItems}
    activeId={route.agentView === "new"
      ? "new-session"
      : route.executionSessionId}
    scopeLabel={route.setupId ? (agent?.name ?? "This agent") : undefined}
    title={agent ? `${agent.name} sessions` : "Sessions"}
    loading={!manager.value}
    threadSidebar={route.agentView === "session" && chat
      ? runtimeSidebar
      : undefined}
    threadTitle={route.agentView === "new"
      ? (agent?.name ?? "New agent chat")
      : (chat?.conversation_title ?? "New conversation")}
    threadMember={route.agentView === "new"
      ? agent && agentMember(agent)
      : chat && {
          ...agentMember({
            id: chat.setup_id ?? chat.id,
            name: chat.title,
            avatar: chat.avatar,
          }),
          acpAgentId: chat.setup_id ?? undefined,
        }}
    threadStatus={route.agentView === "new" || !chat
      ? undefined
      : `${chat.runner} · ${chat.state} · Published ${chat.published_sequence}/${chat.last_sequence}`}
    onBack={() => manager.directory()}
    onNew={start}
    onAll={route.setupId ? () => manager.viewSessions() : undefined}
    onSelect={(id) => {
      if (id !== "new-session")
        manager.openSession(id, route.setupId ?? undefined);
    }}
  >
    {#snippet threadActions({ openSidebar })}<StoreAgentChatActions
        {manager}
        onRuntime={openSidebar}
        session={route.agentView === "session" ? chat : undefined}
        agentId={route.setupId ?? undefined}
      />{/snippet}
    {#if !manager.available()}<p role="status">
        Agent execution is unavailable on this device.
      </p>
    {:else if route.agentView === "new"}<StoreAgentLauncher {manager} />
    {:else if manager.session && manager.session.id === route.executionSessionId}<StoreAgentConversation
        {controller}
        {manager}
      />
    {:else if !manager.value}<CommunitySkeleton
        layout="messages"
        count={5}
        padded
        label="Loading session"
      />
    {:else}<p role="status">
        This session is unavailable or no longer accessible.
      </p>{/if}
    {#if route.agentView !== "session"}<StoreAgentRequestRecovery
        {manager}
      />{/if}
  </CommunityInbox>
{:else}
  <AppShell.Body layout="regions" label="Agents workspace">
    <AppShell.Body.Content
      label={directory ? "Agents directory" : "Agent workspace"}
    >
      {#if directory}<CommunityAgents
          value={manager.value}
          unknown={!!manager.connectionError}
          available={manager.available()}
          busy={manager.busy || !!manager.pending}
          onOpen={(agent) => (manager.detailsId = agent.id)}
          onAdd={() => edit()}
          onEdit={edit}
          onStart={(id) => void manager.startAgent(id)}
          onChat={(id) => manager.newChat(id)}
          onStop={(id) => void manager.stopAgent(id)}
          onSessions={(id) => manager.viewSessions(id)}
          onDelete={(agent) => (deleting = agent)}
          onRefresh={() => void manager.refresh()}
        />
      {:else if !manager.available()}<p role="status">
          Agent execution is unavailable on this device.
        </p>
      {/if}
      <StoreAgentRequestRecovery {manager} />
    </AppShell.Body.Content>
  </AppShell.Body>
{/if}
{#if editorOpen}{#key editing?.id ?? "new"}<StoreAgentSetupEditor
      {manager}
      setup={editing}
      bind:open={editorOpen}
    />{/key}{/if}
<ConfirmDialog
  open={!!deleting}
  onOpenChange={(open) => {
    if (!open) deleting = undefined;
  }}
  title="Delete agent?"
  description="Remove this saved configuration. Existing sessions and their workspaces remain available."
  confirmLabel="Delete agent"
  onConfirm={async () => {
    if (!deleting) return;
    await manager.saveSetup(undefined, deleting.id);
    if (manager.pending || manager.error) return false;
    deleting = undefined;
  }}
/>

<script lang="ts">
  import { onDestroy } from "svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageIcon from "@lucide/svelte/icons/messages-square";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityAgentActivityFeed from "../community-agent-activity/CommunityAgentActivityFeed.svelte";
  import StoreAgentRequestRecovery from "./StoreAgentRequestRecovery.svelte";
  import StoreAgentTranscript from "./StoreAgentTranscript.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { activityRows, runtimeChoices } from "./presentation.js";
  import { runtimeMatches, waitForRuntime } from "./runtime-preparation.js";
  import { conversationError } from "./conversation-error.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  import { completedAgentChanges } from "../../controller/agent-workspace-changes.js";
  import StoreAgentChangesPanel from "../store-agent-chat/StoreAgentChangesPanel.svelte";
  let { manager }: { manager: AgentExecution } = $props();
  const session = $derived(manager.session);
  const facts = $derived(object(session?.capabilities));
  const rows = $derived(activityRows(manager.events));
  const changes = $derived(
    session ? completedAgentChanges(session, manager.events) : [],
  );
  const active = $derived(
    [...manager.events]
      .reverse()
      .find((e) => object(e.payload).type === "started"),
  );
  const idle = $derived(
    session?.connected === true &&
      (session.state === "idle" || session.state === "running"),
  );
  const preferences = $derived(
    manager.value?.setups.find((a) => a.id === session?.setup_id)
      ?.preferences ?? session?.preferences,
  );
  const canConfigure = $derived(
    !!preferences?.mode &&
      runtimeChoices(object(facts.session), "mode").some(
        (mode) => mode.id === preferences?.mode,
      ),
  );
  const ready = $derived(
    facts.modeReady === true && runtimeMatches(session, preferences),
  );
  let preparationError = $state("");
  const error = $derived(
    session &&
      ((manager.errorSessionId === session.id && manager.error) ||
        preparationError ||
        conversationError(session, manager.events)),
  );
  let preparing = $state(false);
  let mounted = true;
  onDestroy(() => {
    mounted = false;
  });
  async function send() {
    const id = manager.selected,
      draft = manager.drafts[id],
      wanted = preferences && { ...preferences };
    if (
      !draft?.trim() ||
      !idle ||
      (!ready && !canConfigure) ||
      manager.pending ||
      manager.busy ||
      preparing
    )
      return;
    preparing = true;
    preparationError = "";
    try {
      if (!ready && wanted) {
        const request = await manager.commandSession(id, {
          action: "configure",
          preferences: wanted,
        });
        if (
          !(await waitForRuntime(manager, id, request, wanted, () => mounted))
        ) {
          if (mounted && manager.selected === id)
            preparationError =
              "Runtime configuration was not confirmed. Your draft is retained. Check the pending request outcome before continuing.";
          return;
        }
      }
      if (
        !mounted ||
        manager.selected !== id ||
        !manager.session?.connected ||
        manager.pending ||
        manager.error ||
        !runtimeMatches(manager.session, wanted) ||
        !["idle", "running"].includes(manager.session.state) ||
        object(manager.session.capabilities).modeReady !== true
      )
        return;
      await manager.commandSession(
        id,
        { action: "prompt", text: draft, max_duration_ms: 900000 },
        id,
      );
    } finally {
      preparing = false;
    }
  }
</script>

{#if session}
  <div class="agent-conversation">
    <CommunityAgentActivityFeed
      conversationKey={session.id}
      latestMessageId={rows.at(-1)?.id}
      working={session.state === "running"}
      empty={!rows.length && !manager.eventsLoading}
    >
      {#snippet messageComposer()}
        {#each changes as selection (`${selection.repository}:${selection.operation}:${selection.commit}`)}
          <StoreAgentChangesPanel
            {selection}
            repositoryName={selection.repository.slice(0, 8)}
            fetcher={manager.client.fetcher}
          />
        {/each}
        <form
          aria-label="Send agent prompt"
          onsubmit={(event) => {
            event.preventDefault();
            void send();
          }}
          onkeydowncapture={(event) => {
            if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
              event.preventDefault();
              event.stopPropagation();
              void send();
            }
          }}
        >
          <CommunityChatComposer
            mode="chat"
            clearOnSubmit={false}
            submitLabel={session.state === "running"
              ? "Queue follow-up"
              : "Send"}
            onSubmit={idle && (ready || canConfigure) && !manager.eventsLoading
              ? send
              : undefined}
            composerStatus={error
              ? { type: "error", message: error }
              : manager.activityError
                ? { type: "warning", message: manager.activityError }
                : !idle
                  ? {
                      type: "warning",
                      message:
                        session.state === "preparing" ||
                        session.state === "opening"
                          ? "Connecting the agent. Your draft is retained."
                          : object(facts.initialize).loadSession === true &&
                              session.state !== "cleaned"
                            ? "Resume this session from Session options to continue. Your draft is retained."
                            : "Start a new session with saved context from Session options to continue. Your draft is retained.",
                    }
                  : !ready && !canConfigure
                    ? {
                        type: "warning",
                        message:
                          "Choose an execution mode in Session options → Workspace and runtime → Edit execution mode before sending.",
                      }
                    : undefined}
            ariaLabel="Message to agent"
            placeholder={session.state === "running"
              ? "Queue a follow-up…"
              : "Message the agent…"}
            bind:value={manager.drafts[session.id]}
            disabled={manager.busy ||
              !!manager.pending ||
              preparing ||
              manager.eventsLoading}
          >
            {#snippet additionalActions()}
              <div class="agent-execution-actions">
                {#if session.state === "running" && active}<Button
                    type="button"
                    variant="outline"
                    size="sm"
                    disabled={manager.busy || !!manager.pending}
                    onclick={() =>
                      manager.command({
                        action: "cancel",
                        target: text(object(active.payload).requestId),
                      })}>Stop turn</Button
                  >{/if}
              </div>
            {/snippet}
          </CommunityChatComposer>
        </form>
        <StoreAgentRequestRecovery {manager} {session} />
      {/snippet}
      {#if manager.eventsLoading}<CommunitySkeleton
          layout="messages"
          count={5}
          padded
          label="Loading session history"
        />
      {:else if !rows.length}<Empty.Root
          ><Empty.Header
            ><Empty.Media variant="icon"
              ><MessageIcon aria-hidden="true" /></Empty.Media
            ><Empty.Title>Start the conversation</Empty.Title><Empty.Description
              >Send a message to {session.title} to begin.</Empty.Description
            ></Empty.Header
          ></Empty.Root
        >{/if}
      {#if !manager.eventsLoading}<StoreAgentTranscript {manager} />{/if}
    </CommunityAgentActivityFeed>
  </div>
{/if}

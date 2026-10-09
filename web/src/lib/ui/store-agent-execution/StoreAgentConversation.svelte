<script lang="ts">
  import type { Snippet } from "svelte";
  let inputActions = $state<Snippet>();
  import { createStandaloneSender } from "../../conversations/standalone-send.svelte.js";

  import StoreConversationRecovery from "../store-agent-chat/StoreConversationRecovery.svelte";
  import { untrack } from "svelte";

  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreConversationRuntime from "../store-agent-chat/StoreConversationRuntime.svelte";
  import StoreLiveConversation from "../store-agent-chat/StoreLiveConversation.svelte";
  import ChatMessageFooter from "../store-agent-chat-body/ChatMessageFooter.svelte";

  import StoreAgentRequestRecovery from "./StoreAgentRequestRecovery.svelte";

  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { activityRows, runtimeChoices } from "./presentation.js";
  import { runtimeMatches } from "./runtime-preparation.js";
  import { conversationError } from "./conversation-error.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  import { completedAgentChanges } from "../../controller/agent-workspace-changes.js";
  import StoreAgentChangesPanel from "../store-agent-chat/StoreAgentChangesPanel.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    manager,
    controller,
  }: { manager: AgentExecution; controller?: StoreController } = $props();
  const session = $derived(manager.session);
  const conversation = $derived(
    manager.conversations.get(`session:${session?.id}`),
  );
  const user = $derived({
    id: session ? manager.discussions.actor(session.repository) : "user",
    name: "You",
    handle: "",
    initials: "YO",
  });
  $effect(() => {
    const current = conversation,
      id = session?.id;
    untrack(() => {
      if (id && !current.initialized) {
        current.comments.message = manager.drafts[id] ?? "";
        current.initialized = true;
      }
    });
  });
  $effect(() => {
    const id = session?.id,
      message = conversation.comments.message;
    if (id) manager.drafts[id] = message;
  });

  $effect(() => {
    const current = session;
    if (current)
      return manager.discussions.watch(current.id, current.repository);
  });
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
  const sender = createStandaloneSender(() => ({
    manager,
    session,
    conversation,
    preferences,
    idle,
    ready,
    canConfigure,
  }));
  const send = sender.send;
  const error = $derived(
    session &&
      ((manager.errorSessionId === session.id && manager.error) ||
        sender.error ||
        conversationError(session, manager.events)),
  );
</script>

{#if session}
  <div class="agent-conversation">
    <StoreLiveConversation
      {controller}
      {manager}
      {conversation}
      loading={manager.eventsLoading}
      {user}
      onSend={send}
      rows={rows.map((row) => ({
        ...row,
        session: session!,
        key: `${session!.id}:${row.id}`,
      }))}
    >
      {#snippet composer(presentation)}
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
            void conversation.comments.send();
          }}
          onkeydowncapture={(event) => {
            if (
              !event.isComposing &&
              event.key === "Enter" &&
              (event.metaKey || event.ctrlKey)
            ) {
              event.preventDefault();
              event.stopPropagation();
              void conversation.comments.send();
            }
          }}
        >
          <CommunityChatComposer
            {presentation}
            dockActions
            bind:documentActions={inputActions}
            member={user}
            hideAvatar={conversation.comments.pending.length > 0}
            mode="chat"
            enterInsertsNewline
            showSubmitButton={false}
            allowEmpty={conversation.comments.pending.length > 0}
            clearOnSubmit={false}
            submitLabel={session.state === "running"
              ? "Queue follow-up"
              : "Send"}
            onSubmit={idle && (ready || canConfigure) && !manager.eventsLoading
              ? async () => {
                  if (!(await conversation.comments.send()))
                    throw new Error(conversation.comments.error);
                }
              : undefined}
            composerStatus={manager.pending
              ? undefined
              : error
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
            placeholder=""
            bind:value={conversation.comments.message}
            disabled={manager.busy ||
              !!manager.pending ||
              sender.preparing ||
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
        <StoreConversationRecovery {conversation} {manager} standalone />
      {/snippet}
      {#snippet footer()}<ChatMessageFooter
          actions={inputActions}
          canSend={idle &&
            (ready || canConfigure) &&
            !sender.preparing &&
            !manager.busy &&
            !manager.pending &&
            conversation.comments.canSend}
          onSend={() => void conversation.comments.send()}
          error={manager.pending || error ? "" : conversation.comments.error}
          >{#snippet controls()}<StoreConversationRuntime
              {manager}
              {session}
            />{/snippet}</ChatMessageFooter
        >{/snippet}
    </StoreLiveConversation>
  </div>
{/if}

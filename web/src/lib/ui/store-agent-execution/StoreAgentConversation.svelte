<script lang="ts">
  import StoreWorkspaceButton from "../store-agent-workspace/StoreWorkspaceButton.svelte";
  import type { Snippet } from "svelte";
  let inputActions = $state<Snippet>();
  import { createStandaloneSender } from "../../conversations/standalone-send.svelte.js";

  import StoreConversationRecovery from "../store-agent-chat/StoreConversationRecovery.svelte";
  import { untrack } from "svelte";

  import StoreConversationRuntime from "../store-agent-chat/StoreConversationRuntime.svelte";
  import StoreLiveConversation from "../store-agent-chat/StoreLiveConversation.svelte";
  import StoreAgentMessageFooter from "../store-agent-chat/StoreAgentMessageFooter.svelte";

  import StoreAgentRequestRecovery from "./StoreAgentRequestRecovery.svelte";

  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { activityRows, runtimeChoices } from "./presentation.js";
  import { runtimeMatches } from "./runtime-preparation.js";
  import { conversationError } from "./conversation-error.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { object, text } from "../../controller/agent-execution-types.js";
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
  const canPrepare = $derived(
    session?.state !== "cleaned" && !!session?.setup_id && !!preferences?.mode,
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
      sessions={[session]}
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
            onSubmit={(canPrepare || (idle && (ready || canConfigure))) &&
            !manager.eventsLoading
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
                  : !idle && !canPrepare
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
          ></CommunityChatComposer>
        </form>
        <StoreConversationRecovery {conversation} {manager} standalone />
      {/snippet}
      {#snippet activity()}<StoreAgentRequestRecovery
          {manager}
          {session}
        />{/snippet}
      {#snippet footer()}<StoreAgentMessageFooter
          {manager}
          sessions={[session]}
          actions={inputActions}
          comments={conversation.comments}
          canSend={(canPrepare || (idle && (ready || canConfigure))) &&
            !sender.preparing &&
            !manager.busy &&
            !manager.pending &&
            conversation.comments.canSend}
          onSend={() => void conversation.comments.send()}
          error={manager.pending || error ? "" : conversation.comments.error}
          >{#snippet workspace()}{#key conversation}<StoreWorkspaceButton
                {manager}
                {controller}
                {conversation}
                sessions={[session]}
              />{/key}{/snippet}
          {#snippet controls()}<StoreConversationRuntime
              {manager}
              {session}
            />{/snippet}</StoreAgentMessageFooter
        >{/snippet}
    </StoreLiveConversation>
  </div>
{/if}

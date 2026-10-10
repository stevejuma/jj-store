<script lang="ts">
  import type { ChatBodyComments } from "../store-agent-chat-body/comments.svelte.js";
  import { hasContent as draftHasContent } from "../store-agent-chat-body/comment-submission.js";
  import type { Snippet } from "svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import { sessionStopAction } from "../../controller/agent-session-activity.js";
  import ChatMessageFooter from "../store-agent-chat-body/ChatMessageFooter.svelte";
  let {
    manager,
    sessions,
    canSend,
    comments,
    hasContent = comments ? draftHasContent(comments) : canSend,
    onSend,
    error = "",
    controls,
    workspace,
    actions,
  }: {
    manager?: AgentExecution;
    sessions: readonly AgentSession[];
    canSend: boolean;
    comments?: ChatBodyComments;
    hasContent?: boolean;
    onSend: () => void;
    error?: string;
    controls?: Snippet;
    workspace?: Snippet;
    actions?: Snippet;
  } = $props();
  const stops = $derived(
    sessions.flatMap((session) => {
      const events =
        manager?.selected === session.id
          ? manager.events
          : (manager?.chat.events[session.id] ?? []);
      const action = sessionStopAction(session, events);
      return action ? [{ id: session.id, action }] : [];
    }),
  );
  const stopping = $derived(
    stops.some((s) => {
      const request = manager?.stops.requests[s.id];
      return (
        request &&
        (request.busy ||
          ["dispatching", "queued", "started"].includes(
            request.outcome?.state ?? "",
          ))
      );
    }),
  );
  function stop() {
    for (const target of stops)
      void manager?.stops.stop(target.id, target.action);
  }
</script>

<ChatMessageFooter
  {canSend}
  {hasContent}
  sendPending={!!comments?.sending || !!comments?.retainedSubmission}
  {onSend}
  {error}
  {controls}
  {workspace}
  {actions}
  onStop={manager && stops.length ? stop : undefined}
  {stopping}
/>

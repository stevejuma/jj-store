import type { StoreController } from "./contracts.js";
import type { ChatOrigin, ChatAttachment } from "./agent-chat.svelte.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
export function originAgentIds(
  manager: AgentExecution | undefined,
  scope: string,
  home: string | null,
) {
  const conversations = new Set(
    manager?.value?.conversations
      .filter((c) => c.origin.scope === scope && c.origin.home === home)
      .map((c) => c.id),
  );
  return [
    ...new Set(
      manager?.value?.sessions
        .filter(
          (s) => s.conversation_id && conversations.has(s.conversation_id),
        )
        .flatMap((s) => (s.setup_id ? [s.setup_id] : [])),
    ),
  ];
}
/** Visible selections are context; only registered code repositories become attachments. */
export function chatContext(controller: StoreController, parent?: string) {
  const r = controller.state.route;
  const project = r.page === "projects" && !!r.projectId;
  const home = project
    ? (r.workHome ?? r.repository)
    : r.threadId
      ? r.repository
      : null;
  const scope = r.threadId
    ? `thread:${r.repository}:${r.threadId}`
    : project
      ? `project:${home}:${r.projectId}:${r.taskId ?? ""}`
      : r.repository
        ? `repository:${r.repository}`
        : "personal";
  const reply = parent
    ? controller.threads?.projection.messages.find((m) => m.id === parent)
    : undefined;
  const origin: ChatOrigin = {
    scope,
    home,
    thread: r.threadId ?? null,
    agent_target: null,
    context: JSON.stringify({
      repository: r.repository || undefined,
      project: r.projectId,
      task: r.taskId,
      thread: r.threadId,
      path: r.path,
      revision: r.commitId,
      operation: r.snapshotOperationId,
      reply: parent
        ? {
            message: parent,
            target: controller.threads?.projection.targets[parent],
            content: reply?.content.slice(0, 8192),
          }
        : undefined,
    }),
  };
  const code = project
    ? r.workRepository
    : r.threadId
      ? undefined
      : r.repository;
  const attachments: ChatAttachment[] = code
    ? [
        {
          repository: code,
          target_id: null,
          operation: project ? null : (r.snapshotOperationId ?? null),
          commit: project
            ? null
            : (r.commitId ?? controller.state.snapshot?.id ?? null),
        },
      ]
    : [];
  return { origin, attachments };
}

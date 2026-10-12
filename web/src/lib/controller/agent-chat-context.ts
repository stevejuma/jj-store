import { pretty } from "../api/client.js";
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
function excerpt(value: string, limit: number) {
  const bytes = new TextEncoder().encode(value);
  if (bytes.length <= limit) return value;
  let end = limit;
  while (end > 0 && (bytes[end] & 0xc0) === 0x80) end--;
  return new TextDecoder().decode(bytes.slice(0, end));
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
  const work = controller.projects?.detail?.state ?? controller.work?.state;
  const projectContent =
    work?.project && work.project.id === r.projectId ? work.project : undefined;
  const taskContent =
    work?.task && work.task.id === r.taskId ? work.task : undefined;
  const origin: ChatOrigin = {
    scope,
    home,
    thread: r.threadId ?? null,
    agent_target: null,
    context: pretty({
      repository: r.repository || undefined,
      project: r.projectId,
      task: r.taskId,
      project_content: projectContent
        ? {
            title: excerpt(projectContent.title, 512),
            body: excerpt(projectContent.body, 4096),
            version: projectContent.version,
          }
        : undefined,
      task_content: taskContent
        ? {
            title: excerpt(taskContent.title, 512),
            body: excerpt(taskContent.body, 4096),
            version: taskContent.version,
          }
        : undefined,
      thread: r.threadId,
      path: r.path,
      revision: r.commitId ?? controller.state.snapshot?.id,
      operation: r.snapshotOperationId,
      file:
        r.page === "files" &&
        r.path &&
        controller.state.blob?.path === r.path &&
        controller.state.status.files === "ready" &&
        (!r.commitId || controller.state.snapshot?.id === r.commitId) &&
        typeof controller.state.blob.contents === "string"
          ? {
              content: excerpt(controller.state.blob.contents, 8192),
              truncated:
                new TextEncoder().encode(controller.state.blob.contents)
                  .length > 8192,
            }
          : undefined,
      reply: parent
        ? {
            message: parent,
            target: controller.threads?.projection.targets[parent],
            content: reply ? excerpt(reply.content, 4096) : undefined,
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
  const thread =
    r.threadId &&
    controller.threads?.detail?.thread.value.thread_id === r.threadId
      ? controller.threads.detail.thread
      : undefined;
  const codebase = thread?.value.codebase ?? (thread ? [r.repository] : []);
  return {
    origin,
    attachments: thread
      ? codebase.map((repository) => ({
          repository,
          target_id: null,
          operation: null,
          commit: null,
        }))
      : attachments,
    codebase: thread?.reference,
    repositories: thread ? codebase : undefined,
  };
}

/** Reply context is part of a conversation's identity, never inferred from text. */
export function originReplyParent(origin: ChatOrigin): string | null {
  if (!origin.thread) return null;
  try {
    const value = JSON.parse(origin.context);
    return typeof value?.reply?.message === "string"
      ? value.reply.message
      : null;
  } catch {
    return null;
  }
}
export function sameChatOrigin(a: ChatOrigin, b: ChatOrigin) {
  return (
    a.scope === b.scope &&
    a.home === b.home &&
    originReplyParent(a) === originReplyParent(b)
  );
}

export function chatAudienceKey(
  origin: ChatOrigin | undefined,
  conversation: string,
) {
  return origin
    ? JSON.stringify([
        "agent-chat",
        origin.home ?? "personal",
        origin.scope,
        conversation,
      ])
    : undefined;
}

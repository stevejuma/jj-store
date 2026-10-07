import { pretty } from "../api/client.js";
import type { WorkController } from "./work-types.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
import { mentionedAgentIds } from "../community/agent-mentions.js";
export function workChatOrigin(work: WorkController, reply?: string | null) {
  const view = work.state,
    home = view.snapshot?.repo_id ?? null;
  const scope = `project:${home}:${view.task?.project ?? view.project?.id ?? ""}:${view.task?.id ?? ""}`;
  const comment = view.comments.find((c) => c.id === reply);
  return {
    scope,
    home,
    thread: null,
    agent_target: null,
    context: pretty({
      project: view.project?.id,
      projectTitle: view.project?.title,
      task: view.task?.id,
      taskTitle: view.task?.title,
      body: view.task?.body.slice(0, 8192),
      reply: comment
        ? {
            id: comment.id,
            version: comment.version,
            body: comment.body?.slice(0, 8192),
          }
        : undefined,
    }),
  };
}
export async function dispatchWorkAgents(
  manager: AgentExecution | undefined,
  work: WorkController,
  content: string,
  reply?: string | null,
) {
  const ids = mentionedAgentIds(content);
  if (!ids.length) return false;
  if (!manager?.available())
    throw new Error("Agent execution is unavailable on this device.");
  const origin = workChatOrigin(work, reply);
  if (!origin.home)
    throw new Error("The project metadata home is unavailable.");
  const repository = work.state.task?.repository;
  manager.chat.show(
    origin,
    repository
      ? [{ repository, target_id: null, operation: null, commit: null }]
      : [],
  );
  await manager.chat.send(content, ids);
  return true;
}

import type { AgentChat } from "./agent-chat.svelte.js";
/** Selection edits never mutate the context retained for an uncertain Send. */
export function selectRepository(
  chat: AgentChat,
  previous: string,
  repository: string,
) {
  if (!chat.draft || chat.busy || chat.pending || previous === repository)
    return;
  chat.removeAttachment(previous);
  if (repository) chat.attach(repository);
}
export function selectWorkspace(
  chat: AgentChat,
  repository: string,
  targetId: string,
) {
  if (!chat.draft || chat.busy || chat.pending) return;
  const attachment = chat.draft.attachments.find(
    (a) => a.repository === repository,
  );
  const target = chat.manager.targets.find(
    (t) => t.id === targetId && t.repository === repository,
  );
  if (!attachment || (targetId && !target)) return;
  Object.assign(attachment, {
    target_id: target?.id ?? null,
    operation: target?.operation ?? null,
    commit: target?.commit ?? null,
  });
}
export function validateWorkspaces(chat: AgentChat) {
  if (
    chat.draft?.attachments.some(
      (a) =>
        !a.target_id &&
        chat.manager.targets.filter((t) => t.repository === a.repository)
          .length > 1,
    )
  )
    throw new Error(
      "Choose a workspace for each repository with multiple workspaces.",
    );
}

export function attach(chat: AgentChat, repository: string) {
  if (
    chat.draft &&
    !chat.busy &&
    !chat.pending &&
    !chat.draft.attachments.some((a) => a.repository === repository) &&
    chat.draft.attachments.length < 8
  )
    chat.draft.attachments.push({
      repository,
      target_id: null,
      operation: null,
      commit: null,
    });
}
export function removeAttachment(chat: AgentChat, repository: string) {
  if (chat.draft && !chat.busy && !chat.pending)
    chat.draft.attachments = chat.draft.attachments.filter(
      (a) => a.repository !== repository,
    );
}

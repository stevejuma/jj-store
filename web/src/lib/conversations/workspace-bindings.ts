import type { AgentSession } from "../controller/agent-execution-types.js";
export type WorkspaceBinding = {
  session: string;
  repository: string;
  repositoryName: string;
  workspace: string;
  location?: string;
  revision?: string;
  agent: string;
};
export function workspaceBindings(
  sessions: readonly AgentSession[],
  names: ReadonlyMap<string, string>,
): WorkspaceBinding[] {
  return sessions
    .filter((s) => s.state !== "cleaned")
    .flatMap((s) =>
      s.repository_bindings.length
        ? s.repository_bindings.map((b) => ({
            session: s.id,
            repository: b.repository,
            repositoryName: names.get(b.repository) ?? b.repository.slice(0, 8),
            workspace: b.workspace_name ?? "Workspace",
            location: b.workspace,
            revision: `${b.operation}:${b.commit}`,
            agent: s.title,
          }))
        : s.workspace_name
          ? [
              {
                session: s.id,
                repository: s.repository,
                repositoryName:
                  names.get(s.repository) ?? s.repository.slice(0, 8),
                workspace: s.workspace_name,
                location: s.workspace,
                revision: `${s.workspace_operation ?? s.operation}:${s.review_commit}`,
                agent: s.title,
              },
            ]
          : [],
    );
}

export const workspaceBindingKey = (binding: WorkspaceBinding) =>
  `${binding.session}:${binding.repository}`;
export function activeWorkspaceBinding(
  bindings: readonly WorkspaceBinding[],
  key: string,
) {
  return bindings.find((b) => workspaceBindingKey(b) === key) ?? bindings[0];
}

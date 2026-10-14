import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
import type {
  WorkflowRun,
  WorkflowVersion,
} from "../../controller/workflow-types.js";
export type WorkflowHistoryRow = {
  home: string;
  workflow: WorkflowVersion;
  run: WorkflowRun;
  position?: number | bigint;
};
export function workflowRepositoryGroups<
  T extends { home: string; workflow: WorkflowVersion },
>(entries: readonly T[], repositoryName: (id: string) => string) {
  const groups = new Map<string, { id: string; name: string; entries: T[] }>();
  for (const entry of entries) {
    const id = entry.workflow.definition.repository;
    if (!groups.has(id))
      groups.set(id, { id, name: repositoryName(id), entries: [] });
    groups.get(id)!.entries.push(entry);
  }
  return [...groups.values()]
    .map((group) => ({
      ...group,
      entries: group.entries.sort(
        (a, b) =>
          a.workflow.definition.name.localeCompare(
            b.workflow.definition.name,
          ) ||
          a.home.localeCompare(b.home) ||
          a.workflow.id.localeCompare(b.workflow.id),
      ),
    }))
    .sort((a, b) => a.name.localeCompare(b.name) || a.id.localeCompare(b.id));
}
export function matchesWorkflowRun(
  row: WorkflowHistoryRow,
  selection: WorkflowSelection,
) {
  const { run, workflow, home } = row,
    current = run.attempts.at(-1);
  return (
    (!selection.scope || home === selection.scope) &&
    (!selection.workflow || workflow.id === selection.workflow) &&
    (!selection.source || run.source.repository === selection.source) &&
    (!selection.state || current?.status === selection.state) &&
    (!selection.author || run.requester === selection.author) &&
    (!selection.trigger ||
      (run.event === null ? "manual" : "event") === selection.trigger) &&
    (!selection.bookmark || run.source.bookmark === selection.bookmark) &&
    `${workflow.definition.name} ${run.id} ${run.requester} ${run.source.bookmark ?? ""} ${current?.host ?? ""} ${current?.status ?? ""}`
      .toLocaleLowerCase()
      .includes(selection.q?.trim().toLocaleLowerCase() ?? "")
  );
}
export function runDuration(run: WorkflowRun, now: number) {
  const attempt = run.attempts.at(-1);
  if (!attempt || attempt.started_at === null) return "—";
  const end =
    attempt.completed_at ??
    (["running", "cancel_requested"].includes(attempt.status) ? now : null);
  if (end === null || Number(end) < Number(attempt.started_at)) return "—";
  const seconds = Math.floor((Number(end) - Number(attempt.started_at)) / 1000);
  return seconds >= 3600
    ? `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`
    : seconds >= 60
      ? `${Math.floor(seconds / 60)}m ${seconds % 60}s`
      : `${seconds}s`;
}
export function workflowRunSelection(
  row: WorkflowHistoryRow,
): WorkflowSelection {
  const status = row.run.attempts.at(-1)?.status;
  return {
    home: row.home,
    run: row.run.id,
    at:
      status &&
      ["completed", "failed", "cancelled", "skipped"].includes(status) &&
      row.position !== undefined
        ? String(row.position)
        : undefined,
    attempt: undefined,
  };
}

import type {
  WorkflowDefinition,
  WorkflowEvidence,
  WorkflowRun,
} from "../../controller/workflow-types.js";

export type ConsoleStep = {
  id: string;
  status: string;
  started?: number;
  startSequence?: number | bigint;
  finished?: number;
  output: Record<string, string>;
  chunks: WorkflowEvidence[];
};
export type ConsoleCacheEntry = {
  chunks: WorkflowEvidence[];
  step?: ConsoleStep;
};
const terminal = new Set([
  "completed",
  "failed",
  "cancelled",
  "unknown",
  "skipped",
]);
const ordered = (a: WorkflowEvidence, b: WorkflowEvidence) =>
  a.sequence < b.sequence ? -1 : a.sequence > b.sequence ? 1 : 0;

/** A presentation of immutable evidence, never a replacement execution state. */
export function workflowConsole(
  run: WorkflowRun,
  definition: WorkflowDefinition | undefined,
  evidence: readonly WorkflowEvidence[],
  selected?: string,
) {
  const attempt = selected
    ? run.attempts.find((row) => row.id === selected)
    : run.attempts.at(-1);
  const rows = evidence
    .filter((row) => row.attempt === attempt?.id)
    .sort(ordered);
  const steps = new Map<string, ConsoleStep>();
  const unscoped: WorkflowEvidence[] = [],
    outputs: WorkflowEvidence[] = [];
  const active = new Set<string>();
  function step(id: string) {
    let value = steps.get(id);
    if (!value) {
      value = { id, status: "pending", chunks: [], output: {} };
      steps.set(id, value);
    }
    return value;
  }
  if (attempt) for (const action of definition?.actions ?? []) step(action.id);
  for (const row of rows) {
    const data = row.data;
    if (data.type === "step") {
      const value = step(data.step);
      value.status = data.status;
      Object.assign(value.output, data.output);
      if (data.status === "running") {
        value.started ??= Number(row.at);
        value.startSequence ??= row.sequence;
        active.add(data.step);
      } else if (terminal.has(data.status)) {
        value.finished = Number(row.at);
        active.delete(data.step);
      }
    } else if (data.type === "log") {
      if (data.step) step(data.step).chunks.push(row);
      else if (active.size === 1) step([...active][0]!).chunks.push(row);
      else unscoped.push(row);
    } else outputs.push(row);
  }
  for (const value of steps.values()) {
    if (
      value.status === "pending" &&
      ["failed", "cancelled"].includes(attempt?.status ?? "")
    )
      value.status = "not_run";
    if (value.status === "running" && attempt?.status === "unknown")
      value.status = "unknown";
  }
  // YAML reports include executor/job envelopes. Show their actual steps once
  // discovered, retaining setup/cleanup output in the run-output disclosure.
  const envelopes = new Set<string>();
  for (const value of steps.values())
    if (value.output.kind === "step" && value.output.parent)
      envelopes.add(value.output.parent);
  for (const value of steps.values())
    if (envelopes.has(value.id) && value.output.parent)
      envelopes.add(value.output.parent);
  const visible = [...steps.values()].filter((value) => {
    if (!envelopes.has(value.id)) return true;
    unscoped.push(...value.chunks);
    return false;
  });
  unscoped.sort(ordered);
  return { attempt, steps: visible, unscoped, outputs };
}

export function stepDuration(step: ConsoleStep, observed: number) {
  const end = step.finished ?? observed;
  if (
    step.started === undefined ||
    !Number.isSafeInteger(end) ||
    !Number.isSafeInteger(step.started) ||
    end < step.started
  )
    return "—";
  const seconds = Math.floor((end - step.started) / 1000);
  return seconds < 60
    ? `${seconds}s`
    : `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
}

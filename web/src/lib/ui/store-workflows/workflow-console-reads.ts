import type {
  WorkflowPage,
  WorkflowTransport,
  WorkflowEvidence,
} from "../../controller/workflow-types.js";

export async function readConsoleEvidence(
  transport: WorkflowTransport,
  snapshot: WorkflowPage,
  attempt: string,
  signal?: AbortSignal,
  from?: number | bigint,
  step?: string,
) {
  const run = snapshot.runs[0]?.id;
  if (!run) return [];
  const evidence: WorkflowEvidence[] = [];
  const scoped = [...snapshot.evidence, ...(snapshot.step_reports ?? [])].some(
    (row) =>
      row.data.type === "log"
        ? row.data.step === step
        : row.data.type === "step" &&
          row.data.step === step &&
          row.data.output.kind === "step",
  );
  const cursors = new Set<string>();
  let after: string | undefined;
  do {
    const page = await transport.page(
      {
        run,
        attempt,
        at: snapshot.position,
        limit: 256,
        evidence_after: from,
        after,
      },
      signal,
    );
    if (
      page.repository !== snapshot.repository ||
      page.epoch !== snapshot.epoch ||
      page.position !== snapshot.position ||
      page.root !== snapshot.root ||
      page.runs.some((row) => row.id !== run) ||
      page.evidence.some((row) => row.run !== run || row.attempt !== attempt)
    )
      throw Error("Log snapshot changed. Retry the selected step.");
    evidence.push(...page.evidence);
    if (
      step &&
      !scoped &&
      page.evidence.some(
        (row) =>
          row.data.type === "step" &&
          row.data.step === step &&
          ["completed", "failed", "cancelled", "unknown", "skipped"].includes(
            row.data.status,
          ),
      )
    )
      break;
    after = page.next ?? undefined;
    if (after && cursors.has(after))
      throw Error("Log continuation did not advance.");
    if (after) cursors.add(after);
  } while (after);
  return [...new Map(evidence.map((row) => [row.id, row])).values()];
}

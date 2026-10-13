import type {
  WorkflowTransport,
  WorkflowEvidence,
} from "../../web/src/lib/controller/workflow-types.js";
export type ConsoleExample =
  "success" | "failed" | "cancelled" | "running" | "paged";
const runId = "51".repeat(16),
  old = "52".repeat(16),
  latest = "89".repeat(16),
  time = 1791880000000;
export function consoleExample(
  base: WorkflowTransport,
  mode: ConsoleExample,
): WorkflowTransport {
  return {
    ...base,
    async page(query, signal) {
      const page = await base.page(query, signal);
      if (!query.run || query.run !== runId) return page;
      const run = page.runs[0]!,
        definition = page.workflows[0]!.definition;
      definition.actions = ["prepare", "test", "summary"].map((id) => ({
        ...definition.actions[0]!,
        id,
      }));
      const status =
        mode === "paged"
          ? "completed"
          : mode === "success"
            ? "completed"
            : mode;
      run.attempts = [
        {
          ...run.attempts[0]!,
          status: "failed",
          result: "Previous attempt failed",
        },
        {
          ...run.attempts[0]!,
          id: latest,
          status,
          updated_at: time + 8000,
          result: status === "completed" ? "All actions finished" : "",
          completed_at: status === "running" ? null : time + 8000,
        },
      ];
      const rows: WorkflowEvidence[] = [];
      const add = (data: WorkflowEvidence["data"], attempt = latest) => {
        const sequence = rows.filter((row) => row.attempt === attempt).length;
        rows.push({
          id: `${attempt.slice(0, 2)}${String(sequence).padStart(30, "0")}`,
          run: runId,
          attempt,
          sequence,
          at: time + sequence * 500,
          data,
        });
      };
      add(
        { type: "step", step: "prepare", status: "running", output: {} },
        old,
      );
      add({ type: "log", stream: "stdout", text: "Old attempt only\n" }, old);
      add({ type: "step", step: "prepare", status: "failed", output: {} }, old);
      add({ type: "step", step: "prepare", status: "running", output: {} });
      add({
        type: "log",
        stream: "stdout",
        text: "\u001b[32mPinned source ready\u001b[0m\n",
      });
      add({ type: "step", step: "prepare", status: "completed", output: {} });
      add({ type: "step", step: "test", status: "running", output: {} });
      add({ type: "log", stream: "stdout", text: "##[group]Test group\n" });
      add({
        type: "log",
        stream: "stdout",
        text: "Nested output " + "x".repeat(160) + "\n",
      });
      add({ type: "log", stream: "stdout", text: "##[endgroup]\n" });
      add({ type: "log", stream: "stderr", text: "⚙ [runner]: diagnostic\n" });
      if (status !== "running")
        add({
          type: "step",
          step: "test",
          status:
            status === "failed"
              ? "failed"
              : status === "cancelled"
                ? "cancelled"
                : "completed",
          output: {},
        });
      if (status === "completed") {
        add({ type: "step", step: "summary", status: "running", output: {} });
        add({ type: "log", stream: "stdout", text: "Done\n" });
        add({
          type: "artifact",
          name: "demo-result.txt",
          media_type: "text/plain",
          bytes: [111, 107],
        });
        add({ type: "step", step: "summary", status: "completed", output: {} });
      }
      run.attempts[1]!.sequence = rows.filter(
        (row) => row.attempt === latest,
      ).length;
      let selected = rows.filter(
        (row) =>
          (!query.attempt || row.attempt === query.attempt) &&
          (query.evidence_after === undefined ||
            row.sequence >= query.evidence_after),
      );
      if (query.after)
        selected = selected.slice(
          selected.findIndex((row) => row.id === query.after) + 1,
        );
      const limit = mode === "paged" ? 3 : 256;
      page.next = selected.length > limit ? selected[limit - 1]!.id : null;
      page.evidence = selected.slice(0, limit);
      return page;
    },
  };
}

import { expect, test } from "vitest";
import { workflowFixture } from "../../../catalog/fixtures/workflows.js";
import {
  workflowConsole,
  stepDuration,
} from "../../src/lib/ui/store-workflows/workflow-console.js";
import {
  WorkflowLogLines,
  consoleLogRows,
} from "../../src/lib/ui/store-workflows/workflow-log-lines.js";
import { readConsoleEvidence } from "../../src/lib/ui/store-workflows/workflow-console-reads.js";
import { consoleExample } from "../../../catalog/fixtures/workflow-console.js";
import type { WorkflowEvidence } from "../../src/lib/controller/workflow-types.js";
const runId = "51".repeat(16),
  attemptId = "52".repeat(16);
function evidence(
  data: WorkflowEvidence["data"],
  sequence: number,
  at = 1000,
  attempt = attemptId,
): WorkflowEvidence {
  return {
    id: String(sequence).padStart(32, "0"),
    run: runId,
    attempt,
    sequence,
    at,
    data,
  };
}
const step = (
  id: string,
  status: "running" | "completed" | "failed",
  sequence: number,
  at = 1000,
) => evidence({ type: "step", step: id, status, output: {} }, sequence, at);
const log = (
  text: string,
  sequence: number,
  stream = "stdout",
  attempt = attemptId,
) => evidence({ type: "log", stream, text }, sequence, 1000, attempt);
async function fixture() {
  const page = await workflowFixture().page({ run: runId });
  return { run: page.runs[0]!, definition: page.workflows[0]!.definition };
}

test("three actions collapse reports and group logs in recorded definition order", async () => {
  const { run, definition } = await fixture();
  definition.actions = ["prepare", "test", "summary"].map((id) => ({
    ...definition.actions[0]!,
    id,
  }));
  const rows = [
    step("prepare", "running", 0),
    log("source", 1),
    step("prepare", "running", 2, 2000),
    step("prepare", "completed", 3, 4000),
    step("test", "running", 4),
    log("diagnostic", 5, "stderr"),
    step("test", "failed", 6, 5000),
  ];
  run.attempts[0]!.status = "failed";
  const result = workflowConsole(run, definition, [...rows].reverse());
  expect(result.steps.map((s) => [s.id, s.status])).toEqual([
    ["prepare", "completed"],
    ["test", "failed"],
    ["summary", "not_run"],
  ]);
  expect(result.steps[0]!.started).toBe(1000);
  expect(result.steps[0]!.chunks).toEqual([rows[1]]);
  expect(result.steps[1]!.chunks).toEqual([rows[5]]);
  expect(result.unscoped).toEqual([]);
});

test("latest and historical attempts never mix evidence or replace missing selections", async () => {
  const { run, definition } = await fixture();
  const retry = "88".repeat(16);
  run.attempts.push({ ...run.attempts[0]!, id: retry, status: "pending" });
  const rows = [
    step("build", "completed", 0),
    log("legacy", 1),
    log("new", 0, "stdout", retry),
  ];
  const latest = workflowConsole(run, definition, rows);
  expect(latest.attempt?.id).toBe(retry);
  expect(latest.unscoped).toEqual([rows[2]]);
  const historic = workflowConsole(run, definition, rows, attemptId);
  expect(historic.unscoped).toEqual([rows[1]]);
  expect(historic.steps[0]!.chunks).toEqual([]);
  expect(
    workflowConsole(run, definition, rows, "missing").attempt,
  ).toBeUndefined();
  expect(workflowConsole(run, definition, rows, "missing").steps).toEqual([]);
});

test("ambiguous logs and terminal-only legacy reports remain unscoped", async () => {
  const { run, definition } = await fixture();
  const rows = [
    log("before", 0),
    step("build", "running", 1),
    step("other", "running", 2),
    log("ambiguous", 3),
    step("other", "completed", 4),
    log("build", 5),
    step("build", "completed", 6),
    log("after", 7),
  ];
  const result = workflowConsole(run, definition, rows);
  expect(result.unscoped).toEqual([rows[0], rows[3], rows[7]]);
  expect(result.steps[0]!.chunks).toEqual([rows[5]]);
});

test("unknown and cancellation preserve uncertainty and reported diagnostics", async () => {
  const { run, definition } = await fixture();
  run.attempts[0]!.status = "unknown";
  const result = workflowConsole(run, definition, [
    step("build", "running", 0),
  ]);
  expect(result.steps[0]!.status).toBe("unknown");
  expect(result.steps[0]!.finished).toBeUndefined();
  run.attempts[0]!.status = "cancelled";
  expect(workflowConsole(run, definition, []).steps[0]!.status).toBe("not_run");
  const rows = [
    evidence(
      {
        type: "step",
        step: "build",
        status: "failed",
        output: { source_export: "refused" },
      },
      0,
    ),
    step("build", "failed", 1),
  ];
  expect(
    workflowConsole(run, definition, rows).steps[0]!.output.source_export,
  ).toBe("refused");
});

test("durations use reported starts and freeze at the supplied observation", () => {
  const value = {
    id: "build",
    status: "running",
    chunks: [],
    output: {},
    started: 1000,
  };
  expect(stepDuration(value, 4500)).toBe("3s");
  expect(stepDuration({ ...value, finished: 62500 }, 100000)).toBe("1m 1s");
  expect(stepDuration({ ...value, started: undefined }, 4500)).toBe("—");
  expect(stepDuration(value, 500)).toBe("—");
});

test("partial lines and split CRLF retain stream ordering without extra blanks", () => {
  const buffer = new WorkflowLogLines();
  const rows = [
    log("hel", 0),
    log("warning\r", 1, "stderr"),
    log("lo\r", 2),
    log("\n", 3, "stderr"),
    log("\n", 4),
  ];
  expect(buffer.read(rows).map((l) => [l.stream, l.html])).toEqual([
    ["stdout", "hello"],
    ["stderr", "warning"],
  ]);
  expect(buffer.read(rows)).toHaveLength(2);
  expect(buffer.read([...rows, log("\nend\n", 5)]).map((l) => l.html)).toEqual([
    "hello",
    "warning",
    "",
    "end",
  ]);
});

test("ANSI split across chunks is colored and escapes HTML without executable links", () => {
  const buffer = new WorkflowLogLines();
  const lines = buffer.read([log("\x1b[3", 0), log("1m<script>\x1b[0m\n", 1)]);
  expect(lines[0]!.html).toContain("ansi-red-fg");
  expect(lines[0]!.html).toContain("&lt;script&gt;");
  expect(lines[0]!.html).not.toContain("<script>");
  expect(
    new WorkflowLogLines().read([
      log("\x1b]8;;javascript:alert(1)\x07click\x1b]8;;\x07", 0),
    ])[0]!.html,
  ).not.toContain("<a");
});

test("refresh and paged continuation append once while a changed snapshot resets buffers", () => {
  const buffer = new WorkflowLogLines(),
    first = [log("first\n", 0)];
  const old = buffer.read(first);
  expect(buffer.read([...first, log("second\n", 1)])[0]!.id).toBe(old[0]!.id);
  expect(buffer.read([...first, log("second\n", 1)])).toHaveLength(2);
  expect(buffer.read([log("historical\n", 8)]).map((l) => l.html)).toEqual([
    "historical",
  ]);
});

test("nested log groups retain icons and omit control marker rows", () => {
  const lines = new WorkflowLogLines().read([
    log(
      "##[group]Outer\n##[group]Inner\n⚙ log\n##[endgroup]\n##[endgroup]\nend\n",
      0,
    ),
  ]);
  const rows = consoleLogRows(lines);
  expect(rows).toHaveLength(4);
  expect(rows[0]!.html).toBe("Outer");
  expect(rows[2]!.parents).toEqual([rows[0]!.id, rows[1]!.id]);
  expect(rows[3]!.parents).toEqual([]);
});

test("lazy step reads and downloads use one exact paged attempt snapshot", async () => {
  const transport = consoleExample(workflowFixture(), "paged");
  const snapshot = await transport.page({ run: runId });
  const attempt = "89".repeat(16);
  const rows = await readConsoleEvidence(
    transport,
    snapshot,
    attempt,
    undefined,
    0,
    "prepare",
  );
  expect(rows.map((row) => row.sequence)).toEqual([0, 1, 2]);
  expect(rows.every((row) => row.attempt === attempt)).toBe(true);
  const full = await readConsoleEvidence(transport, snapshot, attempt);
  expect(full).toHaveLength(13);
  expect(full.at(-1)?.data.type).toBe("step");
});

test("lazy reads reject a changed snapshot and a repeated continuation", async () => {
  const transport = consoleExample(workflowFixture(), "paged"),
    snapshot = await transport.page({ run: runId });
  await expect(
    readConsoleEvidence(
      { ...transport, page: async () => ({ ...snapshot, root: "changed" }) },
      snapshot,
      attemptId,
    ),
  ).rejects.toThrow("Log snapshot changed");
  const loop = {
    ...transport,
    page: async () => ({ ...snapshot, evidence: [], next: "same" }),
  };
  await expect(readConsoleEvidence(loop, snapshot, attemptId)).rejects.toThrow(
    "Log continuation did not advance",
  );
});

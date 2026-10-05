import test from "node:test";
import assert from "node:assert/strict";
import { validateLedger } from "./check-progress.mjs";

const entry = {
  id: "E-M17-001",
  tasks: ["M17", "M17.1"],
  date: "2026-10-03",
  kind: "governance",
  files: ["proof.rs"],
  tests: ["proof.rs#proof"],
  runs: [{ command: "proof", result: "pass", failures: 0 }],
  result: "pass",
  remaining: [],
  versions: "test",
  formats: "unchanged",
};
const rows = `| M17 | Planned | - | JST-GOV-004 | desktop | next | - |
| M18 | In progress | - | JST-GOV-004 | capture | next | E-M17-001 |
| M18.1 | Done | - | JST-GOV-004 | capture foundation | next | E-M17-001 |`;
const mappings = `| E-M17-* | M17 | M18 |
| E-M17-* | M17.1 | M18.1 |`;
function fixture(mapping = mappings, entries = [entry], tasks = rows) {
  return {
    source: `${tasks}\n\n${mapping}\n\n\`\`\`json\n${JSON.stringify(entries)}\n\`\`\``,
    read: () => "#[test] fn proof() {}",
    requirements: new Set(["JST-GOV-004"]),
  };
}
test("historical_capture_tasks_map_without_rewriting_evidence", () => {
  const before = JSON.stringify(entry);
  assert.deepEqual(validateLedger(fixture()), []);
  assert.equal(JSON.stringify(entry), before);
});
test("historical_mapping_does_not_reclassify_desktop_evidence", () => {
  const desktop = { ...entry, id: "E-DESK-001", tasks: ["M17"] };
  assert.deepEqual(
    validateLedger(
      fixture(
        mappings,
        [entry, desktop],
        rows.replace("desktop | next | -", "desktop | next | E-DESK-001"),
      ),
    ),
    [],
  );
});
test("historical_capture_evidence_cannot_qualify_desktop_tasks", () => {
  assert.match(
    validateLedger(
      fixture(
        mappings,
        [entry],
        rows.replace("desktop | next | -", "desktop | next | E-M17-001"),
      ),
    ).join(),
    /M17: missing evidence/,
  );
});
test("historical_mapping_rejects_missing_destination_and_ambiguity", () => {
  assert.match(
    validateLedger(fixture(mappings + "\n| E-M17-* | M17 | M19 | ")).join(),
    /mapping/,
  );
});

test("historical_stacks_mapping_does_not_qualify_freshness", () => {
  const stacks = { ...entry, id: "E-M20-001", tasks: ["M20", "M20.1"] };
  const freshness = { ...entry, id: "E-FRESH-001", tasks: ["M20"] };
  const tasks = `| M20 | In progress | - | JST-GOV-004 | freshness | next | E-FRESH-001 |
| M21 | In progress | - | JST-GOV-004 | stacks | next | E-M20-001 |
| M21.1 | Done | - | JST-GOV-004 | UI | next | E-M20-001 |`;
  const mapping = `| E-M20-* | M20 | M21 |
| E-M20-* | M20.1 | M21.1 |`;
  assert.deepEqual(
    validateLedger(fixture(mapping, [stacks, freshness], tasks)),
    [],
  );
  assert.match(
    validateLedger(
      fixture(
        mapping,
        [stacks, freshness],
        tasks.replace(
          "freshness | next | E-FRESH-001",
          "freshness | next | E-M20-001",
        ),
      ),
    ).join(),
    /M20: missing evidence/,
  );
});

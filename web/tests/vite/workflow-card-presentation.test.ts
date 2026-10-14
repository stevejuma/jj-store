import { expect, test } from "vitest";
import { initialDefinition } from "../../src/lib/ui/store-workflows/workflow-presentation.js";
import { workflowCardSummary } from "../../src/lib/ui/store-workflows/workflow-card-presentation.js";

test("cards summarize the recorded first action and remaining ordered steps", () => {
  const definition = initialDefinition("repo");
  expect(workflowCardSummary(definition)).toBe(
    "When run manually, run “build”",
  );
  definition.actions.push({ ...definition.actions[0]!, id: "test" });
  expect(workflowCardSummary(definition)).toBe(
    "When run manually, run “build”, then 1 more step",
  );
  definition.actions.push({ ...definition.actions[0]!, id: "report" });
  expect(workflowCardSummary(definition)).toBe(
    "When run manually, run “build”, then 2 more steps",
  );
  definition.actions = [];
  expect(workflowCardSummary(definition)).toBe("When run manually");
});
test("card wording includes native trigger limits and agent executor names", () => {
  const definition = initialDefinition("repo");
  definition.trigger = { on: "message_posted" };
  definition.conditions = { text: { operator: "starts_with", value: "hello" } };
  definition.actions[0]!.executor = {
    type: "agent",
    agent: "reviewer",
    prompt: "private prompt",
  };
  expect(workflowCardSummary(definition)).toBe(
    "When a thread message starts with “hello”, ask agent “reviewer”",
  );
  definition.conditions.text = { operator: "is_empty", value: "unused" };
  expect(workflowCardSummary(definition)).toBe(
    "When a thread message is empty, ask agent “reviewer”",
  );
  definition.trigger = { on: "bookmark_changed", bookmark: "release/*" };
  definition.conditions = { revset: 'bookmarks("release/*")' };
  expect(workflowCardSummary(definition)).toContain(
    'When bookmark “release/*” changes matching “bookmarks("release/*")”',
  );
  definition.trigger = { on: "reaction_added" };
  definition.conditions = { emoji: "👍" };
  expect(workflowCardSummary(definition)).toContain(
    "When reaction “👍” is added",
  );
});
test("long user content is normalized for display without mutating its definition", () => {
  const definition = initialDefinition("repo");
  definition.actions[0]!.executor = { type: "command", name: "a".repeat(100) };
  const original = structuredClone(definition);
  expect(workflowCardSummary(definition)).toBe(
    `When run manually, run “${"a".repeat(33)}…”`,
  );
  expect(definition).toEqual(original);
});

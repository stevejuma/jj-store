import { expect, test } from "vitest";
import { createDisclosureLayout } from "../../src/lib/ui/disclosure-layout.svelte.js";
import {
  disclosureKey,
  disclosureStorageKey,
} from "../../src/lib/ui/disclosure-storage.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import { createMemoryStoryStorage } from "../../../catalog/fixtures/memory-storage.js";

const route = (repository = "repo-a") => ({
  ...initialStoreRoute(),
  page: "agents" as const,
  repository,
});
test("saved closed and open sections restore and isolate owners and detail identities", () => {
  const storage = createMemoryStoryStorage();
  let current = route();
  let layout = createDisclosureLayout(storage, () => current);
  layout.write(["collection", "agents"], false);
  layout.write(["task", "task-a", "subtasks"], true);
  layout = createDisclosureLayout(storage, () => current);
  expect(layout.read(["collection", "agents"], true)).toBe(false);
  expect(layout.read(["task", "task-a", "subtasks"], false)).toBe(true);
  expect(layout.read(["task", "task-b", "subtasks"], false)).toBe(false);
  current = route("repo-b");
  expect(layout.read(["collection", "agents"], true)).toBe(true);
  layout.write(["collection", "agents"], true);
  current = route();
  expect(layout.read(["collection", "agents"], true)).toBe(false);
});
test("Projects scope and tabs do not collide with Repository or other project groups", () => {
  const base = {
    ...route(),
    page: "projects" as const,
    workTab: "agents" as const,
    workHome: "home",
    projectId: "project-a",
  };
  expect(disclosureKey(base, ["collection", "agents:repo-a"])).not.toBe(
    disclosureKey(route(), ["collection", "agents"]),
  );
  expect(disclosureKey(base, ["collection", "tasks:mine"])).not.toBe(
    disclosureKey(base, ["collection", "projects:mine"]),
  );
  expect(disclosureKey(base, ["collection", "agents:repo-a"])).not.toBe(
    disclosureKey({ ...base, projectId: "project-b" }, [
      "collection",
      "agents:repo-a",
    ]),
  );
  expect(disclosureKey(base, ["collection", "agents:repo-a"])).toBe(
    disclosureKey(
      { ...base, workQuery: "changed filter", atOp: "new operation" },
      ["collection", "agents:repo-a"],
    ),
  );
});
test.each([
  "invalid",
  '{"version":2,"flags":{}}',
  '{"version":1,"flags":[]}',
  JSON.stringify({ version: 1, flags: { ["a".repeat(64)]: "false" } }),
])("malformed layout falls back without blocking toggles: %s", (value) => {
  const storage = createMemoryStoryStorage();
  storage.setItem(disclosureStorageKey, value);
  const layout = createDisclosureLayout(storage, () => route());
  expect(layout.read(["collection", "agents"], true)).toBe(true);
  layout.write(["collection", "agents"], false);
  expect(layout.read(["collection", "agents"], true)).toBe(false);
});
test("disabled or full storage still preserves choices for this mounted app", () => {
  const storage = createMemoryStoryStorage();
  storage.getItem = () => {
    throw Error("blocked");
  };
  storage.setItem = () => {
    throw Error("quota");
  };
  const layout = createDisclosureLayout(storage, () => route());
  layout.write(["collection", "agents"], false);
  layout.write(["review", "review-a", "files"], true);
  expect(layout.read(["collection", "agents"], true)).toBe(false);
  expect(layout.read(["review", "review-a", "files"], false)).toBe(true);
});
test("storage retains only bounded opaque keys and boolean flags", () => {
  const storage = createMemoryStoryStorage();
  const layout = createDisclosureLayout(storage, () =>
    route("secret-repo-name"),
  );
  for (let i = 0; i < 1025; i++)
    layout.write(["task", `task-${i}`, "activity"], false);
  const serialized = storage.getItem(disclosureStorageKey)!;
  expect(serialized).not.toContain("secret-repo-name");
  expect(serialized).not.toContain("activity");
  expect(Object.keys(JSON.parse(serialized).flags)).toHaveLength(1024);
  expect(
    createDisclosureLayout(storage, () => route("secret-repo-name")).read(
      ["task", "task-1024", "activity"],
      true,
    ),
  ).toBe(false);
});

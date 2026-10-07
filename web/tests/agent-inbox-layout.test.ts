import { test } from "node:test";
import assert from "node:assert/strict";
import {
  agentInboxLayoutKey,
  loadInboxWidth,
  saveInboxWidth,
} from "../src/lib/ui/community-inbox/inbox-layout.js";

test("inbox width survives reopening with an independent version and panel identity", () => {
  const entries = new Map<string, string>([
    ["jj-store/layout/v1/shell", "outer shell"],
  ]);
  const storage = {
    getItem: (key: string) => entries.get(key) ?? null,
    setItem: (key: string, value: string) => entries.set(key, value),
  } as unknown as Storage;
  saveInboxWidth(storage, 288);
  assert.equal(loadInboxWidth(storage), 288);
  assert.equal(entries.get("jj-store/layout/v1/shell"), "outer shell");
  saveInboxWidth(storage, undefined);
  assert.equal(loadInboxWidth(storage), undefined);
  for (const invalid of [
    "invalid JSON",
    '{"version":2,"panel":"agent-sessions","width":288}',
    '{"version":1,"panel":"threads","width":288}',
  ]) {
    entries.set(agentInboxLayoutKey, invalid);
    assert.equal(loadInboxWidth(storage), undefined);
  }
  entries.set(
    agentInboxLayoutKey,
    '{"version":1,"panel":"agent-sessions","width":900}',
  );
  assert.equal(loadInboxWidth(storage), 480);
});

test("blocked device storage keeps inbox layout usable", () => {
  const storage = {
    getItem() {
      throw new Error("blocked");
    },
    setItem() {
      throw new Error("quota");
    },
  } as unknown as Storage;
  assert.equal(loadInboxWidth(storage), undefined);
  assert.doesNotThrow(() => saveInboxWidth(storage, 300));
  assert.equal(loadInboxWidth(), undefined);
});

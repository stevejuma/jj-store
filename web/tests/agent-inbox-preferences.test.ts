import { test } from "node:test";
import assert from "node:assert/strict";
import {
  parseInboxPreferences,
  readInboxPreferences,
  saveInboxPreferences,
  toggleInboxPreference,
  inboxPreferencesKey,
} from "../src/lib/ui/community-inbox/inbox-preferences.js";
test("device pins and archives persist only canonical IDs in independent versioned namespaces", () => {
  const values = new Map<string, string>();
  const storage = {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => values.set(key, value),
  } as Storage;
  const original = { pinned: [], archived: [] };
  const pin = toggleInboxPreference(original, "pinned", "home:thread");
  const archived = toggleInboxPreference(pin, "archived", "home:thread");
  saveInboxPreferences(storage, "thread", archived);
  assert.deepEqual(readInboxPreferences(storage, "thread"), archived);
  assert.deepEqual(readInboxPreferences(storage, "session"), original);
  assert.equal(
    values.get(inboxPreferencesKey("thread")),
    '{"version":1,"pinned":["home:thread"],"archived":["home:thread"]}',
  );
  assert.deepEqual(
    toggleInboxPreference(archived, "archived", "home:thread"),
    pin,
  );
  assert.deepEqual(original, { pinned: [], archived: [] });
});
test("malformed, foreign-version and oversized storage refuses safely while toggles remain bounded", () => {
  for (const value of [
    "invalid",
    '{"version":2,"pinned":[],"archived":[]}',
    '{"version":1,"pinned":[4],"archived":[]}',
  ])
    assert.deepEqual(parseInboxPreferences(value), {
      pinned: [],
      archived: [],
    });
  const storage = {
    getItem: () => {
      throw Error("disabled");
    },
    setItem: () => {
      throw Error("quota");
    },
  } as unknown as Storage;
  assert.deepEqual(readInboxPreferences(storage, "session"), {
    pinned: [],
    archived: [],
  });
  const value = toggleInboxPreference(
    { pinned: Array.from({ length: 1024 }, (_, i) => String(i)), archived: [] },
    "pinned",
    "new",
  );
  assert.equal(value.pinned.length, 1024);
  assert.equal(value.pinned.at(-1), "new");
  saveInboxPreferences(storage, "session", value);
});

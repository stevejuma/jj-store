import assert from "node:assert/strict";
import test from "node:test";
import {
  loadNavigationLayout,
  saveNavigationLayout,
  navigationStorageKey,
} from "../src/lib/ui/navigation-storage.js";

function memory(): Storage {
  const data = new Map<string, string>();
  return {
    getItem: (key) => data.get(key) ?? null,
    setItem: (key, value) => void data.set(key, value),
  } as Storage;
}
test("navigation restores explicit collapsed sections and expanded projects", () => {
  const storage = memory();
  const value = {
    sections: { projects: false, repositories: true },
    items: { "project:home:id": true, "project:home:other": false },
  };
  saveNavigationLayout(storage, value);
  assert.deepEqual(loadNavigationLayout(storage), value);
  assert.deepEqual(
    Object.keys(JSON.parse(storage.getItem(navigationStorageKey)!)),
    ["version", "sections", "items"],
  );
});
test("navigation ignores malformed state and survives unavailable browser storage", () => {
  const storage = memory();
  for (const value of [
    "invalid",
    '{"version":2}',
    '{"version":1,"sections":{"projects":"false"},"items":{}}',
    '{"version":1,"sections":[],"items":{}}',
  ]) {
    storage.setItem(navigationStorageKey, value);
    assert.deepEqual(loadNavigationLayout(storage), {
      sections: {},
      items: {},
    });
  }
  const unavailable = {
    getItem() {
      throw Error("blocked");
    },
    setItem() {
      throw Error("quota");
    },
  } as unknown as Storage;
  assert.deepEqual(loadNavigationLayout(unavailable), {
    sections: {},
    items: {},
  });
  assert.doesNotThrow(() =>
    saveNavigationLayout(unavailable, { sections: {}, items: {} }),
  );
});

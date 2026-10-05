import assert from "node:assert/strict";
import test from "node:test";
import {
  defaultMergeEditorSettings,
  loadMergeEditorSettings,
  mergeEditorSettingsKey,
  saveMergeEditorSettings,
} from "../src/lib/ui/store-conflicts/merge-editor-settings.js";

function memoryStorage(): Storage {
  const values = new Map<string, string>();
  return {
    get length() {
      return values.size;
    },
    clear: () => values.clear(),
    getItem: (key) => values.get(key) ?? null,
    key: (index) => [...values.keys()][index] ?? null,
    removeItem: (key) => void values.delete(key),
    setItem: (key, value) => void values.set(key, value),
  };
}

test("merge display settings default to sync and safely reject malformed or foreign versions", () => {
  const storage = memoryStorage();
  for (const encoded of [
    "null",
    "[]",
    "invalid",
    '{"version":2,"wrap":true,"syncHorizontalScroll":false}',
    '{"version":1,"wrap":"true","syncHorizontalScroll":false}',
    '{"version":1,"wrap":true,"syncHorizontalScroll":false,"ignoreCase":"yes"}',
  ]) {
    storage.setItem(mergeEditorSettingsKey, encoded);
    assert.deepEqual(
      loadMergeEditorSettings(storage),
      defaultMergeEditorSettings,
    );
  }
});

test("merge settings survive a fresh read and serialize only presentation preferences", () => {
  const storage = memoryStorage();
  saveMergeEditorSettings(
    { ...defaultMergeEditorSettings, wrap: true, syncHorizontalScroll: false },
    storage,
  );
  assert.deepEqual(loadMergeEditorSettings(storage), {
    wrap: true,
    syncHorizontalScroll: false,
    ignoreWhitespace: false,
    ignoreCase: false,
  });
  assert.deepEqual(JSON.parse(storage.getItem(mergeEditorSettingsKey)!), {
    version: 1,
    wrap: true,
    syncHorizontalScroll: false,
    ignoreWhitespace: false,
    ignoreCase: false,
  });
});

test("blocked storage does not break conflict editing", () => {
  const storage = memoryStorage();
  storage.getItem = () => {
    throw Error("denied");
  };
  storage.setItem = () => {
    throw Error("quota");
  };
  assert.deepEqual(
    loadMergeEditorSettings(storage),
    defaultMergeEditorSettings,
  );
  assert.doesNotThrow(() =>
    saveMergeEditorSettings(
      {
        ...defaultMergeEditorSettings,
        wrap: true,
        syncHorizontalScroll: false,
      },
      storage,
    ),
  );
});

test("existing version-one scroll preferences retain their values with new comparison defaults", () => {
  const storage = memoryStorage();
  storage.setItem(
    mergeEditorSettingsKey,
    JSON.stringify({ version: 1, wrap: true, syncHorizontalScroll: false }),
  );
  assert.deepEqual(loadMergeEditorSettings(storage), {
    ...defaultMergeEditorSettings,
    wrap: true,
    syncHorizontalScroll: false,
  });
});

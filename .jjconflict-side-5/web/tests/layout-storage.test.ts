import assert from "node:assert/strict";
import test from "node:test";
import type { StorePage } from "../src/lib/ui/community-navigation/model.js";
import {
  loadRepositoryFileTreeLayout,
  loadRepositoryShellLayout,
  repositoryColumnCanvasStorageKey,
  repositoryFileTreeStorageKey,
  repositoryShellStorageKey,
  resolveLayoutStorage,
  saveRepositoryFileTreeLayout,
} from "../src/lib/ui/layout-storage.js";

function createMemoryLayoutStorage(): Storage {
  const values = new Map<string, string>();
  return {
    get length() {
      return values.size;
    },
    clear: () => values.clear(),
    getItem: (key) => values.get(key) ?? null,
    key: (index) => [...values.keys()][index] ?? null,
    removeItem: (key) => void values.delete(key),
    setItem: (key, value) => values.set(key, value),
  };
}

test("layout keys are versioned and isolate shell, files and every canvas page", () => {
  const pages: StorePage[] = ["branches", "log", "oplog"];
  const keys = new Set([
    repositoryShellStorageKey,
    repositoryFileTreeStorageKey,
    ...pages.map(repositoryColumnCanvasStorageKey),
  ]);

  assert.equal(keys.size, 5);
  for (const key of keys) assert.match(key, /jj-store\/layout\/v1/u);
});

test("file-tree layout restores only a valid versioned width and open state", () => {
  const storage = createMemoryLayoutStorage();
  saveRepositoryFileTreeLayout(storage, { open: false, width: 312 });
  assert.deepEqual(loadRepositoryFileTreeLayout(storage), {
    open: false,
    width: 312,
  });

  for (const value of [
    "not json",
    JSON.stringify({ version: 2, open: false, width: 312 }),
    JSON.stringify({ version: 1, open: "no", width: 312 }),
    JSON.stringify({ version: 1, open: true, width: Number.NaN }),
  ]) {
    storage.setItem(repositoryFileTreeStorageKey, value);
    assert.deepEqual(loadRepositoryFileTreeLayout(storage), {
      open: true,
      width: 256,
    });
  }
});

test("shell layout is available synchronously without trusting malformed panels", () => {
  const storage = createMemoryLayoutStorage();
  storage.setItem(
    repositoryShellStorageKey,
    JSON.stringify({
      version: 1,
      panels: {
        left: {
          side: "left",
          collapsed: false,
          closed: false,
          width: 336,
        },
        right: {
          side: "left",
          collapsed: false,
          closed: false,
          width: 360,
        },
      },
    }),
  );
  assert.deepEqual(loadRepositoryShellLayout(storage), {
    left: {
      side: "left",
      collapsed: false,
      closed: false,
      width: 336,
    },
  });
  storage.setItem(repositoryShellStorageKey, "not json");
  assert.deepEqual(loadRepositoryShellLayout(storage), {});
});

test("storage failures fall back without leaking layout errors into rendering", () => {
  const unavailable = {
    getItem(): string | null {
      throw new Error("blocked");
    },
    setItem(): void {
      throw new Error("blocked");
    },
  } as Storage;

  assert.equal(resolveLayoutStorage(unavailable), unavailable);
  assert.deepEqual(loadRepositoryFileTreeLayout(unavailable), {
    open: true,
    width: 256,
  });
  assert.doesNotThrow(() =>
    saveRepositoryFileTreeLayout(unavailable, { open: false, width: 320 }),
  );
});

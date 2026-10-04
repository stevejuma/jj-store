import { expect, test, vi } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import { createReactionLibrary } from "../../src/lib/controller/reaction-library.svelte.js";

test("parallel preview failures await other results and revoke all newly created URLs", async () => {
  const f = createThreadTransport(),
    page = await f.transport.emojis();
  if (page.items.type !== "emojis") throw Error("emoji page");
  const first = page.items.definitions[0]!,
    second = {
      ...first,
      emoji_id: "77".repeat(16),
      preview: { ...first.preview, id: "88".repeat(32) },
    };
  f.transport.emojis = async () => ({
    ...page,
    items: {
      type: "emojis",
      versions: page.items.type === "emojis" ? page.items.versions : {},
      definitions: [first, second],
    },
  });
  let release!: () => void;
  f.transport.asset = (id) =>
    id === first.preview.id
      ? Promise.reject(new Error("failed preview"))
      : new Promise((resolve) => {
          release = () =>
            resolve({ bytes: new Uint8Array([0]), type: "image/png" });
        });
  const create = vi.spyOn(URL, "createObjectURL"),
    revoke = vi.spyOn(URL, "revokeObjectURL");
  const library = createReactionLibrary({
    source: () => f.transport,
    principal: () => ids.actor,
    generation: () => 1,
    valid: () => true,
  });
  const loading = library.load();
  await Promise.resolve();
  release();
  await expect(loading).rejects.toThrow("failed preview");
  expect(create).toHaveBeenCalledTimes(1);
  expect(revoke).toHaveBeenCalledWith(create.mock.results[0]!.value);
  expect(library.emojis).toEqual([]);
  library.clear();
  vi.restoreAllMocks();
});

test("explicit library refresh reuses exact unchanged preview bytes and URLs", async () => {
  const f = createThreadTransport(),
    asset = vi.spyOn(f.transport, "asset");
  const library = createReactionLibrary({
    source: () => f.transport,
    principal: () => ids.actor,
    generation: () => 1,
    valid: () => true,
  });
  await library.load();
  const before = library.emojis[0]!.url;
  library.invalidate();
  await library.load();
  expect(asset).toHaveBeenCalledTimes(1);
  expect(library.emojis[0]!.url).toBe(before);
  library.clear();
});

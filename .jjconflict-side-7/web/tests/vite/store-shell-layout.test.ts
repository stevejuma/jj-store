import { expect, test } from "vitest";
import { createStoreShell } from "../../src/lib/ui/store-application/create-store-shell.js";
import { repositoryShellStorageKey } from "../../src/lib/ui/layout-storage.js";
import { createMemoryStoryStorage } from "../../../catalog/fixtures/memory-storage.js";

test("standard Thread pane and outer details rail persist widths independently", async () => {
  const storage = createMemoryStoryStorage();
  storage.setItem(
    repositoryShellStorageKey,
    JSON.stringify({
      version: 1,
      panels: {
        right: { side: "right", collapsed: false, closed: true, width: 360 },
        "repository-details": {
          side: "right",
          collapsed: false,
          closed: false,
          width: 280,
        },
      },
    }),
  );
  let shell = createStoreShell(storage, false);
  try {
    await shell.restoreLayout();
    const rail = shell.getPanel("repository-details");
    expect(rail).toBeDefined();
    expect(rail).not.toBe(shell.right);
    expect(rail!.width).toBe(280);
    expect(shell.right.width).toBe(360);
    shell.right.setWidth(400);
    shell.right.open();
    rail!.setWidth(300);
    expect(shell.right.width).toBe(400);
    await shell.flushSave();
    await shell.dispose();
    shell = createStoreShell(storage, false);
    await shell.restoreLayout();
    expect(shell.right.width).toBe(400);
    expect(shell.getPanel("repository-details")!.width).toBe(300);
    shell.getPanel("repository-details")!.close();
    expect(shell.right.closed).toBe(false);
  } finally {
    await shell.dispose();
  }
});

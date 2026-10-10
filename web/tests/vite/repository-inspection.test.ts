import { test, expect, vi } from "vitest";
import { RepositoryInspection } from "../../src/lib/conversations/repository-inspection.js";
import { WorkspaceBrowser } from "../../src/lib/conversations/workspace-browser.svelte.js";
import { ConversationSession } from "../../src/lib/conversations/session.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

test("pre-execution browsing pins source reads, caches them and never creates agent authority", async () => {
  const repository = "aa".repeat(16),
    operation = "bb".repeat(64),
    commit = "cc".repeat(64),
    blob = "dd".repeat(64);
  const snapshot = { repo_id: repository, operation_id: operation };
  const source = {
    repository,
    profile: "native",
    snapshot: vi.fn(async () => ({ heads: [operation] })),
    codeSummary: vi.fn(async () => ({
      snapshot,
      data: { latest: { id: commit } },
    })),
    paths: vi.fn(async () => ({
      snapshot,
      data: {
        entries: [
          {
            path: "README.md",
            conflicted: false,
            terms: [{ type: "file", id: blob }],
          },
        ],
      },
      next_cursor: null,
    })),
    file: vi.fn(async () => ({ text: "Pinned README", bytes: 13 })),
    references: vi.fn(async () => ({
      snapshot,
      data: { references: [] },
      next_cursor: null,
    })),
    commit: vi.fn(async () => ({
      snapshot,
      data: { commit: { parents: ["ee".repeat(64)] } },
    })),
    diffPaths: vi.fn(async () => ({
      snapshot,
      data: { changes: [] },
      next_cursor: null,
    })),
    close: vi.fn(),
  };
  const reads = new RepositoryInspection(
    () => source as unknown as StoreDataSource,
  );
  const session = new ConversationSession("thread:fixture");
  const fetcher = vi.fn(async () => {
    throw new Error("No agent endpoint should be called");
  });
  const browser = new WorkspaceBrowser(
    new LocalClient("", fetcher),
    session,
    () => {},
  );
  browser.repositoryReads = reads;
  browser.setBindings([], new Map([[repository, "Example"]]), [repository]);
  await browser.load();
  await browser.load();
  await browser.select("README.md");
  await browser.select("README.md");
  expect(source.snapshot).toHaveBeenCalledTimes(1);
  expect(source.codeSummary).toHaveBeenCalledTimes(1);
  expect(source.file).toHaveBeenCalledTimes(1);
  expect(source.paths).toHaveBeenCalledTimes(1);
  expect(browser.blocks.after?.text).toBe("Pinned README");
  expect(browser.blocks.after?.commentable).toBe(false);
  expect(session.workspaceSources.size).toBe(0);
  expect(fetcher).not.toHaveBeenCalled();
  await browser.selectTab("changes");
  expect(browser.tab).toBe("changes");
  expect(browser.listing.value?.comparison).toMatchObject({
    before_commit: "ee".repeat(64),
    after_commit: commit,
    current_files: false,
  });
  expect(source.diffPaths).toHaveBeenCalledWith(
    operation,
    "ee".repeat(64),
    commit,
    undefined,
    expect.any(AbortSignal),
  );
  browser.dispose();
});

test("repository comparison reads both exact file revisions and refuses unavailable turn context", async () => {
  const { repositoryComparison } =
    await import("../../src/lib/conversations/repository-comparison.js");
  const repository = "aa".repeat(16),
    operation = "bb".repeat(64),
    before = "cc".repeat(20),
    after = "dd".repeat(20);
  const snapshot = { repo_id: repository, operation_id: operation };
  const file = vi.fn(async (id: string) => ({
    text: id === "ee".repeat(20) ? "before café 🦀" : "after café 🦀",
  }));
  const diff = vi.fn(async () => ({
    snapshot,
    data: {
      changes: [
        {
          path: "src/a.ts",
          before: [{ type: "file", id: "ee".repeat(20) }],
          after: [{ type: "file", id: "ff".repeat(20) }],
        },
      ],
    },
  }));
  const source = {
    repository,
    references: async () => ({ snapshot, data: { references: [] } }),
    diff,
    file,
  } as unknown as StoreDataSource;
  const value = await repositoryComparison(
    source,
    operation,
    after,
    "src/a.ts",
    null,
    "branch",
    before,
  );
  expect(value.file).toMatchObject({
    before: "before café 🦀",
    contents: "after café 🦀",
  });
  expect(file).toHaveBeenCalledWith("ee".repeat(20), operation, {
    commit: before,
    path: "src/a.ts",
    signal: undefined,
  });
  expect(file).toHaveBeenCalledWith("ff".repeat(20), operation, {
    commit: after,
    path: "src/a.ts",
    signal: undefined,
  });
  const unavailable = await repositoryComparison(
    source,
    operation,
    after,
    null,
    null,
    "last_turn",
    null,
  );
  expect(unavailable.comparison?.unavailable).toContain("No agent turn");
  expect(diff).toHaveBeenCalledTimes(1);
});

test("native summary adapter coalesces immutable operation reads", async () => {
  const { HttpStoreDataSource } =
    await import("../../src/lib/controller/http-store-data-source.js");
  const { createStoreQueryClient } =
    await import("../../src/lib/controller/store-query-client.js");
  const operation = "bb".repeat(64);
  const fetcher = vi.fn(async () =>
    Response.json({
      snapshot: { repo_id: "aa".repeat(16), operation_id: operation },
      data: { latest: { id: "cc".repeat(64) } },
      next_cursor: null,
    }),
  );
  const source = new HttpStoreDataSource(
    "aa".repeat(16),
    null,
    createStoreQueryClient(),
    "test",
    () => {},
    fetcher,
  );
  await Promise.all([
    source.codeSummary(operation),
    source.codeSummary(operation),
  ]);
  expect(fetcher).toHaveBeenCalledTimes(1);
  source.close();
});

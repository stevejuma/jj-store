import { expect, test, vi } from "vitest";
import { createWorkflowCodeOptions } from "../../src/lib/ui/store-workflows/workflow-code-options.svelte.js";
import { workflowCollaboration } from "../../../catalog/fixtures/workflow-collaboration.js";
import { code, home } from "../../../catalog/fixtures/workflows.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

test("cold source discovery pages history at one explicit operation and query time", async () => {
  const source = workflowCollaboration(code);
  const graph = vi.fn(source.graph);
  const page = await source.graph("log", { operation: "61".repeat(64) });
  graph
    .mockResolvedValueOnce({ ...page, next_cursor: "next-authors" })
    .mockResolvedValueOnce(page);
  // Keep one acquired reader identity, as production does.
  const reader = { ...source, graph };
  const choices = createWorkflowCodeOptions(() => reader);
  await choices.refresh();
  expect(choices.authors).toEqual([
    { id: "code@example.invalid", name: "Code author" },
  ]);
  expect(choices.info?.callables[0]?.name).toBe("bookmarks");
  expect(choices.symbols).toEqual(["main"]);
  expect(choices.hasMore).toBe(true);
  await choices.more();
  expect(graph.mock.calls[0]![1]).toEqual(
    expect.objectContaining({ operation: "61".repeat(64), limit: 50 }),
  );
  expect(graph.mock.calls[1]![1]).toEqual(
    expect.objectContaining({
      cursor: "next-authors",
      atTimeMillis: graph.mock.calls[0]![1].atTimeMillis,
    }),
  );
  expect(choices.hasMore).toBe(false);
});

test("an old source reply cannot populate author or syntax choices for another repository", async () => {
  const old = workflowCollaboration(home);
  let release!: (
    value: Awaited<ReturnType<StoreDataSource["snapshot"]>>,
  ) => void;
  old.snapshot = () =>
    new Promise((resolve) => {
      release = resolve;
    });
  let current = old;
  const choices = createWorkflowCodeOptions(() => current);
  const pending = choices.refresh();
  const next = workflowCollaboration(code);
  current = next;
  release(await next.snapshot());
  await pending;
  expect(choices.authors).toEqual([]);
  expect(choices.info).toBeNull();
  await choices.refresh();
  expect(choices.authors[0]?.id).toBe("code@example.invalid");
});

test("a mismatched history snapshot is refused and retains a retryable error", async () => {
  const reader = workflowCollaboration(code),
    other = workflowCollaboration(home);
  reader.graph = other.graph;
  const choices = createWorkflowCodeOptions(() => reader);
  await choices.refresh();
  expect(choices.authors).toEqual([]);
  expect(choices.error).toContain("Author history changed");
  expect(choices.loading).toBe(false);
});

test("native author discovery excludes independent local companion actors", async () => {
  const { workflowAuthors } =
    await import("../../src/lib/ui/store-workflows/workflow-presentation.js");
  const { createInMemoryStoreController } =
    await import("../../../catalog/controller/in-memory-store-controller.svelte.js");
  const base = createInMemoryStoreController();
  const nativeActor = "11".repeat(16),
    localActor = "22".repeat(16);
  const { createRepositoryThreadNavigation } =
    await import("../../src/lib/controller/repository-thread-navigation.svelte.js");
  const repositories = [
    {
      id: home,
      name: "Home",
      principal: nativeActor,
      permission: "read" as const,
    },
    {
      id: code,
      name: "Local companion",
      profile: "git_sha1_jj045" as const,
      principal: localActor,
      permission: "read" as const,
    },
  ];
  const navigation = createRepositoryThreadNavigation({
    repositories: () => repositories,
    credential: () => null,
    source: (repository) => {
      const reader = workflowCollaboration(repository);
      return {
        close() {},
        list: async () => {
          const page = await reader.threads!.list();
          if (page.items.type !== "threads") throw Error("Expected threads");
          return {
            ...page,
            items: {
              type: "threads" as const,
              threads: page.items.threads.map((t) => ({
                ...t,
                value: {
                  ...t.value,
                  actor: repository === home ? nativeActor : localActor,
                },
              })),
            },
          };
        },
      };
    },
  });
  await navigation.load();
  const authors = workflowAuthors({
    ...base,
    threadNavigation: navigation,
    state: { ...base.state, repositories },
  });
  expect(authors.some((a) => a.id === nativeActor)).toBe(true);
  expect(authors.some((a) => a.id === localActor)).toBe(false);
  navigation.clear();
});

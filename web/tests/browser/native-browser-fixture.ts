import { test, expect, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
export const samples = JSON.parse(
  readFileSync(
    fileURLToPath(
      new URL("../../../.scratch/web-responses.json", import.meta.url),
    ),
    "utf8",
  ),
);
export const sample = (path: string) =>
  structuredClone(
    samples.find(
      (s: any) => s.path === `/v1/repos/{repo}/${path}` && s.status === 200,
    ).data,
  );
const snapshot = sample("snapshot");
export const repo = snapshot.repo_id;
export const operation = snapshot.heads[0];
export const token = "disposable-browser-fixture-token";
export const graph = sample("log");
const commitNode = graph.data.data.nodes.find((n: any) =>
  n.commit.description.startsWith("binary"),
).commit;
export const commit = commitNode.id;
export const parent = commitNode.parents[0];
const tree = sample("commits/{commit}/tree");
const stats = graph.data.stats;
const textChange = {
  path: "bin",
  before: [],
  after: tree.data.entries[0].terms,
  sources: [{ kind: "normal", path: "bin", terms: [] }],
  content: {
    status: "text",
    hunks: [
      {
        kind: "different",
        before_start: 1,
        after_start: 1,
        before: "",
        after: "<script>window.sourceExecuted=true</script>\n",
      },
    ],
  },
};
const windowPage = (data: unknown, next_cursor: string | null = null) => ({
  snapshot: structuredClone(graph.snapshot),
  data,
  previous_cursor: null,
  next_cursor,
});
function referencesPage(next_cursor: string | null = null) {
  return windowPage(
    {
      version: 1,
      operation,
      references: [
        {
          kind: "bookmark",
          name: "main",
          terms: [commit],
          conflicted: false,
          workspaces: [...commitNode.workspaces],
          targets: [
            {
              commit,
              change: commitNode.change_id,
              parents: [...commitNode.parents],
              description: commitNode.description,
              author: structuredClone(commitNode.author),
              committer: structuredClone(commitNode.committer),
              conflicted: commitNode.conflicted,
              workspaces: [...commitNode.workspaces],
            },
          ],
        },
      ],
      previous: null,
      next: next_cursor,
      stats,
    },
    next_cursor,
  );
}
export function pathsPage(
  next_cursor: string | null = null,
  options: {
    path?: string;
    recursive?: boolean;
    entries?: any[];
  } = {},
) {
  return windowPage(
    {
      version: 1,
      commit,
      path: options.path ?? "",
      recursive: options.recursive ?? false,
      entries: (options.entries ?? tree.data.entries).map((entry: any) => ({
        path: entry.path ?? entry.name,
        terms: structuredClone(entry.terms),
        conflicted: entry.conflicted,
        last_change: {
          status: "available",
          commit,
          change: commitNode.change_id,
          description: commitNode.description,
          author: structuredClone(commitNode.author),
          committer: structuredClone(commitNode.committer),
        },
      })),
      previous: null,
      next: next_cursor,
      stats,
    },
    next_cursor,
  );
}
export function operationChangesPage(
  groups = [
    {
      change: commitNode.change_id,
      before: [parent],
      after: [commit],
      comparison: { status: "exact", from: parent, to: commit },
    },
  ],
  next: string | null = null,
) {
  return windowPage(
    {
      version: 1,
      operation,
      parent,
      groups,
      previous: null,
      next,
      stats,
    },
    next,
  );
}
export function diffPathsPage(
  changes = [structuredClone(textChange)],
  next: string | null = null,
) {
  return windowPage(
    {
      version: 1,
      from: parent,
      to: commit,
      path: "",
      changes,
      previous: null,
      next,
      stats,
    },
    next,
  );
}
function diffPage() {
  const value = sample("diff");
  value.data.from = parent;
  value.data.to = commit;
  value.data.path = "bin";
  value.data.changes = [structuredClone(textChange)];
  return value;
}
export async function api(
  page: Page,
  options: { multi?: boolean; changed?: boolean; delayLog?: boolean } = {},
) {
  const requests: string[] = [];
  await page.route("**/v1/repos/**", async (route) => {
    const request = route.request();
    const url = new URL(request.url());
    const path = url.pathname.split(`/v1/repos/${repo}/`)[1];
    requests.push(path);
    expect(request.headers()["authorization"]).toBe(`Bearer ${token}`);
    if (path === "events") {
      const ready = sample("events");
      if (options.multi) ready.snapshot.heads.push("ab".repeat(64));
      const initial = `v1:${repo}:${ready.cursor.epoch}:${ready.cursor.position}`;
      const changed = {
        version: 1,
        type: "changed",
        cursor: { ...ready.cursor, position: ready.cursor.position + 1 },
      };
      await route.fulfill({
        contentType: "text/event-stream",
        body: `id: ${initial}\nevent: ready\ndata: ${JSON.stringify(ready)}\n\n${options.changed ? `id: v1:${repo}:${changed.cursor.epoch}:${changed.cursor.position}\nevent: repository-changed\ndata: ${JSON.stringify(changed)}\n\n` : ""}`,
      });
      return;
    }
    if (path === "snapshot") {
      const value = sample("snapshot");
      if (options.multi) value.heads.push("ab".repeat(64));
      await route.fulfill({ json: value });
      return;
    }
    if (path === "references") {
      await route.fulfill({ json: referencesPage() });
      return;
    }
    if (path?.startsWith("commits/") && path.endsWith("/paths")) {
      await route.fulfill({
        json: pathsPage(null, {
          path: url.searchParams.get("path") ?? "",
          recursive: url.searchParams.get("recursive") === "true",
        }),
      });
      return;
    }
    if (path?.startsWith("operations/") && path.endsWith("/changes")) {
      await route.fulfill({ json: operationChangesPage() });
      return;
    }
    if (path === "diff/paths") {
      await route.fulfill({ json: diffPathsPage() });
      return;
    }
    if (path === "diff") {
      await route.fulfill({ json: diffPage() });
      return;
    }
    if (path === "log" && options.delayLog)
      await new Promise((resolve) => setTimeout(resolve, 250));
    if (path?.startsWith("files/")) {
      await route.fulfill({
        status: 206,
        headers: {
          "content-type": "application/octet-stream",
          "x-jj-store-operation": operation,
          "x-jj-store-repository": repo,
          "content-range": "bytes 0-37/38",
        },
        body: "<script>window.sourceExecuted=true</script>",
      });
      return;
    }
    const template = path
      ?.replace(/commits\/[0-9a-f]+/, "commits/{commit}")
      .replace(/operations\/[0-9a-f]+/, "operations/{operation}");
    const found = samples.find(
      (s: any) =>
        s.path === `/v1/repos/{repo}/${template}` &&
        s.method === request.method().toLowerCase() &&
        s.status < 300,
    );
    if (!found) {
      await route.fulfill({
        status: 404,
        json: { version: 1, code: "fixture_missing" },
      });
      return;
    }
    await route.fulfill({ status: found.status, json: found.data });
  });
  return requests;
}
export async function connect(page: Page) {
  await page.goto("/");
  await page.getByLabel("Repository ID", { exact: true }).fill(repo);
  await page.getByLabel("Repository token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Open repository" }).click();
}

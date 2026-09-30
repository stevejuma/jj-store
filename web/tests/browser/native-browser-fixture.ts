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
export const mutationSample = (
  path: "mutations/preview" | "mutations" | "mutations/{request_id}",
  status: "prepared" | "committed" | "outcome_unknown" | "not_known",
) =>
  structuredClone(
    samples.find(
      (s: any) =>
        s.path === `/v1/repos/{repo}/${path}` &&
        s.status === 200 &&
        s.data.status === status,
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
export const editedCommit = "de".repeat(64);
export const editedOperation = "ef".repeat(64);
const editedFile = "cd".repeat(64);
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
function referencesPage(
  next_cursor: string | null = null,
  state: { commit?: string; operation?: string; description?: string } = {},
) {
  const selectedCommit = state.commit ?? commit;
  const selectedOperation = state.operation ?? operation;
  const snapshot = structuredClone(graph.snapshot);
  snapshot.heads = [selectedOperation];
  snapshot.operation_id = selectedOperation;
  if (state.operation) {
    snapshot.published_seq += 1;
    snapshot.manifest_revision += 1;
  }
  const value = windowPage(
    {
      version: 1,
      operation: selectedOperation,
      references: [
        {
          kind: "bookmark",
          name: "main",
          terms: [selectedCommit],
          conflicted: false,
          workspaces: [...commitNode.workspaces],
          targets: [
            {
              commit: selectedCommit,
              change: state.commit ? "ed".repeat(16) : commitNode.change_id,
              parents: state.commit ? [commit] : [...commitNode.parents],
              description: state.description ?? commitNode.description,
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
  value.snapshot = snapshot;
  return value;
}
export function pathsPage(
  next_cursor: string | null = null,
  options: {
    path?: string;
    recursive?: boolean;
    entries?: any[];
    commit?: string;
    operation?: string;
  } = {},
) {
  const value = windowPage(
    {
      version: 1,
      commit: options.commit ?? commit,
      path: options.path ?? "",
      recursive: options.recursive ?? false,
      entries: (options.entries ?? tree.data.entries).map((entry: any) => ({
        path: entry.path ?? entry.name,
        terms: structuredClone(entry.terms),
        conflicted: entry.conflicted,
        last_change: {
          status: "available",
          commit: options.commit ?? commit,
          change: options.commit ? "ed".repeat(16) : commitNode.change_id,
          description: options.commit
            ? "Edit repository file"
            : commitNode.description,
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
  if (options.operation) {
    value.snapshot.heads = [options.operation];
    value.snapshot.operation_id = options.operation;
    value.snapshot.published_seq += 1;
    value.snapshot.manifest_revision += 1;
  }
  return value;
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
function searchDocument(kind: "commit" | "file" | "operation") {
  const common = {
    body: "",
    actor: kind === "operation" ? "Maya Chen" : null,
    author: "Maya Chen",
    workspace: "demo",
    operation,
    commit: kind === "operation" ? null : commit,
    change: kind === "operation" ? null : commitNode.change_id,
    file: kind === "file" ? tree.data.entries[0].terms[0].id : null,
    parents: kind === "commit" ? [...commitNode.parents] : [],
    predecessors: [],
    attributes: {},
    language: kind === "file" ? "text" : null,
    has_conflict: false,
    timestamp_millis: commitNode.committer.timestamp.millis,
  };
  if (kind === "operation")
    return {
      ...common,
      key: { kind, id: operation },
      title: "Captured native operation",
    };
  if (kind === "commit")
    return {
      ...common,
      key: { kind, id: commit },
      title: commitNode.description,
    };
  return {
    ...common,
    key: { kind, scope: "main", path: "bin" },
    title: "bin",
  };
}
function searchPage(query: string) {
  const kind = query.includes("file")
    ? "file"
    : query.includes("operation")
      ? "operation"
      : "commit";
  return {
    version: 1,
    repository: repo,
    epoch: graph.snapshot.epoch,
    repository_position: graph.snapshot.published_seq,
    indexed_position: graph.snapshot.published_seq,
    index_generation: "aa".repeat(32),
    selected_view: structuredClone(graph.snapshot),
    scope_snapshots: {},
    results: [searchDocument(kind)],
    is_stale: false,
    degraded: false,
    next_cursor: null,
    truncated: false,
  };
}
export async function api(
  page: Page,
  options: {
    multi?: boolean;
    changed?: boolean;
    delayLog?: boolean;
    access?: "authenticated" | "local_trusted";
    editFile?: boolean;
  } = {},
) {
  const requests: string[] = [];
  let edited = false;
  let editCommand: any = null;
  await page.route("**/v1/bootstrap", async (route) => {
    const request = route.request();
    expect(request.headers()["x-jj-store-protocol"]).toBe("1");
    expect(request.headers()["authorization"]).toBeUndefined();
    await route.fulfill({
      json: {
        version: 1,
        access_mode: options.access ?? "authenticated",
        capabilities: sample("capabilities"),
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    });
  });
  await page.route("**/v1/repositories?*", async (route) => {
    const headers = route.request().headers();
    expect(headers["x-jj-store-protocol"]).toBe("1");
    if (options.access === "local_trusted")
      expect(headers["authorization"]).toBeUndefined();
    else expect(headers["authorization"]).toBe(`Bearer ${token}`);
    await route.fulfill({
      json: {
        version: 1,
        generation: 1,
        repositories: [
          { id: repo, name: "Local acceptance", permission: "admin" },
        ],
        next_cursor: null,
      },
    });
  });
  await page.route("**/v1/repos/**", async (route) => {
    const request = route.request();
    const url = new URL(request.url());
    const path = url.pathname.split(`/v1/repos/${repo}/`)[1];
    requests.push(path);
    expect(request.headers()["x-jj-store-protocol"]).toBe("1");
    if (options.access === "local_trusted")
      expect(request.headers()["authorization"]).toBeUndefined();
    else expect(request.headers()["authorization"]).toBe(`Bearer ${token}`);
    if (path === "events") {
      const ready = sample("events");
      if (edited) {
        ready.snapshot.heads = [editedOperation];
        ready.snapshot.published_seq += 1;
        ready.snapshot.manifest_revision += 1;
        ready.cursor.position += 1;
      }
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
      if (edited) {
        value.heads = [editedOperation];
        value.published_seq += 1;
        value.manifest_revision += 1;
      }
      if (options.multi) value.heads.push("ab".repeat(64));
      await route.fulfill({ json: value });
      return;
    }
    if (path === "capabilities") {
      const value = sample("capabilities");
      value.search = true;
      if (options.editFile && !value.typed_mutations.includes("edit_file"))
        value.typed_mutations.push("edit_file");
      await route.fulfill({ json: value });
      return;
    }
    if (path === "search") {
      await route.fulfill({
        json: searchPage(url.searchParams.get("q") ?? ""),
      });
      return;
    }
    if (path === "references") {
      await route.fulfill({
        json: referencesPage(
          null,
          edited
            ? {
                commit: editedCommit,
                operation: editedOperation,
                description: editCommand?.description,
              }
            : {},
        ),
      });
      return;
    }
    if (path?.startsWith("commits/") && path.endsWith("/paths")) {
      const editedEntries = editCommand
        ? [
            {
              name: editCommand.destination,
              terms: [
                {
                  type: "file",
                  id: editedFile,
                  executable: false,
                  copy: structuredClone(tree.data.entries[0].terms[0].copy),
                },
              ],
              conflicted: false,
            },
            ...tree.data.entries.slice(1),
          ]
        : undefined;
      await route.fulfill({
        json: pathsPage(null, {
          path: url.searchParams.get("path") ?? "",
          recursive: url.searchParams.get("recursive") === "true",
          ...(edited
            ? {
                commit: editedCommit,
                operation: editedOperation,
                entries: editedEntries,
              }
            : {}),
        }),
      });
      return;
    }
    if (edited && path === `commits/${editedCommit}`) {
      const value = sample("commits/{commit}");
      value.snapshot.heads = [editedOperation];
      value.snapshot.operation_id = editedOperation;
      value.snapshot.published_seq += 1;
      value.snapshot.manifest_revision += 1;
      value.data.commit.id = editedCommit;
      value.data.commit.change_id = "ed".repeat(16);
      value.data.commit.parents = [commit];
      value.data.commit.description = editCommand.description;
      value.data.commit.tree_terms = [tree.data.entries[0].terms[0].id];
      await route.fulfill({ json: value });
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
    if (path === "log" && edited) {
      const value = structuredClone(graph);
      value.snapshot.heads = [editedOperation];
      value.snapshot.operation_id = editedOperation;
      value.snapshot.published_seq += 1;
      value.snapshot.manifest_revision += 1;
      value.data.operation = editedOperation;
      const node = structuredClone(value.data.data.nodes[0]);
      node.commit.id = editedCommit;
      node.commit.change_id = "ed".repeat(16);
      node.commit.parents = [commit];
      node.commit.description = editCommand.description;
      node.edges = [{ target: commit, kind: "direct" }];
      value.data.data.nodes.unshift(node);
      await route.fulfill({ json: value });
      return;
    }
    if (path === "log" && options.delayLog)
      await new Promise((resolve) => setTimeout(resolve, 250));
    if (path?.startsWith("files/")) {
      const contents = edited
        ? editCommand.contents
        : "<script>window.sourceExecuted=true</script>";
      const byteLength = new TextEncoder().encode(contents).length;
      await route.fulfill({
        status: 206,
        headers: {
          "content-type": "application/octet-stream",
          "x-jj-store-operation": edited ? editedOperation : operation,
          "x-jj-store-repository": repo,
          "content-range": `bytes 0-${byteLength - 1}/${byteLength}`,
        },
        body: contents,
      });
      return;
    }
    if (
      options.editFile &&
      path === "mutations/preview" &&
      request.method() === "POST" &&
      request.postDataJSON().command.type === "edit_file"
    ) {
      const body = request.postDataJSON();
      editCommand = body.command;
      const value = mutationSample("mutations/preview", "prepared");
      value.preview.request_id = body.request_id;
      value.preview.expected = body.expected;
      value.preview.result.operation = editedOperation;
      value.preview.result.created = [editedCommit];
      value.preview.result.bookmarks = {
        [editCommand.bookmark]: {
          before: [...editCommand.expected],
          after: [editedCommit],
        },
      };
      await route.fulfill({ json: value });
      return;
    }
    if (
      options.editFile &&
      path === "mutations" &&
      request.method() === "POST" &&
      request.postDataJSON().command.type === "edit_file"
    ) {
      const body = request.postDataJSON();
      editCommand = body.command;
      edited = true;
      const value = mutationSample("mutations", "committed");
      value.receipt.request_id = body.request_id;
      value.receipt.accepted = {
        ...body.expected,
        position: body.expected.position + 1,
        heads: [editedOperation],
      };
      value.receipt.result.operation = editedOperation;
      value.receipt.result.created = [editedCommit];
      value.receipt.result.bookmarks = {
        [editCommand.bookmark]: {
          before: [...editCommand.expected],
          after: [editedCommit],
        },
      };
      await route.fulfill({ json: value });
      return;
    }
    const template = path
      ?.replace(/commits\/[0-9a-f]+/, "commits/{commit}")
      .replace(/operations\/[0-9a-f]+/, "operations/{operation}")
      .replace(/mutations\/[0-9a-f]+/, "mutations/{request_id}");
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
  await page.getByLabel("Access token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Continue" }).click();
}

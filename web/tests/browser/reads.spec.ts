import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  api,
  commit,
  connect,
  diffPathsPage,
  graph,
  operation,
  operationChangesPage,
  pathsPage,
  repo,
  sample,
  samples,
  token,
} from "./native-browser-fixture.js";

test("changed-file collections consume commit, group and path cursors", async ({
  page,
}) => {
  await api(page);
  let phase: "commit" | "operation" = "commit";
  let operationPathPages = 0;
  const pathCursors: (string | null)[] = [];
  const groupCursors: (string | null)[] = [];
  await page.route("**/diff/paths?**", async (route) => {
    const cursor = new URL(route.request().url()).searchParams.get("cursor");
    pathCursors.push(cursor);
    const first = phase === "commit" ? "src/first.rs" : "src/op-first.rs";
    const second = phase === "commit" ? "src/second.rs" : "src/op-second.rs";
    const next = phase === "commit" && cursor === null ? "paths-2" : null;
    const result = diffPathsPage(undefined, next);
    result.data.from = new URL(route.request().url()).searchParams.get("from")!;
    result.data.to = new URL(route.request().url()).searchParams.get("to")!;
    result.data.changes[0].path =
      phase === "commit"
        ? cursor === null
          ? first
          : second
        : operationPathPages++ === 0
          ? first
          : second;
    await route.fulfill({ json: result });
  });
  await page.route("**/operations/*/changes?**", async (route) => {
    const cursor = new URL(route.request().url()).searchParams.get("cursor");
    groupCursors.push(cursor);
    const result = operationChangesPage(undefined, cursor ? null : "groups-2");
    if (!cursor) {
      result.data.groups[0].comparison = {
        status: "exact",
        from: "11".repeat(64),
        to: "22".repeat(64),
      };
    } else {
      result.data.groups[0].change = "ef".repeat(16);
      result.data.groups[0].comparison = {
        status: "exact",
        from: "ab".repeat(64),
        to: "ef".repeat(64),
      };
    }
    await route.fulfill({ json: result });
  });

  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  await page
    .getByRole("button", { name: /binary <script>/u })
    .first()
    .click();
  const commitFiles = page.getByRole("region", { name: "Changed files" });
  await commitFiles.getByRole("radio", { name: "Show files as list" }).click();
  await commitFiles
    .locator(".community-revision-graph-files__body")
    .evaluate((node) => {
      node.scrollTop = node.scrollHeight;
      node.dispatchEvent(new Event("scroll"));
    });
  await expect.poll(() => pathCursors.slice(0, 2)).toEqual([null, "paths-2"]);
  await expect(
    commitFiles.getByRole("button", { name: "src/second.rs" }),
  ).toBeVisible();

  phase = "operation";
  pathCursors.length = 0;
  await page.getByRole("tab", { name: "Op Log", exact: true }).click();
  await page
    .getByRole("button", { name: "content fixture", exact: true })
    .click();
  const operationFiles = page.getByRole("region", {
    name: "Changed files in operation",
  });
  await operationFiles
    .getByRole("radio", { name: "Show files as list" })
    .click();
  await operationFiles
    .locator(".community-revision-graph-files__body")
    .evaluate((node) => {
      node.scrollTop = node.scrollHeight;
      node.dispatchEvent(new Event("scroll"));
    });
  await expect.poll(() => groupCursors).toEqual([null, "groups-2"]);
  await expect(
    operationFiles.getByRole("button", { name: "src/op-second.rs" }),
  ).toBeVisible();
});
test("native_graphs_files_and_source_rendering_are_accessible", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await api(page);
  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  await expect(
    page.getByRole("button", { name: /binary <script>/ }).first(),
  ).toBeVisible();
  await page
    .getByRole("button", { name: /binary <script>/ })
    .first()
    .click();
  await expect(
    page.getByRole("region", { name: "Changed files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "bin", exact: true }).click();
  await expect(page.getByText("window.sourceExecuted=true")).toBeVisible();
  await page.getByRole("tab", { name: "Overview" }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Open file bin" }).click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toContainText("<script>window.sourceExecuted=true</script>");
  expect(
    await page.evaluate(() => Reflect.get(window, "sourceExecuted")),
  ).toBeUndefined();
  await page.getByRole("tab", { name: "Op Log", exact: true }).click();
  await page
    .getByRole("button", { name: "content fixture", exact: true })
    .click();
  await expect(
    page.getByRole("region", { name: "Changed files in operation" }),
  ).toBeVisible();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  const browserStorage = await page.evaluate(() => ({
    local: Object.fromEntries(
      Array.from({ length: localStorage.length }, (_, index) => {
        const key = localStorage.key(index)!;
        return [key, localStorage.getItem(key)];
      }),
    ),
    sessionLength: sessionStorage.length,
    cookie: document.cookie,
  }));
  expect(Object.keys(browserStorage.local)).not.toHaveLength(0);
  expect(
    Object.keys(browserStorage.local).every((key) =>
      key.startsWith("jj-store/layout/v1/"),
    ),
  ).toBe(true);
  expect(JSON.stringify(browserStorage.local)).not.toContain(repo);
  expect(JSON.stringify(browserStorage.local)).not.toContain(token);
  expect([browserStorage.sessionLength, browserStorage.cookie]).toEqual([
    0,
    "",
  ]);
  expect(page.url()).not.toContain(token);
  expect(errors).toEqual([]);
  await page.screenshot({
    path: ".scratch/browser-history.png",
    fullPage: true,
  });
  await page
    .getByRole("button", { name: "Open profile menu for Repository connected" })
    .click();
  await page.getByRole("menuitem", { name: "Sign out" }).click();
  await expect(page.getByRole("menu", { name: "Profile menu" })).toBeHidden();
  await expect(
    page.getByRole("heading", { name: "Explore native history." }),
  ).toBeVisible();
  await expect(page.getByLabel("Access token", { exact: true })).toHaveValue(
    "",
  );
});
test("branch targets scope native history and expose repository decorations", async ({
  page,
}) => {
  await api(page);
  const revsets: string[] = [];
  await page.route("**/log?**", async (route) => {
    revsets.push(new URL(route.request().url()).searchParams.get("revset")!);
    await route.fulfill({ json: sample("log") });
  });
  await connect(page);
  const workingCopy = page.getByRole("button", {
    name: "Working copy · laptop",
    exact: true,
  });
  await expect(workingCopy).toBeVisible();

  revsets.length = 0;
  await page.getByRole("option", { name: "main", exact: true }).click();
  await expect.poll(() => revsets.at(-1)).toBe(`::${commit}`);
  await expect(
    page
      .locator('[data-chip-tone="danger"]', { hasText: "conflicted" })
      .first(),
  ).toBeVisible();
  await expect(
    page.locator('[data-chip-tone="success"]', {
      hasText: "Working copy · laptop",
    }),
  ).toBeVisible();

  revsets.length = 0;
  await workingCopy.click();
  await expect.poll(() => revsets.at(-1)).toBe(`::${commit}`);
  expect(new URL(page.url()).hash).toContain(`commit=${commit}`);

  await page.getByRole("tab", { name: "Op Log", exact: true }).click();
  const operationGraph = page.getByRole("list", { name: "Operation graph" });
  await expect(
    operationGraph.locator('[data-chip-tone="success"]', {
      hasText: "Working copy · laptop",
    }),
  ).toBeVisible();
  await expect(
    operationGraph.getByText("Snapshot", { exact: true }),
  ).toBeVisible();
  await expect(
    operationGraph.getByText("head 1", { exact: true }),
  ).toHaveAttribute("data-chip-tone", "accent");
});
test("multiple_heads_require_choice_and_events_preserve_the_selected_view", async ({
  page,
}) => {
  const requests = await api(page, { multi: true, changed: true });
  await connect(page);
  await expect(
    page.getByText("Multiple operation heads", { exact: true }),
  ).toBeVisible();
  expect(requests).not.toContain("log");
  await page
    .getByRole("button", { name: operation.slice(0, 12), exact: true })
    .click();
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", {
      name: "Refresh snapshot (new history available)",
    }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Toggle details" }).click();
  await expect(
    page.getByRole("complementary", { name: "Repository details" }),
  ).toBeVisible();
  const rail = page.getByRole("complementary", { name: "Repository details" });
  await expect(rail.getByText("Branches/Bookmarks")).toBeVisible();
  await expect(
    rail.getByRole("region", { name: "Repository context" }),
  ).toHaveCount(0);
  expect(new URL(page.url()).hash).toContain(`at_op=${operation}`);
});
test("disconnect_discards_inflight_history_and_revocation_clears_data", async ({
  page,
}) => {
  await api(page, { delayLog: true });
  await connect(page);
  await page
    .getByRole("button", { name: "Open profile menu for Repository connected" })
    .click();
  await page.getByRole("menuitem", { name: "Sign out" }).click();
  await expect(page.getByRole("menu", { name: "Profile menu" })).toBeHidden();
  await page.waitForTimeout(400);
  await expect(
    page.getByRole("region", { name: "Repository log" }),
  ).toHaveCount(0);
  await page.getByLabel("Access token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Continue" }).click();
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await page.route("**/snapshot", (route) =>
    route.fulfill({ status: 403, json: { version: 1, code: "access_denied" } }),
  );
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect(
    page.getByRole("heading", { name: "Explore native history." }),
  ).toBeVisible();
  await expect(
    page.getByRole("region", { name: "Repository log" }),
  ).toHaveCount(0);
  await expect(page.getByLabel("Access token", { exact: true })).toHaveValue(
    "",
  );
});
test("mobile_connect_and_history_have_no_horizontal_overflow", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await api(page);
  await connect(page);
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
});

test("graph_and_directory_pages_retain_exact_snapshots_and_query_context", async ({
  page,
}) => {
  await api(page);
  const queries: URLSearchParams[] = [];
  await page.route("**/log?**", async (route) => {
    const query = new URL(route.request().url()).searchParams;
    queries.push(query);
    const result = sample("log");
    const queryPage = queries.filter(
      (candidate) => candidate.get("revset") === query.get("revset"),
    ).length;
    result.data.data.nodes = result.data.data.nodes.slice(
      queryPage - 1,
      queryPage,
    );
    result.next_cursor = queryPage === 1 ? "graph-page-2" : null;
    if (queryPage > 1) result.snapshot.manifest_revision++;
    await route.fulfill({ json: result });
  });
  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  const revisionGraph = page.getByRole("list", { name: "Revision graph" });
  await expect
    .poll(
      () =>
        queries.filter((query) => query.get("revset") === `::${commit}`).length,
    )
    .toBe(2);
  await expect(revisionGraph.getByRole("listitem")).toHaveCount(1);
  await page.getByRole("button", { name: "Toggle details" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  const scopedQueries = queries.filter(
    (query) => query.get("revset") === `::${commit}`,
  );
  expect(scopedQueries[1]!.get("at_time_millis")).toBe(
    scopedQueries[0]!.get("at_time_millis"),
  );
  expect(scopedQueries[1]!.get("cursor")).toBe("graph-page-2");
  await expect(revisionGraph.getByRole("listitem")).toHaveCount(1);
  let pages = 0;
  await page.route("**/commits/*/paths?**", async (route) => {
    const result = pathsPage("paths-page-2");
    if (pages++) {
      result.snapshot.manifest_revision++;
      result.next_cursor = null;
    }
    await route.fulfill({ json: result });
  });
  await page.getByRole("tab", { name: "Overview" }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await expect.poll(() => pages).toBe(2);
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  await expect(page.getByRole("button", { name: "Open file bin" })).toHaveCount(
    1,
  );
});

test("repository directories load lazily and flat views request recursive paths", async ({
  page,
}) => {
  await api(page);
  const queries: URLSearchParams[] = [];
  await page.route("**/commits/*/paths?**", async (route) => {
    const query = new URL(route.request().url()).searchParams;
    queries.push(query);
    const path = query.get("path") ?? "";
    const recursive = query.get("recursive") === "true";
    const entries = recursive
      ? [
          {
            path: "src/lib.rs",
            terms: [{ type: "file", id: "fe".repeat(64), executable: false }],
            conflicted: false,
          },
        ]
      : path === "src"
        ? [
            {
              path: "src/lib.rs",
              terms: [{ type: "file", id: "fe".repeat(64), executable: false }],
              conflicted: false,
            },
          ]
        : [
            {
              path: "src",
              terms: [{ type: "tree", id: "dc".repeat(64) }],
              conflicted: false,
            },
          ];
    await route.fulfill({
      json: pathsPage(null, { path, recursive, entries }),
    });
  });

  await connect(page);
  await page.getByRole("tab", { name: "Overview" }).click();
  await expect(
    page.getByRole("button", { name: "src", exact: true }),
  ).toHaveAttribute("aria-expanded", "false");
  await expect(
    page.getByRole("button", { name: "src/lib.rs", exact: true }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "src", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "src/lib.rs", exact: true }),
  ).toBeVisible();
  await page.getByRole("radio", { name: "Show files as list" }).click();
  await expect
    .poll(() => queries.some((query) => query.get("recursive") === "true"))
    .toBe(true);

  expect(
    queries.map((query) => ({
      path: query.get("path"),
      recursive: query.get("recursive"),
    })),
  ).toEqual([
    { path: "", recursive: "false" },
    { path: "src", recursive: "false" },
    { path: "", recursive: "true" },
  ]);
});

test("index_preparation_retries_the_original_query_after_context_edits", async ({
  page,
}) => {
  await api(page);
  const queries: string[] = [];
  await page.route("**/log?**", async (route) => {
    queries.push(new URL(route.request().url()).search);
    await route.fulfill(
      queries.length === 1
        ? {
            status: 409,
            json: {
              version: 1,
              code: "index_rebuild_required",
              snapshot: graph.snapshot,
            },
          }
        : { json: sample("log") },
    );
  });
  await page.route("**/index/tasks", async (route) => {
    expect(route.request().postDataJSON()).toEqual({ operation });
    const result = samples.find(
      (s: any) => s.path.endsWith("index/tasks") && s.method === "post",
    ).data;
    await route.fulfill({ status: 202, json: result });
  });
  await page.route("**/index/tasks/*", (route) =>
    route.fulfill({ json: sample("index/tasks/{task}") }),
  );
  await connect(page);
  await page.getByRole("button", { name: "Toggle details" }).click();
  await expect(
    page.getByRole("button", { name: "Prepare graph index" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Prepare graph index" }).click();
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await expect.poll(() => queries.length).toBe(2);
  expect(queries[1]).toBe(queries[0]);
  await expect(page.getByRole("alert")).toHaveCount(0);
});

test("event_reconnect_resumes_and_reset_requires_explicit_refresh", async ({
  page,
}) => {
  await api(page);
  const resumes: (string | null)[] = [];
  const ready = sample("events");
  const id = `v1:${repo}:${ready.cursor.epoch}:${ready.cursor.position}`;
  await page.route("**/events*", async (route) => {
    resumes.push(new URL(route.request().url()).searchParams.get("after"));
    const data =
      resumes.length !== 2
        ? ready
        : { ...ready, type: "reset", reason: "future_position" };
    await route.fulfill({
      contentType: "text/event-stream",
      body: `id: ${id}\nevent: ${data.type}\ndata: ${JSON.stringify(data)}\n\n`,
    });
  });
  await connect(page);
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", {
      name: "Refresh snapshot (new history available)",
    }),
  ).toBeVisible();
  expect(resumes).toEqual([null, id]);
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect.poll(() => resumes.slice(2).includes(null)).toBe(true);
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
});

test("new_file_selection_discards_an_older_inflight_preview", async ({
  page,
}) => {
  await api(page);
  const fileA = "ab".repeat(64),
    fileB = "cd".repeat(64);
  await page.route("**/commits/*/paths?**", async (route) => {
    const result = pathsPage();
    result.data.entries = [fileA, fileB].map((id, i) => ({
      ...result.data.entries[0],
      path: `file-${i}`,
      terms: [{ ...result.data.entries[0].terms[0], id }],
    }));
    result.next_cursor = null;
    await route.fulfill({ json: result });
  });
  let began!: () => void;
  const started = new Promise<void>((resolve) => {
    began = resolve;
  });
  let finish!: () => void;
  const released = new Promise<void>((resolve) => {
    finish = resolve;
  });
  await page.route("**/files/**", async (route) => {
    const old = route.request().url().includes(fileA);
    if (old) {
      began();
      await released;
    }
    await route.fulfill({
      status: 206,
      headers: {
        "content-type": "application/octet-stream",
        "x-jj-store-operation": operation,
        "x-jj-store-repository": repo,
      },
      body: old ? "older-file" : "selected-file",
    });
  });
  await connect(page);
  await page.getByRole("tab", { name: "Overview" }).click();
  await page.getByRole("button", { name: "Open file file-0" }).click();
  await started;
  await page.getByRole("button", { name: "file-1", exact: true }).click();
  const selected = page.getByRole("region", {
    name: "Source preview for file-1",
  });
  await expect(selected).toContainText("selected-file");
  finish();
  await expect(selected).toContainText("selected-file");
});

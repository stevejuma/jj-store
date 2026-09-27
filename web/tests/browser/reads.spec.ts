import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  api,
  connect,
  graph,
  operation,
  pathsPage,
  repo,
  sample,
  samples,
  token,
} from "./native-browser-fixture.js";
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
  await page.getByRole("link", { name: repo, exact: true }).click();
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
  await page.getByRole("button", { name: "Op Log", exact: true }).click();
  await page
    .getByRole("button", { name: "content fixture", exact: true })
    .click();
  await expect(
    page.getByRole("region", { name: "Changed files in operation" }),
  ).toBeVisible();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  expect(
    await page.evaluate(() => [
      localStorage.length,
      sessionStorage.length,
      document.cookie,
    ]),
  ).toEqual([0, 0, ""]);
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
  await expect(
    page.getByLabel("Repository token", { exact: true }),
  ).toHaveValue("");
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
  await expect(page.getByText(operation.slice(0, 12))).toBeVisible();
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
  await page.getByLabel("Repository ID", { exact: true }).fill(repo);
  await page.getByLabel("Repository token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Open repository" }).click();
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
  await expect(
    page.getByLabel("Repository token", { exact: true }),
  ).toHaveValue("");
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
    result.data.data.nodes = result.data.data.nodes.slice(
      queries.length - 1,
      queries.length,
    );
    result.next_cursor = queries.length === 1 ? "graph-page-2" : null;
    if (queries.length > 1) result.snapshot.manifest_revision++;
    await route.fulfill({ json: result });
  });
  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  const revisionGraph = page.getByRole("list", { name: "Revision graph" });
  await expect(revisionGraph.getByRole("listitem")).toHaveCount(1);
  await page.getByRole("button", { name: "More history" }).click();
  await page.getByRole("button", { name: "Toggle details" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  expect(queries[1].get("revset")).toBe("all()");
  expect(queries[1].get("at_time_millis")).toBe(
    queries[0].get("at_time_millis"),
  );
  expect(queries[1].get("cursor")).toBe("graph-page-2");
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
  await page.getByRole("link", { name: repo, exact: true }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "More files", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  await expect(page.getByRole("button", { name: "Open file bin" })).toHaveCount(
    1,
  );
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
    page.getByRole("button", { name: "Prepare index" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Prepare index" }).click();
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
  await page.getByRole("link", { name: repo, exact: true }).click();
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

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const samples = JSON.parse(
  readFileSync(
    fileURLToPath(
      new URL("../../../.scratch/web-responses.json", import.meta.url),
    ),
    "utf8",
  ),
);
const sample = (path: string) =>
  structuredClone(
    samples.find(
      (s: any) => s.path === `/v1/repos/{repo}/${path}` && s.status === 200,
    ).data,
  );
const snapshot = sample("snapshot");
const repo = snapshot.repo_id;
const operation = snapshot.heads[0];
const token = "disposable-browser-fixture-token";
const graph = sample("log");
const commit = graph.data.data.nodes.find((n: any) =>
  n.commit.description.startsWith("binary"),
).commit.id;
async function api(
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
async function connect(page: Page) {
  await page.goto("/");
  await page.getByLabel("Repository ID", { exact: true }).fill(repo);
  await page.getByLabel("Repository token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Open repository" }).click();
}
test("native_graphs_files_and_source_rendering_are_accessible", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  await api(page);
  await connect(page);
  await expect(
    page.getByRole("button", { name: /binary <script>/ }).first(),
  ).toBeVisible();
  await page
    .getByRole("button", { name: /binary <script>/ })
    .first()
    .click();
  await expect(page.getByRole("heading", { name: "Files" })).toBeVisible();
  await page.getByRole("button", { name: "bin file", exact: true }).click();
  await expect(page.locator("pre.source")).toContainText(
    "<script>window.sourceExecuted=true</script>",
  );
  expect(
    await page.evaluate(() => Reflect.get(window, "sourceExecuted")),
  ).toBeUndefined();
  await page.getByRole("button", { name: "Show diff", exact: true }).click();
  await expect(page.getByText("0 changed paths")).toBeVisible();
  await page
    .getByRole("button", { name: "View evolution", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Evolution", exact: true }),
  ).toHaveAttribute("aria-pressed", "true");
  await expect(page.locator(".graph-rows li").first()).toBeVisible();
  await page.getByRole("button", { name: "Operations", exact: true }).click();
  await page.locator(".graph-row").first().click();
  await expect(
    page.getByRole("heading", { name: "Operation changes" }),
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
  await page.getByRole("button", { name: "Disconnect", exact: true }).click();
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
    page.getByText("Multiple operation heads.", { exact: true }),
  ).toBeVisible();
  expect(requests).not.toContain("log");
  await page
    .getByRole("button", { name: operation.slice(0, 12), exact: true })
    .click();
  await expect(page.locator(".graph-row").first()).toBeVisible();
  await expect(page.getByText(/Newer history is available/)).toBeVisible();
  await expect(
    page.getByLabel("Selected operation", { exact: true }),
  ).toHaveValue(operation);
});
test("disconnect_discards_inflight_history_and_revocation_clears_data", async ({
  page,
}) => {
  await api(page, { delayLog: true });
  await connect(page);
  await page.getByRole("button", { name: "Disconnect", exact: true }).click();
  await page.waitForTimeout(400);
  await expect(page.locator(".graph-row")).toHaveCount(0);
  await page.getByLabel("Repository token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Open repository" }).click();
  await expect(page.locator(".graph-row").first()).toBeVisible();
  await page.route("**/snapshot", (route) =>
    route.fulfill({ status: 403, json: { version: 1, code: "access_denied" } }),
  );
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect(
    page.getByRole("heading", { name: "Explore native history." }),
  ).toBeVisible();
  await expect(page.locator(".graph-row")).toHaveCount(0);
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
  await expect(page.locator(".graph-row").first()).toBeVisible();
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
  await expect(page.locator(".graph-row")).toHaveCount(1);
  await page.getByLabel("Revset", { exact: true }).fill("none()");
  await page
    .getByLabel("Selected operation", { exact: true })
    .fill("ff".repeat(64));
  await expect(
    page.getByRole("region", { name: "Snapshot state" }).locator("code"),
  ).toHaveAttribute("title", operation);
  await page
    .getByRole("button", { name: "Load more at this snapshot" })
    .click();
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  expect(queries[1].get("revset")).toBe(queries[0].get("revset"));
  expect(queries[1].get("at_time_millis")).toBe(
    queries[0].get("at_time_millis"),
  );
  expect(queries[1].get("cursor")).toBe("graph-page-2");
  await expect(page.locator(".graph-row")).toHaveCount(1);
  let pages = 0;
  await page.route("**/tree?**", async (route) => {
    const result = sample("commits/{commit}/tree");
    if (pages++) {
      result.snapshot.manifest_revision++;
      result.next_cursor = null;
    }
    await route.fulfill({ json: result });
  });
  await page.locator(".graph-row").first().click();
  await expect(page.locator(".file-list li")).toHaveCount(2);
  await page.getByRole("button", { name: "Dismiss", exact: true }).click();
  await page.getByRole("button", { name: "More files", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText(
    "pagination snapshot mismatch",
  );
  await expect(page.locator(".file-list li")).toHaveCount(2);
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
  await expect(
    page.getByRole("button", { name: "Prepare index" }),
  ).toBeVisible();
  await page.getByLabel("Revset", { exact: true }).fill("none()");
  await page.getByRole("button", { name: "Prepare index" }).click();
  await expect(page.locator(".graph-row").first()).toBeVisible();
  expect(queries).toHaveLength(2);
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
  await expect(page.locator(".graph-row").first()).toBeVisible();
  await expect(page.getByText("Refresh needed", { exact: true })).toBeVisible();
  expect(resumes).toEqual([null, id]);
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect.poll(() => resumes.length).toBeGreaterThanOrEqual(3);
  expect(resumes[2]).toBe(null);
  await expect(
    page.getByLabel("Selected operation", { exact: true }),
  ).toHaveValue(operation);
  await expect(page.locator(".graph-row").first()).toBeVisible();
});

test("new_file_selection_discards_an_older_inflight_preview", async ({
  page,
}) => {
  await api(page);
  const fileA = "ab".repeat(64),
    fileB = "cd".repeat(64);
  await page.route("**/tree?**", async (route) => {
    const result = sample("commits/{commit}/tree");
    result.data.entries = [fileA, fileB].map((id, i) => ({
      ...result.data.entries[0],
      name: `file-${i}`,
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
  await page
    .getByRole("button", { name: /binary <script>/ })
    .first()
    .click();
  await page.getByRole("button", { name: "file-0 file", exact: true }).click();
  await started;
  await page.getByRole("button", { name: "file-1 file", exact: true }).click();
  await expect(page.locator("pre.source")).toHaveText("selected-file");
  finish();
  await expect(page.locator("pre.source")).toHaveText("selected-file");
});

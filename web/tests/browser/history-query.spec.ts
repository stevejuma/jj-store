import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, repo, operation, graph } from "./native-browser-fixture.js";

// Served production assets enforce STRICT_CSP. Transport replays remain pinned fixtures.
test("cold native history search highlights, completes and retains invalid drafts under CSP", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const requests: URL[] = [];
  await page.route("**/log?**", async (route) => {
    const url = new URL(route.request().url());
    requests.push(url);
    const query = url.searchParams.get("revset");
    if (query === 'description("') {
      await route.fulfill({
        status: 400,
        json: {
          version: 1,
          code: "invalid_query",
          diagnostic: {
            kind: "syntax",
            message: "Close the quoted description.",
          },
        },
      });
      return;
    }
    const value = structuredClone(graph);
    if (query === 'description("binary")')
      value.data.data.nodes = value.data.data.nodes.filter((node: any) =>
        node.commit.description.includes("binary"),
      );
    await route.fulfill({ json: value });
  });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const response = await page.goto(`/#/repos/${repo}/log?at_op=${operation}`);
  expect(response?.headers()["content-security-policy"]).toContain(
    "default-src 'none'",
  );
  expect(response?.headers()["content-security-policy"]).not.toContain(
    "unsafe-inline",
  );
  const input = page.getByRole("searchbox", { name: "Filter JJ history" });
  await expect(input).toBeVisible();
  await input.fill("mut");
  await expect(
    page.getByRole("option", { name: "mutable() alias", exact: true }),
  ).toBeVisible();
  await input.press("Enter");
  await expect(input).toHaveText("mutable()");
  await expect(page.locator(".history-query-alias")).toBeVisible();
  await input.fill('description("binary")');
  await page.getByRole("button", { name: "Run history query" }).click();
  await expect
    .poll(() =>
      new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get(
        "history_query",
      ),
    )
    .toBe('description("binary")');
  const applied = new URL(page.url()).hash;
  await expect(page.locator(".history-query-function")).toBeVisible();
  await expect(page.locator(".history-query-string")).toBeVisible();
  await input.fill('description("');
  await page.getByRole("button", { name: "Run history query" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "Showing previous results. Close the quoted description.",
  );
  await expect(input).toHaveAttribute("aria-invalid", "true");
  expect(new URL(page.url()).hash).toBe(applied);
  expect(
    requests.every((url) => url.searchParams.get("at_op") === operation),
  ).toBe(true);
  const times = requests.map((url) => url.searchParams.get("at_time_millis"));
  expect(new Set(times).size).toBe(1);
  await page.getByRole("button", { name: "Clear search" }).click();
  await expect(input.locator(".cm-placeholder")).toHaveText(
    'mutable() & description("fix")',
  );
  await expect(page.getByRole("button", { name: "Clear search" })).toHaveCount(
    0,
  );
  await expect
    .poll(() =>
      new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get(
        "history_query",
      ),
    )
    .toBeNull();
  await page.getByRole("tab", { name: "Op Log", exact: true }).click();
  const ops = page.getByRole("searchbox", { name: "Filter operation history" });
  await expect(ops).toBeVisible();
  await ops.fill('from:@-- description:"fixture"');
  await page.getByRole("button", { name: "Run history query" }).click();
  await expect
    .poll(() => new URL(page.url()).hash)
    .toContain("history_query=from");
  await expect(page.locator(".history-query-field").first()).toBeVisible();
  await page.getByText("Query context and syntax", { exact: true }).click();
  await expect(page.getByText(/Dates use YYYY-MM-DD/)).toBeVisible();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  expect(errors).toEqual([]);
});

test("cold local-folder history uses pinned custom grammars and local query transport", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const profile = "git_sha1_jj045",
    metadata = "44".repeat(20);
  await page.route("**/v1/repositories?**", (route) =>
    route.fulfill({
      json: {
        version: 1,
        generation: 1,
        repositories: [
          {
            id: repo,
            name: "Local history",
            profile,
            availability: "ready",
            permission: "admin",
          },
        ],
        next_cursor: null,
      },
    }),
  );
  const calls: any[] = [];
  await page.route(`**/v1/local/repos/${repo}/**`, async (route) => {
    const path = new URL(route.request().url()).pathname.split(
      `/local/repos/${repo}/`,
    )[1];
    expect(route.request().headers()["authorization"]).toBeUndefined();
    if (path === "capabilities")
      return route.fulfill({
        json: {
          version: 1,
          profile,
          code_browse: true,
          operation_diff: true,
          work: false,
          agents: false,
          reviews: false,
          search: false,
        },
      });
    if (path === "snapshot")
      return route.fulfill({
        json: {
          version: 1,
          profile,
          repository: repo,
          generation: 1,
          heads: [operation],
          metadata_commit: metadata,
          actor: "55".repeat(16),
        },
      });
    if (path === "events")
      return route.fulfill({
        headers: { "content-type": "text/event-stream" },
        body: "",
      });
    if (path !== "query")
      return route.fulfill({
        status: 404,
        json: { version: 1, code: "fixture_missing" },
      });
    const body = route.request().postDataJSON();
    calls.push(body);
    expect(body.operation).toBe(operation);
    const kind = body.request.kind;
    const snapshot = {
      version: 1,
      profile,
      repository: repo,
      generation: 1,
      operation,
    };
    const stats = {
      object_reads: 0,
      object_bytes: 0,
      work: 0,
      metadata_nodes: 0,
      predicate_file_bytes: 0,
      predicate_lines: 0,
    };
    let data: any;
    if (kind === "query_info")
      data = {
        version: 1,
        jj_version: "0.45.1",
        max_query_bytes: 4096,
        callables: [
          {
            name: "mutable",
            signature: "mutable()",
            insert_text: "mutable()",
            description: "Standard alias",
            kind: "alias",
          },
        ],
        aliases: ["mutable()"],
        operators: ["::", "&"],
        operation_fields: ["description", "user", "from"],
        unsupported: ["files", "at_operation"],
      };
    else if (kind === "references")
      data = {
        version: 1,
        operation,
        references: [],
        previous: null,
        next: null,
        stats,
      };
    else
      data = {
        version: 1,
        operation,
        data: {
          graph: body.request.query[0].graph === "log" ? "log" : "operations",
          nodes: [],
        },
        next: null,
        stats,
      };
    await route.fulfill({
      json: {
        snapshot,
        data: { kind, data },
        next_cursor: null,
        previous_cursor: null,
      },
    });
  });
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}`);
  const input = page.getByRole("searchbox", { name: "Filter JJ history" });
  await expect(input).toBeVisible();
  await input.fill("mutable()");
  await page.getByRole("button", { name: "Run history query" }).click();
  await expect
    .poll(() =>
      calls.some(
        (call) =>
          call.request.kind === "graph" &&
          call.request.query[0].revset === "mutable()",
      ),
    )
    .toBe(true);
  await expect(page.locator(".history-query-alias")).toBeVisible();
  await page.getByText("Query context and syntax", { exact: true }).click();
  await expect(
    page.getByText("Unavailable for this repository: files, at_operation."),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Op Log", exact: true }).click();
  const ops = page.getByRole("searchbox", { name: "Filter operation history" });
  await ops.fill("from:@-- user:steve");
  await page.getByRole("button", { name: "Run history query" }).click();
  await expect
    .poll(() =>
      calls.some(
        (call) =>
          call.request.kind === "graph" &&
          call.request.query[0].graph === "operations_filtered" &&
          call.request.query[0].query === "from:@-- user:steve",
      ),
    )
    .toBe(true);
  await expect(page.locator(".history-query-field").first()).toBeVisible();
});

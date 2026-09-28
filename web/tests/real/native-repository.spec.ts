import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const repository = required("JJ_STORE_REAL_REPOSITORY");
const token = required("JJ_STORE_REAL_TOKEN");
const runtime = required("JJ_STORE_REAL_RUNTIME");
const root = required("JJ_STORE_REAL_ROOT");
const publication = "External browser acceptance publication";

function required(name: string) {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is required`);
  return value;
}

function routeQuery(url: string) {
  const query = new URL(url).hash.split("?", 2)[1] ?? "";
  return new URLSearchParams(query);
}

function publishFromExternalCli() {
  const checkout = join(runtime, "repository");
  writeFileSync(
    join(checkout, "EXTERNAL.md"),
    "# External publication\n\nPublished after the browser captured its snapshot.\n",
  );
  execFileSync(
    join(root, "target", "debug", "jj"),
    [
      "-R",
      checkout,
      "--no-pager",
      "--color=never",
      "describe",
      "-m",
      publication,
    ],
    {
      env: {
        ...process.env,
        JJ_CONFIG: join(runtime, "jj.toml"),
        JJ_STORE_TOKEN: token,
      },
      stdio: "pipe",
    },
  );
}

async function connect(page: import("@playwright/test").Page) {
  await page.goto("/");
  await page.getByLabel("Repository ID", { exact: true }).fill(repository);
  await page.getByLabel("Repository token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Open repository" }).click();
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
}

test("native Tantivy search is snapshot-bound and leaves the route unchanged when closed", async ({
  page,
}) => {
  await connect(page);
  const routeBeforeSearch = new URL(page.url()).hash;
  const selectedOperation = routeQuery(page.url()).get("at_op");
  expect(selectedOperation).toMatch(/^[0-9a-f]{128}$/u);

  await page.getByRole("button", { name: "Search repository" }).click();
  const dialog = page.getByRole("dialog", { name: "Search repository" });
  const responsePromise = page.waitForResponse((response) => {
    const url = new URL(response.url());
    return (
      url.pathname.endsWith(`/v1/repos/${repository}/search`) &&
      url.searchParams.get("q") === "browser"
    );
  });
  await dialog
    .getByRole("combobox", { name: "Search repository" })
    .fill("browser");
  await expect(
    dialog.getByRole("option", { name: /Add repository browser/u }),
  ).toBeVisible();

  const response = await responsePromise;
  expect(response.request().headers()["authorization"]).toBe(`Bearer ${token}`);
  const body = await response.json();
  expect(body.repository).toBe(repository);
  expect(body.selected_view?.operation_id).toBe(selectedOperation);
  expect(body.results).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        key: expect.objectContaining({ kind: "commit" }),
      }),
      expect.objectContaining({
        key: expect.objectContaining({ kind: "file", path: "src/lib.rs" }),
      }),
    ]),
  );

  await dialog.getByRole("button", { name: "Close search" }).click();
  await expect(dialog).not.toBeVisible();
  expect(new URL(page.url()).hash).toBe(routeBeforeSearch);
});

test("external publication waits for explicit refresh and preserves selection", async ({
  page,
}) => {
  const eventRequest = page.waitForRequest((request) =>
    new URL(request.url()).pathname.endsWith(`/v1/repos/${repository}/events`),
  );
  await connect(page);
  expect((await eventRequest).headers()["authorization"]).toBe(
    `Bearer ${token}`,
  );
  const main = page.getByRole("option", { name: "main", exact: true });
  await expect(main).toBeVisible();
  await main.click();
  await page
    .getByRole("button", { name: "Add repository browser", exact: true })
    .click();

  const selectedHash = new URL(page.url()).hash;
  const selected = routeQuery(page.url());
  expect(selected.get("bookmark")).toBe("main");
  expect(selected.get("commit")).toMatch(/^[0-9a-f]{128}$/u);
  expect(selected.get("change")).toMatch(/^[0-9a-f]{32}$/u);
  const oldOperation = selected.get("at_op");
  expect(oldOperation).toMatch(/^[0-9a-f]{128}$/u);

  publishFromExternalCli();
  const refresh = page.getByRole("button", {
    name: "Refresh snapshot (new history available)",
  });
  await expect(refresh).toBeVisible();
  expect(new URL(page.url()).hash).toBe(selectedHash);

  await refresh.click();
  await expect
    .poll(() => routeQuery(page.url()).get("at_op"))
    .not.toBe(oldOperation);
  await expect(main).toBeVisible();
  const refreshed = routeQuery(page.url());
  expect(refreshed.get("bookmark")).toBe(selected.get("bookmark"));
  expect(refreshed.get("commit")).toBe(selected.get("commit"));
  expect(refreshed.get("change")).toBe(selected.get("change"));

  await page.getByRole("button", { name: "JJ Log", exact: true }).click();
  await expect(
    page.getByRole("button", { name: publication, exact: true }),
  ).toBeVisible();
  expect(page.url()).not.toContain(token);
  expect(
    await page.evaluate(() => JSON.stringify({ ...localStorage })),
  ).not.toContain(token);
});

test("native conflict selection previews and publishes one atomic resolution", async ({
  page,
}) => {
  const requests: any[] = [];
  page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (path.endsWith("/mutations/preview") || path.endsWith("/mutations"))
      requests.push(request.postDataJSON());
  });
  await connect(page);
  await page
    .getByRole("option", { name: "conflict/demo", exact: true })
    .click();
  await page
    .getByRole("button", { name: "Unresolved browser conflict", exact: true })
    .click();
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Resolve conflicts" }).click();
  const dialog = page.getByRole("dialog", { name: "Resolve native conflicts" });
  await expect(
    dialog.getByRole("heading", { name: "conflict.txt", exact: true }),
  ).toBeVisible();
  await dialog.getByRole("button", { name: /^Use .*Conflict left/u }).click();
  await dialog.getByRole("button", { name: "Preview 1/1" }).click();
  await expect(dialog.getByText("Preview prepared")).toBeVisible();
  await expect(
    dialog.locator(".store-mutation-preview dd").filter({
      hasText: "conflict.txt",
    }),
  ).toBeVisible();
  await dialog.getByRole("button", { name: "Apply resolution" }).click();
  const confirmation = page.getByRole("alertdialog", {
    name: "Apply atomic conflict resolution?",
  });
  await confirmation.getByRole("button", { name: "Apply resolution" }).click();
  await expect(dialog.getByText("Conflicts resolved")).toBeVisible();

  expect(requests).toHaveLength(2);
  expect(requests[1]).toEqual(requests[0]);
  expect(requests[0].command.type).toBe("resolve_conflicts");
  expect(requests[0].command.paths).toHaveLength(1);
  expect(requests[0].command.paths[0]).toMatchObject({
    path: "conflict.txt",
    resolution: { type: "select_term", term: 0 },
  });
  expect(requests[0].command.paths[0].fingerprint).toMatch(/^[0-9a-f]{128}$/u);
});

test("browser mutation previews and publishes one native request", async ({
  page,
}) => {
  const requests: any[] = [];
  page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (path.endsWith("/mutations/preview") || path.endsWith("/mutations"))
      requests.push(request.postDataJSON());
  });
  await connect(page);
  const previousOperation = routeQuery(page.url()).get("at_op");
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Create change" }).click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  await dialog
    .getByLabel("Description")
    .fill("Create a durable browser mutation");
  await dialog.getByRole("button", { name: "Preview change" }).click();
  await expect(dialog.getByText("Preview prepared")).toBeVisible();
  await dialog.getByRole("button", { name: "Apply command" }).click();
  await expect(dialog.getByText("Repository updated")).toBeVisible();

  expect(requests).toHaveLength(2);
  expect(requests[1]).toEqual(requests[0]);
  expect(requests[0].request_id).toMatch(/^[0-9a-f]{32}$/u);
  expect(requests[0].expected.heads).toEqual([previousOperation]);
  await expect
    .poll(() => routeQuery(page.url()).get("at_op"))
    .not.toBe(previousOperation);
  await dialog
    .getByRole("button", { name: "Close", exact: true })
    .last()
    .click();
  await expect(dialog).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(
        () => getComputedStyle(document.documentElement).pointerEvents,
      ),
    )
    .not.toBe("none");
  await page.getByRole("button", { name: "JJ Log", exact: true }).click();
  await expect(
    page.getByRole("button", {
      name: "Create a durable browser mutation",
      exact: true,
    }),
  ).toBeVisible();
});

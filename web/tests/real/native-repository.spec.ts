import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const repository = required("JJ_STORE_REAL_REPOSITORY");
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
    `# External publication\n\nPublished after the browser captured its snapshot at ${Date.now()}.\n`,
  );
  execFileSync(
    join(root, "target", "debug", "jj-store-jj"),
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
      },
      stdio: "pipe",
    },
  );
  return execFileSync(
    join(root, "target", "debug", "jj-store-jj"),
    [
      "-R",
      checkout,
      "--ignore-working-copy",
      "op",
      "log",
      "--no-graph",
      "--limit",
      "1",
      "-T",
      "id",
    ],
    {
      env: { ...process.env, JJ_CONFIG: join(runtime, "jj.toml") },
      encoding: "utf8",
    },
  ).trim();
}

async function connect(page: import("@playwright/test").Page) {
  await page.goto(`/#/repos/${repository}/branches`);
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
}

test("LocalTrusted bootstrap opens requested repository without browser credentials", async ({
  page,
}) => {
  const bootstrap = page.waitForRequest((request) =>
    new URL(request.url()).pathname.endsWith("/v1/bootstrap"),
  );
  await connect(page);
  expect((await bootstrap).headers()["authorization"]).toBeUndefined();
  expect(new URL(page.url()).hash).toContain(`/repos/${repository}/`);
  expect(page.url()).not.toContain("token");
  expect(
    await page.evaluate(() => JSON.stringify({ ...localStorage })),
  ).not.toContain("jst1_");
});

test("demo history exposes one native divergent change", async ({ page }) => {
  await connect(page);
  await page.getByRole("tab", { name: "Commits", exact: true }).click();

  const alpha = page.getByRole("button", {
    name: "Divergent layout alpha",
    exact: true,
  });
  const beta = page.getByRole("button", {
    name: "Divergent layout beta",
    exact: true,
  });
  await expect(alpha).toBeVisible();
  await expect(beta).toBeVisible();

  const rowFor = (message: typeof alpha) =>
    message.locator(
      "xpath=ancestor::div[contains(@class, 'community-revision-graph__row')]",
    );
  const labelFor = async (message: typeof alpha, kind: "Git" | "JJ") =>
    rowFor(message)
      .getByRole("button", {
        name: new RegExp(
          `^Copy ${kind === "Git" ? "Git commit SHA" : "JJ Change ID"}`,
        ),
      })
      .getAttribute("aria-label");

  expect(await labelFor(alpha, "JJ")).toBe(await labelFor(beta, "JJ"));
  expect(await labelFor(alpha, "Git")).not.toBe(await labelFor(beta, "Git"));
});

test("external publication refreshes automatically and preserves selection", async ({
  page,
}) => {
  const eventRequest = page.waitForRequest(
    (request) => new URL(request.url()).pathname === "/v1/events",
  );
  await connect(page);
  expect((await eventRequest).headers()["authorization"]).toBeUndefined();
  const main = page.getByRole("option", { name: "main", exact: true });
  await expect(main).toBeVisible();
  await main.click();
  await page
    .getByRole("button", { name: "Add repository browser", exact: true })
    .click();

  // The seeded revision has a review association loaded after code selection.
  await expect
    .poll(() => routeQuery(page.url()).get("thread"))
    .toMatch(/^[0-9a-f]{32}$/u);
  await expect(
    page.getByRole("combobox", { name: "Repository workspace", exact: true }),
  ).toBeEnabled({ timeout: 15000 });
  await expect(
    page.getByRole("status").filter({ hasText: "Loading more history" }),
  ).toHaveCount(0, { timeout: 15000 });

  const selected = routeQuery(page.url());
  expect(selected.get("bookmark")).toBe("main");
  expect(selected.get("commit")).toMatch(/^[0-9a-f]{128}$/u);
  expect(selected.get("change")).toMatch(/^[0-9a-f]{32}$/u);
  const oldOperation = selected.get("at_op");
  expect(oldOperation).toMatch(/^[0-9a-f]{128}$/u);

  const publishedOperation = publishFromExternalCli();
  expect(publishedOperation).toMatch(/^[0-9a-f]{128}$/u);
  await expect
    .poll(() => routeQuery(page.url()).get("at_op"))
    .toBe(publishedOperation);
  expect(publishedOperation).not.toBe(oldOperation);
  await expect(main).toBeVisible();
  const refreshed = routeQuery(page.url());
  expect(refreshed.get("bookmark")).toBe(selected.get("bookmark"));
  expect(refreshed.get("commit")).toBe(selected.get("commit"));
  expect(refreshed.get("change")).toBe(selected.get("change"));

  await page.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect(
    page.getByRole("button", { name: publication, exact: true }),
  ).toBeVisible();
  expect(page.url()).not.toContain("token");
  expect(
    await page.evaluate(() => JSON.stringify({ ...localStorage })),
  ).not.toContain("jst1_");
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
  await page.getByRole("button", { name: "Application menu" }).click();
  await page.getByRole("menuitem", { name: "Resolve conflicts" }).click();
  const surface = page.getByRole("region", {
    name: "Resolve native conflicts",
  });
  await expect(
    surface.getByRole("region", { name: "Conflict conflict.txt", exact: true }),
  ).toBeVisible();
  await surface
    .getByRole("button", { name: "Accept All Incoming Changes from Left" })
    .click();
  await surface.getByRole("button", { name: "Preview 1/1" }).click();
  await expect(surface.getByText("Preview prepared")).toBeVisible({
    timeout: 30000,
  });
  await expect(
    surface.locator(".store-mutation-preview dd").filter({
      hasText: "conflict.txt",
    }),
  ).toBeVisible();
  await surface.getByRole("button", { name: "Apply resolution" }).click();
  const confirmation = page.getByRole("alertdialog", {
    name: "Apply atomic conflict resolution?",
  });
  await confirmation.getByRole("button", { name: "Apply resolution" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "Conflicts resolved" }),
  ).toBeVisible({
    timeout: 30000,
  });

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
  const message = `Create a durable browser mutation ${Date.now()}`;
  const requests: any[] = [];
  page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (path.endsWith("/mutations/preview") || path.endsWith("/mutations"))
      requests.push(request.postDataJSON());
  });
  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  await page
    .getByRole("button", { name: "Add repository browser", exact: true })
    .click();
  const previousOperation = routeQuery(page.url()).get("at_op");
  await page.getByRole("button", { name: "Application menu" }).click();
  await page.getByRole("menuitem", { name: "Create change" }).click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  await dialog.getByRole("textbox", { name: "Markdown editor" }).fill(message);
  await dialog.getByRole("button", { name: "Preview change" }).click();
  await expect(dialog.getByText("Preview prepared")).toBeVisible();
  const applied = page.waitForResponse(
    (response) =>
      response.request().method() === "POST" &&
      new URL(response.url()).pathname.endsWith("/mutations") &&
      response.request().postDataJSON().request_id === requests[0].request_id,
  );
  await dialog.getByRole("button", { name: "Apply command" }).click();
  const response = await applied;
  expect(response.ok()).toBe(true);
  expect(await response.json()).toMatchObject({
    status: "committed",
    receipt: { request_id: requests[0].request_id },
  });
  await expect(
    page.getByRole("status").filter({ hasText: "Repository updated" }),
  ).toBeVisible();

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
  await page.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect(
    page.getByRole("button", {
      name: message,
      exact: true,
    }),
  ).toBeVisible();
});

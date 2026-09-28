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

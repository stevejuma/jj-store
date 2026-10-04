import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo, operation } from "./native-browser-fixture.js";
async function setup(page: Page) {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", (route) =>
    route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_repository_conversion: true,
        },
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/local/repositories", (route) =>
    route.fulfill({ json: { version: 1, revision: 0, bindings: [] } }),
  );
}
async function open(page: Page) {
  await page.goto(`/#/repos/${repo}/files`);
  await page.reload();
  await page.getByRole("button", { name: /Open profile menu/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page
    .getByRole("button", {
      name: /Use local Git-backed JJ|Continue conversion/,
    })
    .click();
  return page.getByRole("dialog", { name: "Convert repository" });
}
test("cold export preflight, pending reload, exact retry and verified report", async ({
  page,
}) => {
  await setup(page);
  const writes: any[] = [];
  let accepted: any;
  await page.route("**/v1/local/repositories/conversion/preview", (route) =>
    route.fulfill({
      json: {
        version: 1,
        prepared_source: "55".repeat(32),
        source_bytes: 100,
        history_selections: 3,
        blocker: null,
      },
    }),
  );
  await page.route("**/v1/local/repositories/conversion", async (route) => {
    accepted = route.request().postDataJSON();
    writes.push(accepted);
    await route.fulfill({
      json: { status: "outcome_unknown", request_id: accepted.request_id },
    });
  });
  await page.route("**/v1/local/repositories/conversion/requests/*", (route) =>
    route.fulfill({
      json: {
        status: "committed",
        report: {
          version: 1,
          request_id: accepted.request_id,
          source: repo,
          destination: "33".repeat(16),
          destination_profile: "local_jj_git",
          name: accepted.intent.name,
          operation: "44".repeat(64),
          archive_digest: accepted.prepared_source,
          code_mappings: {
            operations: { [operation]: "44".repeat(64) },
            commits: {},
            views: {},
            trees: {},
            files: {},
            symlinks: {},
          },
          collaboration: { selections: [], objects: {} },
        },
      },
    }),
  );
  let dialog = await open(page);
  await dialog
    .getByRole("textbox", { name: "New folder" })
    .fill("/new/independent");
  await dialog.getByRole("button", { name: "Check conversion" }).click();
  await expect(
    dialog.getByRole("button", { name: "Convert and open destination" }),
  ).toBeEnabled();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await dialog
    .getByRole("button", { name: "Convert and open destination" })
    .click();
  await expect(
    dialog.getByRole("button", { name: "Retry conversion" }),
  ).toBeVisible();
  await page.reload();
  await page.getByRole("button", { name: /Open profile menu/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Continue conversion" }).click();
  dialog = page.getByRole("dialog", { name: "Convert repository" });
  await expect(dialog.getByRole("textbox", { name: "New folder" })).toHaveValue(
    "/new/independent",
  );
  await expect(
    dialog.getByRole("textbox", { name: "New folder" }),
  ).toBeDisabled();
  await dialog.getByRole("button", { name: "Retry conversion" }).click();
  await expect.poll(() => writes.length).toBe(2);
  expect(writes[0]).toEqual(writes[1]);
  await dialog.getByRole("button", { name: "Check status" }).click();
  await expect(dialog).not.toBeVisible();
  await page.goto(`/#/repos/${repo}/files`);
  await page.reload();
  await page.getByRole("button", { name: /Open profile menu/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Last conversion report" }).click();
  const report = page.getByRole("dialog", { name: "Conversion report" });
  await expect(
    report.getByRole("button", { name: "Download mapping report" }),
  ).toBeVisible();
  const download = page.waitForEvent("download");
  await report.getByRole("button", { name: "Download mapping report" }).click();
  expect((await download).suggestedFilename()).toContain(accepted.request_id);
  expect(
    await page.evaluate(() =>
      localStorage.getItem("jj-store.repository-conversion.v1"),
    ),
  ).toBeNull();
});
test("cold unsupported conversion cannot dispatch", async ({ page }) => {
  await setup(page);
  let writes = 0;
  await page.route("**/v1/local/repositories/conversion/preview", (route) =>
    route.fulfill({
      json: {
        version: 1,
        prepared_source: null,
        source_bytes: 0,
        history_selections: 0,
        blocker: "Unsupported signed commit",
      },
    }),
  );
  await page.route("**/v1/local/repositories/conversion", (route) => {
    writes++;
    return route.abort();
  });
  const dialog = await open(page);
  await dialog
    .getByRole("textbox", { name: "New folder" })
    .fill("/new/independent");
  await dialog.getByRole("button", { name: "Check conversion" }).click();
  await expect(
    dialog.getByText("Conversion is unavailable: Unsupported signed commit"),
  ).toBeVisible();
  await expect(
    dialog.getByRole("button", { name: "Convert and open destination" }),
  ).toHaveCount(0);
  expect(writes).toBe(0);
});

test("cold conversion status retries preserve a dismissed error episode", async ({
  page,
}) => {
  await setup(page);
  await page.route("**/v1/local/repositories/conversion/preview", (route) =>
    route.fulfill({
      json: {
        version: 1,
        prepared_source: "55".repeat(32),
        source_bytes: 100,
        history_selections: 3,
        blocker: null,
      },
    }),
  );
  await page.route("**/v1/local/repositories/conversion", (route) =>
    route.fulfill({
      json: {
        status: "outcome_unknown",
        request_id: route.request().postDataJSON().request_id,
      },
    }),
  );
  await page.route("**/v1/local/repositories/conversion/requests/*", (route) =>
    route.fulfill({
      status: 503,
      json: { version: 1, code: "local_folder_unavailable" },
    }),
  );
  const dialog = await open(page);
  await dialog
    .getByRole("textbox", { name: "New folder" })
    .fill("/new/independent");
  await dialog.getByRole("button", { name: "Check conversion" }).click();
  await dialog
    .getByRole("button", { name: "Convert and open destination" })
    .click();
  const status = dialog.getByRole("button", { name: "Check status" });
  await status.click();
  const dismiss = page.getByRole("button", {
    name: "Dismiss Repository conversion failed",
  });
  await dismiss.click();
  const response = page.waitForResponse((r) =>
    r.url().includes("/conversion/requests/"),
  );
  await status.click();
  await response;
  await expect(status).toBeEnabled();
  await expect(dismiss).toHaveCount(0);
  await expect(
    dialog.getByRole("button", { name: "Retry conversion" }),
  ).toBeVisible();
});

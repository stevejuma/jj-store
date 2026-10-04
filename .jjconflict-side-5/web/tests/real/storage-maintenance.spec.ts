import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
const execute = promisify(execFile);
test.skip(
  process.env.JJ_STORE_STORAGE_FIXTURE !== "1",
  "Requires the bounded native fixture from scripts/storage-browser-acceptance.mjs",
);

test("native Storage keeps exact checkpoint targets through cold restart and verifies captured data", async ({
  page,
  request,
}) => {
  test.setTimeout(240000);
  const repo = process.env.JJ_STORE_REAL_REPOSITORY!;
  const origin = new URL(process.env.JJ_STORE_REAL_BASE_URL!).origin;
  const headers = {
    Origin: origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const snapshot = await (
    await request.get(`/v1/repos/${repo}/snapshot`, { headers })
  ).json();
  const route = `/#/repos/${repo}/branches?at_op=${snapshot.heads[0]}`;
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  async function openStorage() {
    await page
      .getByRole("button", { name: "Open profile menu for Local" })
      .click();
    await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
    await page
      .getByRole("navigation", { name: "Settings navigation" })
      .getByRole("button", { name: "Storage", exact: true })
      .click();
    await expect(page.getByText("Selected manifest is readable")).toBeVisible();
    await page.getByRole("tab", { name: "Maintenance", exact: true }).click();
  }
  async function restart() {
    await execute(
      process.execPath,
      ["scripts/storage-browser-acceptance.mjs", "restart"],
      {
        cwd: process.env.JJ_STORE_REAL_ROOT!,
        env: {
          ...process.env,
          JJ_STORE_DEV_ROOT: process.env.JJ_STORE_REAL_RUNTIME!,
          JJ_STORE_DEV_PORT: new URL(origin).port,
        },
        maxBuffer: 16 * 1024 * 1024,
      },
    );
  }
  await page.goto(route);
  await openStorage();
  const hash = new URL(page.url()).hash;
  await page
    .getByRole("button", { name: "Prepare checkpoint", exact: true })
    .click();
  await expect(
    page.getByRole("button", { name: "Publish checkpoint", exact: true }),
  ).toBeVisible();
  const prepared = (
    await (
      await request.get(`/v1/repos/${repo}/local/storage`, { headers })
    ).json()
  ).latest;
  expect(prepared.phase).toBe("prepared");
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Storage", exact: true }).click();
  await page.getByRole("tab", { name: "Maintenance", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Publish checkpoint", exact: true }),
  ).toBeVisible();
  await restart();
  await page.reload();
  await openStorage();
  const resumed = (
    await (
      await request.get(`/v1/repos/${repo}/local/storage`, { headers })
    ).json()
  ).latest;
  expect(resumed.request_id).toBe(prepared.request_id);
  expect(resumed.target).toEqual(prepared.target);
  await page
    .getByRole("button", { name: "Publish checkpoint", exact: true })
    .click();
  await expect(
    page.getByText("The exact captured checkpoint is selected."),
  ).toBeVisible({ timeout: 30000 });
  const after = await (
    await request.get(`/v1/repos/${repo}/snapshot`, { headers })
  ).json();
  expect(after.heads).toEqual(snapshot.heads);
  expect(after.published_seq).toBe(snapshot.published_seq);
  await page
    .getByRole("button", { name: "Check integrity", exact: true })
    .click();
  await expect(page.getByText("Captured data verified")).toBeVisible({
    timeout: 60000,
  });
  expect(new URL(page.url()).hash).toBe(hash);
  await restart();
  const persisted = await (
    await request.get(
      `/v1/repos/${repo}/local/storage/operations/${prepared.request_id}`,
      { headers },
    )
  ).json();
  expect(persisted.selection).toBe("selected");
  expect(persisted.target).toEqual(prepared.target);
  await page.reload();
  await openStorage();
  await expect(page.getByText("Captured data verified")).toBeVisible();
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await page.reload();
  await page
    .getByRole("button", { name: "Open navigation sidebar", exact: true })
    .click();
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page
    .getByRole("button", { name: "Open settings sidebar", exact: true })
    .click();
  await page
    .getByRole("navigation", { name: "Settings navigation" })
    .getByRole("button", { name: "Storage", exact: true })
    .click();
  await page.getByRole("tab", { name: "Maintenance", exact: true }).click();
  await expect(page.getByText("Captured data verified")).toBeVisible();
  expect(new URL(page.url()).hash).toBe(hash);
  expect(errors).toEqual([]);
});

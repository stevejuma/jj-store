import { test, expect } from "@playwright/test";
import { api, repo, operation, graph } from "./native-browser-fixture.js";

test("cold history navigation cancels a query and retries identical text", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const repeated = 'description("retry")';
  let calls = 0;
  let release!: () => void;
  const held = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route("**/log?**", async (route) => {
    const query = new URL(route.request().url()).searchParams.get("revset");
    if (query === repeated && ++calls === 1) await held;
    await route.fulfill({ json: graph }).catch(() => {});
  });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}`);
  await page
    .getByRole("button", { name: "Search JJ Log", exact: true })
    .click();
  const input = page.getByRole("searchbox", { name: "Filter JJ history" });
  const run = page.getByRole("button", { name: "Run history query" });
  await input.fill(repeated);
  await run.click();
  await expect.poll(() => calls).toBe(1);
  await input.fill('description("replacement")');
  await run.click();
  await expect(input).toHaveAttribute("aria-invalid", "false");
  await input.fill(repeated);
  await run.click();
  await expect.poll(() => calls).toBe(2);
  await expect(page.locator(".store-history-query")).toHaveAttribute(
    "aria-busy",
    "false",
  );
  await expect(
    page
      .locator('[data-ui-component="community-projects-virtual-rows"]')
      .first(),
  ).toBeVisible();
  release();
  expect(errors).toEqual([]);
});

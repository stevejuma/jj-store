import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  api,
  operation,
  operationChangesPage,
  repo,
  sample,
} from "./native-browser-fixture.js";
const first = "aa".repeat(64),
  second = "bb".repeat(64);

test("comparison parent dropdown searches descriptions and sends the full recorded identity", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.context().grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.route(`**/v1/repos/${repo}/operations?*`, async (route) => {
    const value = sample("operations");
    const node = value.data.data.nodes[0];
    node.parents = [first, second];
    for (const [id, description] of [
      [first, "Before native import"],
      [second, "After native import"],
    ]) {
      value.data.data.nodes.push({
        ...structuredClone(node),
        id,
        description,
        parents: [],
      });
    }
    await route.fulfill({ json: value });
  });
  await page.route(
    `**/v1/repos/${repo}/operations/${operation}`,
    async (route) => {
      const value = sample("operations/{operation}");
      value.data.operation.data.data.parents = [first, second];
      await route.fulfill({ json: value });
    },
  );
  const selections: string[] = [];
  await page.route(
    `**/v1/repos/${repo}/operations/${operation}/diff*`,
    async (route) => {
      const parent = new URL(route.request().url()).searchParams.get("parent");
      const value = sample("operations/{operation}/diff");
      if (parent) {
        selections.push(parent);
        value.data.data.parent = parent;
      } else
        value.data.data = {
          status: "select_parent",
          operation,
          parents: [first, second],
        };
      await route.fulfill({ json: value });
    },
  );
  await page.route(
    `**/v1/repos/${repo}/operations/${operation}/changes?*`,
    async (route) => {
      const value = operationChangesPage([]);
      value.data.parent = new URL(route.request().url()).searchParams.get(
        "parent",
      );
      await route.fulfill({ json: value });
    },
  );
  await page.goto(
    `/#/repos/${repo}/oplog?at_op=${operation}&operation=${operation}&workspace=desktop`,
  );
  const header = page.locator(".store-application__section-header");
  const picker = header.getByRole("combobox", {
    name: "Comparison parent",
    exact: true,
  });
  await expect(picker).toBeVisible();
  await expect(
    page.getByText("This merge operation has multiple recorded predecessors."),
  ).toHaveCount(0);
  for (const width of [1280, 390]) {
    await page.setViewportSize({ width, height: 800 });
    const comparison = await picker.boundingBox();
    const workspace = await header
      .getByRole("combobox", { name: "Repository workspace" })
      .boundingBox();
    const bounds = await header.boundingBox();
    expect(comparison!.x + comparison!.width).toBeLessThanOrEqual(workspace!.x);
    expect(workspace!.x + workspace!.width).toBeLessThanOrEqual(
      bounds!.x + bounds!.width,
    );
  }
  await picker.click();
  const copy = page
    .getByRole("grid", { name: "Recorded parents" })
    .getByRole("button", {
      name: `Copy full operation ID: ${second}`,
      exact: true,
    });
  await copy.click();
  await expect(copy).toHaveAttribute("data-copied", "true");
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(
    second,
  );
  await copy.press("Enter");
  await expect(picker).toHaveAttribute("aria-expanded", "true");
  expect(selections).toEqual([]);
  expect(new URL(page.url()).hash).not.toContain("parent=");
  const accessibility = await new AxeBuilder({ page })
    .include(".store-comparison-parent-picker__command")
    .analyze();
  expect(accessibility.violations).toEqual([]);
  const search = page.getByRole("combobox", {
    name: "Search comparison parents",
    exact: true,
  });
  await search.fill("After native import");
  await expect(
    page.getByRole("row", { name: /Before native import/ }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("row", { name: /After native import/ }),
  ).toBeVisible();
  await search.press("ArrowDown");
  await search.press("Enter");
  await expect(picker).toHaveCount(0);
  expect(selections).toEqual([second]);
  const hash = new URL(page.url()).hash;
  expect(hash).toContain(`parent=${second}`);
  expect(hash).toContain(`at_op=${operation}`);
  expect(hash).toContain("workspace=desktop");
  await page.reload();
  await expect(
    page
      .getByRole("region", { name: "Selected operation", exact: true })
      .getByRole("button", { name: "content fixture", exact: true }),
  ).toBeVisible();
  await expect(picker).toHaveCount(0);
  expect(new URL(page.url()).hash).toContain(`parent=${second}`);
});

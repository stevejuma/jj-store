import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, repo, sample } from "./native-browser-fixture.js";

test("local trusted bootstrap auto-opens the sole repository without credentials", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", async (route) => {
    const headers = route.request().headers();
    expect(headers["x-jj-store-protocol"]).toBe("1");
    expect(headers["authorization"]).toBeUndefined();
    await route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: sample("capabilities"),
        storage: { ready: true },
        repositories: [{ id: repo, name: "Local acceptance" }],
        backups: [],
      },
    });
  });

  await page.goto("/");
  await expect(
    page.getByRole("button", { name: "Open profile menu for Local" }),
  ).toBeVisible();
  await expect(page.getByRole("option", { name: "main" })).toBeVisible();
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
});

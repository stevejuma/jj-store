import { syncWorker } from "../../../catalog/fixtures/sync-settings.js";
import { repositorySettings } from "./repository-settings-navigation.js";
import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo } from "./native-browser-fixture.js";
import { serverSettingsValue } from "../../../catalog/fixtures/server-settings.js";
import { searchSettingsValue } from "../../../catalog/fixtures/search-settings.js";
import { decode, pretty } from "../../src/lib/api/client.js";
import type { SaveServerSettings } from "../../src/lib/controller/server-configuration.js";
async function setup(page: Page) {
  await api(page, { access: "local_trusted" });
  const capabilities = {
    ...sample("capabilities"),
    local_settings_management: true,
    local_search_management: true,
    local_sync_management: true,
  };
  await page.route("**/v1/bootstrap", (r) =>
    r.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities,
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/repos/*/capabilities", (r) =>
    r.fulfill({ json: capabilities }),
  );
  await page.route("**/v1/local/repositories", (r) =>
    r.fulfill({ json: { version: 1, bindings: [] } }),
  );
  let value = serverSettingsValue();
  const saves: SaveServerSettings[] = [];
  await page.route("**/v1/local/settings", async (r) => {
    expect(r.request().headers()["authorization"]).toBeUndefined();
    expect(r.request().headers()["x-jj-store-protocol"]).toBe("1");
    if (r.request().method() === "POST") {
      const body = decode(r.request().postData()!) as SaveServerSettings;
      saves.push(body);
      expect(body.expected_revision).toBe(value.revision);
      value = {
        ...value,
        revision: (saves.length + 10).toString(16).repeat(64),
        desired: {
          ...value.desired,
          [body.update.section]: body.update.configuration,
        },
        restart_required: true,
      };
    }
    await r.fulfill({ contentType: "application/json", body: pretty(value) });
  });
  await page.route("**/v1/repos/*/local/sync", (r) =>
    r.fulfill({ json: { version: 1, configured: true, worker: syncWorker() } }),
  );
  let search = searchSettingsValue();
  await page.route("**/v1/repos/*/local/search**", (r) => {
    if (r.request().method() === "POST") {
      const command = decode(r.request().postData()!) as any;
      if (command.configuration)
        search = { ...search, desired: command.configuration };
      else search = { ...search, effective: structuredClone(search.desired) };
    }
    return r.fulfill({ contentType: "application/json", body: pretty(search) });
  });
  await page.goto(`/#/repos/${repo}/files`);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  return { saves };
}
async function section(page: Page, name: string, tab: string) {
  const service =
    name === "Local service" ||
    (name === "Repository" && tab === "Protection") ||
    name === "Storage";
  if (service) {
    const button = page.getByRole("button", {
      name: "Local service",
      exact: true,
    });
    await expect(async () => {
      if (!(await button.isVisible())) {
        const back = page.getByRole("button", {
          name: "Back to repository",
          exact: true,
        });
        if (await back.isVisible()) {
          await back.click({ timeout: 1000 });
          await page
            .getByRole("button", {
              name: "Open profile menu for Local",
              exact: true,
            })
            .click({ timeout: 1000 });
          await page
            .getByRole("menuitem", { name: "Settings", exact: true })
            .click({ timeout: 1000 });
        }
      }
      await expect(button).toBeVisible({ timeout: 1000 });
    }).toPass({ timeout: 10000 });
    await button.click();
  } else await repositorySettings(page, name);
  await page
    .getByRole("navigation", { name: "Settings page navigation", exact: true })
    .getByRole("tab", {
      name: name === "Storage" ? "Storage" : tab,
      exact: true,
    })
    .click();
}

test("cold service protection and sync editors retain drafts and saved/running state", async ({
  page,
}) => {
  const f = await setup(page);
  await section(page, "Local service", "Limits");
  const input = page.getByRole("spinbutton", { name: "Concurrent requests" });
  await input.fill("3");
  await page.getByRole("tab", { name: "Launch", exact: true }).click();
  await expect(page.getByText("127.0.0.1:7472", { exact: true })).toBeVisible();
  await page.getByRole("tab", { name: "Limits", exact: true }).click();
  await expect(input).toHaveValue("3");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  await expect(
    page.getByText("Saved changes need a configuration reload."),
  ).toBeVisible();
  expect(f.saves[0].update.section).toBe("service");
  await section(page, "Repository", "Protection");
  await page
    .getByRole("textbox", { name: "Protected bookmarks" })
    .fill("main\nrelease");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  expect(f.saves[1].update.section).toBe("protection");
  await section(page, "Sync", "Configuration");
  await page.getByRole("button", { name: "Add upstream" }).click();
  await page
    .getByRole("textbox", { name: "Upstream origin" })
    .fill("https://upstream.example/");
  await expect(
    page.getByRole("spinbutton", { name: "Origins per batch" }),
  ).toHaveValue("128");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  expect(f.saves[2].update.section).toBe("sync");
  await section(page, "Storage", "Configuration");
  await expect(
    page.getByText("/private/example/rustfs", { exact: true }),
  ).toBeVisible();
});
test("cold Search advanced controls use explicit Apply and validate extraction limits", async ({
  page,
}) => {
  await setup(page);
  await section(page, "Search", "Advanced");
  await page.getByRole("spinbutton", { name: "Polling interval" }).fill("2500");
  await page
    .getByRole("spinbutton", { name: "Maximum text file size" })
    .fill("500000");
  await page
    .getByRole("button", { name: "Save settings", exact: true })
    .click();
  await expect(page.getByText(/Saved changes need applying/)).toBeVisible();
  await page.getByRole("button", { name: "Apply saved settings" }).click();
  await expect(page.getByText(/Matches saved settings/)).toBeVisible();
});
test("configuration controls pass scoped accessibility and mobile overflow checks", async ({
  page,
}) => {
  await setup(page);
  await section(page, "Local service", "Limits");
  const scope = page.locator(".store-settings");
  expect(
    (await new AxeBuilder({ page }).include(".store-settings").analyze())
      .violations,
  ).toEqual([]);
  await page.setViewportSize({ width: 390, height: 844 });
  expect(await scope.evaluate((e) => e.scrollWidth <= e.clientWidth)).toBe(
    true,
  );
  await page.setViewportSize({ width: 1400, height: 1000 });
  await section(page, "Repository", "Protection");
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    (await new AxeBuilder({ page }).include(".store-settings").analyze())
      .violations,
  ).toEqual([]);
  await page.setViewportSize({ width: 1400, height: 1000 });
  await section(page, "Sync", "Configuration");
  await page.getByRole("button", { name: "Add upstream" }).click();
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    (await new AxeBuilder({ page }).include(".store-settings").analyze())
      .violations,
  ).toEqual([]);
});

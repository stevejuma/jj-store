import { expect, test, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

async function settled(page: Page) {
  await page.evaluate(async () => {
    await document.fonts.ready;
    await new Promise<void>((resolve) =>
      requestAnimationFrame(() => requestAnimationFrame(() => resolve())),
    );
  });
}

async function expectLayout(page: Page, mobile: boolean) {
  for (const row of await page
    .locator(".store-settings__action-row:visible")
    .all()) {
    const description = row.locator(".store-settings-footer__description");
    const actions = row.locator(".store-settings__actions");
    if (!(await description.count())) {
      const bounds = (await row.boundingBox())!,
        right = (await actions.boundingBox())!;
      expect(
        bounds.x + bounds.width - right.x - right.width,
      ).toBeLessThanOrEqual(18);
      continue;
    }
    await expect(description.locator("p")).toBeVisible();
    const left = (await description.boundingBox())!;
    const right = (await actions.boundingBox())!;
    if (mobile) expect(right.y).toBeGreaterThanOrEqual(left.y + left.height);
    else expect(left.x + left.width).toBeLessThanOrEqual(right.x);
  }
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  await settled(page);
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
}

test("local settings use shared rows, right controls and icon availability states", async ({
  page,
}) => {
  test.setTimeout(120000);
  const home = process.env.JJ_STORE_REAL_REPOSITORY!;
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  for (const mobile of [false, true]) {
    await page.setViewportSize(
      mobile ? { width: 390, height: 844 } : { width: 1351, height: 988 },
    );
    await page.goto(`/#/repos/${home}/branches`);
    // Settings is transient; a same-hash navigation can retain the mounted shell.
    if (mobile) await page.reload();
    if (mobile)
      await page
        .getByRole("button", { name: "Open navigation sidebar", exact: true })
        .click();
    await page
      .getByRole("button", { name: "Open profile menu for Local" })
      .click();
    await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
    await expect(
      page.getByRole("heading", { name: "Local service", level: 1 }),
    ).toBeVisible();
    const folders = page.getByRole("region", { name: "Git-backed JJ folders" });
    await expect(
      folders.getByRole("button", { name: "Refresh folders" }),
    ).toBeEnabled();
    await folders.getByRole("button", { name: "Refresh folders" }).click();
    await expect(
      folders.getByRole("button", { name: "Refresh folders" }),
    ).toBeEnabled();
    const registrations = folders.getByRole("button", {
      name: "Reconnect folder",
    });
    if (await registrations.count()) {
      await expect(registrations.first()).toBeVisible();
      await expect(
        folders.getByRole("button", { name: "Remove registration" }).first(),
      ).toBeVisible();
    } else
      await expect(
        folders.locator('[data-slot="empty-media"] svg'),
      ).toBeVisible();
    await folders
      .getByRole("button", { name: "Add folder", exact: true })
      .click();
    const dialog = page.getByRole("dialog", { name: "Add local JJ folder" });
    await expect(
      dialog.getByRole("textbox", { name: "Folder", exact: true }),
    ).toBeVisible();
    await dialog.getByRole("button", { name: "Cancel", exact: true }).click();
    await expectLayout(page, mobile);
    await page.screenshot({
      path: `/tmp/jj-settings-rows-service-${mobile ? "mobile" : "desktop"}.png`,
    });

    async function select(name: string) {
      await page.goto(
        `/#/repos/${home}/settings?section=${name.toLowerCase()}`,
      );
    }
    await select("Search");
    const enabled = page.getByRole("switch", { name: "Enable text search" });
    await expect(enabled).toBeVisible();
    const enabledLabel = (await page
      .locator('label[for="search-enabled"]')
      .boundingBox())!;
    const enabledControl = (await enabled.boundingBox())!;
    expect(enabledLabel.x + enabledLabel.width).toBeLessThan(enabledControl.x);
    const rows = page.locator(".store-settings--repository:visible");
    await expect(
      rows.getByText("Currently active", { exact: true }),
    ).toBeVisible();
    const budget = page.getByRole("spinbutton", {
      name: "Index budget",
      exact: true,
    });
    await expect(
      rows.getByRole("spinbutton", { name: "Index budget", exact: true }),
    ).toBeVisible();
    const previous = await enabled.isChecked();
    await enabled.click();
    await expect(
      page.getByRole("button", { name: "Save settings" }),
    ).toBeEnabled();
    await page.getByRole("button", { name: "Cancel changes" }).click();
    expect(await enabled.isChecked()).toBe(previous);
    const originalBudget = await budget.inputValue();
    await budget.fill("256");
    await page.getByRole("button", { name: "Cancel changes" }).click();
    await expect(budget).toHaveValue(originalBudget);
    await expectLayout(page, mobile);
    await page.screenshot({
      path: `/tmp/jj-settings-rows-search-${mobile ? "mobile" : "desktop"}.png`,
    });
    await page.getByRole("tab", { name: "File scopes", exact: true }).click();
    await expect(
      page.getByRole("textbox", { name: "Bookmark scopes" }),
    ).toBeVisible();
    await expectLayout(page, mobile);

    await select("Sync");
    const unavailable = page.getByText(
      "Sync management is not available for this connection.",
      { exact: true },
    );
    if (await unavailable.count()) {
      await expect(
        unavailable
          .locator('xpath=ancestor::*[@data-slot="empty"]')
          .locator('[data-slot="empty-media"] svg'),
      ).toBeVisible();
    } else {
      await expect(
        page.getByRole("button", { name: "Refresh settings", exact: true }),
      ).toBeEnabled();
      await page
        .getByRole("button", { name: "Refresh settings", exact: true })
        .click();
      await expect(
        page.locator(".store-settings__rows:visible").first(),
      ).toBeVisible();
      const workStore = page.getByText("WorkStore sync is not supported.", {
        exact: true,
      });
      if (await workStore.count()) {
        await expect(
          workStore
            .locator('xpath=ancestor::*[@data-slot="empty"]')
            .locator('[data-slot="empty-media"] svg'),
        ).toBeVisible();
        await expect(
          page.getByRole("button", { name: "Pause sync" }),
        ).toHaveCount(0);
      }
    }
    await expectLayout(page, mobile);
    await page.screenshot({
      path: `/tmp/jj-settings-rows-sync-${mobile ? "mobile" : "desktop"}.png`,
    });
  }
  expect(errors).toEqual([]);
});

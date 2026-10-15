import { expect, type Page } from "@playwright/test";

/** Use the repository-owned primary tabs, retaining same-owner commands. */
export async function repositorySettings(page: Page, section: string) {
  const navigation = page.getByRole("navigation", {
    name: "Repository settings navigation",
    exact: true,
  });
  await expect(async () => {
    if (!(await navigation.isVisible())) {
      const globalBack = page.getByRole("button", {
        name: "Back to jj-store",
        exact: true,
      });
      if (await globalBack.isVisible())
        await globalBack.click({ timeout: 1000 });
      if (!(await navigation.isVisible()))
        await page
          .getByRole("button", { name: "Repository settings", exact: true })
          .click({ timeout: 1000 });
    }
    await expect(navigation).toBeVisible({ timeout: 1000 });
  }).toPass({ timeout: 10000 });
  await page.evaluate(
    () =>
      new Promise<void>((resolve) =>
        requestAnimationFrame(() => requestAnimationFrame(() => resolve())),
      ),
  );
  const name = section === "Repository" ? "General" : section;
  await expect(async () => {
    const tab = navigation.getByRole("tab", { name, exact: true });
    if (await tab.isVisible()) {
      if ((await tab.getAttribute("aria-selected")) !== "true") {
        await page.keyboard.press("Escape");
        await tab.click({ timeout: 1000 });
      }
    } else {
      await navigation
        .getByRole("button", {
          name: "More repository settings categories",
          exact: true,
        })
        .click({ timeout: 1000 });
      await page
        .getByRole("menuitem", { name, exact: true })
        .click({ timeout: 1000 });
    }
  }).toPass({ timeout: 10000 });
  await expect(page).toHaveURL(/\/settings\?/);
}

export async function repositorySettingsLayout(
  page: Page,
  tablistName?: string,
) {
  const active = page.locator(".store-repository-settings__page:visible");
  await expect(
    active.locator(".store-settings-card:visible").first(),
  ).toBeVisible();
  await expect(
    page.getByRole("combobox", { name: "Settings repository" }),
  ).toHaveCount(0);
  const navigation = active.getByRole("navigation", {
    name: "Settings page navigation",
  });
  const tabs = tablistName
    ? navigation.getByRole("tablist", { name: tablistName })
    : null;
  if (tabs) await expect(tabs).toBeVisible();
  // Cards may be replaced as the selected category's initial read completes.
  // Retry the complete measurement; never reuse a box from the previous page.
  await expect(async () => {
    const bounds = await active.boundingBox();
    expect(bounds).not.toBeNull();
    const cards = await active.locator(".store-settings-card:visible").all();
    expect(cards.length).toBeGreaterThan(0);
    for (const card of cards) {
      const cardBounds = await card.boundingBox();
      expect(cardBounds).not.toBeNull();
      expect(cardBounds!.x).toBeGreaterThanOrEqual(bounds!.x - 1);
      expect(cardBounds!.x + cardBounds!.width).toBeLessThanOrEqual(
        bounds!.x + bounds!.width + 1,
      );
    }
    if (tabs) {
      const tabBounds = await tabs.boundingBox();
      expect(tabBounds).not.toBeNull();
      expect(tabBounds!.y - bounds!.y).toBeLessThan(16);
    }
  }).toPass({ timeout: 5000 });
  await expect(navigation.getByRole("heading")).toHaveCount(0);
  if (tabs) {
    await expect(tabs.getByRole("tab", { selected: true })).toHaveCSS(
      "font-weight",
      "600",
    );
    return tabs;
  }
  return page.getByRole("tablist", { name: "Repository settings categories" });
}

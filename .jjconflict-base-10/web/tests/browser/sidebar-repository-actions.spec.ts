import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api } from "./native-browser-fixture.js";

test("sidebar add actions open the matching existing dialogs without writes", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const writes: string[] = [];
  page.on("request", (request) => {
    if (
      request.method() !== "GET" &&
      /\/(?:local\/repositories|work\/commands)/.test(request.url())
    )
      writes.push(request.url());
  });
  await page.route("**/v1/local/repositories", async (route) => {
    await route.fulfill({ json: { version: 1, revision: 0, bindings: [] } });
  });
  await page.goto("/");
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  const before = page.url();
  const actions = page.getByRole("button", {
    name: "Add repository",
    exact: true,
  });
  await expect(actions).toHaveCount(1);
  await expect(
    page
      .locator(".lc-community-navigation__section-header")
      .getByRole("button", { name: "Browse threads", exact: true }),
  ).toHaveCount(1);
  for (const label of ["Add project", "Add repository"]) {
    const project = label === "Add project";
    const action = page.getByRole("button", { name: label, exact: true });
    const header = action.locator("..");
    const toggle = header.getByRole("button").first();
    const expanded = await toggle.getAttribute("aria-expanded");
    await action.click();
    if (!project)
      await page
        .getByRole("menuitem", { name: "Add Local Repository", exact: true })
        .click();
    const dialog = page.getByRole("dialog", {
      name: project ? "Create project" : "Add local JJ folder",
    });
    await expect(dialog).toBeVisible();
    if (!project) {
      await expect(
        dialog.getByRole("button", { name: "Add folder", exact: true }),
      ).toBeDisabled();
      await expect(
        dialog.getByRole("button", { name: "Inspect folder" }),
      ).toBeVisible();
    } else
      await expect(
        dialog.getByRole("textbox", { name: "Project title" }),
      ).toBeVisible();
    expect(page.url()).toBe(before);
    expect(writes).toEqual([]);
    await dialog
      .getByRole("button", { name: project ? "Done" : "Cancel", exact: true })
      .click();
    await expect(dialog).not.toBeVisible();
    await expect(toggle).toHaveAttribute("aria-expanded", expanded!);
  }
  await page.getByRole("button", { name: "Collapse left sidebar" }).click();
  for (const name of ["Your projects", "Repositories"]) {
    const project = name === "Your projects";
    const label = project ? "Add project" : "Add Local Repository";
    const trigger = page.getByRole("button", { name, exact: true });
    await trigger.click();
    const action = page.getByRole("option", {
      name: label,
      exact: true,
    });
    await expect(action).toBeVisible();
    await page
      .getByRole("combobox", { name: `${name} navigation` })
      .fill(label);
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog", {
      name: project ? "Create project" : "Add local JJ folder",
    });
    await expect(dialog).toBeVisible();
    await expect(action).not.toBeVisible();
    await expect(dialog.locator(":focus")).toHaveCount(1);
    await dialog
      .getByRole("button", { name: project ? "Done" : "Cancel", exact: true })
      .click();
    await expect(dialog).not.toBeVisible();
    expect(page.url()).toBe(before);
    expect(writes).toEqual([]);
  }
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
});

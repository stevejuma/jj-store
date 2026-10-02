import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

// Reads the running UI; every registration/picker request is intercepted. This
// must never register a folder or mutate the persistent development fixture.
for (const mobile of [false, true]) {
  test(`project repository menu and folder dialog ${mobile ? "mobile" : "desktop"}`, async ({
    page,
  }) => {
    await page.setViewportSize(
      mobile ? { width: 390, height: 844 } : { width: 1351, height: 988 },
    );
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    const requests: Record<string, unknown>[] = [];
    let inspected = false;
    await page.route("**/v1/local/repositories{,/**}", async (route) => {
      const request = route.request(),
        pathname = new URL(request.url()).pathname;
      if (request.method() === "GET" && pathname === "/v1/local/repositories") {
        await route.fulfill({
          json: { version: 1, revision: 7, bindings: [] },
        });
      } else if (pathname.endsWith("/pick")) {
        await route.fulfill({
          json: { version: 1, path: "/mocked/jj-folder" },
        });
      } else if (pathname.endsWith("/inspect")) {
        inspected = true;
        await route.fulfill({
          json: {
            path: "/mocked/jj-folder",
            metadata: "missing",
            repository: null,
          },
        });
      } else if (
        request.method() === "POST" &&
        pathname === "/v1/local/repositories"
      ) {
        expect(inspected).toBe(true);
        expect(request.headers()["authorization"]).toBeUndefined();
        expect(request.headers()["x-jj-store-protocol"]).toBe("1");
        const body = request.postDataJSON();
        requests.push(body);
        await route.fulfill({
          json:
            requests.length === 1
              ? { status: "outcome_unknown", request_id: body.request_id }
              : {
                  status: "committed",
                  request_id: body.request_id,
                  revision: 8,
                  binding: {
                    repository: "ab".repeat(16),
                    name: body.name,
                    path: body.path,
                    generation: 1,
                    availability: "ready",
                  },
                },
        });
      } else {
        throw new Error(
          `Unexpected mocked registration request: ${request.method()} ${pathname}`,
        );
      }
    });
    const home = process.env.JJ_STORE_REAL_REPOSITORY,
      project = process.env.JJ_STORE_REAL_PROJECT;
    await page.goto(
      home && project
        ? `/#/projects?home=${home}&project=${project}&tab=repositories`
        : "/#/projects?tab=repositories",
    );
    const view = page.getByRole("region", {
      name: "Projects and tasks",
      exact: true,
    });
    if (home && project)
      await expect(
        view.getByRole("button", { name: "Link repository", exact: true }),
      ).toBeEnabled();
    const add = view.getByRole("button", { name: "Add", exact: true });
    await expect(add).toBeEnabled();
    await expect(
      view.getByRole("button", { name: "Add local repository", exact: true }),
    ).toHaveCount(0);
    await add.press("Enter");
    const repository = page.getByRole("menuitem", {
      name: "Add local repository",
      exact: true,
    });
    await expect(repository.locator('svg[aria-hidden="true"]')).toBeVisible();
    if (home && project) {
      await expect(
        page.getByRole("menuitem", { name: "Add task", exact: true }),
      ).toBeVisible();
      await expect(
        view.getByRole("combobox", { name: "Repository", exact: true }),
      ).toHaveCount(0);
      await expect(
        view
          .getByRole("button", { name: "Link repository", exact: true })
          .locator("svg"),
      ).toBeVisible();
    }
    await repository.click();
    const dialog = page.getByRole("dialog", {
      name: "Add local JJ folder",
      exact: true,
    });
    await expect(dialog).toBeVisible();
    await expect
      .poll(() =>
        dialog.evaluate((element) => element.contains(document.activeElement)),
      )
      .toBe(true);
    const rows = dialog.locator(".community-settings__option-row");
    await expect(rows).toHaveCount(3);
    await expect(
      dialog.getByRole("button", { name: "Add folder", exact: true }),
    ).toBeDisabled();
    await dialog.getByRole("button", { name: "Browse", exact: true }).click();
    const folder = dialog.getByRole("textbox", { name: "Folder", exact: true });
    await expect(folder).toHaveValue("/mocked/jj-folder");
    await dialog
      .getByRole("textbox", { name: "Name", exact: true })
      .fill("Mocked repository");
    await dialog
      .getByRole("button", { name: "Inspect folder", exact: true })
      .click();
    await expect(
      dialog.getByRole("button", { name: "Add folder", exact: true }),
    ).toBeEnabled();
    // Public Button transitions opacity when inspection completes. Audit the
    // settled enabled controls, retaining the normal contrast rule.
    await dialog.evaluate(async (element) => {
      await document.fonts.ready;
      await new Promise<void>((resolve) =>
        requestAnimationFrame(() => requestAnimationFrame(() => resolve())),
      );
      await Promise.all(
        element
          .getAnimations({ subtree: true })
          .filter(
            (animation) =>
              animation.effect?.getTiming().iterations !== Infinity,
          )
          .map((animation) => animation.finished.catch(() => {})),
      );
    });
    await expect
      .poll(() =>
        dialog.evaluate(
          (element) => element.scrollWidth <= element.clientWidth + 1,
        ),
      )
      .toBe(true);
    expect(
      (
        await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
          .analyze()
      ).violations,
    ).toEqual([]);
    await page.screenshot({
      path: `/tmp/jj-project-repository-folder-${mobile ? "mobile" : "desktop"}.png`,
    });
    await dialog
      .getByRole("button", { name: "Add folder", exact: true })
      .click();
    await expect(
      dialog
        .getByRole("status")
        .filter({ hasText: "Registration outcome is unknown" }),
    ).toContainText("Registration outcome is unknown");
    await expect(folder).toBeDisabled();
    await expect(folder).toHaveValue("/mocked/jj-folder");
    await dialog.getByRole("button", { name: "Cancel", exact: true }).click();
    await expect(add).toBeFocused();
    await add.click();
    await page
      .getByRole("menuitem", { name: "Add local repository", exact: true })
      .click();
    await expect(folder).toHaveValue("/mocked/jj-folder");
    await expect(
      dialog.getByRole("textbox", { name: "Name", exact: true }),
    ).toHaveValue("Mocked repository");
    await dialog
      .getByRole("button", { name: "Retry Add", exact: true })
      .click();
    await expect(dialog).toBeHidden();
    expect(requests).toHaveLength(2);
    expect(requests[1]).toEqual(requests[0]);
    await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
    expect(errors).toEqual([]);
  });
}

test("settings registration and reconnect use the shared dialog rows", async ({
  page,
}) => {
  let writes = 0;
  await page.route("**/v1/local/repositories{,/**}", async (route) => {
    if (route.request().method() !== "GET") {
      writes++;
      throw new Error(
        "Settings presentation test must not submit registration changes",
      );
    }
    await route.fulfill({
      json: {
        version: 1,
        revision: 7,
        bindings: [
          {
            repository: "ab".repeat(16),
            name: "Mocked JJ folder",
            path: "/mocked/jj-folder",
            generation: 1,
            availability: "ready",
          },
        ],
      },
    });
  });
  await page.goto("/#/projects?tab=repositories");
  await page
    .getByRole("button", { name: "Open profile menu for Local", exact: true })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  const folders = page.getByRole("region", { name: "Git-backed JJ folders" });
  await expect(
    folders.getByRole("button", { name: "Reconnect folder" }),
  ).toBeEnabled();
  await folders
    .getByRole("button", { name: "Add folder", exact: true })
    .click();
  const add = page.getByRole("dialog", {
    name: "Add local JJ folder",
    exact: true,
  });
  await expect(add.locator(".community-settings__option-row")).toHaveCount(3);
  await add.getByRole("button", { name: "Cancel", exact: true }).click();
  await folders.getByRole("button", { name: "Reconnect folder" }).click();
  const reconnect = page.getByRole("dialog", {
    name: "Reconnect local folder",
    exact: true,
  });
  await expect(
    reconnect.locator(".community-settings__option-row"),
  ).toHaveCount(2);
  await expect(
    reconnect.getByRole("textbox", { name: "Name", exact: true }),
  ).toHaveCount(0);
  await expect(
    reconnect.getByRole("button", { name: "Reconnect", exact: true }),
  ).toBeDisabled();
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await reconnect.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  expect(writes).toBe(0);
});

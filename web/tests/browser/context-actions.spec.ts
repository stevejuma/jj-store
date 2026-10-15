import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  operation,
  repo,
  sample,
} from "./native-browser-fixture.js";

test("selected JJ Log change restores rail commands and clears them on navigation", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
  await page.getByRole("button", { name: "Toggle details" }).click();
  const rail = page.getByRole("complementary", { name: "Repository details" });
  const actions = rail.getByRole("group", {
    name: "Selected repository actions",
  });
  await expect(
    actions.getByRole("button", { name: "Edit description" }),
  ).toBeEnabled();
  await expect(
    actions.getByRole("button", { name: "Create bookmark" }),
  ).toBeEnabled();
  await expect(
    actions.getByRole("button", { name: "Abandon change" }),
  ).toBeEnabled();
  // The details rail persists its open state. Wait for that write before
  // reloading so this checks restoration rather than racing persistence.
  await expect
    .poll(() =>
      page.evaluate(() => {
        const saved = localStorage.getItem(
          "jj-store/connection/local/jj-store/layout/v1/shell",
        );
        return saved
          ? JSON.parse(saved).panels?.["repository-details"]?.closed
          : undefined;
      }),
    )
    .toBe(false);
  await page.reload();
  await expect(
    page.getByRole("button", { name: "Toggle details" }),
  ).toHaveAttribute("aria-expanded", "true");
  await expect(actions).toBeVisible();
  await actions.getByRole("button", { name: "Edit description" }).click();
  const dialog = page.getByRole("dialog", { name: "Edit description" });
  await expect(
    dialog.getByRole("button", {
      name: `Copy Git commit SHA ${commit}`,
      exact: true,
    }),
  ).toBeVisible();
  await dialog.getByRole("button", { name: "Cancel" }).click();
  await rail.getByRole("button", { name: "Op Log", exact: true }).click();
  await expect(actions).toHaveCount(0);
});

for (const multi of [true, false]) {
  test(`selected Op Log operation shows commands with ${multi ? "multiple" : "one"} head`, async ({
    page,
  }) => {
    await api(page, { access: "local_trusted", multi });
    await page.goto(
      `/#/repos/${repo}/oplog?at_op=${operation}&operation=${operation}`,
    );
    await page.getByRole("button", { name: "Toggle details" }).click();
    const rail = page.getByRole("complementary", {
      name: "Repository details",
    });
    const actions = rail.getByRole("group", {
      name: "Selected repository actions",
    });
    await expect(
      actions.getByRole("button", { name: "Create change" }),
    ).toBeEnabled();
    await expect(
      actions.getByRole("button", { name: "Edit description" }),
    ).toHaveCount(0);
    const reconcile = actions.getByRole("button", {
      name: "Reconcile operations",
    });
    if (multi) {
      await expect(reconcile).toBeEnabled();
      await reconcile.click();
      const dialog = page.getByRole("dialog", { name: "Reconcile operations" });
      await expect(dialog).toBeVisible();
      await dialog.getByRole("button", { name: "Cancel" }).click();
    } else await expect(reconcile).toHaveCount(0);
    await rail.getByRole("button", { name: "JJ Log", exact: true }).click();
    await expect(actions).toHaveCount(0);
  });
}

test("selected change rail commands respect unavailable capabilities", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route(`**/v1/repos/${repo}/capabilities*`, async (route) => {
    const capabilities = sample("capabilities");
    capabilities.typed_mutations = [];
    await route.fulfill({ json: capabilities });
  });
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
  await page.getByRole("button", { name: "Toggle details" }).click();
  const rail = page.getByRole("complementary", { name: "Repository details" });
  const actions = rail.getByRole("group", {
    name: "Selected repository actions",
  });
  await expect(
    actions.getByRole("button", { name: "Evo Log", exact: true }),
  ).toBeEnabled();
  for (const name of [
    "Edit description",
    "Create bookmark",
    "Resolve conflicts",
    "Abandon change",
    "Create change",
  ])
    await expect(actions.getByRole("button", { name })).toBeDisabled();
});

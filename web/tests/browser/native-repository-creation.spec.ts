import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo } from "./native-browser-fixture.js";

test("cold native creation recovers exact intent across entrypoints and reload", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", async (route) =>
    route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_native_repository_creation: true,
        },
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/local/repositories", async (route) =>
    route.fulfill({ json: { version: 1, revision: 0, bindings: [] } }),
  );
  const requests: unknown[] = [];
  let pending: any,
    committed = false;
  await page.route("**/v1/local/repositories/native", async (route) => {
    pending = route.request().postDataJSON();
    requests.push(pending);
    await route.fulfill({
      json: { status: "outcome_unknown", request_id: pending.request_id },
    });
  });
  await page.route(
    "**/v1/local/repositories/native/requests/*",
    async (route) => {
      expect(route.request().url()).toContain(pending.request_id);
      committed = true;
      await route.fulfill({
        json: {
          status: "committed",
          request_id: pending.request_id,
          repository: repo,
          name: pending.name,
        },
      });
    },
  );
  await page.goto("/");
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  await page
    .getByRole("button", { name: "Create server repository", exact: true })
    .click();
  let dialog = page.getByRole("dialog", { name: "Create server repository" });
  await expect(
    dialog.getByRole("button", { name: "Create repository", exact: true }),
  ).toBeDisabled();
  await dialog
    .getByRole("textbox", { name: "Name", exact: true })
    .fill("Independent");
  const submit = dialog.getByRole("button", {
    name: "Create repository",
    exact: true,
  });
  await expect(submit).toBeEnabled();
  await expect
    .poll(() => submit.evaluate((node) => getComputedStyle(node).opacity))
    .toBe("1");
  expect(await new AxeBuilder({ page }).analyze()).toMatchObject({
    violations: [],
  });
  await dialog
    .getByRole("button", { name: "Create repository", exact: true })
    .click();
  await expect(
    dialog.getByRole("button", { name: "Retry creation" }),
  ).toBeVisible();
  await expect(
    dialog.getByRole("textbox", { name: "Name", exact: true }),
  ).toBeDisabled();
  await dialog
    .getByRole("button", { name: "Close", exact: true })
    .last()
    .click();
  await page.reload();
  await page
    .getByRole("button", { name: "Create server repository", exact: true })
    .click();
  dialog = page.getByRole("dialog", { name: "Create server repository" });
  await expect(
    dialog.getByRole("textbox", { name: "Name", exact: true }),
  ).toHaveValue("Independent");
  await dialog.getByRole("button", { name: "Retry creation" }).click();
  await expect.poll(() => requests.length).toBe(2);
  expect(requests[1]).toEqual(requests[0]);
  await dialog.getByRole("button", { name: "Check status" }).click();
  await expect(dialog).not.toBeVisible();
  expect(committed).toBe(true);
  await expect
    .poll(() =>
      page.evaluate(() =>
        localStorage.getItem("jj-store.native-repository-creation.v1"),
      ),
    )
    .toBeNull();
});

test("cold server creation stays unavailable without installation capability", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.goto("/");
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  await expect(
    page.getByRole("button", { name: "Create server repository", exact: true }),
  ).toHaveCount(0);
});

test("cold native creation keeps name rejection inline and permits a corrected request", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", async (route) =>
    route.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_native_repository_creation: true,
        },
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  const writes: any[] = [];
  await page.route("**/v1/local/repositories/native", async (route) => {
    const request = route.request().postDataJSON();
    writes.push(request);
    await route.fulfill(
      request.name === "Taken"
        ? { status: 409, json: { code: "local_registration_conflict" } }
        : {
            json: { status: "outcome_unknown", request_id: request.request_id },
          },
    );
  });
  await page.goto("/");
  await page.getByRole("button", { name: "Expand left sidebar" }).click();
  await page
    .getByRole("button", { name: "Create server repository", exact: true })
    .click();
  const dialog = page.getByRole("dialog", { name: "Create server repository" });
  const name = dialog.getByRole("textbox", { name: "Name", exact: true });
  await name.fill("Taken");
  await dialog
    .getByRole("button", { name: "Create repository", exact: true })
    .click();
  await expect(
    dialog.getByText("This name is already in use. Choose a different name."),
  ).toBeVisible();
  await expect(name).toHaveAttribute("aria-invalid", "true");
  await expect(name).toBeEnabled();
  await name.fill("Corrected");
  await dialog
    .getByRole("button", { name: "Create repository", exact: true })
    .click();
  await expect(
    dialog.getByRole("button", { name: "Retry creation" }),
  ).toBeVisible();
  expect(writes[0].request_id).not.toBe(writes[1].request_id);
  expect(writes[1].name).toBe("Corrected");
});

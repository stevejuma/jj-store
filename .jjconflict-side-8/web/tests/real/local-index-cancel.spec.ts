import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { readFileSync } from "node:fs";

const fixture = process.env.JJ_STORE_LOCAL_BROWSER_FIXTURE
  ? JSON.parse(readFileSync(process.env.JJ_STORE_LOCAL_BROWSER_FIXTURE, "utf8"))
  : null;
test.skip(!fixture, "owned local browser fixture required");

test("local preparation recovery and settings cancel the captured task", async ({
  page,
}) => {
  let pin: Record<string, unknown>;
  let state = "running";
  let generation: number;
  let starts = 0;
  let cancels = 0;
  let polls = 0;
  const id = "ab".repeat(16);
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const root = `**/v1/local/repos/${fixture.repository}`;
  await page.route(`${root}/snapshot`, async (route) => {
    const response = await route.fetch();
    generation = (await response.json()).generation;
    await route.fulfill({ response });
  });
  await page.route(`${root}/query`, async (route) => {
    const body = route.request().postDataJSON();
    if (
      body.request?.kind !== "graph" ||
      body.request.query?.[0]?.graph !== "log"
    ) {
      await route.continue();
      return;
    }
    pin = {
      version: 1,
      profile: "git_sha1_jj045",
      repository: fixture.repository,
      generation,
      operation: body.operation,
    };
    await route.fulfill({
      status: 409,
      json: { version: 1, code: "index_rebuild_required", local_snapshot: pin },
    });
  });
  await page.route(`${root}/index/tasks**`, async (route) => {
    const request = route.request();
    if (request.url().endsWith("/cancel")) {
      cancels++;
      expect(request.method()).toBe("POST");
      expect(request.postData()).toBeNull();
      state = "canceling";
    } else if (request.method() === "POST") {
      starts++;
      expect(request.postDataJSON().operation).toBe(pin.operation);
      state = "running";
    } else polls++;
    await route.fulfill({
      json: { version: 1, id, snapshot: pin, status: state },
    });
  });
  await page.goto(
    `/#/repos/${fixture.repository}/log?at_op=${fixture.operation}`,
  );
  await page
    .getByRole("button", { name: "Prepare graph index", exact: true })
    .click();
  const cancel = page.getByRole("button", {
    name: "Cancel preparation",
    exact: true,
  });
  await expect(cancel).toBeEnabled();
  await cancel.focus();
  await page.keyboard.press("Enter");
  await expect(cancel).toBeDisabled();
  await expect(
    page
      .getByRole("status")
      .filter({ hasText: "Canceling graph index preparation" }),
  ).toBeVisible();
  expect(starts).toBe(1);
  expect(cancels).toBe(1);
  state = "canceled";
  await expect(cancel).toHaveCount(0);

  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("tab", { name: "Advanced", exact: true }).click();
  await page
    .getByRole("button", { name: "Prepare graph index", exact: true })
    .click();
  await expect(cancel).toBeEnabled();
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await cancel.click();
  await expect(
    page.getByRole("button", { name: "Canceling…", exact: true }),
  ).toBeDisabled();
  expect(starts).toBe(2);
  expect(cancels).toBe(2);
  state = "canceled";
  await expect(
    page.getByText("Graph index preparation canceled.", { exact: true }),
  ).toBeVisible();
  expect(polls).toBeGreaterThan(0);
  expect(errors).toEqual([]);
});

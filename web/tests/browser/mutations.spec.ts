import { expect, test } from "@playwright/test";
import {
  api,
  connect,
  graph,
  mutationSample,
} from "./native-browser-fixture.js";

test("repository action menu unlocks the page", async ({ page }) => {
  await api(page);
  await connect(page);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.keyboard.press("Escape");
  await expect(page.getByRole("menu")).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => ({
        overflow: document.body.style.overflow,
        pointerEvents: document.body.style.pointerEvents,
      })),
    )
    .toEqual({ overflow: "", pointerEvents: "" });
});

test("repository command dialog cancel unlocks the page", async ({ page }) => {
  await api(page);
  await connect(page);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Create change" }).click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  await dialog.getByRole("button", { name: "Cancel" }).click();
  await expect(dialog).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => ({
        overflow: document.body.style.overflow,
        pointerEvents: document.body.style.pointerEvents,
      })),
    )
    .toEqual({ overflow: "", pointerEvents: "" });
});

test("repository command preview and apply retain the captured head and request", async ({
  page,
}) => {
  const requests = await api(page);
  const bodies: any[] = [];
  page.on("request", (request) => {
    if (
      request.url().endsWith("/mutations/preview") ||
      request.url().endsWith("/mutations")
    )
      bodies.push(request.postDataJSON());
  });
  await connect(page);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Create change" }).click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  await dialog.getByLabel("Description").fill("Created from the browser");
  await dialog.getByRole("button", { name: "Preview change" }).click();
  await expect(dialog.getByText("Preview prepared")).toBeVisible();
  await dialog.getByRole("button", { name: "Apply command" }).click();
  await expect(dialog.getByText("Repository updated")).toBeVisible();

  expect(bodies).toHaveLength(2);
  expect(bodies[1]).toEqual(bodies[0]);
  expect(bodies[0].request_id).toMatch(/^[0-9a-f]{32}$/u);
  expect(bodies[0].expected).toEqual({
    epoch: graph.snapshot.epoch,
    position: graph.snapshot.published_seq,
    heads: graph.snapshot.heads,
  });
  expect(bodies[0].command).toMatchObject({
    type: "new",
    description: "Created from the browser",
  });
  await expect
    .poll(() => requests.filter((path) => path === "snapshot").length)
    .toBe(2);
  await dialog
    .getByRole("button", { name: "Close", exact: true })
    .last()
    .click();
  await expect(dialog).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => ({
        overflow: document.body.style.overflow,
        pointerEvents: document.body.style.pointerEvents,
      })),
    )
    .toEqual({ overflow: "", pointerEvents: "" });
  await page.getByRole("button", { name: "JJ Log", exact: true }).click();
  await expect(
    page.getByRole("list", { name: "Revision graph" }),
  ).toBeVisible();
});

test("unknown mutation outcome is recovered through durable status", async ({
  page,
}) => {
  await api(page);
  let requestId = "";
  await page.route("**/mutations", async (route) => {
    requestId = route.request().postDataJSON().request_id;
    const value = mutationSample("mutations", "outcome_unknown");
    value.request_id = requestId;
    await route.fulfill({ json: value });
  });
  await page.route("**/mutations/*", async (route) => {
    if (route.request().url().endsWith("/mutations/preview")) {
      await route.fallback();
      return;
    }
    const value = mutationSample("mutations/{request_id}", "committed");
    value.receipt.request_id = requestId;
    await route.fulfill({ json: value });
  });
  await connect(page);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Create change" }).click();
  const dialog = page.getByRole("dialog", { name: "Create change" });
  await dialog.getByRole("button", { name: "Preview change" }).click();
  await dialog.getByRole("button", { name: "Apply command" }).click();
  await expect(dialog.getByText("Publication outcome unknown")).toBeVisible();
  await dialog.getByRole("button", { name: "Check status" }).click();
  await expect(dialog.getByText("Repository updated")).toBeVisible();
  expect(requestId).toMatch(/^[0-9a-f]{32}$/u);
});

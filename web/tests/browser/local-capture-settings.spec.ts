import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample } from "./native-browser-fixture.js";
import {
  captureReceipt,
  captureSources,
  captureStatus,
} from "../../../catalog/fixtures/capture-management.js";
import { decode, pretty } from "../../src/lib/api/client.js";
import type {
  CaptureControl,
  CaptureImport,
} from "../../src/lib/controller/capture-types.js";

async function openSettings(page: Page) {
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings" }).click();
  await page
    .getByRole("button", { name: "Local service", exact: true })
    .click();
  await expect(
    page.getByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Agent transcript capture" }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Agents", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "Agents", level: 1 }),
  ).toBeVisible();
  await expect(
    page.getByRole("combobox", { name: "Settings repository" }),
  ).toHaveCount(0);
  await page.getByRole("tab", { name: "Capture", exact: true }).click();
}
async function bootstrap(page: Page, supported = true) {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", (r) =>
    r.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_capture_management: supported,
        },
        storage: { ready: true },
        repositories: [],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/local/repositories", (r) =>
    r.fulfill({ json: { version: 1, bindings: [] } }),
  );
}

test("cold capture controls preserve intent and snapshot", async ({ page }) => {
  await bootstrap(page);
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  let value = captureStatus(false, 9007199254740993n);
  value.runtime!.unavailable = 14000n;
  const controls: CaptureControl[] = [];
  let imported: CaptureImport | null = null,
    inspections = 0;
  const paths: string[] = [];
  await page.route("**/v1/local/capture**", async (r) => {
    const req = r.request(),
      url = new URL(req.url());
    paths.push(url.pathname);
    expect(req.headers()["authorization"]).toBeUndefined();
    expect(req.headers()["x-jj-store-protocol"]).toBe("1");
    const respond = (result: unknown) =>
      r.fulfill({ contentType: "application/json", body: pretty(result) });
    if (url.pathname.endsWith("/sources")) return respond(captureSources());
    if (url.pathname.endsWith("/control")) {
      const command = decode(req.postData()!) as CaptureControl;
      controls.push(command);
      if (controls.length === 1) return r.abort("failed");
      if (controls.length === 2)
        return r.fulfill({
          status: 409,
          json: { version: 1, code: "capture_policy_changed" },
        });
      value = captureStatus(
        command.action === "pause",
        BigInt(command.expected_revision) + 1n,
      );
      return respond({
        version: 1,
        receipt: {
          generation: command.generation,
          request_id: command.request_id,
          expected_revision: command.expected_revision,
          revision: value.policy!.revision,
          paused: value.policy!.paused,
        },
        replayed: controls.length === 3,
        runtime_requested: controls.length !== 3,
        status: value,
      });
    }
    if (url.pathname.endsWith("/rescan")) {
      expect(decode(req.postData()!)).toEqual({
        version: 1,
        generation: value.policy!.generation,
        expected_revision: value.policy!.revision,
      });
      value.runtime!.discovery.rescan_pending = true;
      return respond({ version: 1, accepted: true, status: value });
    }
    if (url.pathname.endsWith("/import")) {
      imported = decode(req.postData()!) as CaptureImport;
      return r.abort("failed");
    }
    if (url.pathname.endsWith("/imports")) {
      expect(url.searchParams.get("request_id")).toBe(imported!.request_id);
      inspections++;
      const outcome = captureReceipt(imported!);
      return respond({
        version: 1,
        request_id: imported!.request_id,
        receipt: inspections === 1 ? null : outcome.receipt,
        progress:
          inspections === 1
            ? {
                ...outcome.progress,
                phase: "pending",
                boundary_available: false,
              }
            : outcome.progress,
      });
    }
    return respond(value);
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  const route = await page.evaluate(() => location.hash);
  await openSettings(page);
  await expect(
    page.getByText("/Users/capture/.codex/sessions", { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText("Publication backlog", { exact: true }),
  ).toHaveCount(0);
  await page.screenshot({
    path: test.info().outputPath("agents-settings-capture.png"),
  });
  await expect
    .poll(() => paths.includes("/v1/local/capture/sources"))
    .toBe(true);
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await expect(page.getByText("Codex CLI source 1")).toBeVisible();
  const sourceRow = page.getByText("Codex CLI source 1").locator("..");
  const columns = await sourceRow.evaluate((row) => {
    const label = row.querySelector("dt")!.getBoundingClientRect();
    const control = row.querySelector("dd")!.getBoundingClientRect();
    return { labelRight: label.right, controlLeft: control.left };
  });
  expect(columns.controlLeft).toBeGreaterThan(columns.labelRight);
  await page.getByRole("tab", { name: "Capture", exact: true }).click();
  await page.getByRole("button", { name: "Pause capture" }).click();
  await expect(page.getByText("Capture outcome unknown")).toBeVisible();
  await page
    .getByRole("button", { name: "Retry original capture request" })
    .click();
  await expect(page.getByText("Capture outcome unknown")).toBeVisible();
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  expect(await page.evaluate(() => location.hash)).toBe(route);
  await openSettings(page);
  await expect(page.getByText("Capture outcome unknown")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Pause capture" }),
  ).toBeDisabled();
  await page
    .getByRole("button", { name: "Retry original capture request" })
    .click();
  await expect(
    page.getByRole("button", { name: "Resume capture" }),
  ).toBeEnabled();
  expect(controls[0].expected_revision).toBe(9007199254740993n);
  expect(controls.slice(1)).toEqual([controls[0], controls[0]]);
  await page.getByRole("button", { name: "Resume capture" }).click();
  await expect(
    page.getByRole("button", { name: "Pause capture" }),
  ).toBeEnabled();
  expect(controls[3].request_id).not.toBe(controls[0].request_id);
  await page.getByRole("button", { name: "Rescan capture" }).click();
  await expect(page.getByText(/Rescan requested/)).toBeVisible();
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await expect(
    page.getByRole("button", { name: /Import history for/ }),
  ).toBeDisabled();
  await page.getByRole("button", { name: "Refresh capture" }).click();
  await page.getByRole("button", { name: /Import history for/ }).click();
  await expect(page.getByText("Capture outcome unknown")).toBeVisible();
  await page
    .getByRole("button", { name: "Check original import receipt" })
    .click();
  await expect(page.getByText("No receipt available")).toBeVisible();
  await expect(page.getByText("Capture outcome unknown")).toBeVisible();
  await page
    .getByRole("button", { name: "Check original import receipt" })
    .click();
  await expect(page.getByText("Capture outcome unknown")).toHaveCount(0);
  await expect(page.getByText("completed · Boundary available")).toBeVisible();
  await expect(page.getByText("2,048 / 1,024 bytes")).toBeVisible();
  await page.getByRole("button", { name: "Refresh import status" }).click();
  const historyAction = page.getByRole("button", {
    name: "Refresh import status",
  });
  await expect(historyAction).toBeEnabled();
  // Public Button animates disabled opacity; inspect the settled enabled view.
  await expect
    .poll(() =>
      historyAction.evaluate((button) => getComputedStyle(button).opacity),
    )
    .toBe("1");
  await page.screenshot({
    path: test.info().outputPath("agents-settings-desktop.png"),
  });
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.setViewportSize({ width: 390, height: 844 });
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  const mobileColumns = await sourceRow.evaluate((row) => {
    const label = row.querySelector("dt")!.getBoundingClientRect();
    const control = row.querySelector("dd")!.getBoundingClientRect();
    return {
      labelLeft: label.left,
      labelBottom: label.bottom,
      controlLeft: control.left,
      controlTop: control.top,
    };
  });
  expect(mobileColumns.controlLeft).toBe(mobileColumns.labelLeft);
  expect(mobileColumns.controlTop).toBeGreaterThan(mobileColumns.labelBottom);
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.getByRole("button", { name: "Open settings sidebar" }).click();
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  expect(await page.evaluate(() => location.hash)).toBe(route);
  expect(errors).toEqual([]);
});

test("cold capture capability gate withholds transport", async ({ page }) => {
  await bootstrap(page, false);
  const requests: string[] = [];
  page.on("request", (request) => {
    if (request.url().includes("/v1/local/capture"))
      requests.push(request.url());
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await openSettings(page);
  await expect(
    page.getByText("Capture management is not available for this connection."),
  ).toBeVisible();
  expect(requests).toEqual([]);
});

test("visible capture follows observations and stops when settings closes", async ({
  page,
}) => {
  await bootstrap(page);
  await page.clock.install();
  let reads = 0;
  const value = captureStatus();
  await page.route("**/v1/local/capture**", (r) => {
    if (new URL(r.request().url()).pathname.endsWith("/sources"))
      return r.fulfill({
        contentType: "application/json",
        body: pretty(captureSources()),
      });
    reads++;
    return r.fulfill({ contentType: "application/json", body: pretty(value) });
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await openSettings(page);
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await expect(page.getByText("Codex CLI source 1")).toBeVisible();
  await page.getByRole("tab", { name: "Imports", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Previous import request ID" })
    .fill("ef".repeat(16));
  await page.getByRole("tab", { name: "Capture", exact: true }).click();
  value.policy!.paused = true;
  value.runtime!.phase = "paused";
  await page.clock.fastForward(5151);
  await expect(
    page.getByRole("button", { name: "Resume capture" }),
  ).toBeEnabled();
  expect(reads).toBeGreaterThan(1);
  await page
    .getByRole("button", { name: "Local service", exact: true })
    .click();
  const serviceReads = reads;
  await page.clock.fastForward(15000);
  expect(reads).toBe(serviceReads);
  await page.getByRole("button", { name: "Agents", exact: true }).click();
  await page.getByRole("tab", { name: "Imports", exact: true }).click();
  await expect(
    page.getByRole("textbox", { name: "Previous import request ID" }),
  ).toHaveValue("ef".repeat(16));
  await expect.poll(() => reads).toBeGreaterThan(serviceReads);
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  const closedReads = reads;
  await page.clock.fastForward(15000);
  expect(reads).toBe(closedReads);
  await openSettings(page);
  await page.getByRole("tab", { name: "Imports", exact: true }).click();
  await expect(
    page.getByRole("textbox", { name: "Previous import request ID" }),
  ).toHaveValue("ef".repeat(16));
  await expect.poll(() => reads).toBeGreaterThan(closedReads);
});

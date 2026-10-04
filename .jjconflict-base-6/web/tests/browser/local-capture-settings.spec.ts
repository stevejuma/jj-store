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
  const sourceRow = page
    .getByRole("row")
    .filter({ hasText: "Codex CLI source 1" });
  const columns = await sourceRow.evaluate((row) => {
    const label = row.querySelector("td:first-child")!.getBoundingClientRect();
    const control = row.querySelector("td:last-child")!.getBoundingClientRect();
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
  await expect(
    page.getByRole("table", { name: "Capture sources" }),
  ).toBeVisible();
  const scroll = page.locator(
    '.store-capture-sources__table [data-slot="table-container"]',
  );
  expect(
    await scroll.evaluate(
      (element) => element.scrollWidth > element.clientWidth,
    ),
  ).toBe(true);
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

test("capture notifications retain dismissal and drafts until observed recovery", async ({
  page,
}) => {
  await bootstrap(page);
  let unavailable = false;
  await page.route("**/v1/local/capture**", (route) => {
    if (unavailable)
      return route.fulfill({
        status: 503,
        json: { version: 1, code: "capture_management_unavailable" },
      });
    return route.fulfill({
      contentType: "application/json",
      body: pretty(
        new URL(route.request().url()).pathname.endsWith("/sources")
          ? captureSources()
          : captureStatus(),
      ),
    });
  });
  await page.goto("/");
  await expect(
    page.getByRole("option", { name: "main", exact: true }),
  ).toBeVisible();
  await openSettings(page);
  await expect(
    page.getByText("/Users/capture/.codex/sessions", { exact: true }),
  ).toBeVisible();
  await page.getByRole("tab", { name: "Imports", exact: true }).click();
  const draft = page.getByRole("textbox", {
    name: "Previous import request ID",
  });
  await draft.fill("ef".repeat(16));
  unavailable = true;
  const refresh = page.getByRole("button", { name: "Refresh capture" });
  const toast = page.locator(
    '[data-notification-toasts] [data-severity="error"]',
  );
  await refresh.click();
  await expect(toast).toHaveCount(1);
  await expect(toast).toContainText("Capture request unavailable");
  await expect(page.getByRole("alert")).toHaveCount(1);
  const id = await toast.getAttribute("data-notification-id");
  await refresh.click();
  await expect(refresh).toBeEnabled();
  await expect(toast).toHaveAttribute("data-notification-id", id!);
  await page
    .getByRole("button", { name: "Dismiss Capture request unavailable" })
    .click();
  await refresh.click();
  await expect(refresh).toBeEnabled();
  await expect(toast).toHaveCount(0);
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await page.getByRole("button", { name: "Back to jj-store" }).click();
  await openSettings(page);
  await expect(refresh).toBeEnabled();
  await expect(toast).toHaveCount(0);
  await page.getByRole("tab", { name: "Imports", exact: true }).click();
  await expect(draft).toHaveValue("ef".repeat(16));
  unavailable = false;
  await refresh.click();
  await expect(refresh).toBeEnabled();
  unavailable = true;
  await refresh.click();
  await expect(toast).toHaveCount(1);
  await page.setViewportSize({ width: 390, height: 844 });
  const bounds = await toast.boundingBox();
  expect(bounds!.x).toBeGreaterThanOrEqual(0);
  expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(390);
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.screenshot({
    path: test.info().outputPath("capture-notification-mobile.png"),
  });
  unavailable = false;
  await refresh.click();
  await expect(toast).toHaveCount(0);
  await expect(draft).toHaveValue("ef".repeat(16));
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

test("cold local configuration editing retains drafts and announces restart", async ({
  page,
}) => {
  const { captureSettings } =
    await import("../../../catalog/fixtures/capture-management.js");
  await bootstrap(page);
  let settings = captureSettings(false);
  const saves: unknown[] = [];
  await page.route("**/v1/local/capture**", async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname;
    expect(req.headers()["authorization"]).toBeUndefined();
    expect(req.headers()["x-jj-store-protocol"]).toBe("1");
    if (path.endsWith("/settings")) {
      if (req.method() === "POST") {
        const command = decode(
          req.postData()!,
        ) as import("../../src/lib/controller/capture-configuration.js").CaptureSaveSettings;
        saves.push(command);
        expect(command.expected_revision).toBe(settings.revision);
        settings = {
          ...settings,
          desired: command.configuration,
          revision: "cd".repeat(32),
          restart_required: true,
        };
      }
      return r.fulfill({
        contentType: "application/json",
        body: pretty(settings),
      });
    }
    if (path.endsWith("/sources"))
      return r.fulfill({
        json: {
          ...captureSources(),
          enabled: false,
          phase: null,
          runtime_generation: null,
          total: 0,
          sources: [],
        },
      });
    return r.fulfill({
      json: { version: 1, enabled: false, policy: null, runtime: null },
    });
  });
  await page.goto("/");
  await openSettings(page);
  await page.getByRole("tab", { name: "Configuration", exact: true }).click();
  await page.getByRole("switch", { name: "Enable automatic capture" }).click();
  await page
    .getByRole("textbox", { name: "Codex CLI folders" })
    .fill("/tmp/browser-capture/sessions");
  await page.getByRole("spinbutton", { name: "Concurrent readers" }).fill("2");
  await page.getByRole("tab", { name: "Sources", exact: true }).click();
  await page.getByRole("button", { name: "Refresh capture" }).click();
  await page.getByRole("tab", { name: "Configuration", exact: true }).click();
  await expect(
    page.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toHaveValue("/tmp/browser-capture/sessions");
  await expect(
    page.getByRole("spinbutton", { name: "Concurrent readers" }),
  ).toHaveValue("2");
  await page
    .getByRole("button", { name: "Save configuration", exact: true })
    .click();
  await expect(page.getByText("Disabled · Restart required")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Save configuration", exact: true }),
  ).toBeDisabled();
  expect(saves).toHaveLength(1);
  await expect(
    page.getByText("/Users/capture/.codex/sessions", { exact: true }),
  ).toBeVisible();
  const axe = await new AxeBuilder({ page })
    .include('section[aria-labelledby="capture-configuration-title"]')
    .analyze();
  expect(axe.violations).toEqual([]);
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await expect(
    page.getByRole("textbox", { name: "Codex CLI folders" }),
  ).toBeVisible();
});

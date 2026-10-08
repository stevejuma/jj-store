// Cold, isolated acceptance for native input that Storybook userEvent cannot model.
// Run from the workspace root: node catalog/fixtures/delta-chat/browser.acceptance.mjs
import { chromium, expect } from "@playwright/test";
import { spawn } from "node:child_process";
import { createServer } from "node:net";
import { mkdirSync, openSync, closeSync } from "node:fs";
import { largeCaptureAcceptance } from "./large-capture-native.mjs";
import { messageActionAcceptance } from "./message-action-native.mjs";
import { readerInputAcceptance } from "./reader-input-native.mjs";
import { commentQueueAcceptance } from "./comment-queue-native.mjs";
import { nextMessageAcceptance } from "./next-message-native.mjs";

mkdirSync(".local", { recursive: true });
const reservation = createServer();
await new Promise((resolve) => reservation.listen(0, "127.0.0.1", resolve));
const port = reservation.address().port;
await new Promise((resolve) => reservation.close(resolve));
const log = openSync(".local/chat-body-cold.log", "w");
const server = spawn(
  "pnpm",
  [
    "exec",
    "storybook",
    "dev",
    "--host",
    "127.0.0.1",
    "--port",
    String(port),
    "--no-open",
    "--disable-telemetry",
    "--ci",
  ],
  {
    detached: true,
    stdio: ["ignore", log, log],
  },
);
closeSync(log);
let browser;
try {
  const base = `http://127.0.0.1:${port}`;
  await expect
    .poll(
      async () => {
        if (server.exitCode !== null)
          throw new Error(
            "Isolated Storybook exited; inspect .local/chat-body-cold.log",
          );
        try {
          return (await fetch(`${base}/index.json`)).status;
        } catch {
          return 0;
        }
      },
      { timeout: 120000 },
    )
    .toBe(200);
  browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1100, height: 760 },
  });
  const errors = [];
  page.on("pageerror", (error) => {
    errors.push(error.message);
    console.error(error.message);
  });
  const story = `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--interaction-playground&viewMode=story`;
  await page.goto(story);
  const source = page.getByRole("textbox", {
    name: "Assistant explanation",
    exact: true,
  });
  const body = page
    .getByRole("region", { name: "Agent chat body" })
    .locator('[data-ui-part="scroll-area-viewport"]');
  await readerInputAcceptance(page, source);
  const mac = process.platform === "darwin";
  // Appends retain a reader's scrolled position; resize preserves submitted threads.
  await body.evaluate((node) => {
    node.scrollTop = 30;
  });
  const top = await body.evaluate((node) => node.scrollTop);
  await page.getByRole("button", { name: "Append response" }).click();
  await expect.poll(() => body.evaluate((node) => node.scrollTop)).toBe(top);
  await page
    .getByRole("button", { name: "Expand Read a file, ran a command" })
    .click();
  const desktopRead = page.getByRole("button", {
    name: "Show details for Read example.ts",
  });
  const rawIcon = page.getByRole("button", {
    name: "Show raw output for Read example.ts",
  });
  await source.focus();
  await page.mouse.move(10, 10);
  await expect
    .poll(() =>
      rawIcon.evaluate(
        (node) => getComputedStyle(node.closest(".chat-raw-toggle")).opacity,
      ),
    )
    .toBe("0");
  await desktopRead.hover();
  for (const label of [
    "Collapse Read a file, ran a command",
    "Show details for Read example.ts",
    "Hide details for Apply patch example.ts",
    "Hide captured file example.ts",
  ]) {
    const header = page.getByRole("button", { name: label });
    const gap = await header.evaluate(
      (node) =>
        node.querySelector("svg:last-child").getBoundingClientRect().left -
        node.querySelector("span:not(.sr-only)").getBoundingClientRect().right,
    );
    expect(gap).toBeGreaterThanOrEqual(7);
    expect(gap).toBeLessThanOrEqual(9);
    await expect(header).toHaveCSS("background-color", "rgba(0, 0, 0, 0)");
    await header.hover();
    await expect(header).toHaveCSS("background-color", "rgba(0, 0, 0, 0)");
  }
  await desktopRead.hover();
  await expect
    .poll(() =>
      rawIcon.evaluate(
        (node) => getComputedStyle(node.closest(".chat-raw-toggle")).opacity,
      ),
    )
    .toBe("1");
  await page
    .getByRole("button", { name: "Collapse Read a file, ran a command" })
    .click();
  await page.setViewportSize({ width: 390, height: 700 });
  await expect
    .poll(() =>
      body.evaluate((node) => node.scrollWidth <= node.clientWidth + 1),
    )
    .toBe(true);
  await page.getByRole("button", { name: "Hide body" }).click();
  await page.getByRole("button", { name: "Show body" }).click();
  await expect(
    page
      .locator('[data-placement="inline"]')
      .getByRole("article", { name: "Comment by You" })
      .filter({ hasText: "Keyboard draft" }),
  ).toHaveText(/Keyboard draft/);
  await page
    .getByRole("button", { name: "Expand Read a file, ran a command" })
    .click();
  const readOpener = page.getByRole("button", {
    name: "Show details for Read example.ts",
  });
  await readOpener.click();
  const pane = page.getByRole("complementary", { name: "Read tool details" });
  await expect(pane).toBeVisible();
  await expect(
    pane.getByRole("textbox", { name: "Captured example.ts" }),
  ).toHaveText(/export const answer/);
  await pane.getByRole("button", { name: "Close read tool details" }).click();
  await expect(readOpener).toBeFocused();
  await page.getByRole("button", { name: "Preview turn 1" }).hover();
  const summary = page.getByRole("group", { name: "Turn summary" });
  await expect(summary).toContainText("Turn Summary");
  await summary.getByRole("button", { name: "Scroll to turn 1" }).click();
  await expect(page.locator('[data-chat-item="prose"]')).toBeFocused();
  const end = page.getByRole("button", { name: "Jump to end" });
  await end.hover();
  await expect(end).toHaveCSS("outline-style", "none");
  await expect(page.getByRole("tooltip")).toContainText("Jump to End");
  await end.click();
  await expect
    .poll(() =>
      body.evaluate(
        (node) => node.scrollHeight - node.clientHeight - node.scrollTop,
      ),
    )
    .toBeLessThanOrEqual(8);
  await expect(
    page.getByRole("textbox", { name: "Next message", exact: true }),
  ).toBeFocused();
  const marker = page.getByRole("button", { name: "Preview turn 1" });
  await marker.click();
  await expect.poll(() => body.evaluate((node) => node.scrollTop)).toBe(0);
  await marker.focus();
  await page.keyboard.press("Tab");
  const keyboardMarker = page.getByRole("button", { name: "Preview turn 2" });
  await expect(keyboardMarker).toBeFocused();
  await expect(keyboardMarker).toHaveCSS("outline-style", "solid");
  await page.keyboard.press(mac ? "Meta+ArrowDown" : "Control+ArrowDown");
  await expect
    .poll(() =>
      body.evaluate(
        (node) => node.scrollHeight - node.clientHeight - node.scrollTop,
      ),
    )
    .toBeLessThanOrEqual(8);
  await page.setViewportSize({ width: 1440, height: 900 });
  await largeCaptureAcceptance(page, base);
  await messageActionAcceptance(page, base);
  await nextMessageAcceptance(page, base);
  await commentQueueAcceptance(page, base);
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--delta-main-thread-reference&viewMode=story`,
  );
  await expect(
    page.getByRole("textbox", {
      name: "Captured patch · Sources/BedtimeCore/Schedule.swift",
      exact: true,
    }),
  ).toBeVisible({ timeout: 60000 });
  await page.screenshot({ path: ".local/chat-body-preview.png" });
  expect(errors).toEqual([]);
  console.log(
    "PASS cold native pointer/keyboard input, immutability, focus, custom sections, scroll, resize, teardown, mobile read pane and turn navigation",
  );
} finally {
  await browser?.close();
  if (server.exitCode === null) {
    process.kill(-server.pid, "SIGTERM");
    await new Promise((resolve) => server.once("exit", resolve));
  }
}

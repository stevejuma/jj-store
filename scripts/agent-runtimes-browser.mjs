import { chromium, expect } from "@playwright/test";
import { accessible } from "./agent-execution-browser-ui.mjs";
// Read-only cold settings acceptance: opens and cancels install confirmation.
const [origin] = process.argv.slice(2);
const url = new URL(origin);
if (
  url.protocol !== "http:" ||
  url.hostname !== "127.0.0.1" ||
  !url.port ||
  url.pathname !== "/" ||
  url.search ||
  url.hash ||
  url.username ||
  url.password
)
  throw Error("Use a literal loopback origin with a port and trailing slash.");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1351, height: 988 },
  });
  const page = await context.newPage();
  const errors = [],
    commands = [];
  page.on("pageerror", (e) => errors.push(e.message));
  page.on("request", (r) => {
    if (
      r.method() === "POST" &&
      new URL(r.url()).pathname.startsWith("/v1/local/agents/")
    )
      commands.push(r.url());
  });
  await page.goto(`${origin}#/agents`);
  await page.getByRole("button", { name: /Open profile menu for/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page
    .getByRole("navigation", { name: "Settings navigation" })
    .getByRole("button", { name: "Agents", exact: true })
    .click();
  const read = page
    .waitForResponse((r) => r.url() === `${origin}v1/local/agents/runtimes`)
    .then((r) => r.json());
  await page.getByRole("tab", { name: "Runtimes", exact: true }).click();
  const value = await read;
  expect(value.version).toBe(1);
  expect(value.runtimes.map((r) => r.id)).toEqual(["codex", "cursor"]);
  await expect(
    page.getByRole("heading", { name: "Agent runtimes", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Check again", exact: true }),
  ).toBeEnabled();
  for (const runtime of value.runtimes.filter((r) => r.state === "ready")) {
    expect(runtime.action).toBeNull();
    await expect(
      page.getByRole("button", {
        name: `Install ${runtime.label} runtime`,
        exact: true,
      }),
    ).toHaveCount(0);
  }
  const guide = page.getByRole("link", { name: "Update guide", exact: true });
  await expect(guide).toBeVisible();
  const guideLayout = await guide.evaluate((e) => {
    const style = getComputedStyle(e);
    const icon = e.querySelector("svg").getBoundingClientRect();
    const text = [...e.childNodes].find(
      (node) => node.nodeType === Node.TEXT_NODE && node.textContent?.trim(),
    );
    const range = document.createRange();
    range.selectNode(text);
    const label = range.getBoundingClientRect();
    return {
      display: style.display,
      direction: style.flexDirection,
      align: style.alignItems,
      iconWidth: icon.width,
      textGap: label.left - icon.right,
      verticalOffset: Math.abs(
        label.top + label.height / 2 - icon.top - icon.height / 2,
      ),
    };
  });
  expect(["flex", "inline-flex"]).toContain(guideLayout.display);
  expect(guideLayout.direction).toBe("row");
  expect(guideLayout.align).toBe("center");
  expect(guideLayout.iconWidth).toBeGreaterThanOrEqual(12);
  expect(guideLayout.iconWidth).toBeLessThanOrEqual(20);
  expect(guideLayout.textGap).toBeGreaterThanOrEqual(0);
  expect(guideLayout.verticalOffset).toBeLessThanOrEqual(2);
  await expect(
    page.getByRole("region", { name: "Agent capture settings" }),
  ).toHaveCount(0);
  const ready = page.getByText("Ready", { exact: true });
  for (const chip of await ready.all())
    await expect(chip).toHaveAttribute("data-chip-tone", "success");
  const action = page.getByRole("button", {
    name: /^(Install|Update) Codex runtime$/,
  });
  let confirmationChecked = false;
  if (value.install_supported && value.runtimes[0].action) {
    await action.focus();
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog", {
      name: /^(Install|Update) Codex runtime/,
    });
    await expect(dialog).toBeVisible();
    await accessible(page, "runtime confirmation", '[role="dialog"]');
    await dialog.getByRole("button", { name: "Cancel", exact: true }).click();
    await expect(dialog).toHaveCount(0);
    await expect
      .poll(() =>
        page.evaluate(() => getComputedStyle(document.body).pointerEvents),
      )
      .not.toBe("none");
    confirmationChecked = true;
  }
  for (const mode of ["light", "dark"]) {
    await page.emulateMedia({ colorScheme: mode });
    await expect
      .poll(() =>
        page.evaluate(() =>
          document.documentElement.classList.contains("dark"),
        ),
      )
      .toBe(mode === "dark");
    await accessible(page, `${mode} runtime settings`, ".store-settings");
  }
  await page.setViewportSize({ width: 390, height: 844 });
  await accessible(page, "narrow runtime settings", ".store-settings");
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  await page.setViewportSize({ width: 1351, height: 988 });
  await page.getByRole("button", { name: "Check again", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Check again", exact: true }),
  ).toBeEnabled();
  expect(commands).toEqual([]);
  expect(errors).toEqual([]);
  console.log(
    JSON.stringify({
      runtimes: value.runtimes.map((r) => ({
        id: r.id,
        state: r.state,
        managed: r.managed,
      })),
      commands: commands.length,
      errors: errors.length,
      accessibility: "light/dark/narrow passed",
      confirmation: confirmationChecked
        ? "keyboard confirmation and cancel passed"
        : "not offered for ready runtimes",
    }),
  );
} finally {
  await browser.close();
}

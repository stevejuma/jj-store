import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
// Read-only presentation acceptance against an existing populated local chat.
const [origin, sessionId, mode = "light"] = process.argv.slice(2);
const parsed = new URL(origin);
if (
  parsed.protocol !== "http:" ||
  parsed.hostname !== "127.0.0.1" ||
  !parsed.port ||
  parsed.pathname !== "/" ||
  parsed.search ||
  parsed.hash ||
  parsed.username ||
  parsed.password ||
  !/^[0-9a-f]{32}$/.test(sessionId) ||
  !["light", "dark"].includes(mode)
)
  throw Error(
    "Use a literal loopback origin, exact session ID and light/dark mode.",
  );
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1351, height: 988 },
    colorScheme: mode,
  });
  const page = await context.newPage();
  const commands = [],
    errors = [];
  page.on("request", (r) => {
    if (
      r.method() === "POST" &&
      /\/local\/agents\/(command|launch|setups)$/.test(
        new URL(r.url()).pathname,
      )
    )
      commands.push(r.url());
  });
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto(origin + "/#/agents");
  const refresh = page.getByRole("button", {
    name: "Refresh agents",
    exact: true,
  });
  const sessions = page.getByRole("button", {
    name: "View sessions",
    exact: true,
  });
  await expect(refresh).toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => document.documentElement.classList.contains("dark")),
    )
    .toBe(mode === "dark");
  await expect(sessions).toBeVisible();
  expect((await sessions.boundingBox()).x).toBeLessThan(
    (await refresh.boundingBox()).x,
  );
  await refresh.hover();
  await expect(page.getByRole("tooltip")).toHaveText("Refresh agents");
  await sessions.click();
  const chat = page.getByRole("button", {
    name: new RegExp("Open session .*" + sessionId),
  });
  await expect(chat).toBeVisible();
  await chat.click();
  const workspace = page.getByRole("main", { name: "Agent sessions inbox" });
  await expect(
    workspace
      .locator(
        '[data-ui-component="ai-chat-system-message"][data-variant="divider"]',
      )
      .first(),
  ).toBeVisible();
  await expect(
    workspace.getByText("Time unknown", { exact: true }),
  ).toHaveCount(0);
  await expect(chat).not.toHaveText("Offline");
  await expect(chat.locator('[data-ui-part="presence"]')).toHaveAttribute(
    "data-presence",
    /^(online|offline|unknown)$/,
  );
  const repository = chat.locator("..").getByRole("link");
  await expect(repository).toHaveAttribute(
    "href",
    new RegExp("^#/repos/[0-9a-f]{32}/files\\?at_op=[0-9a-f]{128}$"),
  );
  expect(await repository.evaluate((e) => !!e.closest("button"))).toBe(false);
  const list = workspace.getByRole("complementary", {
    name: "Session chats",
    exact: true,
  });
  const initial = (await list.boundingBox()).width;
  const handle = workspace.getByRole("button", {
    name: "Resize session chats",
    exact: true,
  });
  await handle.focus();
  await page.keyboard.press("Home");
  await expect
    .poll(async () => Math.round((await list.boundingBox()).width))
    .toBe(224);
  const box = await handle.boundingBox(),
    x = box.x + box.width / 2,
    y = box.y + 120;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 64, y, { steps: 5 });
  await page.mouse.up();
  await expect
    .poll(async () => Math.round((await list.boundingBox()).width))
    .toBe(288);
  await page.reload();
  await expect
    .poll(async () => Math.round((await list.boundingBox()).width))
    .toBe(288);
  await handle.dblclick();
  await expect
    .poll(async () => Math.round((await list.boundingBox()).width))
    .toBe(Math.round(initial));
  const viewport = workspace
    .locator(
      '[data-ui-component="ai-chat-layout"] [data-ui-part="scroll-area-viewport"]',
    )
    .first();
  await viewport.evaluate((e) => {
    e.scrollTop = 0;
    e.dispatchEvent(new Event("scroll"));
  });
  const first = workspace.getByRole("article").first();
  await first.hover();
  await expect(
    first.getByRole("button", { name: "Reply", exact: true }),
  ).toBeVisible();
  const actions = first.locator('[data-ui-part="message-actions"]');
  expect((await actions.boundingBox()).y).toBeGreaterThanOrEqual(
    (await viewport.boundingBox()).y,
  );
  const turns = workspace
    .locator('[data-ui-component="ai-chat-system-message"]')
    .filter({ hasText: "Turn completed" });
  // Sessions without a completed turn still qualify date and navigation presentation.
  for (const turn of await turns.all()) {
    await expect(turn).toHaveAttribute("data-variant", "divider");
    await expect(turn.locator('[data-ui-component="separator"]')).toHaveCount(
      2,
    );
  }
  const axe = await new AxeBuilder({ page })
    .include('[aria-label="Agent sessions inbox"]')
    .analyze();
  expect(
    axe.violations.map((v) => ({
      id: v.id,
      targets: v.nodes.map((n) => n.target),
    })),
  ).toEqual([]);
  await page.setViewportSize({ width: 430, height: 900 });
  await expect(handle).toHaveCount(0);
  const mobile = await new AxeBuilder({ page })
    .include('[aria-label="Agent sessions inbox"]')
    .analyze();
  expect(
    mobile.violations.map((v) => ({
      id: v.id,
      targets: v.nodes.map((n) => n.target),
    })),
  ).toEqual([]);
  expect(commands).toEqual([]);
  expect(errors).toEqual([]);
  console.log(
    "Current UI: icon tooltips, exact repository link, avatar presence, legacy/date dividers, turn borders, pointer/keyboard resize/reset, first hover toolbar and accessibility pass. No agent command or user message was sent.",
  );
  await context.close();
} finally {
  await browser.close();
}

import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
const [origin, id, mode = "light"] = process.argv.slice(2);
const url = new URL(origin);
if (
  url.protocol !== "http:" ||
  url.hostname !== "127.0.0.1" ||
  !url.port ||
  url.pathname !== "/" ||
  url.search ||
  url.hash ||
  url.username ||
  url.password ||
  !/^[0-9a-f]{32}$/.test(id) ||
  !["light", "dark"].includes(mode)
)
  throw Error(
    "Use a loopback origin, exact saved session ID and light/dark mode.",
  );
const escape = (value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1070, height: 988 },
    colorScheme: mode,
  });
  const page = await context.newPage();
  const commands = [],
    errors = [],
    repositories = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("request", (request) => {
    if (
      request.method() === "POST" &&
      /\/local\/agents\/(command|launch|setups)$/.test(
        new URL(request.url()).pathname,
      )
    )
      commands.push(request.url());
  });
  page.on("response", async (response) => {
    if (
      new URL(response.url()).pathname === "/v1/repositories" &&
      response.ok()
    )
      repositories.push(...(await response.json()).repositories);
  });
  const statusWait = page.waitForResponse(
    (response) => new URL(response.url()).pathname === "/v1/local/agents",
  );
  await page.goto(`${origin}/#/agents/sessions/${id}`);
  const status = await (await statusWait).json();
  const session = status.sessions.find((session) => session.id === id);
  expect(session?.recorded_session_ids.length).toBeGreaterThan(0);
  expect(session.workspace_name).toBeTruthy();
  expect(session.workspace_operation).toMatch(/^[0-9a-f]{128}$/);
  const chat = page.getByRole("button", {
    name: new RegExp(`Open session .*${id}`),
  });
  await expect(chat).toBeVisible();
  const link = chat.locator("..").getByRole("link");
  await expect
    .poll(
      () =>
        repositories.find(
          (repository) =>
            repository.repository_id === session.repository ||
            repository.id === session.repository,
        )?.name,
    )
    .toBeTruthy();
  const repository = repositories.find(
    (repository) =>
      repository.repository_id === session.repository ||
      repository.id === session.repository,
  );
  await expect(link).toHaveAccessibleName(`Open ${repository.name}`);
  await link.hover();
  await expect(page.getByRole("tooltip")).toHaveText(`Open ${repository.name}`);
  const query = new URLSearchParams(
    (await link.getAttribute("href")).split("?")[1],
  );
  expect(query.get("workspace")).toBe(session.workspace_name);
  expect(query.get("at_op")).toBe(session.workspace_operation);
  await link.click();
  const picker = page.getByRole("combobox", {
    name: "Repository workspace",
    exact: true,
  });
  await expect(picker).toHaveText(session.workspace_name);
  await picker.click();
  const workspace = page.getByRole("option", {
    name: new RegExp(escape(session.workspace_name)),
  });
  await expect(
    workspace.locator('[data-ui-component="community-member-avatar"]'),
  ).toBeVisible();
  await expect(workspace.getByLabel("Selected workspace")).toBeVisible();
  await page.keyboard.press("Escape");
  await page.goBack();
  await expect(chat).toBeVisible();
  await page.goForward();
  await expect(picker).toHaveText(session.workspace_name);
  await page.goto(`${origin}/#/repos/${session.repository}/agents?${query}`);
  const row = page.locator(
    `[data-session-id="${session.recorded_session_ids[0]}"]`,
  );
  await expect(
    row.getByRole("button", {
      name: new RegExp(`^Open ${escape(session.title)} `),
    }),
  ).toBeVisible();
  const avatar = row.locator('[data-ui-component="community-member-avatar"]');
  await expect(avatar).toBeVisible();
  if (await avatar.locator("img").count())
    await expect(avatar.locator("img")).toHaveAttribute("src", session.avatar);
  else
    await expect(avatar.locator('[data-ui-part="emoji"]')).toHaveText(
      session.avatar,
    );
  // Timing is optional for legacy repository snapshots; never substitute journal time.
  const metadata = row.locator(".store-collaboration__row-meta");
  if (await metadata.getByLabel(/^Last activity:/).count())
    expect(
      await metadata
        .getByLabel(/^Last activity:/)
        .evaluate(
          (element) =>
            !!(
              element.compareDocumentPosition(
                element.parentElement.querySelector(
                  '[data-ui-component="community-chip"]',
                ),
              ) & Node.DOCUMENT_POSITION_FOLLOWING
            ),
        ),
    ).toBe(true);
  await page.reload();
  await expect(row).toHaveText(new RegExp(escape(session.title)));
  await expect
    .poll(() =>
      page.evaluate(() => document.documentElement.classList.contains("dark")),
    )
    .toBe(mode === "dark");
  expect(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Agent activity"]')
        .analyze()
    ).violations,
  ).toEqual([]);
  await page.setViewportSize({ width: 430, height: 900 });
  expect(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Agent activity"]')
        .analyze()
    ).violations,
  ).toEqual([]);
  expect(commands).toEqual([]);
  expect(errors).toEqual([]);
  console.log(
    "Managed identity, captured avatar, repository tooltip, exact workspace navigation, Back/Forward, reload and light/dark/mobile accessibility pass. Zero execution POSTs.",
  );
  await context.close();
} finally {
  await browser.close();
}

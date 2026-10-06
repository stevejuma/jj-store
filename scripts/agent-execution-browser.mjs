import {
  validateAgentAcceptanceOrigin,
  verifyRecordedHistory,
} from "./agent-execution-browser-input.mjs";
// Cold production-browser acceptance against the disposable native service.
import {
  verifyChatFeedback,
  verifyChatWorking,
} from "./agent-execution-chat-feedback.mjs";
import { verifyAgentDiscussion } from "./agent-execution-discussion.mjs";
import { chromium, expect } from "@playwright/test";
import {
  accessible,
  prepareAgent,
  sessionOption,
  openRuntime,
  runtimeHint,
  configureRuntime,
  sendPrompt,
  waitForConnection,
} from "./agent-execution-browser-ui.mjs";
const [origin, repository, profile = "native"] = process.argv.slice(2);
if (!["native", "local_jj_git"].includes(profile))
  throw Error("Unknown acceptance profile");
const agentName =
  profile === "native"
    ? "Browser qualification"
    : "Browser qualification local";
validateAgentAcceptanceOrigin(origin, repository);
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
      viewport: { width: 1440, height: 1100 },
    }),
    page = await context.newPage();
  const errors = [];
  const admission = {
    "x-jj-store-protocol": "1",
    Origin: origin,
    "Sec-Fetch-Site": "same-origin",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Dest": "empty",
  };
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`${origin}/#/agents`);
  const directory = page.getByRole("region", {
    name: "Your agents",
    exact: true,
  });
  await expect(
    page.getByRole("heading", { name: "Agents", exact: true }),
  ).toBeVisible();
  for (const name of ["Refresh snapshot", "Toggle terminal", "Toggle details"])
    await expect(page.getByRole("button", { name, exact: true })).toHaveCount(
      0,
    );
  await expect(
    page.getByRole("dialog", { name: "Agent sessions", exact: true }),
  ).toHaveCount(0);
  await accessible(page, "standalone gallery", '[aria-label="Your agents"]');
  const { card, setup } = await prepareAgent(
    page,
    origin,
    admission,
    agentName,
  );
  await card
    .getByRole("button", { name: `Start ${agentName}`, exact: true })
    .click();
  const launch = page.getByRole("form", {
    name: "Start agent session",
    exact: true,
  });
  const targets = await page.request.get(`${origin}/v1/local/agents/targets`, {
    headers: {
      "x-jj-store-protocol": "1",
      Origin: origin,
      "Sec-Fetch-Site": "same-origin",
      "Sec-Fetch-Mode": "cors",
      "Sec-Fetch-Dest": "empty",
    },
  });
  const target = (await targets.json()).find(
    (t) => t.repository === repository,
  );
  if (!target) throw Error("Authorized target missing");
  await page
    .getByRole("combobox", { name: "Repository workspace", exact: true })
    .click();
  await page
    .getByRole("option", {
      name: `${target.name} ${target.folder}`,
      exact: true,
    })
    .click();
  await launch
    .getByRole("textbox", { name: "Markdown editor", exact: true })
    .fill("Keep this launch draft");
  await accessible(
    page,
    "new draft chat",
    '[aria-label="Agent sessions inbox"]',
  );
  await page
    .getByRole("button", { name: "Connect agent", exact: true })
    .click();
  await expect(page).toHaveURL(/#\/agents\/sessions\/[a-f0-9]{32}(?:\?|$)/, {
    timeout: 90000,
  });
  const workspace = page.getByRole("region", {
    name: "Selected agent session",
    exact: true,
  });
  const composer = workspace.getByRole("textbox", {
    name: "Markdown editor",
    exact: true,
  });
  await expect(composer).toBeVisible({ timeout: 90000 });
  await expect(composer).toHaveText("Keep this launch draft");
  const sessionId = new URL(page.url()).hash.match(
    /^#\/agents\/sessions\/([a-f0-9]{32})(?:\?|$)/,
  )?.[1];
  if (!sessionId) throw Error("Selected session route missing its exact ID");
  await waitForConnection(page, origin, admission, sessionId);
  const runtime = await openRuntime(page);
  await accessible(
    page,
    "runtime sidebar",
    '[aria-label="Workspace and runtime"]',
  );
  await page.emulateMedia({ colorScheme: "dark" });
  await expect(page.locator("html")).toHaveClass(/dark/);
  await accessible(
    page,
    "dark runtime sidebar",
    '[aria-label="Workspace and runtime"]',
  );
  await page.emulateMedia({ colorScheme: "light" });
  await configureRuntime(page, runtime);
  await runtime
    .getByRole("button", { name: "Close workspace and runtime", exact: true })
    .click();
  await expect(
    workspace.getByText(runtimeHint, {
      exact: true,
    }),
  ).toHaveCount(0, { timeout: 30000 });
  // Returning to management does not terminate the connected session.
  await page
    .getByRole("navigation", { name: "Repository navigation" })
    .first()
    .getByRole("button", { name: "Agents", exact: true })
    .click();
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Online",
    {
      timeout: 30000,
    },
  );
  await page.goto(`${origin}/#/agents/sessions/${sessionId}`);
  await composer.fill(
    "Reply with the phrase browser integration accepted. Do not run tools or change files.",
  );
  await sendPrompt(page, workspace, sessionId, origin, admission);
  await verifyChatWorking(workspace);
  await page
    .getByRole("navigation", { name: "Repository navigation" })
    .first()
    .getByRole("button", { name: "Agents", exact: true })
    .click();
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Working",
    {
      timeout: 30000,
    },
  );
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Online",
    {
      timeout: 180000,
    },
  );
  await page.goto(`${origin}/#/agents/sessions/${sessionId}`);
  await expect(
    workspace.getByText(/^browser integration accepted\.?$/),
  ).toBeVisible({ timeout: 30000 });
  await expect(
    workspace.getByText("Turn completed", { exact: true }),
  ).toBeVisible();
  await verifyChatFeedback(page, workspace, origin, admission, sessionId);
  await verifyAgentDiscussion(
    page,
    workspace,
    origin,
    admission,
    sessionId,
    repository,
  );
  await composer.fill("Retained inbox draft");
  await sessionOption(page, "Activity log");
  await expect(page).toHaveURL(new RegExp(`/agents/sessions/${sessionId}`));
  await expect(
    page.getByText(/^browser integration accepted\.?$/),
  ).toBeVisible();
  await accessible(
    page,
    "activity chat",
    '[aria-label="Agent sessions inbox"]',
  );
  await expect(composer).toHaveText("Retained inbox draft");
  await page.goto(
    `${origin}/#/agents/activity?agent=${setup.id}&session=${sessionId}`,
  );
  await expect(page).toHaveURL(new RegExp(`/agents/sessions/${sessionId}`));
  await expect(composer).toBeVisible();
  await sessionOption(page, "End session");
  await page
    .getByRole("navigation", { name: "Repository navigation" })
    .first()
    .getByRole("button", { name: "Agents", exact: true })
    .click();
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Offline",
    {
      timeout: 30000,
    },
  );
  await page.goto(`${origin}/#/agents/sessions/${sessionId}`);
  const inboxRow = page.getByRole("button", {
    name: `Open session ${agentName} · ${sessionId}`,
    exact: true,
  });
  await expect(inboxRow.locator("time")).toHaveAttribute("datetime", /T/);
  await expect(inboxRow).not.toContainText("ended ·");
  await expect(inboxRow).not.toContainText(repository);
  await workspace
    .getByRole("button", { name: "Session options", exact: true })
    .click();
  await expect(
    page.getByRole("menuitem", {
      name: "New session with saved context",
      exact: true,
    }),
  ).toBeVisible({ timeout: 30000 });
  await page.keyboard.press("Escape");
  await page
    .getByRole("navigation", { name: "Repository navigation" })
    .first()
    .getByRole("button", { name: "Agents", exact: true })
    .click();
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Offline",
    {
      timeout: 30000,
    },
  );
  await page.reload();
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Offline",
  );
  await page.goto(`${origin}/#/agents/sessions/${sessionId}`);
  await openRuntime(page);
  await runtime
    .getByRole("button", { name: "Review changes", exact: true })
    .click();
  await expect(page).toHaveURL(new RegExp(`repos/${repository}/.*commit=`));
  await page.goBack();
  await expect(composer).toBeVisible();
  await page.goForward();
  await expect(page).toHaveURL(new RegExp(`repos/${repository}/.*commit=`));
  await expect
    .poll(
      async () => {
        const status = await (
          await page.request.get(`${origin}/v1/local/agents`, {
            headers: admission,
          })
        ).json();
        const session = status.sessions.find((s) => s.id === sessionId);
        return session?.published_sequence === session?.last_sequence;
      },
      { timeout: 30000 },
    )
    .toBe(true);
  await verifyRecordedHistory(context, origin, repository);
  if (errors.length) throw Error(`Production errors: ${errors.join("; ")}`);
  console.log(
    "Cold standalone management, editor, live presence cycle, full-page session, reload/history/review and accessibility passed",
  );
} finally {
  await browser.close();
}

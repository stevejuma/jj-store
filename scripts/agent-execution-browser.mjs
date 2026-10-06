// Cold production-browser acceptance against the disposable native service.
import { chromium } from "@playwright/test";
import { expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
const [origin, repository] = process.argv.slice(2);
if (
  !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin ?? "") ||
  !/^[a-f0-9]{32}$/.test(repository ?? "")
)
  throw Error("explicit disposable native origin and repository required");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1440, height: 1100 },
  });
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`${origin}/#/repos/${repository}/agents`);
  await page
    .getByRole("button", { name: "New agent session", exact: true })
    .click();
  const host = page.getByRole("dialog", {
    name: "Agent sessions",
    exact: true,
  });
  await expect(host).toBeVisible();
  await host
    .getByRole("button", { name: "Add saved setup", exact: true })
    .click();
  const editor = page.getByRole("dialog", {
    name: "New saved setup",
    exact: true,
  });
  await editor
    .getByLabel("Name", { exact: true })
    .fill("Browser qualification");
  await editor.getByRole("tab", { name: "Instructions", exact: true }).click();
  await editor
    .getByRole("textbox", { name: "Instructions", exact: true })
    .fill("Reply briefly and preserve the source folder.");
  await editor.getByRole("tab", { name: "Runtime", exact: true }).click();
  await editor
    .getByLabel("Execution mode preference", { exact: true })
    .fill("read-only");
  const setupResponse = page.waitForResponse(
    (response) =>
      response.url() === `${origin}/v1/local/agents/setups` &&
      response.request().method() === "POST",
  );
  await editor.getByRole("button", { name: "Save setup", exact: true }).click();
  const saved = await setupResponse;
  if (!saved.ok()) {
    const failure = await saved.json();
    throw Error(`Saved setup rejected: ${saved.status()} ${failure.code}`);
  }
  await expect(editor).not.toBeVisible();
  console.log("Cold browser: saved setup accepted");
  await host
    .getByLabel("Saved setup", { exact: true })
    .selectOption({ label: "🤖 Browser qualification" });
  await host
    .getByRole("button", { name: "Create session workspace", exact: true })
    .click();
  const composer = host.getByRole("textbox", {
    name: "Message to agent",
    exact: true,
  });
  await expect(composer).toBeVisible({ timeout: 90000 });
  console.log("Cold browser: isolated session connected");
  await expect(
    host.getByRole("button", { name: "Send", exact: true }),
  ).toBeDisabled();
  await composer.fill(
    "Reply with the phrase browser integration accepted. Do not run tools or change files.",
  );
  await expect(
    host.getByRole("button", { name: "Send", exact: true }),
  ).toBeEnabled({ timeout: 30000 });
  await host.getByRole("button", { name: "Send", exact: true }).click();
  await expect(host.getByText(/^browser integration accepted\.?$/)).toBeVisible(
    { timeout: 180000 },
  );
  await expect(host.getByText("Turn completed", { exact: true })).toBeVisible({
    timeout: 180000,
  });
  console.log("Cold browser: streamed turn completed");
  const axe = await new AxeBuilder({ page })
    .include('[role="dialog"]')
    .analyze();
  if (axe.violations.length)
    throw Error(
      `Agent accessibility violations: ${JSON.stringify(
        axe.violations.flatMap((v) =>
          v.nodes.map((node) => ({
            rule: v.id,
            target: node.target,
            reason: node.failureSummary,
          })),
        ),
      )}`,
    );
  console.log("Cold browser: accessibility passed");
  await host.getByRole("button", { name: "End session", exact: true }).click();
  await expect(
    host.getByRole("button", {
      name: "New session with saved context",
      exact: true,
    }),
  ).toBeVisible({ timeout: 30000 });
  await host.getByText("Workspace and runtime", { exact: true }).click();
  await host
    .getByRole("button", { name: "Review changes", exact: true })
    .click();
  await expect(host).not.toBeVisible();
  await expect(page).toHaveURL(new RegExp(`repos/${repository}/.*commit=`));
  if (errors.length)
    throw Error(`Production page errors: ${errors.join("; ")}`);
  console.log(
    "Cold production UI: saved setup, native isolated launch, streamed prompt, end, retained review navigation and accessibility passed",
  );
} finally {
  await browser.close();
}

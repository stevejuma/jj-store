// Cold production-browser qualification against the disposable native service.
import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { validateAgentAcceptanceOrigin } from "./agent-execution-browser-input.mjs";
const [origin, repository] = process.argv.slice(2);
validateAgentAcceptanceOrigin(origin, repository);
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1440, height: 1100 },
  });
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`${origin}/#/repos/${repository}/files`);
  await expect(
    page.getByRole("button", {
      name: "Open file fleet-acceptance.txt",
      exact: true,
    }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  const composer = page.getByRole("group", { name: "Message to agents" });
  await expect(composer).toBeVisible();
  const editor = composer.getByRole("textbox", { name: "Markdown editor" });
  await editor.fill("@Native");
  await page.getByRole("option", { name: /Native fleet helper/ }).click();
  await expect(editor).toContainText("Native fleet helper");
  await expect(
    composer.getByRole("img", {
      name: "Native fleet helper, Idle",
      exact: true,
    }),
  ).toBeVisible();
  await composer.getByLabel("Automatically mention agents").check();
  await editor.pressSequentially(
    'Reply with exactly "Browser fleet accepted." Do not access or modify files.',
  );
  const sent = page.waitForResponse(
    (response) =>
      response.url().endsWith("/v1/local/agents/chat") &&
      response.request().method() === "POST",
    { timeout: 120000 },
  );
  await composer.getByRole("button", { name: "Send message" }).click();
  const response = await sent;
  const attachment = response.request().postDataJSON().attachments[0];
  expect(attachment.operation).toMatch(/^[a-f0-9]{128}$/);
  expect(attachment.commit).toMatch(/^[a-f0-9]{128}$/);
  if (!response.ok()) throw Error(`contextual chat HTTP ${response.status()}`);
  const outcome = await response.json();
  const failures = outcome.agents.filter((agent) => agent.error);
  if (failures.length)
    throw Error(
      "contextual chat preparation failed: " +
        failures.map((agent) => agent.error).join(","),
    );
  if (
    !outcome.agents.length ||
    outcome.agents.some((agent) => !agent.session_id)
  )
    throw Error("contextual chat did not prepare participation");
  await expect(
    page.getByText("Browser fleet accepted.", { exact: true }),
  ).toBeVisible({ timeout: 180000 });
  await expect(editor).toContainText("Native fleet helper");
  await expect(
    composer.getByRole("img", {
      name: "Native fleet helper, Idle",
      exact: true,
    }),
  ).toBeVisible({ timeout: 15000 });
  await page.getByRole("button", { name: "Close", exact: true }).click();
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  await expect(
    composer.getByLabel("Automatically mention agents"),
  ).toBeChecked();
  const findings = await new AxeBuilder({ page })
    .include('[data-ui-component="agent-chat"]')
    .analyze();
  if (findings.violations.length)
    throw Error(
      "contextual chat accessibility violations: " +
        findings.violations.map((v) => v.id).join(","),
    );
  if (errors.length) throw Error("production browser runtime errors");
  console.log(
    "cold production browser sent a tagged native chat; canonical reply, retained pins and Axe passed",
  );
  await context.close();
} finally {
  await browser.close();
}

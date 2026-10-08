import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
const [origin, session] = process.argv.slice(2);
if (
  !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin) ||
  !/^[a-f0-9]{32}$/.test(session)
)
  throw Error("Expected isolated loopback origin and exact session");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
    viewport: { width: 1400, height: 1100 },
  });
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(
    `${origin}/#/agents/sessions/${session}?agent=context-fixture`,
  );
  await expect(
    page.getByText("First authored message", { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText("Second authored message", { exact: true }),
  ).toBeVisible();
  const inspectors = page.locator('[data-ui-component="agent-sent-context"]');
  await expect(inspectors).toHaveCount(2);
  const first = inspectors.first();
  await expect(first).not.toHaveAttribute("open");
  await first.locator(":scope > summary").focus();
  await page.keyboard.press("Enter");
  await expect(first).toHaveAttribute("open", "");
  await first
    .getByRole("button", { name: "<agent_context>", exact: true })
    .click();
  await first
    .getByRole("button", { name: "<instructions>", exact: true })
    .click();
  await expect(
    first.getByRole("heading", { name: "Standing instructions" }),
  ).toBeVisible();
  await expect(first.locator("strong", { hasText: "Markdown" })).toBeVisible();
  expect(
    await first.evaluate((node) =>
      Boolean(
        node.compareDocumentPosition(
          [...document.querySelectorAll(".community-message__content")].find(
            (n) => n.textContent === "First authored message",
          ),
        ) & Node.DOCUMENT_POSITION_FOLLOWING,
      ),
    ),
  ).toBe(true);
  const axe = await new AxeBuilder({ page })
    .include('[data-ui-component="agent-sent-context"]')
    .analyze();
  expect(axe.violations).toEqual([]);
  await page.reload();
  await expect(inspectors).toHaveCount(2);
  expect(errors).toEqual([]);
  console.log(
    "cold built UI: one context per applicable message, Markdown, keyboard, reload and Axe passed",
  );
} finally {
  await browser.close();
}

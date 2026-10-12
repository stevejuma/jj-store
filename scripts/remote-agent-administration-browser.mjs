/** Cold production App acceptance. Disposable credentials arrive on stdin only. */
import { mkdir } from "node:fs/promises";
import assert from "node:assert/strict";
import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
let input = "";
for await (const chunk of process.stdin) input += chunk;
const fixture = JSON.parse(input),
  errors = [],
  violations = [];
const browser = await chromium.launch({ headless: true });
async function signIn(token) {
  const context = await browser.newContext({
    viewport: { width: 1200, height: 900 },
  });
  const page = await context.newPage();
  page.setDefaultTimeout(20000);
  page.on("pageerror", (e) => errors.push(e.message));
  await page.addInitScript(() => {
    globalThis.__acceptanceCsp = [];
    document.addEventListener("securitypolicyviolation", (e) =>
      globalThis.__acceptanceCsp.push(e.violatedDirective),
    );
  });
  await page.goto(`${fixture.local}/#/agents`);
  await page
    .getByRole("button", { name: "Select Acceptance host server", exact: true })
    .click();
  await page.getByLabel("Access token", { exact: true }).fill(token);
  await page.getByRole("button", { name: "Connect", exact: true }).click();
  await page.waitForFunction(() => location.hash.includes("server="));
  await page.getByRole("dialog").waitFor({ state: "hidden" });
  await page.getByRole("button", { name: "Agents", exact: true }).click();

  await page
    .getByRole("button", { name: "Manage server access", exact: true })
    .click()
    .catch(async (error) => {
      console.error(
        "Cold agent page",
        await page.locator("body").innerText(),
        errors,
      );
      throw error;
    });
  return page;
}
async function axe(page) {
  const result = await new AxeBuilder({ page })
    .include('[role="dialog"]')
    .analyze();
  assert.deepEqual(
    result.violations.map((v) => ({
      id: v.id,
      nodes: v.nodes.map((n) => n.target),
    })),
    [],
  );
}
async function close(page) {
  await page.getByRole("button", { name: "Close server agent access" }).click();
  await page
    .getByRole("dialog", { name: "Server agent access" })
    .waitFor({ state: "hidden" });
}
try {
  const identityPage = await signIn(fixture.identities);
  assert.equal(
    await identityPage
      .getByRole("tab", { name: "Configuration and sharing" })
      .count(),
    0,
  );
  const enrollment = identityPage.getByRole("region", {
    name: "Enroll installation identity",
  });
  await expect(
    enrollment.getByLabel("User access token", { exact: true }),
  ).toHaveValue("");
  await enrollment
    .getByLabel("Principal ID", { exact: true })
    .fill(fixture.principal);
  await enrollment
    .getByLabel("User access token", { exact: true })
    .fill(fixture.user);
  const enrolled = identityPage.waitForResponse(
    (r) =>
      r.url().endsWith("/directory/identities") &&
      r.request().method() === "POST",
  );
  await enrollment
    .getByRole("button", { name: "Enroll token", exact: true })
    .click();
  assert.equal((await enrolled).status(), 200);
  await expect(
    enrollment.getByLabel("User access token", { exact: true }),
  ).toHaveValue("");
  const grant = identityPage.getByRole("region", {
    name: "Grant repository Write access",
  });
  for (const name of ["home", "code"]) {
    await grant.getByRole("button", { name: "Repository" }).click();
    await identityPage
      .getByRole("option", { name, exact: true })
      .click()
      .catch(async (error) => {
        await mkdir(".local/server-agent-review", { recursive: true });
        await identityPage.screenshot({
          path: ".local/server-agent-review/picker.png",
        });
        console.error(
          "Picker geometry",
          await identityPage
            .locator(
              "[data-select-content], [data-select-trigger], [data-floating-ui-content], [data-ui-dialog-frame]",
            )
            .evaluateAll((nodes) =>
              nodes.map((e) => ({
                slot: e.getAttribute("data-slot"),
                rect: e.getBoundingClientRect().toJSON(),
                style: e.getAttribute("style"),
                parent: e.parentElement?.getAttribute("style"),
              })),
            ),
        );
        throw error;
      });
    await expect(
      grant.getByLabel("User access token", { exact: true }),
    ).toHaveValue("");
    await grant
      .getByLabel("Principal ID", { exact: true })
      .fill(fixture.principal);
    await grant
      .getByLabel("User access token", { exact: true })
      .fill(fixture.user);
    const granted = identityPage.waitForResponse(
      (r) =>
        r.url().endsWith("/agent-access") && r.request().method() === "POST",
    );
    await grant
      .getByRole("button", { name: "Grant Write access", exact: true })
      .click();
    assert.equal((await granted).status(), 200);
    await expect(
      grant.getByLabel("User access token", { exact: true }),
    ).toHaveValue("");
  }
  await axe(identityPage);
  await mkdir(".local/server-agent-review", { recursive: true });
  await identityPage.screenshot({
    path: ".local/server-agent-review/onboarding.png",
  });
  await close(identityPage);
  violations.push(
    ...(await identityPage.evaluate(() => globalThis.__acceptanceCsp)),
  );
  await identityPage.close();
  const page = await signIn(fixture.operator);
  const dialog = page.getByRole("dialog", { name: "Server agent access" });
  await dialog
    .locator(`[role="checkbox"][id$="-${fixture.principal}"]`)
    .check();
  await dialog
    .getByRole("textbox", { name: "Name", exact: true })
    .fill("Remote builder");
  await dialog.getByRole("tab", { name: "Access onboarding" }).click();
  assert.equal(
    await dialog
      .getByRole("region", { name: "Enroll installation identity" })
      .count(),
    0,
  );
  await dialog.getByRole("tab", { name: "Configuration and sharing" }).click();
  assert.equal(
    await dialog
      .getByRole("textbox", { name: "Name", exact: true })
      .inputValue(),
    "Remote builder",
  );
  const saved = page.waitForResponse(
    (r) =>
      r.url().endsWith("/operator/agents") && r.request().method() === "POST",
  );
  await dialog.getByRole("button", { name: "Save configuration" }).click();
  assert.equal((await saved).status(), 200);
  await dialog
    .getByText("Configuration reload required", { exact: true })
    .waitFor();
  await page.waitForFunction(
    () =>
      !document.querySelector('[aria-label="Close server agent access"]')
        .disabled,
  );
  await axe(page);
  await page.screenshot({ path: ".local/server-agent-review/sharing.png" });
  await page.setViewportSize({ width: 390, height: 844 });
  assert(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  );
  assert(
    await dialog
      .getByRole("textbox", { name: "ACP executable" })
      .evaluate((e) => e.getBoundingClientRect().right <= innerWidth),
  );
  await axe(page);
  await close(page);
  const url = page.url();
  await page.reload();
  await page.getByLabel("Access token", { exact: true }).fill(fixture.operator);
  await page.getByRole("button", { name: "Connect", exact: true }).click();
  assert.equal(page.url(), url);
  await page
    .getByRole("button", { name: "Manage server access", exact: true })
    .click()
    .catch(async (error) => {
      await page.screenshot({ path: ".local/server-agent-review/reload.png" });
      console.error(
        "Reload route",
        page.url(),
        await page.locator("body").innerText(),
        errors,
      );
      throw error;
    });
  assert.equal(
    await page.getByRole("textbox", { name: "Name", exact: true }).inputValue(),
    "Remote builder",
  );
  assert(
    await page
      .locator(`[role="checkbox"][id$="-${fixture.principal}"]`)
      .isChecked(),
  );
  await close(page);
  await page.setViewportSize({ width: 1200, height: 900 });
  await page
    .getByRole("button", { name: "Select Local server", exact: true })
    .click();
  await page.waitForFunction(() => !location.hash.includes("server="));
  await page.goBack();
  await page.waitForFunction(() => location.hash.includes("server="));
  await page
    .getByRole("button", { name: "Manage server access", exact: true })
    .waitFor();
  const preferences = await page.evaluate(() =>
    JSON.stringify({ ...localStorage, ...sessionStorage }),
  );
  for (const token of [fixture.operator, fixture.identities, fixture.user])
    assert(!preferences.includes(token));
  violations.push(...(await page.evaluate(() => globalThis.__acceptanceCsp)));
  assert.deepEqual(errors, []);
  assert.deepEqual(violations, []);
  console.log(
    "PASS cold built App: independent role controls, identity/Write onboarding, sharing save, drafts, mobile, Axe, reload, Back, CSP",
  );
} finally {
  await browser.close();
}

// Run only against the isolated disposable host supplied by the RustFS qualification test.
import { chromium, expect as baseExpect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import assert from "node:assert/strict";
const expect = baseExpect.configure({ timeout: 60000 });
const [base, source, destination, operation] = process.argv.slice(2);
assert(base?.startsWith("http://127.0.0.1:"));
assert(/^[0-9a-f]{32}$/.test(source));
const browser = await chromium.launch({ headless: true });
try {
  const context = await browser.newContext({
    viewport: { width: 1400, height: 1000 },
  });
  const page = await context.newPage();
  const errors = [];
  page.on("response", async (response) => {
    if (
      response.url().includes("/repositories/conversion") &&
      !response.url().endsWith("preview")
    ) {
      const text = await response.text().catch(() => "");
      console.log("conversion response", response.status(), text.slice(0, 512));
    }
  });
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(`${base}/#/repos/${source}/files`);
  async function settings(label) {
    await page.getByRole("button", { name: /Open profile menu/ }).click();
    await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
    await page.getByRole("button", { name: "Repository", exact: true }).click();
    await page.getByRole("button", { name: label, exact: true }).click();
    return page.getByRole("dialog", { name: "Convert repository" });
  }
  async function selectOperation(dialog, expected) {
    await dialog.getByRole("button", { name: "Committed operation" }).click();
    const option = expected
      ? page.getByRole("option", { name: expected, exact: true })
      : page.getByRole("option").first();
    await expect(option).toBeVisible();
    await option.click();
  }
  async function settle(dialog) {
    await expect
      .poll(
        async () => {
          if (!(await dialog.isVisible())) return true;
          const status = dialog.getByRole("button", { name: "Check status" });
          if ((await status.isVisible()) && (await status.isEnabled()))
            await status.click();
          return !(await dialog.isVisible());
        },
        { timeout: 60000, intervals: [1000] },
      )
      .toBe(true);
  }
  let dialog = await settings("Use server storage");
  await dialog
    .getByRole("textbox", { name: "Destination name" })
    .fill("Browser native");
  await selectOperation(dialog, operation);
  await dialog.getByRole("button", { name: "Check conversion" }).click();
  await expect(
    dialog.getByRole("button", { name: "Convert and open destination" }),
  ).toBeEnabled();
  assert.deepEqual((await new AxeBuilder({ page }).analyze()).violations, []);
  await dialog
    .getByRole("button", { name: "Convert and open destination" })
    .click();
  await settle(dialog);
  await expect(page).toHaveURL(
    new RegExp(`/repos/(?!${source})[0-9a-f]{32}/files`),
  );
  const native = page.url().match(/\/repos\/([0-9a-f]{32})/)[1];
  dialog = await settings("Use local Git-backed JJ");
  await dialog
    .getByRole("textbox", { name: "Destination name" })
    .fill("Browser local");
  await dialog.getByRole("textbox", { name: "New folder" }).fill(destination);
  await selectOperation(dialog);
  await dialog.getByRole("button", { name: "Check conversion" }).click();
  await expect(
    dialog.getByRole("button", { name: "Convert and open destination" }),
  ).toBeEnabled();
  await dialog
    .getByRole("button", { name: "Convert and open destination" })
    .click();
  await settle(dialog);
  await expect(page).toHaveURL(
    new RegExp(`/repos/(?!${native}|${source})[0-9a-f]{32}/files`),
  );
  await page.getByRole("button", { name: /Open profile menu/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("button", { name: "Last conversion report" }).click();
  const report = page.getByRole("dialog", { name: "Conversion report" });
  const downloaded = page.waitForEvent("download");
  await report.getByRole("button", { name: "Download mapping report" }).click();
  const stream = await (await downloaded).createReadStream();
  const chunks = [];
  for await (const chunk of stream) chunks.push(chunk);
  const result = JSON.parse(Buffer.concat(chunks).toString());
  assert.equal(result.source, native);
  assert.equal(result.destination_profile, "local_jj_git");
  assert(result.collaboration.selections.length > 0);
  assert.deepEqual(errors, []);
  console.log(
    "Cold production browser: both directions, report download, accessibility and navigation passed.",
  );
} finally {
  await browser.close();
}

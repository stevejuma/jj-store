import { chromium, expect } from "@playwright/test";
const [origin] = process.argv.slice(2);
const url = new URL(origin);
if (url.hostname !== "127.0.0.1" || url.protocol !== "http:")
  throw Error("Isolated loopback fixture required");
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(origin);
  await page.getByRole("button", { name: /Open profile menu for/ }).click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page
    .getByRole("navigation", { name: "Settings navigation" })
    .getByRole("button", { name: "Compute", exact: true })
    .click();
  await expect(
    page.getByRole("region", { name: "Compute configuration", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Stop compute", exact: true }),
  ).toBeVisible();
  expect(errors).toEqual([]);
  console.log("Cold production Compute settings browser: passed");
} finally {
  await browser.close();
}

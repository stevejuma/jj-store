// Cold production-App acceptance with deterministic API fixtures; no existing service/data use.
import { spawn } from "node:child_process";
import { chromium } from "@playwright/test";
import assert from "node:assert/strict";
const port = 7590,
  origin = `http://127.0.0.1:${port}`;
const server = spawn(
  process.execPath,
  [
    "node_modules/vite/bin/vite.js",
    "--config",
    "web/vite.config.ts",
    "--host",
    "127.0.0.1",
    "--port",
    String(port),
    "--strictPort",
  ],
  { stdio: ["ignore", "pipe", "pipe"] },
);
let log = "";
for (const output of [server.stdout, server.stderr])
  output.on("data", (chunk) => {
    log += chunk;
  });
let browser, page;
const errors = [];
try {
  const deadline = Date.now() + 45000;
  while (true) {
    try {
      if ((await fetch(origin)).ok) break;
    } catch {}
    if (Date.now() > deadline || server.exitCode !== null) throw new Error(log);
    await new Promise((r) => setTimeout(r, 100));
  }
  browser = await chromium.launch({ headless: true });
  page = await browser.newPage({
    viewport: { width: 1200, height: 900 },
  });
  page.on("pageerror", (error) => errors.push(error.message));
  const connection = {
    id: "a".repeat(32),
    name: "Build host",
    origin: "https://build.example",
    installation: "b".repeat(32),
  };
  const calls = [];
  await page.route("**/v1/**", async (route) => {
    const request = route.request(),
      path = new URL(request.url()).pathname;
    calls.push({ path, headers: request.headers() });
    let value,
      status = 200;
    if (path === "/v1/local/connections") value = [connection];
    else if (path === "/v1/local/connections/connect")
      value = { connection, handle: "c".repeat(32), principal: "d".repeat(32) };
    else if (path === "/v1/local/connections/disconnect")
      value = { disconnected: true };
    else if (path.endsWith("/bootstrap"))
      value = {
        version: 1,
        access_mode: path.includes("/api/") ? "authenticated" : "local_trusted",
        storage: { ready: true },
        capabilities: {},
        backups: [],
      };
    else if (path.endsWith("/repositories"))
      value = { repositories: [], next_cursor: null };
    else {
      value = { code: "not_available" };
      status = 503;
    }
    await route.fulfill({
      status,
      contentType: "application/json",
      body: JSON.stringify(value),
    });
  });
  await page.goto(`${origin}/#/settings?section=appearance&tab=theme`);
  await page
    .getByRole("button", { name: "Select Local server", exact: true })
    .waitFor();
  assert.equal(
    await page
      .getByRole("button", { name: "Select Local server", exact: true })
      .getAttribute("aria-pressed"),
    "true",
  );
  await page.getByRole("button", { name: "Add server", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Name", exact: true })
    .fill("Build host");
  await page
    .getByRole("textbox", { name: "Server address", exact: true })
    .fill(connection.origin);
  await page
    .getByLabel("Access token", { exact: true })
    .fill("memory-only-token");
  await page.getByRole("button", { name: "Connect", exact: true }).click();
  await page
    .getByRole("button", { name: "Select Build host server", exact: true })
    .waitFor();
  await page.waitForFunction(() => location.hash.includes("server="));
  await page.evaluate((id) => {
    location.hash = `#/settings?section=appearance&tab=theme&server=${id}`;
  }, connection.id);
  await page.getByRole("navigation", { name: "Settings navigation" }).waitFor();
  await page.getByRole("button", { name: "Appearance", exact: true }).waitFor();
  assert.equal(
    await page
      .getByRole("button", { name: "Local service", exact: true })
      .count(),
    0,
  );
  assert.equal(
    await page.getByRole("button", { name: "Compute", exact: true }).count(),
    0,
  );
  assert.equal(
    await page.getByRole("button", { name: "Agents", exact: true }).count(),
    0,
  );
  await page
    .getByRole("button", { name: "Select Local server", exact: true })
    .click();
  await page.waitForFunction(
    () =>
      location.hash.includes("section=appearance") &&
      !location.hash.includes("server="),
  );
  await page.goBack();
  await page.waitForFunction(() => location.hash.includes("server="));
  await page.goForward();
  await page.waitForFunction(() => !location.hash.includes("server="));
  await page
    .getByRole("button", { name: "Select Build host server", exact: true })
    .click();
  const remoteUrl = page.url();
  await page.reload();
  await page.getByLabel("Access token", { exact: true }).waitFor();
  assert.equal(page.url(), remoteUrl);
  const preferences = await page.evaluate(() =>
    localStorage.getItem("jj-store/connections/v1"),
  );
  assert(preferences);
  assert.equal(JSON.parse(preferences).selected, connection.id);
  assert(!JSON.stringify(preferences).includes("memory-only-token"));
  await page.goto(`${origin}/#/settings?section=appearance&tab=theme`);
  assert.equal(
    await page
      .getByRole("button", { name: "Select Local server", exact: true })
      .getAttribute("aria-pressed"),
    "true",
  );
  assert(
    calls.some((c) =>
      c.path.includes(`/connections/${connection.id}/api/bootstrap`),
    ),
  );
  assert(
    calls
      .filter((c) => c.path.includes("/api/"))
      .every(
        (c) =>
          !c.headers.authorization &&
          c.headers["x-jj-store-connection-session"] === "c".repeat(32),
      ),
  );
  assert.deepEqual(errors, []);
  console.log(
    "PASS cold production App: scoped selection, Back/Forward, reload authentication, explicit Local and memory-only credentials",
  );
} catch (cause) {
  console.error(log, errors);
  if (page) {
    console.error(await page.locator("body").innerText());
    await page.screenshot({ path: "/tmp/jj-remote-cold-failure.png" });
  }
  throw cause;
} finally {
  await browser?.close();
  server.kill("SIGTERM");
}

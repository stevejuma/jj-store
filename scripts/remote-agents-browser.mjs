/** Cold built-App acceptance against two actual servers; credentials arrive only on stdin. */
import assert from "node:assert/strict";
import { chromium } from "@playwright/test";
let input = "";
for await (const chunk of process.stdin) input += chunk;
const fixture = JSON.parse(input);
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1200, height: 900 } });
page.setDefaultTimeout(15000);
const errors = [],
  violations = [],
  calls = [];
page.on("pageerror", (error) => errors.push(error.message));
page.on("request", (request) => {
  if (request.url().includes("/v1/"))
    calls.push({ url: request.url(), headers: request.headers() });
});
try {
  await page.addInitScript(() => {
    globalThis.__remoteAcceptanceCsp = [];
    document.addEventListener("securitypolicyviolation", (event) => {
      globalThis.__remoteAcceptanceCsp.push({
        directive: event.violatedDirective,
        blocked: event.blockedURI,
      });
    });
  });
  await page.goto(`${fixture.local}/#/settings?section=appearance&tab=theme`);
  await page
    .getByRole("button", { name: "Select Build host server", exact: true })
    .click();
  await page.getByLabel("Access token", { exact: true }).fill(fixture.token);
  await page.getByRole("button", { name: "Connect", exact: true }).click();
  await page.waitForFunction(() => location.hash.includes("server="));
  const selected = page.getByRole("button", {
    name: "Select Build host server",
    exact: true,
  });
  assert.equal(await selected.getAttribute("aria-pressed"), "true");
  await page.evaluate((id) => {
    location.hash = `#/settings?section=profile&tab=details&server=${id}`;
  }, fixture.connection.id);
  await page
    .getByText("Remote collaborator", { exact: true })
    .first()
    .waitFor();
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
  const saved = await page.evaluate(async () => {
    const connection = await fetch("/v1/local/connections", {
      headers: { "x-jj-store-protocol": "1" },
    }).then((r) => r.json());
    return connection;
  });
  assert.equal(saved[0].id, fixture.connection.id);
  const fresh = await page.evaluate(
    async ({ id, origin, token }) => {
      const r = await fetch("/v1/local/connections/connect", {
        method: "POST",
        headers: {
          "x-jj-store-protocol": "1",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ id, name: "Build host", origin, token }),
      });
      if (!r.ok) throw Error(`connect ${r.status}`);
      return r.json();
    },
    { id: fixture.connection.id, origin: fixture.remote, token: fixture.token },
  );
  const event = await page.evaluate(
    async ({ id, handle }) => {
      return new Promise((resolve, reject) => {
        const url = new URL(
          `/v1/local/connections/${id}/events`,
          location.origin,
        );
        url.protocol = "ws:";
        const ws = new WebSocket(url, "jj-store-events-v1");
        const timer = setTimeout(() => {
          ws.close();
          reject(Error("freshness deadline"));
        }, 15000);
        ws.onopen = () =>
          ws.send(JSON.stringify({ version: 1, token: handle, after: null }));
        ws.onmessage = async (frame) => {
          try {
            const delivered = JSON.parse(frame.data);
            if (delivered.event?.type === "ready") {
              const headers = {
                "x-jj-store-protocol": "1",
                "x-jj-store-connection-session": handle,
              };
              const prefix = `/v1/local/connections/${id}/api/directory/`;
              const profile = await fetch(prefix + "profile", { headers }).then(
                (r) => r.json(),
              );
              const saved = await fetch(prefix + "commands", {
                method: "POST",
                headers: { ...headers, "Content-Type": "application/json" },
                body: JSON.stringify({
                  request_id: crypto.randomUUID().replaceAll("-", ""),
                  command: {
                    type: "save_profile",
                    expected: profile.revision,
                    details: {
                      ...profile.details,
                      description: "Remote freshness accepted",
                    },
                  },
                }),
              });
              if (!saved.ok) throw Error(`profile save ${saved.status}`);
            } else if (
              delivered.event?.type === "changed" &&
              delivered.event.change.domain === "profiles"
            ) {
              clearTimeout(timer);
              ws.close();
              resolve(delivered);
            }
          } catch (error) {
            clearTimeout(timer);
            ws.close();
            reject(error);
          }
        };
        ws.onerror = () => {
          clearTimeout(timer);
          reject(Error("freshness transport failed"));
        };
        ws.onclose = (frame) => {
          clearTimeout(timer);
          reject(Error(`freshness closed ${frame.code}`));
        };
      });
    },
    { id: fixture.connection.id, handle: fresh.handle },
  );
  assert.equal(event.event.type, "changed");
  assert.equal(event.event.change.domain, "profiles");
  assert.equal(typeof event.id, "string");
  const media = await page.evaluate(
    async ({ id, handle, avatar }) => {
      const response = await fetch(
        `/v1/local/connections/${id}/api/directory/media/${avatar.replace("blob:", "")}`,
        {
          headers: {
            "x-jj-store-protocol": "1",
            "x-jj-store-connection-session": handle,
          },
        },
      );
      if (!response.ok) throw Error(`media ${response.status}`);
      return [...new Uint8Array(await response.arrayBuffer())];
    },
    { id: fixture.connection.id, handle: fresh.handle, avatar: fixture.avatar },
  );
  assert.deepEqual(media, fixture.image);
  await page
    .getByRole("button", { name: "Select Local server", exact: true })
    .click();
  await page.waitForFunction(() => !location.hash.includes("server="));
  await page.goBack();
  await page.waitForFunction(() => location.hash.includes("server="));
  await page
    .getByText("Remote collaborator", { exact: true })
    .first()
    .waitFor();
  await page.goForward();
  await page.waitForFunction(() => !location.hash.includes("server="));
  await selected.click();
  await page
    .getByText("Remote collaborator", { exact: true })
    .first()
    .waitFor();
  const remoteUrl = page.url();
  violations.push(
    ...(await page.evaluate(() => globalThis.__remoteAcceptanceCsp)),
  );
  await page.reload();
  await page.getByLabel("Access token", { exact: true }).waitFor();
  assert.equal(page.url(), remoteUrl);
  await page.getByLabel("Access token", { exact: true }).fill(fixture.token);
  await page.getByRole("button", { name: "Connect", exact: true }).click();
  await page
    .getByText("Remote collaborator", { exact: true })
    .first()
    .waitFor();
  const persisted = await page.evaluate(() => ({
    ...localStorage,
    ...sessionStorage,
  }));
  assert(!JSON.stringify(persisted).includes(fixture.token));
  assert(!JSON.stringify(persisted).includes(fresh.handle));
  violations.push(
    ...(await page.evaluate(() => globalThis.__remoteAcceptanceCsp)),
  );
  await page.goto(`${fixture.local}/#/settings?section=appearance&tab=theme`);
  await page
    .getByRole("button", { name: "Select Local server", exact: true })
    .waitFor();
  assert.equal(
    await page
      .getByRole("button", { name: "Select Local server", exact: true })
      .getAttribute("aria-pressed"),
    "true",
  );
  violations.push(
    ...(await page.evaluate(() => globalThis.__remoteAcceptanceCsp)),
  );
  assert.deepEqual(errors, []);
  assert.deepEqual(violations, []);
  const remoteCalls = calls.filter((c) => c.url.includes("/api/"));
  assert(remoteCalls.length > 0);
  assert(
    remoteCalls.every(
      (c) =>
        !c.headers.authorization && c.headers["x-jj-store-connection-session"],
    ),
  );
  console.log(
    "PASS cold built App: gateway events, avatar bytes, scoped Settings, Back/Forward, reload sign-in, explicit Local, CSP",
  );
} catch (cause) {
  console.error(
    "Browser acceptance:",
    errors,
    await page.evaluate(() => globalThis.__remoteAcceptanceCsp),
  );
  console.error(await page.locator("body").innerText());
  throw cause;
} finally {
  await browser.close();
}

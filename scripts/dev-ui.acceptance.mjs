#!/usr/bin/env node
import { developmentApi } from "./dev-ui-api.mjs";
import { developmentFixtureIdentity } from "./dev-ui-identity.mjs";
import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, rm, writeFile, copyFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { chromium } from "@playwright/test";
import { createUiNativeFixture } from "./dev-ui-native-fixture.mjs";
import { runUiDevelopment } from "./dev-ui.mjs";
import { stopPrevious, execute } from "./dev-demo-services.mjs";

const root = resolve(import.meta.dirname, "..");
const fixture = await createUiNativeFixture(root, {
  ...process.env,
  JJ_STORE_DEV_PORT: process.env.JJ_STORE_DEV_PORT ?? "7480",
  JJ_STORE_DEV_BACKEND_PORT: process.env.JJ_STORE_DEV_BACKEND_PORT ?? "7482",
  JJ_STORE_DEV_BROWSER_ANNOTATIONS: "0",
});
const { layout, repositoryId, environment } = fixture;
const probe = join(root, "web/.dev-ui-acceptance");
const deniedProbe = join(root, ".local", `hmr-deny-${process.pid}.txt`);
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext();
await context.route(
  (url) =>
    url.hostname === "www.gravatar.com" &&
    /^\/avatar\/[a-f0-9]{32}$/u.test(url.pathname) &&
    url.searchParams.get("d") === "identicon" &&
    url.searchParams.get("s") === "80",
  (route) => route.abort(),
);
const page = await context.newPage();
const browserErrors = [];
page.on("pageerror", (error) => {
  browserErrors.push(error);
  console.error("Test browser runtime error:", error);
});
page.on("console", (message) => {
  if (message.type() === "error")
    console.error("Test browser console error:", message.text());
});
const identity = () => developmentFixtureIdentity(layout);
const initialIdentity = await identity();
let runtime;
let phase = "startup";
try {
  runtime = await runUiDevelopment(root, environment);
  const { api, agentsAvailable } = developmentApi(runtime.origin);
  await agentsAvailable();
  const html = await (await fetch(runtime.origin)).text();
  assert.ok(
    html.includes("/@vite/client"),
    "UI must serve Vite source rather than built HTML",
  );
  await mkdir(join(root, ".local"), { recursive: true });
  await writeFile(deniedProbe, "private development state sentinel", {
    mode: 0o600,
  });
  const privateFile = await fetch(`${runtime.origin}@fs${deniedProbe}`);
  assert.equal(
    privateFile.status,
    403,
    "Vite must not expose development state",
  );
  await page.goto(runtime.origin);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .waitFor();
  await page
    .getByText("Native HMR acceptance", { exact: true })
    .first()
    .waitFor();
  phase = "API guards";
  assert.equal(
    await page.evaluate(
      async () =>
        (
          await fetch("/v1/bootstrap", {
            headers: { "x-jj-store-protocol": "1" },
          })
        ).status,
    ),
    200,
  );
  const wrongOrigin = await fetch(`${runtime.origin}v1/bootstrap`, {
    headers: { ...headers, Origin: "http://127.0.0.1:1234" },
  });
  assert.equal(wrongOrigin.status, 403, "Proxy must retain origin rejection");
  const wrongHost = await fetch(`${runtime.apiOrigin}v1/bootstrap`, {
    headers,
  });
  assert.equal(
    wrongHost.status,
    403,
    "Private listener must still require the public authority",
  );
  const streamAbort = new AbortController();
  phase = "SSE and native write";
  const events = await fetch(
    `${runtime.origin}v1/repos/${repositoryId}/events`,
    {
      headers,
      signal: AbortSignal.any([streamAbort.signal, AbortSignal.timeout(10000)]),
    },
  );
  assert.equal(events.status, 200);
  assert.match(events.headers.get("content-type"), /text\/event-stream/u);
  const reader = events.body.getReader();
  let stream = new TextDecoder().decode((await reader.read()).value);
  const outcome = await page.evaluate(
    async ({ repositoryId, id }) => {
      const response = await fetch(`/v1/repos/${repositoryId}/work/commands`, {
        method: "POST",
        headers: {
          "x-jj-store-protocol": "1",
          "content-type": "application/json",
        },
        body: JSON.stringify({
          version: 1,
          request_id: id,
          command: {
            command: "create_project",
            id,
            title: "Native HMR write",
            body: "Disposable development acceptance",
          },
        }),
      });
      return { status: response.status, value: await response.json() };
    },
    { repositoryId, id: randomUUID().replaceAll("-", "") },
  );
  assert.equal(outcome.status, 200);
  assert.equal(outcome.value.status, "committed");
  while (!stream.includes("repository-changed")) {
    const chunk = await reader.read();
    assert.equal(chunk.done, false);
    stream += new TextDecoder().decode(chunk.value);
  }
  streamAbort.abort();
  await reader.cancel().catch(() => {});
  const snapshot = await api(`repos/${repositoryId}/snapshot`);
  phase = "HMR";
  await mkdir(probe);
  await writeFile(
    join(probe, "index.html"),
    '<!doctype html><html><body><div id="probe"></div><script type="module" src="./main.ts"></script></body></html>',
  );
  await writeFile(
    join(probe, "main.ts"),
    'import { mount } from "svelte"; import Probe from "./Probe.svelte"; import "./probe.css"; mount(Probe, { target: document.getElementById("probe")! });',
  );
  await writeFile(join(probe, "label.ts"), 'export const label = "First";');
  const component =
    '<script lang="ts">import {label} from "./label.ts";</script><p>{label} component-before</p>';
  await writeFile(join(probe, "Probe.svelte"), component);
  await writeFile(join(probe, "probe.css"), ":root { --hmr-probe: before; }");
  await page.goto(`${runtime.origin}.dev-ui-acceptance/index.html`);
  await page.getByText("First component-before", { exact: true }).waitFor();
  await page.evaluate(() => {
    window.__hmrProbe = "same-document";
  });
  await writeFile(
    join(probe, "Probe.svelte"),
    component.replace("component-before", "component-after"),
  );
  await page.getByText("First component-after", { exact: true }).waitFor();
  await writeFile(join(probe, "label.ts"), 'export const label = "Second";');
  await page.getByText("Second component-after", { exact: true }).waitFor();
  await writeFile(join(probe, "probe.css"), ":root { --hmr-probe: after; }");
  await page.waitForFunction(
    () =>
      getComputedStyle(document.documentElement)
        .getPropertyValue("--hmr-probe")
        .trim() === "after",
  );
  assert.equal(
    await page.evaluate(() => window.__hmrProbe),
    "same-document",
    "HMR must update without document reload",
  );
  assert.deepEqual(await api(`repos/${repositoryId}/snapshot`), snapshot);
  phase = "backend restart";
  const restarted = await runUiDevelopment(root, environment, true);
  await agentsAvailable();
  assert.equal(restarted.uiPid, runtime.uiPid);
  assert.notEqual(restarted.serverPid, runtime.serverPid);
  assert.deepEqual(await identity(), initialIdentity);
  assert.deepEqual(await api(`repos/${repositoryId}/snapshot`), snapshot);
  assert.equal(await page.evaluate(() => window.__hmrProbe), "same-document");
  assert.deepEqual(
    browserErrors,
    [],
    "Cold application and HMR must have no browser runtime errors",
  );
  phase = "stop";
  await stopPrevious({
    owner: root,
    runtimeRoot: layout.runtimeRoot,
    runtimeFile: layout.runtimeFile,
  });
  assert.throws(() => process.kill(restarted.uiPid, 0), { code: "ESRCH" });
  assert.throws(() => process.kill(restarted.serverPid, 0), { code: "ESRCH" });
  assert.deepEqual(await identity(), initialIdentity);
  console.log(
    JSON.stringify(
      {
        nativeWrites: "committed through browser proxy",
        streaming: "native SSE ready and changed",
        hmr: ["Svelte", "TypeScript", "CSS"],
        documentReloads: 0,
        backendRestart: "Vite PID retained",
        agentExecution: "bundled runtime available before and after restart",
        originGuards: "wrong Origin and Host rejected",
        stop: "both processes stopped, fixture retained",
        runtimeRoot: layout.runtimeRoot,
      },
      null,
      2,
    ),
  );
  if (process.env.JJ_STORE_UI_TEST_FULL_RESTART === "1") {
    const active = await runUiDevelopment(root, environment);
    const result = await execute("pnpm", ["dev:restart"], {
      cwd: root,
      env: environment,
    });
    console.log(result.stdout);
    await agentsAvailable();
    assert.throws(() => process.kill(active.uiPid, 0), { code: "ESRCH" });
    assert.throws(() => process.kill(active.serverPid, 0), { code: "ESRCH" });
    const built = await readFile(join(root, "web/dist/index.html"));
    assert.ok(
      Buffer.from(await (await fetch(runtime.origin)).arrayBuffer()).equals(
        built,
      ),
    );
    assert.deepEqual(await identity(), initialIdentity);
    const projects = await api(`repos/${repositoryId}/work?limit=256`);
    assert.ok(
      projects.projects.some((project) => project.title === "Native HMR write"),
      "Full restart must retain the project committed through Vite",
    );
    await stopPrevious({
      owner: root,
      runtimeRoot: layout.runtimeRoot,
      runtimeFile: layout.runtimeFile,
    });
    console.log(
      "Full-stack restart smoke passed with built UI and retained native fixture identity.",
    );
  }
} catch (error) {
  console.error(`Native HMR acceptance failed during ${phase}:`, error);
  console.error(
    "Rendered test page:",
    await page
      .locator("body")
      .innerText()
      .catch(() => "unavailable"),
  );
  for (const name of ["ui.log", "server.log", "runtime.json"])
    await copyFile(
      join(layout.runtimeRoot, name),
      `/tmp/jj-hmr-native-failed-${name}`,
    ).catch(() => {});
  throw error;
} finally {
  await context.close();
  await browser.close();
  await stopPrevious({
    owner: root,
    runtimeRoot: layout.runtimeRoot,
    runtimeFile: layout.runtimeFile,
  });
  await rm(probe, { recursive: true, force: true });
  await rm(deniedProbe, { force: true });
  await rm(layout.runtimeRoot, { recursive: true, force: true });
}

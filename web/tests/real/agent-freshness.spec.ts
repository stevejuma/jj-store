import { randomUUID } from "node:crypto";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { expect, test } from "@playwright/test";

test("cold agent directory is idle, follows scoped changes across tabs and recovers missed events", async ({
  page,
  context,
  request,
  baseURL,
}) => {
  test.skip(
    process.env.JJ_STORE_REAL_AGENT_EVENTS !== "1",
    "Requires the isolated agent freshness fixture",
  );
  test.setTimeout(120000);
  const headers = {
    Origin: new URL(baseURL!).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const id = () => randomUUID().replaceAll("-", "");
  const read = async (path: string) => {
    const response = await request.get(`/v1/local/agents${path}`, { headers });
    expect(response.ok()).toBe(true);
    return response.json();
  };
  const post = async (path: string, data: unknown) => {
    const response = await request.post(`/v1/local/agents${path}`, {
      headers,
      data,
    });
    expect(response.ok()).toBe(true);
    return response.json();
  };
  let setup = {
    id: id(),
    name: "Event test agent",
    avatar: "🤖",
    description: "Isolated freshness acceptance",
    instructions: "",
    runner: "codex",
    preferences: { model: null, effort: null, mode: "ask" },
  };
  const save = async (name: string) => {
    setup = { ...setup, name };
    await post("/setups", {
      version: 1,
      request_id: id(),
      expected_revision: (await read("")).revision,
      setup,
      remove: null,
    });
  };
  await save(setup.name);
  const observer = await context.newPage();
  const counts = { status: 0, settings: 0, targets: 0 };
  const changes = (target: typeof page) =>
    target.on("request", (r) => {
      if (r.method() !== "GET") return;
      const path = new URL(r.url()).pathname;
      if (path === "/v1/local/agents") counts.status++;
      if (path === "/v1/local/agents/settings") counts.settings++;
      if (path === "/v1/local/agents/targets") counts.targets++;
    });
  changes(page);
  changes(observer);
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  observer.on("pageerror", (error) => errors.push(error.message));
  try {
    await Promise.all([page.goto("/#/agents"), observer.goto("/#/agents")]);
    for (const target of [page, observer])
      await expect(
        target.getByRole("button", { name: `View ${setup.name}`, exact: true }),
      ).toBeVisible();
    await page.waitForTimeout(1000);
    const idle = { ...counts };
    await page.waitForTimeout(12000);
    expect(counts).toEqual(idle);
    await save("Agent updated by another client");
    for (const target of [page, observer])
      await expect(
        target.getByRole("button", { name: `View ${setup.name}`, exact: true }),
      ).toBeVisible({ timeout: 5000 });
    expect(counts.settings).toBe(idle.settings);
    expect(counts.targets).toBe(idle.targets);
    const beforeSettings = { ...counts };
    const settings = await read("/settings");
    await post("/settings", {
      version: 1,
      request_id: id(),
      expected_revision: settings.revision,
      settings: { ...settings, keep_awake: !settings.keep_awake },
    });
    await expect.poll(() => counts.settings).toBe(beforeSettings.settings + 2);
    expect(counts.targets).toBe(beforeSettings.targets);
    const beforeCode = { ...counts };
    await promisify(execFile)(process.env.JJ_STORE_REAL_AGENT_JJ!, [
      "-R",
      process.env.JJ_STORE_REAL_AGENT_REPOSITORY_PATH!,
      "describe",
      "-m",
      "External code update invalidates agent targets",
    ]);
    await expect
      .poll(() => counts.targets, { timeout: 10000 })
      .toBeGreaterThan(beforeCode.targets);
    expect(counts.settings).toBe(beforeCode.settings);
    await context.setOffline(true);
    await page.waitForTimeout(500);
    await save("Agent updated during disconnection");
    await context.setOffline(false);
    for (const target of [page, observer])
      await expect(
        target.getByRole("button", { name: `View ${setup.name}`, exact: true }),
      ).toBeVisible({ timeout: 20000 });
    await page.waitForTimeout(1000);
    const recovered = { ...counts };
    await page.waitForTimeout(6000);
    expect(counts).toEqual(recovered);
    expect(errors).toEqual([]);
    console.log(
      JSON.stringify({
        idle,
        recovered,
        steadyState: "zero periodic agent GETs",
        externalChanges: "setup, settings, code, disconnected replay",
      }),
    );
  } finally {
    await context.setOffline(false);
    await observer.close();
  }
});

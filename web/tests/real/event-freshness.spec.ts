import { expect, test, type Page } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { randomUUID } from "node:crypto";
const runtime = JSON.parse(
  readFileSync(
    resolve(process.env.JJ_STORE_REAL_RUNTIME!, "runtime.json"),
    "utf8",
  ),
);
const home = runtime.repositoryId as string;
const local = runtime.repositoryProfiles.localGit as string;
const headers = (baseURL: string) => ({
  Origin: new URL(baseURL).origin,
  "Sec-Fetch-Dest": "empty",
  "Sec-Fetch-Mode": "cors",
  "Sec-Fetch-Site": "same-origin",
  "X-JJ-Store-Protocol": "1",
});
function traffic(page: Page) {
  const data: string[] = [],
    events: string[] = [],
    errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (!path.startsWith("/v1/")) return;
    (path === "/v1/events" ? events : data).push(path);
  });
  return { data, events, errors };
}
function publishLocalOperation() {
  return execFileSync(
    resolve(process.env.JJ_STORE_REAL_ROOT!, "target/debug/jj-store-jj"),
    [
      "-R",
      runtime.repositoryProfiles.localFolder,
      "--no-pager",
      "describe",
      "-m",
      `External freshness ${randomUUID()}`,
    ],
    {
      stdio: "pipe",
      env: {
        ...process.env,
        JJ_CONFIG: resolve(process.env.JJ_STORE_REAL_RUNTIME!, "jj.toml"),
      },
    },
  );
}
test("cold native and filesystem views stay idle through real five-minute renewal", async ({
  page,
  context,
}) => {
  test.setTimeout(390000);
  const a = traffic(page),
    observer = await context.newPage(),
    b = traffic(observer);
  await page.goto(`/#/repos/${home}/files`);
  await observer.goto(`/#/repos/${local}/files`);
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(
    observer.getByRole("tab", { name: "Overview", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect.poll(() => a.events.length).toBe(1);
  await expect.poll(() => b.events.length).toBe(1);
  await page.waitForTimeout(5000);
  a.data.length = 0;
  b.data.length = 0;
  await page.waitForTimeout(65000);
  expect(a.data).toEqual([]);
  expect(b.data).toEqual([]);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Storage", exact: true }).click();
  await page.waitForTimeout(3000);
  a.data.length = 0;
  await page.waitForTimeout(240000);
  expect(a.data).toEqual([]);
  expect(b.data).toEqual([]);
  expect(a.events).toHaveLength(2);
  expect(b.events).toHaveLength(2);
  expect(a.errors).toEqual([]);
  expect(b.errors).toEqual([]);
  await observer.close();
});
test("an external local operation triggers a committed refresh within five seconds", async ({
  page,
}) => {
  const t = traffic(page);
  await page.goto(`/#/repos/${local}/files`);
  await expect.poll(() => t.events.length).toBe(1);
  await page.waitForTimeout(3000);
  const retained = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("at_op");
  expect(retained).toMatch(/^[0-9a-f]{128}$/);
  t.data.length = 0;
  const changed = page.waitForRequest(
    (request) =>
      new URL(request.url()).pathname === `/v1/local/repos/${local}/snapshot`,
    { timeout: 5000 },
  );
  const started = Date.now();
  publishLocalOperation();
  await changed;
  expect(Date.now() - started).toBeLessThanOrEqual(5000);
  await expect
    .poll(() =>
      new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("at_op"),
    )
    .not.toBe(retained);
  // An explicit historical route remains available after a further publication.
  await page.goto(`/#/repos/${local}/files?at_op=${retained}`);
  await page.waitForTimeout(1500);
  const historicalRefresh = page.waitForRequest(
    (request) =>
      new URL(request.url()).pathname === `/v1/local/repos/${local}/snapshot`,
    { timeout: 5000 },
  );
  publishLocalOperation();
  await historicalRefresh;
  await page.waitForTimeout(1000);
  expect(
    new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("at_op"),
  ).toBe(retained);
  expect(t.errors).toEqual([]);
});
test("two clients react to Work events, preserve drafts, and recover missed replay", async ({
  page,
  context,
  request,
  baseURL,
}) => {
  test.setTimeout(90000);
  const auth = headers(baseURL!);
  const response = await request.get(`/v1/repos/${home}/work?limit=256`, {
    headers: auth,
  });
  expect(response.ok()).toBe(true);
  const directory = await response.json(),
    project = directory.projects[0].id;
  const taskPage = await request.get(
    `/v1/repos/${home}/work?project=${project}&limit=256`,
    { headers: auth },
  );
  expect(taskPage.ok()).toBe(true);
  const work = await taskPage.json(),
    task = work.tasks.find((row: any) => !row.parent);
  expect(task).toBeTruthy();
  const route = `/#/projects?home=${home}&project=${project}&task=${task.id}&tab=tasks`;
  const observer = await context.newPage(),
    errors = traffic(page),
    otherErrors = traffic(observer);
  await page.goto(route);
  await observer.goto(route);
  const detail = page.getByRole("article", { name: "Task details" }),
    other = observer.getByRole("article", { name: "Task details" });
  await expect(detail).toBeVisible();
  await expect(other).toBeVisible();
  const draft = other
    .getByRole("group", { name: "Comment", exact: true })
    .getByRole("textbox");
  await draft.fill("Unsent event draft");
  const add = async (body: string) => {
    const current = await request.get(
      `/v1/repos/${home}/work?task=${task.id}`,
      { headers: auth },
    );
    const rows = await current.json();
    const result = await request.post(`/v1/repos/${home}/work/commands`, {
      headers: auth,
      data: {
        version: 1,
        request_id: randomUUID().replaceAll("-", ""),
        command: {
          command: "add_comment",
          id: randomUUID().replaceAll("-", ""),
          task: task.id,
          expected_task_version: rows.tasks[0].version,
          reply_to: null,
          body,
        },
      },
    });
    expect(result.ok()).toBe(true);
  };
  const first = `Event comment ${randomUUID()}`;
  await add(first);
  await expect(
    detail.getByRole("list", { name: "Task comments" }),
  ).toContainText(first, { timeout: 5000 });
  await expect(
    other.getByRole("list", { name: "Task comments" }),
  ).toContainText(first, { timeout: 5000 });
  await expect(draft).toHaveText("Unsent event draft");
  await context.setOffline(true);
  await page.waitForTimeout(1000);
  const missed = `Replay comment ${randomUUID()}`;
  await add(missed);
  await context.setOffline(false);
  await expect(
    other.getByRole("list", { name: "Task comments" }),
  ).toContainText(missed, { timeout: 15000 });
  await expect(draft).toHaveText("Unsent event draft");
  await observer.evaluate(() => {
    Object.defineProperty(document, "hidden", {
      configurable: true,
      get: () => true,
    });
    document.dispatchEvent(new Event("visibilitychange"));
  });
  otherErrors.data.length = 0;
  const hidden = `Hidden comment ${randomUUID()}`;
  await add(hidden);
  await page.waitForTimeout(2000);
  expect(otherErrors.data).toEqual([]);
  await observer.evaluate(() => {
    Object.defineProperty(document, "hidden", {
      configurable: true,
      get: () => false,
    });
    document.dispatchEvent(new Event("visibilitychange"));
  });
  await expect(
    other.getByRole("list", { name: "Task comments" }),
  ).toContainText(hidden, { timeout: 5000 });
  await expect(draft).toHaveText("Unsent event draft");
  expect(errors.errors).toEqual([]);
  expect(otherErrors.errors).toEqual([]);
  await observer.close();
});

test("a stale server cursor resets continuity once and exact local preparation retains its pin", async ({
  page,
  request,
  baseURL,
}) => {
  const t = traffic(page);
  let injected = false;
  await page.route("**/v1/events", async (route) => {
    if (injected) return route.continue();
    injected = true;
    return route.continue({
      headers: {
        ...route.request().headers(),
        "last-event-id": `jse1:${"00".repeat(16)}:${"11".repeat(16)}:0`,
      },
    });
  });
  const snapshot = await request.get(`/v1/local/repos/${local}/snapshot`, {
    headers: headers(baseURL!),
  });
  expect(snapshot.ok()).toBe(true);
  const selection = await snapshot.json(),
    operation = selection.heads[0];
  await page.goto(`/#/repos/${local}/log?at_op=${operation}`);
  await expect.poll(() => t.events.length).toBe(2);
  const prepare = page.getByRole("button", {
    name: "Prepare graph index",
    exact: true,
  });
  await expect(prepare).toBeVisible();
  await prepare.click();
  await expect(prepare).toHaveCount(0, { timeout: 15000 });
  expect(
    new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("at_op"),
  ).toBe(operation);
  await page.waitForTimeout(1000);
  const reads = t.data.length;
  await page.waitForTimeout(5000);
  expect(t.data).toHaveLength(reads);
  expect(t.errors).toEqual([]);
});

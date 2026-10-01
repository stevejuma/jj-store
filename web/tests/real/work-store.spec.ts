import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
const home = process.env.JJ_STORE_REAL_REPOSITORY!;
const runtime = process.env.JJ_STORE_REAL_RUNTIME!;
const root = process.env.JJ_STORE_REAL_ROOT!;
function cli(args: string[]) {
  return execFileSync(join(root, "target/debug/jj"), args, {
    encoding: "utf8",
    env: {
      ...process.env,
      JJ_CONFIG: join(runtime, "jj.toml"),
      JJ_STORE_DEVELOPMENT_STATE_ROOT: runtime,
    },
  });
}
function target(name: string) {
  return JSON.parse(
    cli([
      "--no-pager",
      "store",
      "--format=json",
      "init",
      join(runtime, name),
      "--name",
      `work/${name}`,
      "--cache",
      join(runtime, `${name}-cache`),
      "--state-dir",
      join(runtime, `${name}-state`),
      "--workspace",
      name,
      "--unix-socket",
      join(runtime, "local/native.sock"),
    ]),
  ).repository as string;
}
test("native projects tasks and discussion persist on RustFS and refresh explicitly", async ({
  page,
  request,
}) => {
  test.setTimeout(120000);
  const run = Date.now().toString(36);
  const title = `Native release ${run}`;
  const a = target(`code-a-${run}`),
    b = target(`code-b-${run}`);
  await page.goto(`/#/repos/${home}/projects`);
  const view = page.getByRole("region", { name: "Projects and tasks" });
  await expect(
    view.getByRole("button", { name: "New project", exact: true }),
  ).toBeVisible();
  const headers = {
    Origin: new URL(page.url()).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const snapshot = await request.get(`/v1/repos/${home}/snapshot`, { headers });
  expect(snapshot.ok()).toBe(true);
  const operation = (await snapshot.json()).heads[0] as string;
  expect(operation).toMatch(/^[0-9a-f]{128}$/);
  await page.goto(`/#/repos/${home}/projects?at_op=${operation}`);
  await view.getByRole("button", { name: "New project", exact: true }).click();
  await view.getByLabel("Project title", { exact: true }).fill(title);
  await view.getByLabel("Project description").fill("Two repositories");
  await view
    .getByRole("button", { name: "Create project", exact: true })
    .click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await expect(
    view.getByRole("button", { name: title, exact: true }),
  ).toHaveCount(0);
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await view.getByRole("button", { name: title, exact: true }).click();
  for (const id of [a, b]) {
    await view.getByLabel("Repository ID", { exact: true }).fill(id);
    await view.getByRole("button", { name: "Link repository" }).click();
    await expect(view.getByRole("status")).toContainText("Saved");
    await view.getByRole("button", { name: "Refresh projects" }).click();
    await expect(view.getByRole("button", { name: "Retire link" })).toHaveCount(
      id === a ? 1 : 2,
    );
  }
  async function createTask(
    title: string,
    repository: string,
    parent?: string,
  ) {
    await view.getByRole("button", { name: "New task", exact: true }).click();
    await view.getByLabel("Task title", { exact: true }).fill(title);
    await view.getByLabel("Task description").fill(`${title} body`);
    await view.getByLabel("Target repository").selectOption(repository);
    if (parent)
      await view.getByLabel("Parent task").selectOption({ label: parent });
    await view
      .getByRole("button", { name: "Create task", exact: true })
      .click();
    await expect(view.getByRole("status")).toContainText("Saved");
    await view.getByRole("button", { name: "Refresh projects" }).click();
  }
  await createTask("Parent task", a);
  await createTask("Child task", b, "Parent task");
  await expect(view.getByRole("list", { name: "Project tasks" })).toContainText(
    "Subtask of Parent task",
  );
  await view.getByRole("button", { name: "Child task", exact: true }).click();
  await view.getByRole("button", { name: "Edit task", exact: true }).click();
  await view.getByLabel("Task state").selectOption("in_progress");
  await view.getByRole("button", { name: "Save task", exact: true }).click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await expect(
    view.getByRole("article", { name: "Task details" }),
  ).toContainText("in progress");
  await view.getByLabel("Comment", { exact: true }).fill("Original comment");
  await view.getByRole("button", { name: "Add comment" }).click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  const original = view
    .getByRole("listitem")
    .filter({ hasText: "Original comment" });
  await original.getByRole("button", { name: "Reply", exact: true }).click();
  await view.getByLabel("Comment", { exact: true }).fill("Threaded reply");
  await view.getByRole("button", { name: "Add comment" }).click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await original.getByRole("button", { name: "Edit comment" }).click();
  await view.getByLabel("Comment", { exact: true }).fill("Edited comment");
  await view.getByRole("button", { name: "Save comment" }).click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await view
    .getByRole("listitem")
    .filter({ hasText: "Edited comment" })
    .getByRole("button", { name: "Remove comment" })
    .click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await expect(
    view.getByText("Comment removed", { exact: true }),
  ).toBeVisible();
  await expect(view.getByText("Threaded reply", { exact: true })).toBeVisible();
  // Lost acknowledgement: publish to the real server, then lose the response.
  await page.route(
    "**/work/commands",
    async (route) => {
      await route.fetch();
      await route.abort("failed");
    },
    { times: 1 },
  );
  await view
    .getByLabel("Comment", { exact: true })
    .fill("Unknown outcome comment");
  await view.getByRole("button", { name: "Add comment" }).click();
  await expect(view.getByRole("status")).toContainText("Outcome unknown");
  await view.getByRole("button", { name: "Refresh projects" }).click();
  await expect(
    view.getByRole("button", { name: "Check task command" }),
  ).toBeVisible();
  await view.getByRole("button", { name: "Check task command" }).click();
  await expect(view.getByRole("status")).toContainText("Saved");
  await expect(
    view.getByRole("button", { name: "Check task command" }),
  ).toHaveCount(0);
  const response = await request.get(
    `/v1/repos/${home}/work?task=${new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("task")}`,
    {
      headers,
    },
  );
  expect(response.ok()).toBe(true);
  const work = await response.json();
  expect(
    work.comments.filter(
      (c: { body: string }) => c.body === "Unknown outcome comment",
    ),
  ).toHaveLength(1);
  const codePin = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("at_op");
  expect(codePin).toBe(operation);
  const path = new URL(page.url()).hash;
  await page.reload();
  await expect(view.getByText("Threaded reply", { exact: true })).toBeVisible();
  expect(new URL(page.url()).hash).toBe(path);
  expect(
    new URLSearchParams(new URL(page.url()).hash.split("?")[1]).get("at_op"),
  ).toBe(codePin);
  const accessibility = await new AxeBuilder({ page })
    .include(".store-work")
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(accessibility.violations).toEqual([]);
  await page.screenshot({ path: test.info().outputPath("work-desktop.png") });
  await page.setViewportSize({ width: 420, height: 900 });
  expect(
    await view.evaluate((el) => el.scrollWidth <= el.clientWidth + 1),
  ).toBe(true);
});

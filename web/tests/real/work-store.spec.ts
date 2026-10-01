import { expect, test, type Page, type Locator } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { randomUUID } from "node:crypto";
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
  test.setTimeout(180000);
  page.setDefaultTimeout(15000);
  const runtimeErrors: string[] = [];
  page.on("pageerror", (error) => runtimeErrors.push(error.message));
  const run = Date.now().toString(36);
  const title = `Native release ${run}`;
  const a = target(`code-a-${run}`),
    b = target(`code-b-${run}`),
    second = target(`home-${run}`);
  await page.goto("/#/projects");
  const view = page.getByRole("region", { name: "Projects and tasks" });
  const status = (scope = view) =>
    scope.getByRole("status", { name: "Work command status" });
  const editor = (label: string, scope = view) =>
    scope.getByRole("group", { name: label, exact: true }).getByRole("textbox");
  async function refresh() {
    await view.getByRole("button", { name: "Refresh projects" }).click();
    await expect(view).toHaveAttribute("aria-busy", "false", {
      timeout: 30000,
    });
  }
  const headers = {
    Origin: new URL(page.url()).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  async function chooseRepository(
    scope: Page | Locator,
    label: string,
    repository: string,
  ) {
    await scope.getByRole("combobox", { name: label, exact: true }).click();
    await page
      .getByRole("combobox", {
        name: `Select ${label.toLowerCase()}`,
        exact: true,
      })
      .fill(repository);
    await page.getByRole("option").click();
  }
  async function createProject(name: string, repository: string) {
    await view
      .getByRole("button", { name: "New project", exact: true })
      .click();
    const dialog = page.getByRole("dialog", { name: "Create project" });
    await chooseRepository(dialog, "Home repository", repository);
    await dialog.getByLabel("Project title", { exact: true }).fill(name);
    await editor("Project description", dialog).fill(
      "Two repositories with **native tasks**",
    );
    await dialog
      .getByRole("button", { name: "Create project", exact: true })
      .click();
    await expect(status(dialog)).toContainText("Saved");
    await dialog.getByRole("button", { name: "Done", exact: true }).click();
    await expect(
      view.getByRole("button", { name: `Open project ${name}`, exact: true }),
    ).toHaveCount(0);
    await refresh();
    await expect(
      view.getByRole("button", { name: `Open project ${name}`, exact: true }),
    ).toBeVisible();
  }
  await expect(view.getByRole("tab")).toHaveText([
    "Activity",
    "Projects",
    "Tasks",
    "Reviews",
    "Repositories",
  ]);
  await createProject(title, home);
  await createProject(`Second home ${run}`, second);
  await expect(
    view.getByRole("button", { name: `Open project ${title}`, exact: true }),
  ).toBeVisible();
  await view
    .getByRole("button", { name: `Open project ${title}`, exact: true })
    .click();
  const projectId = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("project")!;
  expect(projectId).toMatch(/^[0-9a-f]{32}$/);
  await view.getByRole("button", { name: "Edit project" }).click();
  const editProject = page.getByRole("dialog", { name: "Edit project" });
  await editor("Project description", editProject).fill(
    "Edited **native** description",
  );
  await editProject.getByRole("button", { name: "Save project" }).click();
  await expect(status(editProject)).toContainText("Saved");
  await editProject.getByRole("button", { name: "Done", exact: true }).click();
  await refresh();
  await view.getByRole("button", { name: "Edit project" }).click();
  await expect(editor("Project description", editProject)).toHaveText(
    "Edited **native** description",
  );
  await editProject.getByRole("button", { name: "Done", exact: true }).click();
  await view.getByRole("tab", { name: "Repositories", exact: true }).click();
  const links = view.getByRole("region", { name: "Linked repositories" });
  for (const id of [a, b]) {
    await chooseRepository(links, "Repository", id);
    await links
      .getByRole("button", { name: "Link repository", exact: true })
      .click();
    await expect(status()).toContainText("Saved");
    await refresh();
    await expect(
      links.getByRole("button", { name: "Retire link" }),
    ).toHaveCount(id === a ? 1 : 2);
  }
  const sidebar = page.getByRole("complementary", {
    name: "Repository navigation sidebar",
  });
  // A project expands to its linked code repositories in the primary sidebar.
  if (
    await page
      .getByRole("button", { name: "Expand left sidebar", exact: true })
      .isVisible()
  )
    await page
      .getByRole("button", { name: "Expand left sidebar", exact: true })
      .click();
  await sidebar
    .getByRole("button", { name: `Expand ${title}`, exact: true })
    .click();
  await expect(
    sidebar.getByRole("button", { name: `work/code-a-${run}`, exact: true }),
  ).toHaveCount(2);
  await view.getByRole("tab", { name: "Tasks", exact: true }).click();
  async function createTask(name: string, repository: string, parent?: string) {
    await view.getByRole("button", { name: "New task", exact: true }).click();
    const dialog = page.getByRole("dialog", { name: "Create task" });
    await dialog.getByLabel("Task title", { exact: true }).fill(name);
    await editor("Task description", dialog).fill(`${name} body`);
    await chooseRepository(dialog, "Target repository", repository);
    await dialog
      .getByRole("combobox", { name: "Parent task", exact: true })
      .click();
    if (parent) {
      await page
        .getByRole("combobox", { name: "Select parent task", exact: true })
        .fill(parent);
      await page.getByRole("option", { name: new RegExp(parent) }).click();
    } else
      await page
        .getByRole("option", { name: "No parent", exact: true })
        .click();
    await dialog
      .getByRole("button", { name: "Create task", exact: true })
      .click();
    await expect(status(dialog)).toContainText("Saved");
    await dialog.getByRole("button", { name: "Done", exact: true }).click();
    await refresh();
  }
  await createTask("Parent task", a);
  await createTask("Child task", b, "Parent task");
  await expect(view.getByRole("list", { name: "Project tasks" })).toContainText(
    "Subtask of Parent task",
  );
  await view
    .getByRole("button", { name: "Open task Child task", exact: true })
    .click();
  const taskDetail = view.getByRole("article", { name: "Task details" });
  await expect(
    taskDetail.getByRole("button", { name: "Description", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
  for (const name of ["Subtasks", "Activity"]) {
    await expect(
      taskDetail.getByRole("button", { name, exact: true }),
    ).toHaveAttribute("aria-expanded", "false");
  }
  await taskDetail
    .getByRole("button", { name: "Activity", exact: true })
    .click();
  await view.getByRole("button", { name: "Edit task", exact: true }).click();
  const editTask = page.getByRole("dialog", { name: "Edit task" });
  await editTask
    .getByRole("combobox", { name: "Task state", exact: true })
    .selectOption("in_progress");
  await editTask
    .getByRole("button", { name: "Save task", exact: true })
    .click();
  await expect(status(editTask)).toContainText("Saved");
  await editTask.getByRole("button", { name: "Done", exact: true }).click();
  await refresh();
  await expect(
    view.getByRole("article", { name: "Task details" }),
  ).toContainText("In progress");
  const selectedTask = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("task")!;
  const current = await request.get(
    `/v1/repos/${home}/work?task=${selectedTask}`,
    { headers },
  );
  expect(current.ok()).toBe(true);
  const taskVersion = (await current.json()).tasks[0].version;
  const external = await request.post(`/v1/repos/${home}/work/commands`, {
    headers,
    data: {
      version: 1,
      request_id: randomUUID().replaceAll("-", ""),
      command: {
        command: "add_comment",
        id: randomUUID().replaceAll("-", ""),
        task: selectedTask,
        expected_task_version: taskVersion,
        reply_to: null,
        body: "External discussion update",
      },
    },
  });
  expect(external.ok()).toBe(true);
  await expect(
    view.getByText("Updates available. Refresh projects to view them."),
  ).toBeVisible();
  await expect(
    view.getByText("External discussion update", { exact: true }),
  ).toHaveCount(0);
  await refresh();
  await expect(
    view.getByText("External discussion update", { exact: true }),
  ).toBeVisible();
  async function comment(body: string, action = "Add comment") {
    await editor("Comment").fill(body);
    await view.getByRole("button", { name: action, exact: true }).click();
    await expect(status()).toContainText("Saved");
    await refresh();
  }
  await comment("Original comment");
  const original = view
    .getByRole("listitem")
    .filter({ hasText: "Original comment" });
  await original.locator(".community-message").hover();
  await expect(original.locator(".community-message")).toHaveAttribute(
    "data-actions-visible",
    "",
  );
  await original.getByRole("button", { name: "Reply", exact: true }).click();
  await comment("Threaded reply");
  await original.locator(".community-message").hover();
  await expect(original.locator(".community-message")).toHaveAttribute(
    "data-actions-visible",
    "",
  );
  await original.getByRole("button", { name: "More message actions" }).click();
  await page
    .getByRole("menuitem", { name: "Edit comment", exact: true })
    .click();
  await expect(
    view.getByRole("group", { name: "Editing message" }),
  ).toContainText("Original comment");
  await comment("Edited comment", "Save comment");
  const edited = view
    .getByRole("listitem")
    .filter({ hasText: "Edited comment" });
  await edited.locator(".community-message").hover();
  await expect(edited.locator(".community-message")).toHaveAttribute(
    "data-actions-visible",
    "",
  );
  await edited.getByRole("button", { name: "More message actions" }).click();
  await page
    .getByRole("menuitem", { name: "Remove comment", exact: true })
    .click();
  await page
    .getByRole("alertdialog", { name: "Remove comment?" })
    .getByRole("button", { name: "Remove comment", exact: true })
    .click();
  await expect(status()).toContainText("Saved");
  await refresh();
  await expect(
    view.getByText("Comment removed", { exact: true }),
  ).toBeVisible();
  await expect(view.getByText("Threaded reply", { exact: true })).toBeVisible();
  // The real service commits; only its acknowledgement is lost.
  let command: unknown;
  await page.route(
    "**/work/commands",
    async (route) => {
      command = route.request().postDataJSON();
      await route.fetch();
      await route.abort("failed");
    },
    { times: 1 },
  );
  await editor("Comment").fill("Unknown outcome comment");
  await view.getByRole("button", { name: "Add comment" }).click();
  await expect(status()).toContainText("Outcome unknown");
  const taskPath = new URL(page.url()).hash;
  await page.getByRole("link", { name: "Projects", exact: true }).click();
  await view
    .getByRole("button", { name: `Open project ${title}`, exact: true })
    .click();
  await view
    .getByRole("button", { name: "Open task Child task", exact: true })
    .click();
  await view.getByRole("button", { name: "Activity", exact: true }).click();
  await expect(editor("Comment")).toHaveText("Unknown outcome comment");
  await view.getByRole("button", { name: "Check task command" }).click();
  await expect(status()).toContainText("Saved");
  const taskId = (command as { command: { task: string } }).command.task;
  const response = await request.get(`/v1/repos/${home}/work?task=${taskId}`, {
    headers,
  });
  expect(response.ok()).toBe(true);
  expect(
    (await response.json()).comments.filter(
      (c: { body: string }) => c.body === "Unknown outcome comment",
    ),
  ).toHaveLength(1);
  await refresh();
  const path = new URL(page.url()).hash;
  await page.reload();
  await view.getByRole("button", { name: "Activity", exact: true }).click();
  await expect(view.getByText("Threaded reply", { exact: true })).toBeVisible();
  expect(new URL(page.url()).hash).toBe(path);
  expect(path).toBe(taskPath);
  // Legacy IDs restore the same native task.
  await page.goto(
    `/#/repos/${home}/projects?project=${projectId}&task=${taskId}`,
  );
  await expect(
    view.getByRole("article", { name: "Task details" }),
  ).toContainText("Child task");
  await page.getByRole("link", { name: title, exact: true }).click();
  await view.getByRole("tab", { name: "Repositories", exact: true }).click();
  const row = links
    .getByRole("listitem")
    .filter({ hasText: `work/code-b-${run}` });
  await row.getByRole("button", { name: "Retire link" }).click();
  await expect(status()).toContainText("Saved");
  await refresh();
  await expect(row).toContainText("Retired");
  const projectPath = new URL(page.url()).hash;
  await row.getByRole("button", { name: "Open repository" }).click();
  await expect
    .poll(() => new URL(page.url()).hash)
    .toContain(`/repos/${b}/files`);
  await page.goBack();
  await expect(
    view.getByRole("region", { name: "Linked repositories" }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toBe(projectPath);
  await view.getByRole("tab", { name: "Tasks", exact: true }).click();
  const detailsToggle = page.getByRole("button", {
    name: "Toggle details",
    exact: true,
  });
  if ((await detailsToggle.getAttribute("aria-expanded")) === "false")
    await detailsToggle.click();
  await page
    .getByRole("button", { name: "Filters for tasks", exact: true })
    .click();
  await page.getByRole("button", { name: "Task state", exact: true }).click();
  await page
    .getByRole("menuitemradio", { name: "In progress", exact: true })
    .click();
  await expect(
    view.getByRole("button", { name: "Open task Child task", exact: true }),
  ).toBeVisible();
  await expect(
    view.getByRole("button", { name: "Open task Parent task", exact: true }),
  ).toHaveCount(0);
  await page.getByLabel("Search tasks", { exact: true }).fill("Child");
  await page.getByRole("button", { name: "Search", exact: true }).click();
  await expect(
    view.getByRole("button", { name: "Open task Child task", exact: true }),
  ).toBeVisible();
  const filtered = new URL(page.url()).hash;
  await page.reload();
  await page
    .getByRole("button", { name: "Filters for tasks", exact: true })
    .click();
  await expect(page.getByLabel("Search tasks", { exact: true })).toHaveValue(
    "Child",
  );
  expect(new URL(page.url()).hash).toBe(filtered);
  await page
    .getByRole("button", { name: "Close filters", exact: true })
    .click();
  // Filters on the main task view do not remove sidebar projects.
  await expect(
    sidebar.getByRole("button", { name: title, exact: true }),
  ).toBeAttached();
  await page.getByRole("link", { name: "Projects", exact: true }).click();
  await view.getByRole("tab", { name: "Reviews", exact: true }).click();
  const reviewResponse = await request.get(
    `/v1/repos/${home}/reviews/threads`,
    { headers },
  );
  expect(reviewResponse.ok()).toBe(true);
  const thread = (await reviewResponse.json()).threads[0];
  expect(thread).toBeTruthy();
  await page
    .getByRole("button", { name: "Filters for reviews", exact: true })
    .click();
  await chooseRepository(page, "Repository filter", home);
  await page
    .getByRole("button", { name: "Close filters", exact: true })
    .click();
  const reviewPath = new URL(page.url()).hash;
  await view
    .getByRole("button", { name: /^Open .* review with/ })
    .first()
    .click();
  await expect
    .poll(() => new URL(page.url()).hash)
    .toContain(`/repos/${home}/reviews`);
  await expect
    .poll(() => new URL(page.url()).hash)
    .toContain(`thread=${thread.thread_id}`);
  await page.goBack();
  await expect(
    view.getByRole("tab", { name: "Reviews", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  expect(new URL(page.url()).hash).toBe(reviewPath);
  expect(thread.presentation.excerpt).toBeTruthy();
  const reviewRow = view.locator("article").filter({
    has: page.getByRole("button", { name: `Open review ${thread.thread_id}` }),
  });
  await expect(reviewRow).toContainText(thread.presentation.excerpt);
  await view.getByRole("radio", { name: "Grid layout", exact: true }).click();
  await expect(reviewRow.locator("time")).toBeVisible();
  await view.getByRole("tab", { name: "Repositories", exact: true }).click();
  const repositoryRow = view.locator("article").filter({
    has: page.getByRole("button", {
      name: "Open repository jj-store demo",
      exact: true,
    }),
  });
  await expect(
    repositoryRow.getByRole("group", { name: /commits,.*reviews,.*tasks/ }),
  ).toBeVisible();
  await expect(repositoryRow.getByRole("alert")).toHaveCount(0);
  await expect(repositoryRow.locator("time")).toBeVisible();
  await expect(
    repositoryRow.getByRole("button", { name: /^View .*profile$/ }),
  ).toBeVisible();
  await page.goto(`/${filtered}`);
  await page
    .getByRole("button", { name: "Filters for tasks", exact: true })
    .click();
  await expect(page.getByLabel("Search tasks", { exact: true })).toHaveValue(
    "Child",
  );
  await page
    .getByRole("button", { name: "Close filters", exact: true })
    .click();
  const accessibility = await new AxeBuilder({ page })
    .include(".store-projects")
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(accessibility.violations).toEqual([]);
  await page.screenshot({ path: test.info().outputPath("work-desktop.png") });
  await page.setViewportSize({ width: 420, height: 900 });
  expect(
    await view.evaluate((el) => el.scrollWidth <= el.clientWidth + 1),
  ).toBe(true);
  expect(runtimeErrors).toEqual([]);
});

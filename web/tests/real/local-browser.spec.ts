import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { readFileSync, renameSync } from "node:fs";
import { resolve } from "node:path";
const fixture = process.env.JJ_STORE_LOCAL_BROWSER_FIXTURE
  ? JSON.parse(readFileSync(process.env.JJ_STORE_LOCAL_BROWSER_FIXTURE, "utf8"))
  : null;
test.skip(!fixture, "owned local browser fixture required");
const binary = resolve(process.env.CARGO_TARGET_DIR ?? "target", "debug/jj");
function state() {
  const jj = (args: string[]) =>
    execFileSync(
      binary,
      ["-R", fixture.folder, "--ignore-working-copy", "--no-pager", ...args],
      { encoding: "utf8" },
    );
  return {
    readme: readFileSync(fixture.folder + "/README.md", "utf8"),
    operations: jj(["op", "log", "-n", "10", "-T", 'id ++ "\\n"']),
    bookmarks: jj(["bookmark", "list", "-T", 'name ++ "\\n"']),
    companion: readFileSync(
      fixture.folder + "/.git/refs/lapis/state/v1",
      "utf8",
    ),
  };
}
test("local_repository_opens_files_history_agents_and_accessible_folder_controls", async ({
  page,
}) => {
  const before = state();
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message));
  const requests: string[] = [];
  page.on("request", (r) => {
    if (r.url().includes("/v1/")) requests.push(r.url());
  });
  await page.goto(`/#/repos/${fixture.repository}/files`);
  await expect(
    page.getByText("Local browser acceptance.", { exact: false }).first(),
  ).toBeVisible();
  const blockedStyle = await page.evaluate(() => {
    const node = document.createElement("div");
    document.body.append(node);
    node.setAttribute("style", "pointer-events: none");
    const value = getComputedStyle(node).pointerEvents;
    node.remove();
    return value;
  });
  expect(blockedStyle).toBe("auto");
  const hash = await page.evaluate(() => location.hash);
  expect(hash).toContain("at_op=");
  expect(hash).toMatch(/commit=[0-9a-f]{40}(?:&|$)/);
  for (const [section, label] of [
    ["log", "Commits"],
    ["oplog", "Op Log"],
    ["agents", "Agents"],
  ]) {
    await page.goto(
      `/#/repos/${fixture.repository}/${section}?at_op=${fixture.operation}`,
    );
    await expect(
      page.getByRole("tab", { name: label, exact: true }),
    ).toHaveAttribute("aria-selected", "true");
    await expect(page.locator('[role="alert"]')).toHaveCount(0);
  }
  const sessions = page.getByRole("list", {
    name: "Agent sessions",
    exact: true,
  });
  await expect(
    sessions.getByRole("button", { name: /^Open .* agent session$/ }),
  ).toHaveCount(3);
  await sessions
    .getByRole("button", { name: /^Open Codex .* agent session$/ })
    .click();
  await expect(
    page.getByRole("log", { name: "Agent conversation activity" }),
  ).toBeVisible();
  await expect(
    page.getByRole("group", { name: /tool call$/ }).first(),
  ).toBeVisible();
  await page.goto(`/#/repos/${fixture.repository}/files`);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("tab", { name: "Local folders" }).click();
  await expect(
    page.getByRole("button", { name: "Reconnect folder" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Add folder", exact: true }).click();
  await expect(page.getByRole("dialog")).toBeVisible();
  await page
    .getByRole("textbox", { name: "Folder", exact: true })
    .fill(fixture.folder);
  await page.getByRole("button", { name: "Inspect folder" }).click();
  await expect(
    page.getByText("Supported Git-backed JJ folder.", { exact: false }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  const a11y = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(a11y.violations).toEqual([]);
  await page
    .getByRole("button", { name: "Back to jj-store", exact: true })
    .click();
  await page.goto("/#/projects?tab=repositories");
  const add = page.getByRole("button", {
    name: "Add local repository",
    exact: true,
  });
  await expect(add).toBeEnabled();
  await add.focus();
  await add.press("Enter");
  await expect(
    page.getByRole("dialog", { name: "Add local JJ folder" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Cancel", exact: true }).click();
  expect(
    requests.filter((r) => r.includes(`/v1/repos/${fixture.repository}/`)),
  ).toEqual([]);
  expect(errors).toEqual([]);
  expect(state()).toEqual(before);
});

test("local_projects_tasks_comments_match_cli_and_recover_unknown_outcomes", async ({
  page,
}) => {
  test.setTimeout(90000);
  const title = `Local project ${Date.now()}`;
  const editor = (scope: any, label: string) =>
    scope.getByRole("group", { name: label, exact: true }).getByRole("textbox");
  await page.goto("/#/projects");
  const view = page.getByRole("region", { name: "Projects and tasks" });
  await view.getByRole("button", { name: "New project", exact: true }).click();
  const dialog = page.getByRole("dialog", { name: "Create project" });
  await dialog
    .getByRole("combobox", { name: "Home repository", exact: true })
    .click();
  await page
    .getByRole("combobox", { name: "Select home repository", exact: true })
    .fill(fixture.repository);
  await page.getByRole("option").click();
  await dialog.getByLabel("Project title", { exact: true }).fill(title);
  await editor(dialog, "Project description").fill("Local metadata parity");
  await dialog
    .getByRole("button", { name: "Create project", exact: true })
    .click();
  await expect(
    dialog.getByRole("status", { name: "Work command status" }),
  ).toContainText("Saved");
  await dialog.getByRole("button", { name: "Done", exact: true }).click();
  await view
    .getByRole("button", { name: "Refresh projects", exact: true })
    .click();
  await view
    .getByRole("button", { name: `Open project ${title}`, exact: true })
    .click();
  const project = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("project")!;
  await view.getByRole("button", { name: "New task", exact: true }).click();
  const taskDialog = page.getByRole("dialog", { name: "Create task" });
  await taskDialog.getByLabel("Task title", { exact: true }).fill("Local task");
  await taskDialog
    .getByRole("button", { name: "Create task", exact: true })
    .click();
  await expect(
    taskDialog.getByRole("status", { name: "Work command status" }),
  ).toContainText("Saved");
  await taskDialog.getByRole("button", { name: "Done", exact: true }).click();
  await view
    .getByRole("button", { name: "Refresh projects", exact: true })
    .click();
  await view
    .getByRole("button", { name: "Open task Local task", exact: true })
    .click();
  const taskId = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("task")!;
  await view.getByRole("button", { name: "Activity", exact: true }).click();
  await page.route(
    "**/v1/local/repos/*/work/commands",
    async (route) => {
      await route.fetch();
      await route.abort("failed");
    },
    { times: 1 },
  );
  await editor(view, "Comment").fill("Local unknown outcome comment");
  await view.getByRole("button", { name: "Add comment", exact: true }).click();
  await expect(
    view.getByRole("status", { name: "Work command status" }),
  ).toContainText("Outcome unknown");
  await view
    .getByRole("button", { name: "Check task command", exact: true })
    .click();
  await expect(
    view.getByRole("status", { name: "Work command status" }),
  ).toContainText("Saved");
  await view
    .getByRole("button", { name: "Refresh projects", exact: true })
    .click();
  await expect(
    view.getByText("Local unknown outcome comment", { exact: true }),
  ).toBeVisible();
  const cli = (args: string[]) =>
    JSON.parse(
      execFileSync(
        binary,
        [
          "-R",
          fixture.folder,
          "--ignore-working-copy",
          ...args,
          "--format=json",
        ],
        { encoding: "utf8" },
      ),
    );
  const projects = cli(["project", "list"]);
  expect(projects.projects.find((p: any) => p.id === project).title).toBe(
    title,
  );
  const task = cli(["task", "show", taskId]);
  expect(task.tasks[0].title).toBe("Local task");
  expect(
    task.comments.filter(
      (c: any) => c.body === "Local unknown outcome comment",
    ),
  ).toHaveLength(1);
});

test("local_folder_reconnect_and_removal_preserve_code_and_collaboration", async ({
  page,
}) => {
  const before = state();
  const original = fixture.folder;
  const moved = `${original}-moved`;
  await page.goto(`/#/repos/${fixture.repository}/files`);
  await page
    .getByRole("button", { name: "Open profile menu for Local" })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await page.getByRole("button", { name: "Repository", exact: true }).click();
  await page.getByRole("tab", { name: "Local folders" }).click();
  renameSync(original, moved);
  fixture.folder = moved;
  try {
    await page
      .getByRole("button", { name: "Refresh folders", exact: true })
      .click();
    await expect(page.getByText("unavailable", { exact: true })).toBeVisible();
    await page
      .getByRole("button", { name: "Reconnect folder", exact: true })
      .click();
    const dialog = page.getByRole("dialog", { name: "Reconnect local folder" });
    await dialog
      .getByRole("textbox", { name: "Folder", exact: true })
      .fill(moved);
    await dialog
      .getByRole("button", { name: "Inspect folder", exact: true })
      .click();
    await expect(
      dialog.getByText("Supported Git-backed JJ folder.", { exact: false }),
    ).toBeVisible();
    await dialog
      .getByRole("button", { name: "Reconnect", exact: true })
      .click();
    await expect(dialog).not.toBeVisible();
    await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
    await expect(page.getByText("ready", { exact: true })).toBeVisible();
    await page
      .getByRole("button", { name: "Back to jj-store", exact: true })
      .click();
    await page.reload();
    await expect(
      page.getByText("Local browser acceptance.", { exact: false }).first(),
    ).toBeVisible();
    await page
      .getByRole("button", { name: "Open profile menu for Local" })
      .click();
    await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
    await page.getByRole("button", { name: "Repository", exact: true }).click();
    await page.getByRole("tab", { name: "Local folders" }).click();
    await page
      .getByRole("button", { name: "Remove registration", exact: true })
      .click();
    await expect(
      page.getByRole("button", { name: "Reconnect folder", exact: true }),
    ).toHaveCount(0);
    expect(state()).toEqual(before);
  } finally {
    renameSync(moved, original);
    fixture.folder = original;
  }
});

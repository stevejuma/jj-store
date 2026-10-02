import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { readFileSync, renameSync, writeFileSync } from "node:fs";
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
  const toolGroup = page.getByRole("button", {
    name: "Expand 3 tool calls",
    exact: true,
  });
  await expect(toolGroup).toHaveAttribute("aria-expanded", "false");
  await toolGroup.click();
  await expect(
    page.getByRole("group", { name: /^(Read|apply_patch|Bash) tool call$/ }),
  ).toHaveCount(3);
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
    name: "Add",
    exact: true,
  });
  await expect(add).toBeEnabled();
  await add.focus();
  await add.press("Enter");
  await page
    .getByRole("menuitem", { name: "Add local repository", exact: true })
    .click();
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
  const createdProject = view.getByRole("button", {
    name: `Open project ${title}`,
    exact: true,
  });
  await expect(createdProject).toBeVisible();
  await expect(
    dialog.getByRole("button", { name: "Create project", exact: true }),
  ).toBeEnabled();
  await expect(dialog.getByRole("alert")).toHaveCount(0);
  await expect(
    dialog.getByRole("status", { name: "Work command status" }),
  ).toHaveCount(0);
  await dialog.getByRole("button", { name: "Done", exact: true }).click();
  await createdProject.click();
  const project = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("project")!;
  await view.getByRole("tab", { name: "Tasks", exact: true }).click();
  await view.getByRole("button", { name: "Add", exact: true }).click();
  await page.getByRole("menuitem", { name: "Add task", exact: true }).click();
  const taskDialog = page.getByRole("dialog", { name: "Create task" });
  await taskDialog.getByLabel("Task title", { exact: true }).fill("Local task");
  await taskDialog
    .getByRole("button", { name: "Create task", exact: true })
    .click();
  await expect(
    view.getByRole("button", { name: "Open task Local task", exact: true }),
  ).toBeVisible();
  await expect(
    taskDialog.getByRole("button", { name: "Create task", exact: true }),
  ).toBeEnabled();
  await expect(taskDialog.getByRole("alert")).toHaveCount(0);
  await expect(
    taskDialog.getByRole("status", { name: "Work command status" }),
  ).toHaveCount(0);
  await taskDialog.getByRole("button", { name: "Done", exact: true }).click();
  await view
    .getByRole("button", { name: "Open task Local task", exact: true })
    .click();
  const taskId = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("task")!;
  await expect(
    view.getByRole("button", { name: "Activity", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
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
    view.getByText("Local unknown outcome comment", { exact: true }),
  ).toBeVisible();
  await expect(
    view.getByRole("status", { name: "Work command status" }),
  ).toHaveCount(0);
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

test("filesystem reviews create and reply with CLI parity and pinned historical reads", async ({
  page,
}) => {
  test.skip(!fixture, "filesystem profile is not active");
  const headers = { "x-jj-store-protocol": "1" };
  const endpoint = `/v1/local/repos/${fixture.repository}`;
  const before = await (
    await page.request.get(`${endpoint}/snapshot`, { headers })
  ).json();
  await page.goto(
    `/#/repos/${fixture.repository}/reviews?at_op=${fixture.operation}`,
  );
  const create = page.getByRole("button", {
    name: "New repository review",
    exact: true,
  });
  await expect(create).toBeEnabled();
  await create.focus();
  await page.keyboard.press("Enter");
  const dialog = page.getByRole("dialog", { name: "New repository review" });
  await expect(dialog).toBeVisible();
  const editor = dialog
    .getByRole("group", { name: "Repository review message", exact: true })
    .getByRole("textbox");
  await editor.fill("Review from filesystem browser");
  const a11y = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(a11y.violations).toEqual([]);
  await page.route(
    "**/v1/local/repos/*/reviews/commands",
    async (route) => {
      await route.fetch();
      await route.abort("failed");
    },
    { times: 1 },
  );
  await dialog
    .getByRole("button", { name: "Create review", exact: true })
    .click();
  await expect(
    dialog.getByRole("button", { name: "Check request status", exact: true }),
  ).toBeVisible();
  await expect(editor).toHaveText("Review from filesystem browser");
  await dialog
    .getByRole("button", { name: "Check request status", exact: true })
    .click();
  await expect(dialog).not.toBeVisible();
  await expect(
    page.getByText("Review from filesystem browser", { exact: true }).first(),
  ).toBeVisible();
  const thread = new URLSearchParams(
    new URL(page.url()).hash.split("?")[1],
  ).get("thread")!;
  expect(thread).toMatch(/^[0-9a-f]{32}$/);
  const after = await (
    await page.request.get(`${endpoint}/snapshot`, { headers })
  ).json();
  expect(after.heads).toEqual(before.heads);
  const old = await (
    await page.request.post(`${endpoint}/reviews/page`, {
      headers,
      data: {
        metadata_commit: before.metadata_commit,
        limit: 50,
        cursor: null,
      },
    })
  ).json();
  expect(old.data.threads.some((t: any) => t.thread_id === thread)).toBe(false);
  const current = await (
    await page.request.post(`${endpoint}/reviews/detail`, {
      headers,
      data: { metadata_commit: after.metadata_commit, thread_id: thread },
    })
  ).json();
  expect(current.comments[0].content.text).toBe(
    "Review from filesystem browser",
  );
  const cli = JSON.parse(
    execFileSync(
      `${process.env.CARGO_TARGET_DIR ?? "target"}/debug/jj`,
      [
        "-R",
        fixture.folder,
        "--ignore-working-copy",
        "review",
        "--format=json",
        "show",
        thread,
        "--at-metadata",
        after.metadata_commit,
      ],
      {
        encoding: "utf8",
        env: { ...process.env, JJ_CONFIG: `${fixture.root}/jj.toml` },
      },
    ),
  );
  expect(cli).toEqual(current);
  const reply = page
    .getByRole("group", { name: "Review comment", exact: true })
    .getByRole("textbox");
  await reply.fill("Reply from browser");
  await page.getByRole("button", { name: "Add comment", exact: true }).click();
  await expect(
    page.getByText("Reply from browser", { exact: true }).first(),
  ).toBeVisible();
  await expect(reply.locator(".cm-placeholder")).toHaveText("Add a comment…");
  await page.reload();
  await expect(
    page.getByText("Reply from browser", { exact: true }).first(),
  ).toBeVisible();
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
    const headers = {
      "x-jj-store-protocol": "1",
      Origin: new URL(fixture.origin).origin,
      "Sec-Fetch-Dest": "empty",
      "Sec-Fetch-Mode": "cors",
      "Sec-Fetch-Site": "same-origin",
    };
    const bindings = await (
      await page.request.get("/v1/local/repositories", { headers })
    ).json();
    if (
      !bindings.bindings.some(
        (binding: any) => binding.repository === fixture.repository,
      )
    ) {
      const registered = await page.request.post("/v1/local/repositories", {
        headers,
        data: {
          version: 1,
          request_id: crypto.randomUUID().replaceAll("-", ""),
          expected_revision: bindings.revision,
          path: original,
          name: "Local Git-backed JJ",
        },
      });
      expect(registered.ok()).toBe(true);
      expect((await registered.json()).status).toBe("committed");
    }
  }
});

test("physical working copy saves retain drafts and leave recorded state unchanged", async ({
  page,
}) => {
  const before = state();
  const path = fixture.folder + "/README.md";
  const physical = "# Physical file\n\nUnsnapshotted before browser editing.\n";
  const draft = "# Physical saved draft\n\nSaved separately from Snapshot.\n";
  writeFileSync(path, physical);
  try {
    await page.goto(
      `/#/repos/${fixture.repository}/files?at_op=${fixture.operation}&file=README.md`,
    );
    await expect(
      page.getByRole("group", { name: "File display" }),
    ).toBeVisible();
    await page
      .getByRole("button", { name: "Working copy", exact: true })
      .focus();
    await page.keyboard.press("Enter");
    const editor = page.getByRole("region", {
      name: "Working copy README.md",
      exact: true,
    });
    await expect(editor).toBeVisible();
    const text = editor.locator(".cm-content");
    await expect(text).toContainText("Unsnapshotted before browser editing.");
    await expect(page.getByLabel("File name", { exact: true })).toHaveCount(0);
    await text.fill(draft);
    const violations = (
      await new AxeBuilder({ page }).include(".store-file-editor").analyze()
    ).violations;
    expect(violations).toEqual([]);
    await page.route(
      "**/working-copy/save",
      async (route) => {
        await route.fetch();
        await route.abort("failed");
      },
      { times: 1 },
    );
    await editor.getByRole("button", { name: "Save changes" }).click();
    await expect(editor.getByRole("alert")).toContainText(
      "Save outcome is unknown",
    );
    await expect(text).toContainText("Physical saved draft");
    await expect(
      editor.getByRole("button", { name: "Save changes" }),
    ).toBeDisabled();
    await editor.getByRole("button", { name: "Check working file" }).click();
    await expect(editor.getByRole("status")).toContainText(
      "matches your draft",
    );
    expect(readFileSync(path, "utf8")).toBe(draft);
    const after = state();
    expect(after.operations).toBe(before.operations);
    expect(after.bookmarks).toBe(before.bookmarks);
    expect(after.companion).toBe(before.companion);
    await text.fill("Keep this stale draft");
    writeFileSync(path, "External edit must survive");
    await editor.getByRole("button", { name: "Save changes" }).click();
    await expect(editor.getByRole("alert")).toContainText(
      "working file changed",
    );
    await expect(text).toContainText("Keep this stale draft");
    expect(readFileSync(path, "utf8")).toBe("External edit must survive");
    await editor.getByRole("button", { name: "Cancel", exact: true }).click();
    await page.getByRole("button", { name: "Source", exact: true }).click();
    await expect(
      page.getByText("# Filesystem repository", { exact: true }),
    ).toBeVisible();
    await expect(
      page.getByRole("region", { name: "Working copy README.md" }),
    ).toHaveCount(0);
  } finally {
    writeFileSync(path, before.readme);
  }
});

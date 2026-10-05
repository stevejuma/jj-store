import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const fixture = process.env.JJ_STORE_STACKS_BROWSER_FIXTURE
  ? JSON.parse(
      readFileSync(process.env.JJ_STORE_STACKS_BROWSER_FIXTURE, "utf8"),
    )
  : null;
test.skip(!fixture, "Owned isolated Stacks browser fixture required");

test("cold_focused_stacks_navigation_history_reload_and_return", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(
    `/#/repos/${fixture.repository}/files?at_op=${fixture.operation}&bookmark=main`,
  );
  const prepare = page.getByRole("button", {
    name: "Prepare graph index",
    exact: true,
  });
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toBeVisible();
  const reference = page.getByRole("combobox", {
    name: "Repository bookmark or change",
    exact: true,
  });
  await expect(reference).toContainText("main");
  const entry = page.url();
  await page.getByRole("tab", { name: "Stacks", exact: true }).click();
  const tabs = page.getByRole("tablist", {
    name: "Local repository views",
    exact: true,
  });
  await expect(tabs.getByRole("tab")).toHaveText([
    "Branches",
    "Workspace",
    "Commits",
    "Op Log",
  ]);
  await expect(
    page.getByRole("tablist", { name: "Repository sections" }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("button", { name: "Repository actions", exact: true }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("button", { name: "Show terminal", exact: true }),
  ).toHaveCount(0);
  const back = page.getByRole("button", {
    name: "Back to repository",
    exact: true,
  });
  const geometry = await back.evaluate((element) => ({
    width: element.getBoundingClientRect().width,
    height: element.getBoundingClientRect().height,
    radius: parseFloat(getComputedStyle(element).borderRadius),
  }));
  expect(Math.abs(geometry.width - geometry.height)).toBeLessThanOrEqual(1);
  expect(geometry.radius).toBeGreaterThanOrEqual(geometry.width / 2);
  const canvas = page.locator(".stack-workspace__canvas-view");
  await expect(canvas).toBeVisible();
  await canvas.evaluate((element) =>
    element.setAttribute("data-retained-test", "same-canvas"),
  );
  await tabs.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect(page).toHaveURL(/\/log\?context=stacks/);
  const graph = page.getByRole("list", { name: "Revision graph", exact: true });
  await expect
    .poll(async () => (await graph.isVisible()) || (await prepare.isVisible()))
    .toBe(true);
  if (await prepare.isVisible()) await prepare.click();
  await expect(graph).toBeVisible();
  const target = graph.getByRole("button", {
    name: "Workspace target",
    exact: true,
  });
  await target.click();
  await expect(
    target.locator('xpath=ancestor::*[@role="listitem"]'),
  ).toHaveAttribute("aria-current", "true");
  const selected = page.url();
  await tabs.getByRole("tab", { name: "Op Log", exact: true }).click();
  await expect(page).toHaveURL(/\/oplog\?context=stacks/);
  await expect(
    page.getByRole("list", { name: "Operation graph", exact: true }),
  ).toBeVisible();
  await tabs.getByRole("tab", { name: "Workspace", exact: true }).click();
  await expect(canvas).toHaveAttribute("data-retained-test", "same-canvas");
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  await tabs.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect(page).toHaveURL(selected);
  await back.click();
  await expect(page).toHaveURL(entry);
  await page.getByRole("tab", { name: "Stacks", exact: true }).click();
  await tabs.getByRole("tab", { name: "Op Log", exact: true }).click();
  const history = page.url();
  await page.reload();
  await expect(page).toHaveURL(history);
  await expect(
    tabs.getByRole("tab", { name: "Op Log", exact: true }),
  ).toHaveAttribute("data-state", "active");
  await expect(
    page.getByRole("combobox", { name: "Active stack workspace", exact: true }),
  ).toBeVisible();
  await back.click();
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toHaveAttribute("data-state", "active");
  await page.goBack();
  await expect(
    tabs.getByRole("tab", { name: "Op Log", exact: true }),
  ).toBeVisible();
  await page.goForward();
  await expect(
    page.getByRole("tab", { name: "Overview", exact: true }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});

test("cold_local_megamerges_apply_unapply_and_named_wip_survive_reload", async ({
  page,
}) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const companion = readFileSync(
    join(fixture.folder, ".git/refs/lapis/state/v1"),
    "utf8",
  );
  const view = (name: "Branches" | "Workspace") =>
    page
      .getByRole("tablist", { name: "Local repository views", exact: true })
      .getByRole("tab", { name, exact: true });
  const confirm = async () => {
    await page
      .getByRole("dialog")
      .getByRole("button", { name: "Apply command", exact: true })
      .click({ timeout: 10000 });
    await expect(page.getByRole("dialog")).toHaveCount(0);
  };
  const create = async (name: string, first = false) => {
    if (first)
      await page
        .getByRole("button", { name: "Create workspace", exact: true })
        .click();
    else {
      await page
        .getByRole("button", { name: "Workspace actions", exact: true })
        .click();
      await page
        .getByRole("menuitem", { name: "Create workspace", exact: true })
        .click();
    }
    await page
      .getByRole("dialog")
      .getByRole("textbox", { name: "Name", exact: true })
      .fill(name);
    await page
      .getByRole("dialog")
      .getByRole("button", { name: "Preview command", exact: true })
      .click();
    await confirm();
    await expect(
      page.getByRole("combobox", {
        name: "Active stack workspace",
        exact: true,
      }),
    ).toContainText(name);
  };
  await page.goto(
    `/#/repos/${fixture.repository}/stacks?at_op=${fixture.operation}`,
  );
  await expect(
    page.getByRole("region", { name: "Local stack workspace", exact: true }),
  ).toBeVisible();
  const target = page.getByRole("combobox", {
    name: "Workspace target branch",
    exact: true,
  });
  await expect(target).toBeEnabled();
  await target.click();
  await page.getByRole("option", { name: /main/ }).click();
  await create("Primary", true);
  const unstaged = page.getByRole("region", {
    name: "Unstaged files",
    exact: true,
  });
  await expect(
    unstaged.getByRole("button", { name: "unsnapshotted.txt", exact: true }),
  ).toBeVisible();
  await view("Branches").click();
  await expect(
    page.getByText("2 unstaged files", { exact: true }),
  ).toBeVisible();
  await page.getByRole("option", { name: /topic/ }).click();
  const topic = page.getByRole("region", { name: "Stack topic", exact: true });
  const prepare = page.getByRole("button", {
    name: "Prepare graph index",
    exact: true,
  });
  const ancestor = topic.getByRole("button", {
    name: "Topic ancestor",
    exact: true,
  });
  await expect
    .poll(
      async () => (await ancestor.isVisible()) || (await prepare.isVisible()),
    )
    .toBe(true);
  if (await prepare.isVisible()) await prepare.click();
  await expect(
    topic.getByRole("button", { name: "Topic ancestor", exact: true }),
  ).toBeVisible();
  await topic.getByRole("button", { name: "Apply topic", exact: true }).click();
  await confirm();
  await view("Workspace").click();
  await expect(topic).toBeVisible();
  expect(readFileSync(join(fixture.folder, "src/topic.ts"), "utf8")).toContain(
    "export const two = 2",
  );
  await unstaged
    .getByRole("button", { name: "unsnapshotted.txt", exact: true })
    .click();
  const diff = page.getByRole("region", { name: /Selected changes from/ });
  // Applying a stack advances the operation independently of graph preparation.
  // Prepare that new pin explicitly if the adjacent diff needs its graph index.
  await expect(diff).toBeVisible();
  if (await prepare.isVisible()) {
    await prepare.click();
    await expect(ancestor).toBeVisible();
    await unstaged
      .getByRole("button", { name: "unsnapshotted.txt", exact: true })
      .click();
  }
  await expect(diff).toContainText("new physical file");
  expect(
    await diff
      .getByRole("group", { name: /Draggable hunk/ })
      .getAttribute("draggable"),
  ).toBe("true");
  await create("Second");
  writeFileSync(join(fixture.folder, "second.txt"), "second workspace edit\n");
  await page.getByRole("button", { name: "Refresh", exact: true }).click();
  await expect(
    unstaged.getByRole("button", { name: "second.txt", exact: true }),
  ).toBeVisible();
  const picker = page.getByRole("combobox", {
    name: "Active stack workspace",
    exact: true,
  });
  await expect(
    page.getByRole("button", { name: "Refresh", exact: true }),
  ).toBeEnabled();
  await picker.click();
  await expect(page.getByRole("option", { name: /Primary/ })).toBeEnabled();
  await page.getByRole("option", { name: /Primary/ }).click();
  await confirm();
  await expect(
    unstaged.getByRole("button", { name: "unsnapshotted.txt", exact: true }),
  ).toBeVisible();
  await expect(
    unstaged.getByRole("button", { name: "second.txt", exact: true }),
  ).toHaveCount(0);
  await view("Branches").click();
  await page.getByRole("option", { name: /topic/ }).click();
  await topic
    .getByRole("button", { name: "Unapply topic", exact: true })
    .click();
  await confirm();
  await view("Workspace").click();
  await expect(topic).toHaveCount(0);
  await page.reload();
  await expect(picker).toContainText("Primary");
  await expect(
    unstaged.getByRole("button", { name: "unsnapshotted.txt", exact: true }),
  ).toBeVisible();
  await picker.click();
  await expect(page.getByRole("option", { name: /Second/ })).toBeEnabled();
  await page.getByRole("option", { name: /Second/ }).click();
  await confirm();
  await expect(
    unstaged.getByRole("button", { name: "second.txt", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Workspace actions", exact: true })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  const settings = page.getByRole("complementary", {
    name: "Workspace settings",
    exact: true,
  });
  await expect(settings).toBeVisible();
  await settings
    .getByRole("textbox", { name: "Workspace name", exact: true })
    .fill("Second renamed");
  await settings
    .getByRole("combobox", { name: "Workspace target branch", exact: true })
    .click();
  await page.getByRole("option", { name: /^docs/ }).click();
  await settings.getByRole("button", { name: "Save", exact: true }).click();
  await expect(page.getByRole("dialog")).toContainText("Second renamed");
  await confirm();
  await expect(picker).toContainText("Second renamed");
  expect(readFileSync(join(fixture.folder, "second.txt"), "utf8")).toBe(
    "second workspace edit\n",
  );
  await settings
    .getByRole("button", { name: "Close workspace settings", exact: true })
    .click();
  await page.reload();
  await expect(picker).toContainText("Second renamed");
  await page
    .getByRole("button", { name: "Workspace actions", exact: true })
    .click();
  await page.getByRole("menuitem", { name: "Settings", exact: true }).click();
  await expect(
    settings.getByRole("textbox", { name: "Workspace name", exact: true }),
  ).toHaveValue("Second renamed");
  await expect(
    settings.getByRole("combobox", {
      name: "Workspace target branch",
      exact: true,
    }),
  ).toContainText("docs");
  await settings
    .getByRole("button", { name: "Delete workspace", exact: true })
    .click();
  await page
    .getByRole("dialog")
    .getByRole("button", { name: "Cancel preview", exact: true })
    .click();
  await expect(picker).toContainText("Second renamed");
  const wcBeforeDelete = execFileSync(
    fixture.jj,
    [
      "-R",
      fixture.folder,
      "--ignore-working-copy",
      "log",
      "-r",
      "@",
      "--no-graph",
      "-T",
      "commit_id",
    ],
    { encoding: "utf8" },
  );
  await settings
    .getByRole("button", { name: "Delete workspace", exact: true })
    .click();
  await page
    .getByRole("dialog")
    .getByRole("button", { name: "Delete workspace", exact: true })
    .click();
  await expect(
    page
      .locator('[data-ui-part="empty-title"]')
      .getByText("No stack workspace", { exact: true }),
  ).toBeVisible();
  expect(readFileSync(join(fixture.folder, "second.txt"), "utf8")).toBe(
    "second workspace edit\n",
  );
  expect(
    execFileSync(
      fixture.jj,
      [
        "-R",
        fixture.folder,
        "--ignore-working-copy",
        "log",
        "-r",
        "@",
        "--no-graph",
        "-T",
        "commit_id",
      ],
      { encoding: "utf8" },
    ),
  ).toBe(wcBeforeDelete);
  await expect(
    page.getByRole("button", { name: "Refresh", exact: true }),
  ).toBeEnabled();
  await picker.click();
  await expect(page.getByRole("option", { name: /Primary/ })).toBeEnabled();
  await page.getByRole("option", { name: /Primary/ }).click();
  await confirm();
  await expect(
    unstaged.getByRole("button", { name: "unsnapshotted.txt", exact: true }),
  ).toBeVisible();
  expect(
    readFileSync(join(fixture.folder, ".jj/jj-store/stacks.toml"), "utf8"),
  ).not.toContain("Second renamed");
  const config = readFileSync(
    join(fixture.folder, ".jj/jj-store/stacks.toml"),
    "utf8",
  );
  expect(config).toContain("[[workspaces]]");
  expect(config).toContain('target = "main"');
  const tracked = execFileSync(
    fixture.jj,
    ["-R", fixture.folder, "--ignore-working-copy", "file", "list"],
    { encoding: "utf8" },
  );
  expect(tracked).not.toContain(".jj/");
  expect(tracked).not.toContain("obstacle.tmp");
  expect(
    readFileSync(join(fixture.folder, ".git/refs/lapis/state/v1"), "utf8"),
  ).toBe(companion);
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  expect(errors).toEqual([]);
});

test("cold_native_selected_file_and_hunk_mutations_with_undo_redo", async ({
  page,
}) => {
  test.setTimeout(120000);
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const command = (...args: string[]) =>
    execFileSync(fixture.jj, ["-R", fixture.folder, "--no-pager", ...args], {
      encoding: "utf8",
      cwd: fixture.folder,
      env: { ...process.env, JJ_CONFIG: fixture.jjConfig },
    }).trim();
  const observe = (...args: string[]) =>
    command("--ignore-working-copy", ...args);
  const confirm = async () => {
    await page
      .getByRole("dialog")
      .getByRole("button", { name: "Apply command", exact: true })
      .click({ timeout: 10000 });
    await expect(page.getByRole("dialog")).toHaveCount(0);
  };
  const prepare = page.getByRole("button", {
    name: "Prepare graph index",
    exact: true,
  });
  const view = (name: string) =>
    page
      .getByRole("tablist", { name: "Local repository views" })
      .getByRole("tab", { name, exact: true });
  await page.goto(`/#/repos/${fixture.repository}/stacks?stack_tab=workspace`);
  const picker = page.getByRole("combobox", {
    name: "Active stack workspace",
    exact: true,
  });
  await expect(picker).toBeVisible();
  // Wait for the current workspace read before deciding whether setup is needed.
  await expect(
    page.getByRole("button", { name: "Workspace actions", exact: true }),
  ).toBeEnabled();
  // This test also supports an exact standalone run against the same cold seed.
  if (
    await page
      .getByRole("combobox", { name: "Workspace target branch", exact: true })
      .isVisible()
  ) {
    await page
      .getByRole("combobox", { name: "Workspace target branch", exact: true })
      .click();
    await page.getByRole("option", { name: /^main/ }).click();
    await page
      .getByRole("button", { name: "Create workspace", exact: true })
      .click();
  } else {
    await page
      .getByRole("button", { name: "Workspace actions", exact: true })
      .click();
    await page
      .getByRole("menuitem", { name: "Create workspace", exact: true })
      .click();
  }
  await page
    .getByRole("dialog")
    .getByRole("textbox", { name: "Name", exact: true })
    .fill("Mutations");
  await page
    .getByRole("dialog")
    .getByRole("button", { name: "Preview command", exact: true })
    .click();
  await confirm();
  await view("Branches").click();
  await page.getByRole("option", { name: /^topic/ }).click();
  const topic = page.getByRole("region", { name: "Stack topic", exact: true });
  const ancestor = topic.getByRole("button", {
    name: "Topic ancestor",
    exact: true,
  });
  await expect
    .poll(
      async () => (await ancestor.isVisible()) || (await prepare.isVisible()),
    )
    .toBe(true);
  if (await prepare.isVisible()) await prepare.click();
  await expect(ancestor).toBeVisible();
  await topic.getByRole("button", { name: "Apply topic", exact: true }).click();
  await confirm();
  await view("Workspace").click();
  const ensureHistory = async () => {
    await expect
      .poll(
        async () => (await ancestor.isVisible()) || (await prepare.isVisible()),
      )
      .toBe(true);
    if (await prepare.isVisible()) await prepare.click();
    await expect(ancestor).toBeVisible();
  };
  await ensureHistory();
  const mergeChange = observe(
    "log",
    "-r",
    "@-",
    "--no-graph",
    "-T",
    "change_id",
  );
  const wipChange = observe("log", "-r", "@", "--no-graph", "-T", "change_id");
  writeFileSync(join(fixture.folder, "stage.txt"), "selected whole file\n");
  const lines = Array.from({ length: 40 }, (_, index) => `line-${index}\n`);
  lines[0] = "new-first\n";
  lines[39] = "new-last\n";
  writeFileSync(join(fixture.folder, "hunks.txt"), lines.join(""));
  await page.getByRole("button", { name: "Refresh", exact: true }).click();
  const unstaged = page.getByRole("region", {
    name: "Unstaged files",
    exact: true,
  });
  const file = unstaged.getByRole("button", { name: "stage.txt", exact: true });
  await expect(file).toBeVisible();
  // The new file proves the refresh selected the new operation. Only then
  // wait for that operation's graph, rather than accepting retained old rows.
  await ensureHistory();
  const composer = topic.getByRole("group", {
    name: "Commit composer for topic",
    exact: true,
  });
  const stage = composer.getByRole("button", {
    name: "Assign to topic",
    exact: true,
  });
  await expect(stage).toBeEnabled();
  await expect(file).toHaveAttribute("draggable", "true");
  await file.dragTo(stage, { timeout: 10000 });
  await expect(composer).toContainText("stage.txt");
  await composer
    .getByRole("textbox", { name: "Commit title", exact: true })
    .fill("Selected whole file");
  await composer
    .getByRole("button", { name: "Create commit", exact: true })
    .click();
  await confirm();
  await ensureHistory();
  const newCommit = topic.getByRole("button", {
    name: "Selected whole file",
    exact: true,
  });
  await expect(newCommit).toBeVisible();
  await expect(file).toHaveCount(0);
  expect(observe("file", "show", "-r", "topic", "--", "stage.txt")).toBe(
    "selected whole file",
  );
  expect(observe("log", "-r", "@-", "--no-graph", "-T", "change_id")).toBe(
    mergeChange,
  );
  expect(observe("log", "-r", "@", "--no-graph", "-T", "change_id")).toBe(
    wipChange,
  );
  await newCommit.click();
  await topic.getByRole("button", { name: "stage.txt", exact: true }).click();
  await unstaged
    .getByRole("button", { name: "hunks.txt", exact: true })
    .click();
  const wipDiff = page.getByRole("region", {
    name: "Selected changes from working",
    exact: true,
  });
  const hunks = wipDiff.getByRole("group", { name: /^Draggable hunk/ });
  await expect(hunks).toHaveCount(2);
  await expect(
    page.getByRole("region", { name: /^Selected changes from stack-/ }),
  ).toHaveCount(1);
  await expect(hunks.first()).toHaveAttribute("draggable", "true");
  const target = topic.locator("[data-revision-id]").filter({
    has: page.getByRole("button", { name: "Topic ancestor", exact: true }),
  });
  await expect(target).toHaveCount(1);
  const ancestorCommit = await target.getAttribute("data-revision-id");
  expect(ancestorCommit).toMatch(/^[0-9a-f]{40}$/);
  const ancestorChange = observe(
    "log",
    "-r",
    ancestorCommit!,
    "--no-graph",
    "-T",
    "change_id",
  );
  expect(ancestorChange).not.toBe("");
  // Start on the card surface, not selectable code or its checkbox.
  await hunks.first().dragTo(target, {
    sourcePosition: { x: 5, y: 5 },
    timeout: 10000,
  });
  await expect(page.getByRole("dialog")).toContainText(
    "Assign selected changes",
  );
  await expect(
    page.getByRole("dialog").getByRole("combobox", { name: "Destination" }),
  ).toContainText("Topic ancestor");
  await page
    .getByRole("dialog")
    .getByRole("button", { name: "Preview command", exact: true })
    .click();
  await confirm();
  await ensureHistory();
  await expect(hunks).toHaveCount(1);
  const rewritten = observe(
    "log",
    "-r",
    ancestorChange,
    "--no-graph",
    "-T",
    "commit_id",
  );
  expect(rewritten).toMatch(/^[0-9a-f]{40}$/);
  expect(
    observe("log", "-r", ancestorChange, "--no-graph", "-T", "description"),
  ).toBe("Topic ancestor");
  const content = observe("file", "show", "-r", rewritten, "--", "hunks.txt");
  expect(content).toContain("new-first");
  expect(content).not.toContain("new-last");
  expect(readFileSync(join(fixture.folder, "hunks.txt"), "utf8")).toContain(
    "new-last",
  );
  const config = readFileSync(
    join(fixture.folder, ".jj/jj-store/stacks.toml"),
    "utf8",
  );
  await page.getByRole("button", { name: "Undo", exact: true }).click();
  await confirm();
  await ensureHistory();
  await expect(hunks).toHaveCount(2);
  await page.getByRole("button", { name: "Redo", exact: true }).click();
  await confirm();
  await ensureHistory();
  await expect(hunks).toHaveCount(1);
  expect(
    readFileSync(join(fixture.folder, ".jj/jj-store/stacks.toml"), "utf8"),
  ).toBe(config);
  // Repository-wide Undo includes an ordinary CLI operation.
  command("describe", "-r", "topic", "-m", "CLI reword");
  await page.getByRole("button", { name: "Refresh", exact: true }).click();
  await ensureHistory();
  await expect(
    topic.getByRole("button", { name: "CLI reword", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Undo", exact: true }).click();
  await confirm();
  await ensureHistory();
  await expect(newCommit).toBeVisible();
  await page.getByRole("button", { name: "Redo", exact: true }).click();
  await confirm();
  await ensureHistory();
  await expect(
    topic.getByRole("button", { name: "CLI reword", exact: true }),
  ).toBeVisible();
  await page.reload();
  await ensureHistory();
  await expect(picker).toContainText("Mutations");
  await expect(
    topic.getByRole("button", { name: "CLI reword", exact: true }),
  ).toBeVisible();
  expect(observe("log", "-r", "@-", "--no-graph", "-T", "change_id")).toBe(
    mergeChange,
  );
  expect(observe("log", "-r", "@", "--no-graph", "-T", "change_id")).toBe(
    wipChange,
  );
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  expect(errors).toEqual([]);
});

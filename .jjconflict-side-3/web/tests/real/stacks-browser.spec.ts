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
      .click();
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
  await page
    .getByRole("button", { name: "Prepare graph index", exact: true })
    .click();
  const topic = page.getByRole("region", { name: "Stack topic", exact: true });
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
  await expect(diff).toContainText("new physical file");
  expect(
    await diff
      .getByRole("group", { name: /Draggable hunk/ })
      .getAttribute("draggable"),
  ).toBe("false");
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
  await picker.click();
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
  await page.getByRole("option", { name: /Second/ }).click();
  await confirm();
  await expect(
    unstaged.getByRole("button", { name: "second.txt", exact: true }),
  ).toBeVisible();
  const config = readFileSync(
    join(fixture.folder, ".jj/jj-store/stacks.toml"),
    "utf8",
  );
  expect(config).toContain("[[workspaces]]");
  expect(config).toContain('target = "main"');
  const tracked = execFileSync(
    "/opt/homebrew/bin/jj",
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

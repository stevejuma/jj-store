import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
const fixture = process.env.JJ_STORE_STACKS_BROWSER_FIXTURE
  ? JSON.parse(
      readFileSync(process.env.JJ_STORE_STACKS_BROWSER_FIXTURE, "utf8"),
    )
  : null;
test.skip(!fixture, "Owned isolated Stacks browser fixture required");
const binary = resolve(process.env.CARGO_TARGET_DIR ?? "target", "debug/jj");
function state() {
  const jj = (...args: string[]) =>
    execFileSync(
      binary,
      ["-R", fixture.folder, "--ignore-working-copy", "--no-pager", ...args],
      { encoding: "utf8" },
    );
  const files = (folder: string): unknown[] =>
    readdirSync(folder, { withFileTypes: true })
      .filter((entry) => ![".jj", ".git"].includes(entry.name))
      .sort((a, b) => a.name.localeCompare(b.name))
      .map((entry) => [
        entry.name,
        entry.isDirectory()
          ? files(join(folder, entry.name))
          : readFileSync(join(folder, entry.name)).toString("base64"),
      ]);
  return {
    operations: jj("op", "log", "-n", "20", "-T", 'id ++ "\\n"'),
    bookmarks: jj("bookmark", "list", "-T", 'name ++ "\\n"'),
    companion: readFileSync(
      join(fixture.folder, ".git/refs/lapis/state/v1"),
      "utf8",
    ),
    files: files(fixture.folder),
    checkout: ["checkout", "tree_state"].map((name) =>
      readFileSync(join(fixture.folder, ".jj/working_copy", name)).toString(
        "base64",
      ),
    ),
  };
}
test("cold_registered_stacks_browse_real_ancestors_files_and_diffs_without_writes", async ({
  page,
}) => {
  const before = state(),
    errors: string[] = [],
    commands: string[] = [],
    ordinaryGraphs: string[] = [];
  const observations: any[] = [],
    observationRequests: unknown[] = [];
  page.on("response", async (response) => {
    if (response.url().endsWith("/working-tree/observe") && response.ok())
      observations.push(await response.json());
  });
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("request", (request) => {
    if (request.url().endsWith("/working-tree/observe"))
      observationRequests.push(request.postDataJSON());
    if (request.url().includes("/commands")) commands.push(request.url());
    if (
      request.url().endsWith("/query") &&
      request.postDataJSON()?.request?.kind === "graph"
    )
      ordinaryGraphs.push(request.url());
  });
  await page.goto(
    `/#/repos/${fixture.repository}/stacks?at_op=${fixture.operation}`,
  );
  await expect(
    page.getByRole("region", { name: "Local stack workspace", exact: true }),
  ).toBeVisible();
  await expect.poll(() => observations.length).toBeGreaterThan(0);
  expect(observations[0].snapshot.operation).toBe(fixture.operation);
  expect(
    observations[0].changes.map((change: any) => [change.path, change.status]),
  ).toEqual([
    ["README.md", "modified"],
    ["unsnapshotted.txt", "added"],
  ]);
  expect(observations[0].obstacle_count).toBeGreaterThanOrEqual(2);
  expect(observationRequests[0]).toEqual({
    version: 1,
    binding_generation: observations[0].snapshot.binding_generation,
    operation: fixture.operation,
  });
  await page.getByRole("tab", { name: "Branches", exact: true }).click();
  await page
    .getByRole("combobox", { name: "Workspace target branch", exact: true })
    .click();
  await page.getByRole("option", { name: /main/ }).click();
  await page.getByRole("option", { name: /topic/ }).click();
  const prepare = page.getByRole("button", {
    name: "Prepare graph index",
    exact: true,
  });
  await expect(prepare).toBeVisible();
  await prepare.click();
  const stack = page.getByRole("region", { name: "Stack topic", exact: true });
  await expect(
    stack.getByRole("button", { name: "Topic tip", exact: true }),
  ).toBeVisible();
  await stack
    .getByRole("button", { name: "Topic ancestor", exact: true })
    .click();
  await stack
    .getByRole("button", { name: "src/topic.ts", exact: true })
    .click();
  const diff = page.getByRole("region", { name: /^Selected changes from/ });
  await expect(
    diff.getByRole("group", { name: /^Draggable hunk/ }),
  ).toBeVisible();
  await expect(diff).toContainText("export const one = 1");
  await expect(
    diff.getByRole("group", { name: /^Draggable hunk/ }),
  ).toHaveAttribute("draggable", "false");
  await expect(
    page.getByRole("button", { name: "Create stack", exact: true }),
  ).toBeDisabled();
  await page.getByRole("button", { name: "Refresh", exact: true }).click();
  await expect(
    stack.getByRole("button", { name: "Topic ancestor", exact: true }),
  ).toBeVisible();
  await expect(
    diff.getByRole("group", { name: /^Draggable hunk/ }),
  ).toBeVisible();
  await stack
    .getByRole("button", { name: "Show all changes in topic", exact: true })
    .click();
  await expect(
    stack.getByRole("button", { name: "src/topic.ts", exact: true }),
  ).toBeVisible();
  await page.getByRole("option", { name: /main/ }).click();
  await expect(
    page.getByRole("region", { name: "Stack main", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("tablist", { name: "Local repository views", exact: true })
    .getByRole("tab", { name: "Workspace", exact: true })
    .click();
  await expect(
    page.getByText("No stack workspace", { exact: true }).first(),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Create workspace", exact: true }),
  ).toBeDisabled();
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  expect(ordinaryGraphs).toEqual([]);
  expect(commands).toEqual([]);
  expect(errors).toEqual([]);
  expect(state()).toEqual(before);
  if (process.env.JJ_STORE_WORKING_OBSERVATION_CAPTURE)
    writeFileSync(
      process.env.JJ_STORE_WORKING_OBSERVATION_CAPTURE,
      JSON.stringify(
        observations.map((data) => ({
          path: "/v1/local/repos/{repo}/working-tree/observe",
          status: 200,
          data,
        })),
      ),
    );
});

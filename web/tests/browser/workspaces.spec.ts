import { expect, test, type Page } from "@playwright/test";
import {
  api,
  commit,
  graph,
  operation,
  repo,
} from "./native-browser-fixture.js";

async function workspaceApi(page: Page) {
  await api(page, { access: "local_trusted", changed: true });
  const queries: URL[] = [];
  page.on("request", (request) => {
    if (request.url().includes(`/v1/repos/${repo}/`))
      queries.push(new URL(request.url()));
  });
  // Two logical workspaces share a native commit, on separate reference pages.
  await page.route(`**/v1/repos/${repo}/references?*`, async (route) => {
    const query = new URL(route.request().url()).searchParams;
    expect(query.get("at_op")).toBe(operation);
    const more = query.has("cursor");
    const workspaces = [more ? "agent/review" : "default"];
    const node = graph.data.data.nodes.find(
      (node: any) => node.commit.id === commit,
    ).commit;
    const next = more ? null : "workspace-page-2";
    await route.fulfill({
      json: {
        snapshot: structuredClone(graph.snapshot),
        data: {
          version: 1,
          operation,
          references: [
            {
              kind: more ? "change" : "bookmark",
              name: more ? "" : "main",
              change: node.change_id,
              terms: [commit],
              conflicted: false,
              workspaces,
              targets: [
                {
                  commit,
                  change: node.change_id,
                  parents: [...node.parents],
                  description: node.description,
                  author: structuredClone(node.author),
                  committer: structuredClone(node.committer),
                  conflicted: node.conflicted,
                  workspaces,
                },
              ],
            },
          ],
          previous: null,
          next,
          stats: graph.data.stats,
        },
        previous_cursor: null,
        next_cursor: next,
      },
    });
  });
  return queries;
}

async function choose(page: Page, workspace: string) {
  await page.getByRole("combobox", { name: "Repository workspace" }).click();
  await page
    .getByRole("combobox", { name: "Select workspace", exact: true })
    .fill(workspace);
  await page.getByRole("option", { name: new RegExp(workspace) }).click();
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toHaveText(workspace);
}

test("workspace names sharing a commit survive cold reload, tabs, history and explicit Refresh", async ({
  page,
}) => {
  const queries = await workspaceApi(page);
  await page.goto("/");
  await choose(page, "default");
  await page.getByRole("tab", { name: "Overview", exact: true }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  const defaultHash = new URL(page.url()).hash;
  expect(defaultHash).toBe(
    `#/repos/${repo}/files?at_op=${operation}&workspace=default`,
  );
  expect(
    queries.some((url) => url.pathname.endsWith(`/commits/${commit}/paths`)),
  ).toBe(true);

  await page.reload();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toHaveText("default");
  expect(new URL(page.url()).hash).toBe(defaultHash);

  await choose(page, "agent/review");
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  const reviewHash = new URL(page.url()).hash;
  expect(reviewHash).toBe(
    `#/repos/${repo}/files?at_op=${operation}&workspace=agent%2Freview`,
  );
  await page.goBack();
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toHaveText("default");
  expect(new URL(page.url()).hash).toBe(defaultHash);
  await page.goForward();
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toHaveText("agent/review");
  expect(new URL(page.url()).hash).toBe(reviewHash);

  await page.getByRole("tab", { name: "Commits", exact: true }).click();
  await expect
    .poll(() =>
      queries.some(
        (url) =>
          url.pathname.endsWith("/log") &&
          url.searchParams.get("workspace") === "agent/review" &&
          url.searchParams.get("revset") === `::${commit}`,
      ),
    )
    .toBe(true);
  await page.getByRole("tab", { name: "Overview", exact: true }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  await page.getByRole("button", { name: /Refresh snapshot/ }).click();
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).toBe(reviewHash);
});

test("missing workspace stays explicit and can recover to repository overview", async ({
  page,
}) => {
  await workspaceApi(page);
  await page.goto(
    `/#/repos/${repo}/files?at_op=${operation}&workspace=forgotten`,
  );
  await expect(page.getByRole("alert")).toContainText(
    "The requested workspace is unavailable at this operation snapshot.",
  );
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("combobox", { name: "Repository workspace" }),
  ).toHaveText("forgotten");
  await choose(page, "Repository overview");
  await expect(
    page.getByRole("table", { name: "Repository files" }),
  ).toBeVisible();
  expect(new URL(page.url()).hash).not.toContain("workspace=");
});

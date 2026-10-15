import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  connect,
  operation,
  repo,
  sample,
  graph,
  token,
} from "./native-browser-fixture.js";

test("command search opens native results and routes through stable identities", async ({
  page,
}) => {
  const requests = await api(page);
  await page.route("**/capabilities", (r) =>
    r.fulfill({ json: { ...sample("capabilities"), search: true } }),
  );
  await page.route("**/content/search?**", (r) =>
    r.fulfill({
      json: {
        version: 1,
        snapshot: { repo_id: repo },
        results: [],
        next_cursor: null,
      },
    }),
  );
  await page.route(new RegExp(`/v1/repos/${repo}/search(?:\\?|$)`), (r) => {
    const url = new URL(r.request().url());
    expect(r.request().headers().authorization).toBe(`Bearer ${token}`);
    expect(url.searchParams.get("at_op")).toBe(operation);
    expect(url.searchParams.get("mode")).toBe("visible_commits");
    requests.push("search");
    const node = graph.data.data.nodes.find(
      (n: any) => n.commit.id === commit,
    ).commit;
    const file = url.searchParams.get("q") === "file";
    return r.fulfill({
      json: {
        version: 1,
        repository: repo,
        epoch: sample("snapshot").epoch,
        repository_position: 1,
        indexed_position: 1,
        index_generation: "aa".repeat(32),
        selected_view: { ...sample("snapshot"), operation_id: operation },
        scope_snapshots: {
          main: { state: "ready", operation, commits: [commit] },
        },
        results: [
          {
            key: file
              ? { kind: "file", scope: "main", path: "bin" }
              : { kind: "commit", id: commit },
            title: file ? "bin" : node.description,
            body: "",
            actor: null,
            author: node.author.name,
            workspace: null,
            operation,
            commit,
            change: node.change_id,
            file: file ? "ef".repeat(64) : null,
            parents: node.parents,
            predecessors: [],
            attributes: {},
            language: null,
            has_conflict: false,
            timestamp_millis: 0,
          },
        ],
        is_stale: false,
        degraded: false,
        next_cursor: null,
        truncated: false,
      },
    });
  });
  await connect(page);
  await page
    .getByRole("button", { name: "Search repository", exact: true })
    .click();
  const dialog = page.getByRole("dialog", {
    name: "Search workspace",
    exact: true,
  });
  await dialog
    .getByRole("combobox", { name: "Search workspace" })
    .fill("binary");
  await dialog.getByRole("option", { name: /binary/u }).click();
  await expect(dialog).not.toBeVisible();
  await expect
    .poll(() =>
      page.evaluate(() => ({
        overflow: document.body.style.overflow,
        pointerEvents: document.body.style.pointerEvents,
      })),
    )
    .toEqual({ overflow: "", pointerEvents: "" });
  await expect(
    page.getByRole("list", { name: "Repository breadcrumb" }),
  ).toHaveCount(0);
  await expect(page.getByRole("tab", { name: "Commits" })).toHaveAttribute(
    "data-state",
    "active",
  );
  const commitHash = new URL(page.url()).hash;
  expect(commitHash).toContain(`at_op=${operation}`);
  expect(commitHash).toContain(`commit=${commit}`);
  expect(commitHash).not.toContain("binary");
  expect(commitHash).not.toContain("cursor");
  expect(requests).toContain("search");

  await expect(page.locator(".store-search-dialog")).toHaveCount(0);
  await page.keyboard.press("Control+K");
  await page
    .getByRole("dialog", { name: "Search workspace", exact: true })
    .getByRole("combobox", { name: "Search workspace" })
    .fill("file");
  await page
    .getByRole("dialog", { name: "Search workspace", exact: true })
    .getByRole("option", { name: /^bin/u })
    .click();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Search repository", exact: true })
    .click();
  await expect(
    page.getByRole("dialog", { name: "Search workspace", exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Close search" }).click();
  await expect(
    page.getByRole("dialog", { name: "Search workspace", exact: true }),
  ).not.toBeVisible();
  await page.goBack();
  await expect(
    page.getByRole("list", { name: "Repository breadcrumb" }),
  ).toHaveCount(0);
  await page.goForward();
  await expect(
    page.getByRole("region", { name: "Source preview for bin" }),
  ).toBeVisible();
  const fileHash = new URL(page.url()).hash;
  expect(fileHash).toContain("bookmark=main");
  expect(fileHash).toContain(`commit=${commit}`);
  expect(fileHash).toContain("file=bin");
  expect(fileHash).not.toContain("file%20");
});

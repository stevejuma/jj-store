import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  graph,
  operation,
  repo,
} from "./native-browser-fixture.js";

const previous = "ec".repeat(64);

test("Evo Log opens a pinned column, pages native predecessors, restores and closes independently", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const source = graph.data.data.nodes.find(
    (node: any) => node.commit.id === commit,
  ).commit;
  const requests: URL[] = [];
  await page.route(`**/v1/repos/${repo}/evolution*`, async (route) => {
    const url = new URL(route.request().url());
    requests.push(url);
    expect(url.searchParams.get("at_op")).toBe(operation);
    expect(route.request().headers()["authorization"]).toBeUndefined();
    const older =
      url.searchParams.has("cursor") ||
      url.searchParams.get("commit") === previous;
    const value = structuredClone(graph);
    value.data.data = {
      graph: "evolution",
      nodes: [
        {
          commit: {
            ...source,
            id: older ? previous : commit,
            description: older
              ? "Earlier native rewrite"
              : "Selected native change",
            bookmarks: [],
            workspaces: [],
          },
          predecessors: older ? [] : [previous],
          predecessors_known: true,
          operation,
          legacy_predecessors: [],
        },
      ],
    };
    value.next_cursor = older ? null : "native-evo-page-2";
    await route.fulfill({ json: value });
  });
  await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
  await page.getByRole("button", { name: "Toggle details" }).click();
  const rail = page.getByRole("complementary", { name: "Repository details" });
  await rail.getByRole("button", { name: "Evo Log", exact: true }).click();
  const evolution = page.getByRole("list", { name: "Evolution log" });
  await expect(evolution).toBeVisible();
  expect(requests[0]?.searchParams.get("commit")).toBe(commit);
  await expect(page).toHaveURL(new RegExp(`evolution=${commit}`));
  await expect(
    evolution.getByRole("button", {
      name: "Earlier native rewrite",
      exact: true,
    }),
  ).toBeVisible();
  expect(requests.at(-1)?.searchParams.get("cursor")).toBe("native-evo-page-2");
  await evolution
    .getByRole("button", { name: "Earlier native rewrite", exact: true })
    .click();
  await expect(page).toHaveURL(new RegExp(`evolution=${previous}`));
  expect(new URL(page.url()).hash).toContain(`commit=${commit}`);
  await page.reload();
  await expect(evolution).toBeVisible();
  expect(requests.at(-1)?.searchParams.get("commit")).toBe(previous);
  await page.getByRole("button", { name: "Close Evo Log column" }).click();
  await expect(evolution).toHaveCount(0);
  expect(new URL(page.url()).hash).not.toContain("evolution=");
  expect(new URL(page.url()).hash).toContain(`commit=${commit}`);
  await page.goBack();
  await expect(evolution).toBeVisible();
  if (!(await rail.isVisible()))
    await page.getByRole("button", { name: "Toggle details" }).click();
  await expect(rail).toBeVisible();
  await rail.getByRole("button", { name: "Op Log", exact: true }).click();
  await expect(evolution).toHaveCount(0);
  await expect(
    rail.getByRole("button", { name: "Evo Log", exact: true }),
  ).toHaveCount(0);
});

import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  operation,
  pathsPage,
  repo,
} from "./native-browser-fixture.js";

test("directory navigation shows loading once and reuses completed rows and metadata", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const requests: string[] = [];
  let release!: () => void;
  const pending = new Promise<void>((resolve) => {
    release = resolve;
  });
  await page.route("**/commits/*/paths?**", async (route) => {
    const query = new URL(route.request().url()).searchParams;
    const path = query.get("path") ?? "",
      metadata = query.get("include_last_change") === "true";
    requests.push(`${path}:${metadata}`);
    if (path === "arch" && !metadata) await pending;
    const result = pathsPage(null, { path });
    const file = result.data.entries[0];
    result.data.entries = path
      ? [{ ...file, path: "arch/example.c" }]
      : [
          {
            ...file,
            path: "arch",
            terms: [{ type: "tree", id: "aa".repeat(64) }],
          },
        ];
    for (const entry of result.data.entries)
      entry.last_change = metadata
        ? { ...file.last_change, description: "Actual edit" }
        : { status: "unavailable", reason: "not_requested" };
    await route.fulfill({ json: result });
  });
  try {
    await page.goto(
      `/#/repos/${repo}/files?at_op=${operation}&bookmark=main&commit=${commit}`,
    );
    const table = page.getByRole("table", {
      name: /^(Repository files|Files in arch)$/,
    });
    await expect(table.getByText("Actual edit", { exact: true })).toBeVisible();
    const folder = table.getByRole("button", { name: "Open directory arch" });
    await folder.focus();
    await folder.press("Enter");
    await expect(
      page.getByRole("status", { name: "Loading repository files" }),
    ).toBeVisible();
    release();
    await expect(
      table.getByRole("button", { name: "Open file example.c" }),
    ).toBeVisible();
    await expect(table.getByText("Actual edit", { exact: true })).toBeVisible();
    const before = [...requests];
    await page.getByRole("link", { name: "Files", exact: true }).click();
    await table.getByRole("button", { name: "Open directory arch" }).click();
    await expect(table.getByText("Actual edit", { exact: true })).toBeVisible();
    expect(requests).toEqual(before);
  } finally {
    release();
  }
});

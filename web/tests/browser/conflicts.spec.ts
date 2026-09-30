import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  graph,
  mutationSample,
  operation,
  repo,
} from "./native-browser-fixture.js";

const fingerprint = (byte: string) => byte.repeat(128);
const file = (byte: string) => ({
  type: "file",
  id: byte.repeat(128),
  executable: false,
  copy: { type: "placeholder" },
});
const conflicts = [
  {
    path: "src/lib.rs",
    fingerprint: fingerprint("a"),
    terms: [
      { id: 0, role: "add", label: "left", value: file("b") },
      { id: 1, role: "remove", label: "base", value: file("c") },
      { id: 2, role: "add", label: "right", value: file("d") },
    ],
    editor: {
      status: "text",
      contents: [
        "pub fn left() {}\n",
        "pub fn base() {}\n",
        "pub fn right() {}\n",
      ],
      executable: false,
    },
  },
  {
    path: "assets/logo.bin",
    fingerprint: fingerprint("e"),
    terms: [
      { id: 0, role: "add", label: "left", value: file("f") },
      { id: 1, role: "remove", label: "base", value: null },
      { id: 2, role: "add", label: "right", value: file("1") },
    ],
    editor: { status: "unsupported", reason: "binary" },
  },
];

test("native conflict editor stages and previews one complete atomic selection", async ({
  page,
}) => {
  await api(page, { access: "local_trusted", changed: true });
  let upload: { url: URL; bytes: Buffer } | undefined;
  const previews: any[] = [];
  await page.route("**/commits/*/conflicts?*", async (route) => {
    await route.fulfill({
      json: {
        snapshot: graph.snapshot,
        data: {
          version: 1,
          commit,
          change: graph.data.data.nodes.find(
            (node: any) => node.commit.id === commit,
          ).commit.change_id,
          conflicts,
          stats: graph.data.stats,
        },
        next_cursor: null,
      },
    });
  });
  await page.route("**/conflict-uploads?*", async (route) => {
    upload = {
      url: new URL(route.request().url()),
      bytes: route.request().postDataBuffer()!,
    };
    await route.fulfill({
      json: {
        version: 1,
        id: "23".repeat(32),
        file: "34".repeat(64),
        bytes: upload.bytes.byteLength,
        binding: {
          expected: {
            epoch: graph.snapshot.epoch,
            position: graph.snapshot.published_seq,
            heads: [operation],
          },
          commit,
          path: conflicts[0]!.path,
          fingerprint: conflicts[0]!.fingerprint,
        },
      },
    });
  });
  await page.route("**/mutations/preview", async (route) => {
    const body = route.request().postDataJSON();
    previews.push(body);
    const value = mutationSample("mutations/preview", "prepared");
    value.preview.request_id = body.request_id;
    value.preview.expected = body.expected;
    await route.fulfill({ json: value });
  });

  await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Resolve conflicts" }).click();
  const dialog = page.getByRole("region", { name: "Resolve native conflicts" });
  const header = dialog.locator('[data-ui-part="surface-layer-header"]');
  const body = dialog.locator('[data-ui-part="surface-layer-body"]');
  const browser = dialog.getByRole("complementary", {
    name: "Conflict file browser",
  });
  await expect(
    browser.locator('[data-ui-component="file-listing"]'),
  ).toBeVisible();
  await expect(dialog.locator(".store-conflict-editor__footer")).toHaveCount(0);
  await expect(dialog.getByRole("button", { name: "Cancel" })).toHaveCount(0);
  for (const name of ["Delete path", "Stage merged file", "Preview 0/2"]) {
    await expect(header.getByRole("button", { name })).toBeVisible();
  }
  const bodyBox = (await body.boundingBox())!;
  const browserBox = (await browser.boundingBox())!;
  expect(Math.abs(browserBox.height - bodyBox.height)).toBeLessThan(1);
  expect(
    await browser.evaluate(
      (element) => getComputedStyle(element).borderRightWidth,
    ),
  ).toBe("1px");
  await header
    .getByRole("button", { name: "Hide conflict file browser" })
    .click();
  await expect(browser).toBeHidden();
  await header
    .getByRole("button", { name: "Show conflict file browser" })
    .click();
  await expect(browser).toBeVisible();
  await expect(
    page.locator(
      '[data-ui-part="main"]:not(.store-conflict-editor [data-ui-part="main"])',
    ),
  ).toHaveAttribute("inert", "");
  await expect(dialog.getByTitle(conflicts[0]!.fingerprint)).toHaveCount(0);
  await dialog
    .getByRole("button", { name: "Accept All Incoming Changes from Left" })
    .click();
  await expect(
    dialog.getByRole("button", { name: "Preview 1/2" }),
  ).toBeVisible();
  await dialog
    .getByRole("textbox", { name: "Edit Resolved" })
    .fill("pub fn merged() {}\n");
  await expect(
    dialog.getByRole("button", { name: "Preview 0/2" }),
  ).toBeVisible();
  await dialog.getByRole("button", { name: "Stage merged file" }).click();
  await expect(dialog.getByText(/Merged file staged/u)).toBeVisible();
  await dialog.getByRole("button", { name: "assets/logo.bin" }).click();
  await expect(
    dialog.getByText(/native descriptor reports binary/u),
  ).toBeVisible();
  await dialog.getByRole("button", { name: "Use right" }).click();
  await dialog.getByRole("button", { name: "Preview 2/2" }).click();
  await expect(dialog.getByText("Preview prepared")).toBeVisible();

  expect(upload?.url.searchParams.get("operation")).toBe(operation);
  expect(upload?.url.searchParams.get("commit")).toBe(commit);
  expect(upload?.url.searchParams.get("path")).toBe("src/lib.rs");
  expect(upload?.url.searchParams.get("fingerprint")).toBe(
    conflicts[0]!.fingerprint,
  );
  expect(upload?.bytes.toString("utf8")).toBe("pub fn merged() {}\n");
  expect(previews).toHaveLength(1);
  expect(previews[0].command).toEqual({
    type: "resolve_conflicts",
    commit,
    paths: [
      {
        path: "src/lib.rs",
        fingerprint: conflicts[0]!.fingerprint,
        resolution: {
          type: "uploaded_file",
          upload: "23".repeat(32),
          executable: false,
          copy_from_term: 0,
        },
      },
      {
        path: "assets/logo.bin",
        fingerprint: conflicts[1]!.fingerprint,
        resolution: { type: "select_term", term: 2 },
      },
    ],
  });
  await dialog.getByRole("button", { name: "Back", exact: true }).click();
  await dialog.getByRole("button", { name: "src/lib.rs" }).click();
  await expect(
    dialog.getByRole("textbox", { name: "Edit Resolved" }),
  ).toHaveValue("pub fn merged() {}\n");
  await dialog
    .getByRole("textbox", { name: "Edit Resolved" })
    .fill("pub fn revised() {}\n");
  await expect(
    dialog.getByRole("button", { name: "Preview 1/2" }),
  ).toBeVisible();
  await dialog.getByRole("button", { name: "Close conflict editor" }).click();
  await expect(dialog).toHaveCount(0);
  await expect(
    page.locator(
      '[data-ui-part="main"]:not(.store-conflict-editor [data-ui-part="main"])',
    ),
  ).not.toHaveAttribute("inert", "");
  await expect(
    page.getByRole("button", { name: "Repository actions" }),
  ).toBeFocused();
});

for (const [side, term] of [
  ["Left", 0],
  ["Right", 2],
] as const) {
  test(`accept-all ${side} preserves the exact native term and sheet lifecycle`, async ({
    page,
  }) => {
    await api(page, { access: "local_trusted", changed: true });
    await page.route("**/commits/*/conflicts?*", (route) =>
      route.fulfill({
        json: {
          snapshot: graph.snapshot,
          data: {
            version: 1,
            commit,
            change: "ab".repeat(16),
            conflicts: [conflicts[0]],
            stats: graph.data.stats,
          },
          next_cursor: null,
        },
      }),
    );
    const previews: any[] = [];
    await page.route("**/mutations/preview", async (route) => {
      const body = route.request().postDataJSON();
      previews.push(body);
      const value = mutationSample("mutations/preview", "prepared");
      value.preview.request_id = body.request_id;
      value.preview.expected = body.expected;
      await route.fulfill({ json: value });
    });
    await page.goto(`/#/repos/${repo}/log?at_op=${operation}&commit=${commit}`);
    await page.getByRole("button", { name: "Repository actions" }).click();
    await page.getByRole("menuitem", { name: "Resolve conflicts" }).click();
    const sheet = page.getByRole("region", {
      name: "Resolve native conflicts",
    });
    const editor = sheet.locator('[data-ui-component="merge-editor"]');
    await expect(editor).toBeVisible();
    for (const pane of ["left", "base", "right"]) {
      await expect(
        editor
          .locator(
            `[data-ui-part="merge-view"][data-merge-side="${pane}"] .ui-code-token-keyword`,
          )
          .first(),
      ).toBeVisible();
    }
    await sheet
      .getByRole("button", { name: `Accept All Incoming Changes from ${side}` })
      .click();
    if (side === "Right") {
      await page.setViewportSize({ width: 390, height: 800 });
      await expect(page.locator(".store-conflict-editor")).toHaveAttribute(
        "data-mobile-placement",
        "root",
      );
      await expect(
        sheet.getByRole("textbox", { name: "Edit Resolved" }),
      ).toBeVisible();
      await expect(
        sheet.getByRole("button", { name: "Stage merged file" }),
      ).toBeVisible();
      const browser = sheet.getByRole("complementary", {
        name: "Conflict file browser",
      });
      await expect(browser).toBeHidden();
      await sheet
        .getByRole("button", { name: "Show conflict file browser" })
        .click();
      await expect(browser).toBeVisible();
      await expect(
        sheet.getByRole("textbox", { name: "Edit Resolved" }),
      ).toBeHidden();
      await browser.getByRole("button", { name: "src/lib.rs" }).click();
      await expect(browser).toBeHidden();
      await expect(
        sheet.getByRole("textbox", { name: "Edit Resolved" }),
      ).toBeVisible();
    }
    await expect(
      sheet.getByRole("textbox", { name: "Edit Resolved" }),
    ).toHaveValue(conflicts[0]!.editor.contents![term]!);
    await sheet.getByRole("button", { name: "Preview 1/1" }).click();
    await expect(sheet.getByText("Preview prepared")).toBeVisible();
    expect(previews[0].command.paths).toEqual([
      {
        path: conflicts[0]!.path,
        fingerprint: conflicts[0]!.fingerprint,
        resolution: { type: "select_term", term },
      },
    ]);
    await sheet.getByRole("button", { name: "Back", exact: true }).click();
    await page.keyboard.press("Escape");
    await expect(sheet).toHaveCount(0);
    await expect(
      page.locator(
        '[data-ui-part="main"]:not(.store-conflict-editor [data-ui-part="main"])',
      ),
    ).not.toHaveAttribute("inert", "");
  });
}

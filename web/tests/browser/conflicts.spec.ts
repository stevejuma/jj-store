import { expect, test } from "@playwright/test";
import {
  api,
  commit,
  connect,
  graph,
  mutationSample,
  operation,
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
      contents: ["left\n", "base\n", "right\n"],
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
  await api(page);
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

  await connect(page);
  await page.getByRole("option", { name: "main", exact: true }).click();
  await page
    .getByRole("button", { name: /binary <script>/u })
    .first()
    .click();
  await page.getByRole("button", { name: "Repository actions" }).click();
  await page.getByRole("menuitem", { name: "Resolve conflicts" }).click();
  const dialog = page.getByRole("dialog", { name: "Resolve native conflicts" });
  await expect(dialog.getByTitle(conflicts[0]!.fingerprint)).toBeVisible();
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
  expect(upload?.bytes.toString("utf8")).toContain("base");
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
});

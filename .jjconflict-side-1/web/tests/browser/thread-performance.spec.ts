import { test, expect } from "@playwright/test";
import { api, repo, operation, sample } from "./native-browser-fixture.js";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";

test("cold Threads load conversation before directory and confirmed reaction refreshes one target", async ({
  page,
}) => {
  await page.setViewportSize({ width: 1800, height: 1000 });
  await api(page, { access: "local_trusted" });
  const f = createThreadTransport();
  const library = await f.transport.emojis();
  if (library.items.type !== "emojis") throw Error("emoji library");
  const preview = library.items.definitions[0]!.preview.id;
  const root = f.history.get(ids.thread)![0]!.value.event;
  const reply = "77".repeat(16);
  f.addMessage(
    ids.thread,
    reply,
    "Retained reply",
    root.type === "message" ? root.revision : null,
  );
  const counts = new Map<string, number>();
  let releaseDirectory!: () => void;
  const directory = new Promise<void>((resolve) => {
    releaseDirectory = resolve;
  });
  await page.route(`**/v1/repos/${repo}/**`, async (route) => {
    const path = new URL(route.request().url()).pathname.split(
      `/v1/repos/${repo}/`,
    )[1]!;
    counts.set(path, (counts.get(path) ?? 0) + 1);
    let value: any;
    if (path === "snapshot")
      value = { ...sample("snapshot"), actor: ids.actor };
    else if (path === "threads") {
      await directory;
      value = await f.transport.list();
    } else if (/^threads\/[^/]+\/timeline$/.test(path))
      value = await f.transport.events(ids.thread);
    else if (path === `threads/${ids.thread}`)
      value = await f.transport.thread(ids.thread);
    else if (path === "reaction-emojis") value = await f.transport.emojis();
    else if (path === "reactions/query")
      value = await f.transport.reactions(
        route.request().postDataJSON().target,
      );
    else if (path === "reactions/commands")
      value = await f.transport.command(
        "reactions",
        route.request().postDataJSON(),
      );
    else if (path.startsWith("reaction-assets/")) {
      const asset = await f.transport.asset(path.split("/")[1]!);
      await route.fulfill({
        body: Buffer.from(asset.bytes),
        contentType: asset.type,
      });
      return;
    } else {
      await route.fallback();
      return;
    }
    await route.fulfill({
      json: {
        ...value,
        ...(value.snapshot
          ? {
              snapshot: {
                ...value.snapshot,
                repo_id: repo,
                operation_heads: [operation],
              },
            }
          : {}),
      },
    });
  });
  await page.goto(
    `/#/repos/${repo}/threads?at_op=${operation}&thread=${ids.thread}&reply=${reply}`,
  );
  const log = page.getByRole("log", { name: "general messages" });
  await expect(log.getByText("Welcome to the channel")).toBeVisible();
  // The directory is still blocked: conversation loading is independent.
  expect(counts.get("threads")).toBe(1);
  releaseDirectory();
  const message = page
    .getByRole("complementary", { name: "Thread replies" })
    .locator(`[data-message-id="${reply}"]`);
  await expect(message.getByText("Retained reply")).toBeVisible();
  await message.hover();
  await message
    .getByRole("button", { name: "Add reaction", exact: true })
    .click();
  await page.getByRole("button", { name: ":lapis:", exact: true }).waitFor();
  const before = new Map(counts);
  await page.getByRole("button", { name: ":lapis:", exact: true }).click();
  await expect(
    message.getByRole("button", { name: /1 reaction with Lapis/ }),
  ).toBeVisible();
  expect(counts.get("reactions/query")).toBe(
    (before.get("reactions/query") ?? 0) + 1,
  );
  for (const path of [
    "threads",
    `threads/${ids.thread}`,
    `threads/${ids.thread}/timeline`,
    "reaction-emojis",
    `reaction-assets/${preview}`,
  ])
    expect(counts.get(path)).toBe(before.get(path));
  expect(counts.get("references")).toBeUndefined();
  expect(counts.get("log")).toBeUndefined();
  await expect(log.getByText("Welcome to the channel")).toBeVisible();
});

import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, repo, operation } from "./native-browser-fixture.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";

test("cold LocalTrusted Threads shell settings references scoped search and reply focus", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  const fixture = createThreadTransport();
  const scopes: URLSearchParams[] = [];
  const patch = (value: any) => ({
    ...value,
    snapshot: {
      ...value.snapshot,
      repo_id: repo,
      operation_heads: [operation],
    },
  });
  await page.route(`**/v1/repos/${repo}/**`, async (route) => {
    const url = new URL(route.request().url()),
      path = url.pathname.split(`/v1/repos/${repo}/`)[1]!;
    const transport = fixture.transport;
    let value: any;
    if (path === "threads") {
      value = await transport.list();
      for (const thread of value.items.threads)
        thread.directory = {
          excerpt: "Welcome to the channel",
          participants: [THREAD_IDS.actor],
          reply_count: 0,
          message_count: 1,
          last_activity_at_millis: 1725000000000,
        };
    } else if (/^threads\/[^/]+\/timeline$/.test(path))
      value = await transport.events(path.split("/")[1]!);
    else if (/^threads\/[0-9a-f]{32}$/.test(path))
      value = await transport.thread(path.split("/")[1]!);
    else if (path === "reaction-emojis") value = await transport.emojis();
    else if (path === "reactions/query")
      value = await transport.reactions(route.request().postDataJSON().target);
    else if (/^(threads|reactions)\/commands$/.test(path))
      value = await transport.command(
        path.split("/")[0] as "threads" | "reactions",
        route.request().postDataJSON(),
      );
    else if (path.startsWith("reaction-assets/")) {
      const asset = await transport.asset(path.split("/")[1]!);
      await route.fulfill({
        body: Buffer.from(asset.bytes),
        contentType: asset.type,
      });
      return;
    } else if (path === "content/search") {
      scopes.push(url.searchParams);
      const thread = (await transport.list()).items as any;
      const q = url.searchParams.get("q") ?? "";
      const messages = fixture.history
        .get(THREAD_IDS.thread)!
        .filter((row) => row.message?.body?.includes(q));
      value = {
        version: 1,
        snapshot: { repo_id: repo },
        next_cursor: null,
        results: q
          ? messages.map((row) => ({
              kind: "message",
              id: row.message!.message_id,
              title: "general",
              excerpt: row.message!.body,
              actor: THREAD_IDS.actor,
              at_millis: row.message!.accepted_at_millis,
              revision: row.reference.id,
              thread: THREAD_IDS.thread,
              message: row.message!.message_id,
              project: null,
              task: null,
              comment: null,
            }))
          : thread.threads.map((t: any) => ({
              kind: "thread",
              id: t.value.thread_id,
              title: t.value.title,
              excerpt: "Welcome to the channel",
              actor: THREAD_IDS.actor,
              at_millis: 1725000000000,
              revision: t.reference.id,
              thread: t.value.thread_id,
              message: null,
              project: null,
              task: null,
              comment: null,
            })),
      };
    } else {
      await route.fallback();
      return;
    }
    await route.fulfill({ json: patch(value) });
  });
  await page.goto(`/#/repos/${repo}/threads?at_op=${operation}`);
  await expect(
    page.getByRole("button", { name: "New thread", exact: true }),
  ).toBeVisible();
  await expect(
    page
      .getByRole("region", { name: "Threads listing" })
      .getByText("Welcome to the channel"),
  ).toBeVisible();
  const topic = page.getByRole("button", {
    name: "Open thread general",
    exact: true,
  });
  await expect(topic).toBeVisible();
  await topic.click();
  await expect(
    page.getByRole("log", { name: "general messages" }),
  ).toBeVisible();
  await expect(
    page.getByRole("tab", { name: "Branches", exact: true }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Thread settings" }).click();
  const settings = page.getByRole("complementary", { name: "Thread settings" });
  await expect(settings).toBeVisible();
  const bounds = (await settings.boundingBox())!;
  const composition = page.locator(".community-app-shell__surface");
  const compositionBounds = (await composition.boundingBox())!;
  const inset = await settings.evaluate((element) =>
    parseFloat(getComputedStyle(element).marginInlineEnd),
  );
  expect(
    Math.abs(
      bounds.x +
        bounds.width -
        compositionBounds.x -
        compositionBounds.width +
        inset,
    ),
  ).toBeLessThan(2);
  const conversation = page.locator(".community-conversation");
  const conversationBounds = (await conversation.boundingBox())!;
  const mainBounds = (await page.getByRole("main").boundingBox())!;
  expect(Math.abs(conversationBounds.width - mainBounds.width)).toBeLessThan(3);
  const layoutBounds = (await conversation
    .locator('[data-ui-component="ai-chat-layout"]')
    .boundingBox())!;
  expect(Math.abs(layoutBounds.width - conversationBounds.width)).toBeLessThan(
    2,
  );
  await expect(settings.getByText("Code baseline")).toBeVisible();
  await settings.getByRole("button", { name: /^Name/ }).click();
  const rename = page.getByRole("dialog", { name: "Rename thread" });
  await rename.getByRole("textbox", { name: "Thread name" }).fill("planning");
  await rename.getByRole("button", { name: "Save name" }).click();
  await expect(page.locator('[data-ui-part="room-title"]')).toHaveText(
    "planning",
  );
  await expect(rename).toBeHidden();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  await page.getByRole("button", { name: "Close thread settings" }).click();
  const composer = page.getByRole("textbox");
  await composer.fill("/threads/planning");
  await page.getByRole("option", { name: /planning/ }).click();
  await expect(
    composer.locator('[data-ui-part="composer-entity-reference"]'),
  ).toBeVisible();
  await composer.press("Enter");
  await expect(
    page
      .getByRole("log")
      .getByRole("button", { name: "planning", exact: true }),
  ).toBeVisible();
  const root = page.locator(`[data-message-id="${THREAD_IDS.message}"]`);
  await root.hover();
  await root.getByRole("button", { name: "Reply in thread" }).click();
  const replies = page.getByRole("complementary", { name: "Thread replies" });
  await replies.getByRole("textbox").fill("Cold reply needle");
  await replies.getByRole("textbox").press("Enter");
  await expect(
    replies.getByText("Cold reply needle", { exact: true }),
  ).toBeVisible();
  const createdReply = replies
    .locator("[data-message-id]")
    .filter({ hasText: "Cold reply needle" });
  await createdReply.hover();
  await createdReply
    .getByRole("button", { name: "Add reaction", exact: true })
    .click();
  await page.getByRole("button", { name: ":lapis:", exact: true }).click();
  await expect(
    createdReply.getByRole("button", { name: "Lapis", exact: true }),
  ).toBeVisible();
  await replies
    .getByRole("button", { name: "Who reacted to replies", exact: true })
    .click();
  const actors = page.getByRole("dialog", {
    name: "Who reacted to replies",
    exact: true,
  });
  await expect(actors).toBeVisible();
  await actors.getByRole("button", { name: "Close", exact: true }).click();
  await expect(actors).toBeHidden();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  await page.keyboard.press("ControlOrMeta+k");
  const search = page.getByRole("dialog", { name: "Search workspace" });
  await expect(
    search.getByText("Recent activity", { exact: true }),
  ).toBeVisible();
  await expect(
    search.getByRole("option", { name: /Create new thread/ }),
  ).toBeVisible();
  await search
    .getByRole("option", {
      name: "Search in #planning Search within this section",
      exact: true,
    })
    .click();
  const scopedSearch = page.getByRole("dialog", {
    name: "Search #planning",
    exact: true,
  });
  await expect(
    scopedSearch.getByRole("button", {
      name: "Remove #planning search scope",
      exact: true,
    }),
  ).toBeVisible();
  await scopedSearch
    .getByRole("combobox", { name: "Search workspace" })
    .fill("Cold reply needle");
  const result = scopedSearch.getByRole("option", {
    name: /Cold reply needle/,
  });
  await expect(result).toBeVisible();
  expect(
    scopes.some(
      (params) =>
        params.get("q") === "Cold reply needle" &&
        params.get("section") === "threads" &&
        params.get("thread") === THREAD_IDS.thread,
    ),
  ).toBe(true);
  await result.click();
  await expect(scopedSearch).toBeHidden();
  await expect(page).toHaveURL(/thread=.*reply=/);
  const reply = page
    .getByRole("complementary", { name: "Thread replies" })
    .getByText("Cold reply needle", { exact: true })
    .locator("xpath=ancestor::*[@data-message-id][1]");
  await expect(reply).toBeFocused();
  await page.reload();
  await expect(
    page
      .getByRole("complementary", { name: "Thread replies" })
      .getByText("Cold reply needle", { exact: true }),
  ).toBeVisible();
  await page.keyboard.press("ControlOrMeta+k");
  await page
    .getByRole("dialog", { name: "Search workspace", exact: true })
    .getByRole("option", { name: "Create new thread", exact: true })
    .click();
  const create = page.getByRole("dialog", { name: "New thread", exact: true });
  await create
    .getByRole("textbox", { name: "Thread name", exact: true })
    .fill("release planning");
  await create
    .getByRole("button", { name: "Create thread", exact: true })
    .click();
  await expect(create).toBeHidden();
  await expect(
    page.getByRole("heading", { name: "release planning", exact: true }),
  ).toBeVisible();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations).toEqual([]);
});

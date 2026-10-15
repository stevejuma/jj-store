import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, repo, operation, sample } from "./native-browser-fixture.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";

test("cold LocalTrusted Threads shell settings references scoped search and reply focus", async ({
  page,
}) => {
  await page.setViewportSize({ width: 1800, height: 1000 });
  await page.addInitScript(() => {
    const copies: string[] = [];
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: {
        writeText: async (value: string) => {
          copies.push(value);
        },
        readText: async () => JSON.stringify(copies),
      },
    });
    const key = "jj-store/connection/local/jj-store/layout/v1/shell";
    if (!localStorage.getItem(key))
      localStorage.setItem(
        key,
        JSON.stringify({
          version: 1,
          panels: {
            right: {
              side: "right",
              collapsed: false,
              closed: true,
              width: 360,
            },
            "repository-details": {
              side: "right",
              collapsed: false,
              closed: false,
              width: 280,
            },
          },
        }),
      );
  });
  await api(page, { access: "local_trusted" });
  const fixture = createThreadTransport();
  let releaseThreads!: () => void;
  const initialThreads = new Promise<void>((resolve) => {
    releaseThreads = resolve;
  });
  const scopes: URLSearchParams[] = [];
  await page.route("**/v1/local/threads", async (route) => {
    const request = route.request().postDataJSON();
    expect(request.command.type).toBe("create");
    const thread = "ab".repeat(16);
    await fixture.transport.command("threads", {
      version: 1,
      request_id: request.request_id,
      command: {
        type: "create",
        thread_id: thread,
        title: request.command.title,
        baseline: null,
      },
    } as any);
    const detail = await fixture.transport.thread(thread);
    await route.fulfill({
      json: {
        request_id: request.request_id,
        home: repo,
        thread,
        revision: detail.thread.reference,
      },
    });
  });
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
    if (path === "snapshot")
      value = { ...sample("snapshot"), actor: THREAD_IDS.actor };
    else if (path === "threads") {
      await initialThreads;
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
  const skeleton = page.getByRole("status", {
    name: "Loading threads",
    exact: true,
  });
  await expect(skeleton).toBeVisible();
  await expect(skeleton).toHaveAttribute("aria-busy", "true");
  await expect(
    skeleton.locator(".inbox-session-tree__skeleton-row"),
  ).toHaveCount(5);
  releaseThreads();
  await expect(
    page.getByRole("button", {
      name: "New Thread",
      exact: true,
    }),
  ).toBeVisible();
  await expect(
    page
      .getByRole("navigation", { name: "Threads by repository" })
      .getByRole("button", { name: "Open Thread general", exact: true }),
  ).toBeVisible();
  await page.screenshot({ path: "/tmp/jj-thread-fix-forum.png" });
  const topic = page.getByRole("button", {
    name: "Open Thread general",
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
  const menu = page.getByRole("button", {
    name: "Conversation details",
    exact: true,
  });
  await menu.click();
  await expect(page.getByRole("menu")).toHaveCount(0);
  const settings = page.getByRole("complementary", {
    name: "Conversation details",
    exact: true,
  });
  await expect(settings).toBeVisible();
  const copiedValues: string[] = [];
  for (const label of ["Thread ID", "Thread revision"]) {
    const copy = settings.getByRole("button", {
      name: new RegExp(`^Copy full ${label}:`),
    });
    const accessibleName = (await copy.getAttribute("aria-label"))!;
    const value = accessibleName.split(": ")[1];
    const row = copy.locator(
      'xpath=ancestor::*[contains(concat(" ", normalize-space(@class), " "), " store-readonly-row ")][1]',
    );
    for (const area of ["dt", ".store-readonly-row__value"]) {
      await row.scrollIntoViewIfNeeded();
      const box = (await row.locator(area).boundingBox())!;
      const point = { x: box.x + 8, y: box.y + box.height / 2 };
      await page.mouse.move(point.x, point.y);
      await page.mouse.down();
      try {
        // The stretched copy target must cover the row throughout a held press.
        await expect
          .poll(() =>
            page.evaluate(
              ({ x, y }) =>
                document
                  .elementFromPoint(x, y)
                  ?.closest("button")
                  ?.getAttribute("aria-label"),
              point,
            ),
          )
          .toBe(accessibleName);
      } finally {
        await page.mouse.up();
      }
      await expect(copy).toHaveAttribute("data-copied", "true");
      await expect(copy.locator("svg circle")).toBeVisible();
      copiedValues.push(value);
      expect(
        JSON.parse(await page.evaluate(() => navigator.clipboard.readText())),
      ).toEqual(copiedValues);
    }
  }
  await settings
    .getByRole("button", { name: "Open thread conversation" })
    .click();
  await expect(settings).toBeHidden();
  expect(
    JSON.parse(await page.evaluate(() => navigator.clipboard.readText())),
  ).toEqual(copiedValues);
  await menu.click();
  await expect(settings).toBeVisible();
  const paneHeader = settings.locator(".lc-community-inbox__thread-header");
  const toolbar = page
    .locator(
      ".lc-community-inbox__thread-region > [data-ui-part=scroll-area] .lc-community-inbox__thread-header",
    )
    .first();
  const toolbarBounds = (await toolbar.boundingBox())!;
  const paneHeaderBounds = (await paneHeader.boundingBox())!;
  expect(Math.abs(paneHeaderBounds.y - toolbarBounds.y)).toBeLessThan(1);
  await expect(paneHeader).toHaveCSS("border-bottom-width", "0px");
  await expect(toolbar).toHaveCSS("border-bottom-width", "0px");
  const bounds = (await settings.boundingBox())!;
  const mainBounds = (await page.getByRole("main").boundingBox())!;
  expect(bounds.x).toBeGreaterThan(mainBounds.x);
  expect(bounds.x + bounds.width).toBeLessThanOrEqual(
    mainBounds.x + mainBounds.width + 1,
  );
  const menuBounds = (await menu.boundingBox())!;
  const refreshBounds = (await page
    .getByRole("button", { name: "Refresh Threads", exact: true })
    .boundingBox())!;
  expect(menuBounds.x).toBeGreaterThan(refreshBounds.x);
  const conversation = page.locator(".thread-conversation");
  const conversationBounds = (await conversation.boundingBox())!;
  const layoutBounds = (await conversation
    .locator(".chat-body-reading-surface")
    .boundingBox())!;
  expect(Math.abs(layoutBounds.width - conversationBounds.width)).toBeLessThan(
    2,
  );
  await expect(settings.getByText("Code baseline")).toBeVisible();
  const archive = settings.getByRole("button", { name: "Archive thread" });
  const remove = settings.getByRole("button", { name: "Remove thread" });
  await expect(archive).toBeVisible();
  await expect(archive.locator("svg[data-icon=inline-start]")).toBeVisible();
  await expect(remove.locator("svg[data-icon=inline-start]")).toBeVisible();
  expect(await remove.evaluate((el) => getComputedStyle(el).color)).not.toBe(
    await archive.evaluate((el) => getComputedStyle(el).color),
  );
  await page.screenshot({ path: "/tmp/jj-thread-fix-settings.png" });
  await settings.getByRole("button", { name: /^Name/ }).click();
  const rename = page.getByRole("dialog", { name: "Rename thread" });
  await rename.getByRole("textbox", { name: "Thread name" }).fill("planning");
  await rename.getByRole("button", { name: "Save name" }).click();
  await expect(
    page.getByRole("heading", { name: "planning", exact: true }),
  ).toHaveText("planning");
  await expect(rename).toBeHidden();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  await page
    .getByRole("button", { name: "Close conversation details" })
    .click();
  // This transport fixture has no publication feed; refresh its directory after rename.
  await page
    .getByRole("button", { name: "Refresh Threads", exact: true })
    .click();
  const composer = page
    .getByRole("group", { name: "Message composer", exact: true })
    .getByRole("textbox", { name: "Markdown editor" });
  await composer.fill("/threads/planning");
  await page.getByRole("option", { name: /planning/ }).click();
  await expect(
    composer.locator('[data-ui-part="composer-entity-reference"]'),
  ).toBeVisible();
  await page.getByRole("button", { name: "Send message", exact: true }).click();
  await expect(
    page
      .getByRole("log")
      .getByRole("button", { name: "planning", exact: true }),
  ).toBeVisible();
  const root = page.locator(`[data-message-id="${THREAD_IDS.message}"]`);
  await root.hover();
  await root.getByRole("button", { name: "More message actions" }).click();
  await page
    .getByRole("menuitem", { name: "Edit message", exact: true })
    .click();
  const edit = root.getByRole("group", { name: "Edit message", exact: true });
  await edit
    .getByRole("textbox", { name: "Markdown editor" })
    .fill("Edited root body");
  await edit.getByRole("button", { name: "Send message", exact: true }).click();
  await expect(root.getByText("Edited", { exact: true })).toHaveText("Edited");
  await expect(
    root.getByRole("textbox", { name: "Message", exact: true }),
  ).toHaveText("Edited root body");

  await root.hover();
  await root
    .getByRole("button", { name: "More message actions", exact: true })
    .click();
  await page
    .getByRole("menuitem", { name: "Reply in thread", exact: true })
    .click();
  const replies = page.getByRole("complementary", { name: "Thread replies" });
  const threadResize = page.getByRole("slider", {
    name: "Resize thread details",
    exact: true,
  });
  await expect(threadResize).toHaveAttribute("aria-valuenow", "360");
  await threadResize.focus();
  await page.keyboard.press("ArrowLeft");
  const threadWidth = await threadResize.getAttribute("aria-valuenow");
  await expect
    .poll(() =>
      page.evaluate(() => {
        const panels = JSON.parse(
          localStorage.getItem(
            "jj-store/connection/local/jj-store/layout/v1/shell",
          )!,
        ).panels;
        return [panels.right.width, panels["repository-details"].width];
      }),
    )
    .toEqual([Number(threadWidth), 280]);
  const replyRoot = replies.locator(
    `[data-message-id="${THREAD_IDS.message}"]`,
  );
  const replyHeaderBounds = (await replies
    .locator('[data-ui-part="sidebar-header"]')
    .boundingBox())!;
  // The inbox and application sidebar have independent header heights.
  expect(Math.abs(replyHeaderBounds.y - toolbarBounds.y)).toBeLessThan(1);
  await replyRoot.hover();
  const actions = replyRoot.getByRole("group", {
    name: "Actions for Message",
    exact: true,
  });
  const actionBounds = (await actions.boundingBox())!;
  const viewportBounds = (await replies
    .locator('.store-agent-chat-body [data-ui-part="scroll-area-viewport"]')
    .boundingBox())!;
  expect(actionBounds.y).toBeGreaterThanOrEqual(viewportBounds.y);
  await replies
    .getByRole("group", { name: "Message composer", exact: true })
    .getByRole("textbox", { name: "Markdown editor" })
    .fill("Cold reply needle");
  await replies
    .getByRole("button", { name: "Send message", exact: true })
    .click();
  await expect(
    replies.getByText("Cold reply needle", { exact: true }),
  ).toBeVisible();
  await expect(
    page
      .getByRole("log", { name: "planning messages" })
      .getByText("Edited root body", { exact: true }),
  ).toBeVisible();
  await expect(
    page
      .getByRole("log", { name: "planning messages" })
      .locator(`[data-message-id="${THREAD_IDS.message}"] [role="textbox"]`),
  ).toHaveText("Edited root body");
  const createdReply = replies
    .locator("[data-message-id]")
    .filter({ hasText: "Cold reply needle" });
  await createdReply.hover();
  await createdReply
    .getByRole("button", { name: /^Add reaction(?: to .+)?$/ })
    .click();
  await page.getByRole("button", { name: ":lapis:", exact: true }).click();
  await expect(
    createdReply.getByRole("button", {
      name: /1 reaction with Lapis/,
      exact: true,
    }),
  ).toBeVisible();
  await createdReply
    .getByRole("button", { name: /1 reaction with Lapis/, exact: true })
    .hover();
  await expect(
    page
      .locator('[data-ui-component="tooltip"][data-ui-part="tooltip-content"]')
      .filter({ hasText: /1 reaction with Lapis/ }),
  ).toBeVisible();
  await expect(page.getByRole("button", { name: /Who reacted/ })).toHaveCount(
    0,
  );
  await page.screenshot({ path: "/tmp/jj-thread-fix-replies.png" });
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
    .locator("xpath=ancestor::*[@data-chat-item][1]");
  await expect(reply).toBeFocused();
  await page.reload();
  await expect(
    page.getByRole("slider", { name: "Resize thread details", exact: true }),
  ).toHaveAttribute("aria-valuenow", threadWidth!);
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
  const create = page.getByRole("dialog").filter({
    has: page.getByRole("heading", { name: "Create Thread", exact: true }),
  });
  await expect(create).toBeVisible();
  expect(
    (await new AxeBuilder({ page }).include("[data-ui-dialog-frame]").analyze())
      .violations,
  ).toEqual([]);
  await create
    .getByRole("textbox", { name: "Thread title", exact: true })
    .fill("release planning");
  await create
    .getByRole("button", { name: "Create Thread", exact: true })
    .click();
  await expect(create).toBeHidden();
  await expect(
    page.getByRole("heading", { name: "release planning", exact: true }),
  ).toBeVisible();
  await expect(page.locator("body")).not.toHaveCSS("pointer-events", "none");
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  await page.goto("/#/projects?tab=projects");
  const rail = page.getByRole("complementary", {
    name: "Repository details",
    exact: true,
  });
  await expect(rail).toBeVisible();
  await expect(rail).toHaveAttribute("data-variant", "rail");
  const railResize = page.getByRole("slider", {
    name: "Resize repository details",
    exact: true,
  });
  await expect(railResize).toHaveAttribute("aria-valuenow", "280");
  await railResize.focus();
  await page.keyboard.press("ArrowRight");
  const railWidth = await railResize.getAttribute("aria-valuenow");
  await expect
    .poll(() =>
      page.evaluate(() => {
        const panels = JSON.parse(
          localStorage.getItem(
            "jj-store/connection/local/jj-store/layout/v1/shell",
          )!,
        ).panels;
        return [panels.right.width, panels["repository-details"].width];
      }),
    )
    .toEqual([Number(threadWidth), Number(railWidth)]);
  await page.reload();
  await expect(
    page.getByRole("slider", {
      name: "Resize repository details",
      exact: true,
    }),
  ).toHaveAttribute("aria-valuenow", railWidth!);
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations).toEqual([]);
});

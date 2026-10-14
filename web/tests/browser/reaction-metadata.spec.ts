import { test, expect } from "@playwright/test";
import { api, repo, operation, sample } from "./native-browser-fixture.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";

for (const reply of [false, true]) {
  test(`cold uncertain reaction metadata in ${reply ? "reply" : "main"} message`, async ({
    page,
  }) => {
    await page.setViewportSize({ width: 1800, height: 1000 });
    await api(page, { access: "local_trusted" });
    const f = createThreadTransport({ unknownFirst: true });
    const replyId = "77".repeat(16);
    {
      const root = f.history.get(THREAD_IDS.thread)![0]!;
      f.addMessage(
        THREAD_IDS.thread,
        replyId,
        "A retained reply",
        root.value.event.type === "message" ? root.value.event.revision : null,
      );
    }
    let releaseReaction!: () => void;
    const reactionGate = new Promise<void>((resolve) => {
      releaseReaction = resolve;
    });
    await page.route(`**/v1/repos/${repo}/**`, async (route) => {
      const path = new URL(route.request().url()).pathname.split(
        `/v1/repos/${repo}/`,
      )[1]!;
      const t = f.transport;
      let value: any;
      if (path === "snapshot")
        value = { ...sample("snapshot"), actor: THREAD_IDS.actor };
      else if (path === "threads") value = await t.list();
      else if (/^threads\/[^/]+\/timeline$/.test(path))
        value = await t.events(path.split("/")[1]!);
      else if (/^threads\/[0-9a-f]{32}$/.test(path))
        value = await t.thread(path.split("/")[1]!);
      else if (path === "reaction-emojis") value = await t.emojis();
      else if (path === "reactions/query")
        value = await t.reactions(route.request().postDataJSON().target);
      else if (path === "reactions/commands") {
        await reactionGate;
        value = await t.command("reactions", route.request().postDataJSON());
      } else if (/^reactions\/commands\/[0-9a-f]+$/.test(path))
        value = await t.status("reactions", path.split("/")[2]!);
      else if (path.startsWith("reaction-assets/")) {
        const asset = await t.asset(path.split("/")[1]!);
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
      `/#/repos/${repo}/threads?at_op=${operation}&thread=${THREAD_IDS.thread}${reply ? `&reply=${replyId}` : ""}`,
    );
    const log = page.getByRole("log", { name: "general messages" });
    await expect(log.getByText("Welcome to the channel")).toBeVisible();
    const message = (
      reply ? page.getByRole("complementary", { name: "Thread replies" }) : log
    ).locator(`[data-message-id="${reply ? replyId : THREAD_IDS.message}"]`);
    await message.hover();
    await message
      .getByRole("button", { name: /^Add reaction(?: to .+)?$/ })
      .click();
    await page.getByRole("button", { name: ":lapis:", exact: true }).click();
    const row = message.locator(".community-message__metadata");
    await expect(row.locator('[data-ui-component="spinner"]')).toBeVisible();
    await expect(
      row.getByRole("button", { name: "Check action status" }),
    ).toHaveCount(0);
    releaseReaction();
    const metadata = message
      .locator('[data-ui-component="ai-chat-message-metadata"]')
      .filter({ has: page.getByRole("status") });
    await expect(metadata.getByRole("status")).toHaveText(
      "Awaiting reaction confirmation.",
    );
    await expect(
      row.getByRole("button", { name: "Check action status" }),
    ).toBeVisible();
    await expect(
      row.getByRole("button", { name: "Retry same action" }),
    ).toHaveText("");
    if (!reply) {
      await expect(
        row.getByRole("button", { name: "1 reply", exact: true }),
      ).toBeVisible();
      const centers = await row.evaluate((element) => {
        const timestamp = element
          .querySelector('[data-ui-part="timestamp"]')!
          .getBoundingClientRect();
        const replies = element
          .querySelector(".community-message-reply-summary")!
          .getBoundingClientRect();
        const status = element
          .querySelector('[role="status"]')!
          .getBoundingClientRect();
        return [timestamp, replies, status].map((rect) => ({
          x: rect.x + rect.width / 2,
          y: rect.y + rect.height / 2,
        }));
      });
      expect(centers[0]!.x).toBeLessThan(centers[1]!.x);
      expect(centers[1]!.x).toBeLessThan(centers[2]!.x);
      expect(Math.abs(centers[0]!.y - centers[2]!.y)).toBeLessThan(2);
    }
    await expect(
      page.locator('.store-threads > [data-ui-component="alert"]'),
    ).toHaveCount(0);
    await expect(
      message.getByRole("textbox", { name: "Message", exact: true }),
    ).toHaveText(reply ? "A retained reply" : "Welcome to the channel");
    if (reply)
      await expect(
        log.locator(
          '[data-ui-component="ai-chat-message-metadata"] [role="status"]',
        ),
      ).toHaveCount(0);
    await page.screenshot({
      path: `/tmp/jj-thread-metadata-${reply ? "reply" : "main"}.png`,
    });
    await metadata
      .getByRole("button", { name: "Check action status", exact: true })
      .click();
    await expect(
      message.getByText("Awaiting reaction confirmation."),
    ).toHaveCount(0);
    await expect(
      message.getByRole("button", { name: /1 reaction with Lapis/ }),
    ).toBeVisible();
    expect(f.requests).toHaveLength(1);
  });
}

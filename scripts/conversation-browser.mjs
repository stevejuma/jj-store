import { chromium, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { checkOrdinaryThread } from "./conversation-thread-browser.mjs";
const [origin, session, setup, profile, mode] = process.argv.slice(2);
if (
  !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin) ||
  !/^[a-f0-9]{32}$/.test(session) ||
  !["native", "local_jj_git"].includes(profile) ||
  !["send", "retained"].includes(mode)
)
  throw Error("Expected isolated conversation fixture");
const browser = await chromium.launch();
try {
  const context = await browser.newContext({
      viewport: { width: 1440, height: 1100 },
    }),
    page = await context.newPage();
  const errors = [],
    resizeWarnings = [];
  page.on("pageerror", (error) =>
    (error.message.includes("ResizeObserver") ? resizeWarnings : errors).push(
      error.message,
    ),
  );
  page.on("pageerror", (error) =>
    console.error("Browser exception:", error.stack),
  );
  page.on("response", async (response) => {
    if (response.status() >= 400)
      console.error(
        "HTTP failure:",
        response.status(),
        new URL(response.url()).pathname,
        (await response.json().catch(() => ({}))).code ?? "",
      );
  });
  const headers = {
    "x-jj-store-protocol": "1",
    Origin: origin,
    "Sec-Fetch-Site": "same-origin",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Dest": "empty",
  };
  const feedback = async () => {
    const r = await page.request.get(
      `${origin}/v1/local/agents/feedback?session=${session}`,
      { headers },
    );
    expect(r.ok()).toBe(true);
    return r.json();
  };
  await page.goto(`${origin}/#/agents/sessions/${session}?agent=${setup}`);
  await expect(page.locator("[data-conversation-view]")).toHaveAttribute(
    "data-conversation-view",
    "document",
  );
  await expect(
    page
      .getByRole("textbox", { name: "Assistant message", exact: true })
      .first(),
  ).toContainText("Fixture source");
  if (mode === "send" && profile === "native") {
    const initial = await feedback();
    expect(initial.write_unavailable).toContain("collaboration replication");
    const status = await (
      await page.request.get(`${origin}/v1/local/agents`, { headers })
    ).json();
    const repository = status.sessions.find((s) => s.id === session).repository;
    const home = await page.request.post(
      `${origin}/v1/repos/${repository}/work/commands`,
      {
        headers,
        data: {
          version: 1,
          request_id: crypto.randomUUID().replaceAll("-", ""),
          command: {
            command: "create_project",
            id: crypto.randomUUID().replaceAll("-", ""),
            title: "Conversation qualification home",
            body: "Disposable supported native WorkStore fixture",
          },
        },
      },
    );
    expect(home.ok(), await home.text()).toBe(true);
    expect((await home.json()).status).toBe("committed");
    await expect
      .poll(async () => (await feedback()).write_unavailable)
      .toBeUndefined();
    await page.reload();
  }
  if (mode === "send") {
    await expect(
      page
        .getByRole("button", {
          name: "Comment on Assistant message",
          exact: true,
        })
        .first(),
    ).toBeEnabled({ timeout: 30000 });
    const source = page
      .getByRole("textbox", { name: "Assistant message", exact: true })
      .first();
    await source.focus();
    await page.keyboard.press("Control+Home");
    for (let i = 0; i < 7; i++) await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Control+Alt+m");
    const drafts = page.locator('[data-location="inline"] [role="textbox"]');
    await expect(drafts).toHaveCount(1);
    await drafts.first().pressSequentially("First question 🦀");
    await page
      .getByRole("button", { name: "Add another comment", exact: true })
      .click();
    await expect(drafts).toHaveCount(2);
    await drafts.nth(1).pressSequentially("Second question");
    await source.focus();
    await page.keyboard.press("Control+End");
    for (let i = 0; i < 9; i++) await page.keyboard.press("Shift+ArrowLeft");
    await page.keyboard.press("Control+Alt+m");
    await expect(drafts).toHaveCount(3);
    await drafts.nth(2).pressSequentially("Third question");
    await expect(
      page.locator('[data-location="queue"] [role="textbox"]'),
    ).toHaveCount(3);
    const requests = [];
    page.on("request", (r) => {
      if (r.url().endsWith("/local/agents/command") && r.method() === "POST") {
        const body = r.postDataJSON();
        if (body.action?.action === "prompt") requests.push(body);
      }
    });
    if (profile === "local_jj_git") {
      let lost = false;
      await page.route("**/v1/local/agents/outcome?request=**", (route) =>
        route.abort("connectionreset"),
      );
      await page.route("**/v1/local/agents/command", async (route) => {
        if (
          !lost &&
          route.request().postDataJSON()?.action?.action === "prompt"
        ) {
          lost = true;
          const response = await route.fetch();
          expect(response.ok(), await response.text()).toBe(true);
          await route.abort("connectionreset");
        } else await route.continue();
      });
    }
    await page
      .getByRole("button", { name: "Send message", exact: true })
      .click();
    if (profile === "local_jj_git") {
      const recovery = page.getByRole("region", {
        name: "Comment submission recovery",
      });
      await expect(recovery).toBeVisible({ timeout: 30000 });
      await page.unroute("**/v1/local/agents/outcome?request=**");
      await recovery
        .getByRole("button", { name: "Check retained request" })
        .click();
      await expect(recovery).toHaveCount(0);
      await page.unroute("**/v1/local/agents/command");
    }
    await expect
      .poll(
        async () => {
          const f = await feedback();
          return f.messages
            .flatMap((m) => m.replies)
            .filter((r) => r.answers.length).length;
        },
        { timeout: 45000 },
      )
      .toBe(3);
    expect(requests).toHaveLength(1);
    expect(requests[0].action.text).toBe("");
    expect(requests[0].action.comments.comments).toHaveLength(3);
    const before = await feedback();
    const duplicate = await page.request.post(
      `${origin}/v1/local/agents/command`,
      { headers, data: requests[0] },
    );
    expect(duplicate.ok()).toBe(true);
    expect(await feedback()).toEqual(before);
  }
  // A bounded feedback refresh discovers publication-selected links; a reload also
  // proves these are durable server links, independent of a browser's tag parse.
  await page.reload();
  const inline = page.locator('.chat-document [data-placement="inline"]');
  await expect(
    inline.getByRole("button", { name: "Go to reply", exact: true }),
  ).toHaveCount(3, { timeout: 30000 });
  const f = await feedback();
  const replies = f.messages.flatMap((m) => m.replies);
  expect(replies).toHaveLength(3);
  const destinations = new Set(
    replies.flatMap((r) =>
      r.answers.map(
        (a) => `${a.answer.target.content.id}:${a.answer.selection.start_byte}`,
      ),
    ),
  );
  expect(destinations.size).toBe(2);
  for (const reply of replies) {
    expect(
      reply.answers[0].comments.some(
        (c) => c.revision.id === reply.revision.id,
      ),
    ).toBe(true);
  }
  await inline
    .getByRole("button", { name: "Go to reply", exact: true })
    .first()
    .click();
  await expect(page.locator('[data-chat-item^="answer:"]:focus')).toHaveCount(
    1,
  );
  await expect(page.locator('[data-chat-item^="answer:"]:focus')).toContainText(
    "Answered 🦀.",
  );
  await expect(
    page.locator('[data-chat-item^="answer:"] .chat-reply-indicator'),
  ).toHaveCount(2);
  await page
    .locator('[data-chat-item^="answer:"]:focus')
    .getByRole("button", { name: /Go to source/ })
    .first()
    .click();
  await expect(
    page
      .getByRole("textbox", { name: "Assistant message", exact: true })
      .first(),
  ).toBeFocused();
  const axe = await new AxeBuilder({ page })
    .include(".agent-conversation")
    .analyze();
  expect(axe.violations).toEqual([]);
  await checkOrdinaryThread(page, { origin, session, profile, mode, headers });
  expect(errors).toEqual([]);
  console.log(
    JSON.stringify({
      profile,
      mode,
      comments: replies.length,
      answers: destinations.size,
      resizeObserverWarnings: resizeWarnings.length,
      warnings: resizeWarnings,
    }),
  );
} finally {
  await browser.close();
}

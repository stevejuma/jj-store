import { expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { createHash } from "node:crypto";

/** Real ordinary-thread publication shares the same native/local batch authority. */
export async function checkOrdinaryThread(
  page,
  { origin, session, profile, mode, headers },
) {
  const status = await (
    await page.request.get(`${origin}/v1/local/agents`, { headers })
  ).json();
  const repository = status.sessions.find((s) => s.id === session).repository;
  const thread = createHash("sha256")
    .update(`${session}/ordinary-thread`)
    .digest("hex")
    .slice(0, 32);
  if (mode === "send") {
    const command = {
      type: "create",
      thread_id: thread,
      title: "Ordinary conversation",
      baseline: null,
    };
    const envelope = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      command:
        profile === "native" ? command : { domain: "thread", intent: command },
    };
    const response = await page.request.post(
      `${origin}/v1/${profile === "native" ? `repos/${repository}/threads/commands` : `local/repos/${repository}/collaboration/commands`}`,
      { headers, data: envelope },
    );
    expect(response.ok(), await response.text()).toBe(true);
    expect((await response.json()).status).toBe("committed");
  }
  await page.goto(`${origin}/#/repos/${repository}/threads?thread=${thread}`);
  const body = page.getByRole("region", {
    name: "Thread conversation",
    exact: true,
  });
  await expect(body).toBeVisible({ timeout: 30000 });
  if (mode === "send") {
    await body
      .getByRole("textbox", { name: "Markdown editor", exact: true })
      .fill("Human source 🦀");
    await page
      .getByRole("button", { name: "Send message", exact: true })
      .click();
    const source = body.getByRole("textbox", { name: "Message", exact: true });
    await expect(source).toContainText("Human source 🦀");
    await source.focus();
    await page.keyboard.press("Control+Home");
    for (let i = 0; i < 5; i++) await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Control+Alt+m");
    const input = body.locator('[data-location="inline"] [role="textbox"]');
    await expect(input).toHaveCount(1);
    await input.fill("Human inline feedback");
    const submissions = [];
    const capture = (request) => {
      if (request.method() !== "POST" || !request.url().endsWith("/commands"))
        return;
      const body = request.postDataJSON();
      const command = body.command?.intent ?? body.command;
      if (command?.type === "submit") submissions.push(command.submission);
    };
    page.on("request", capture);
    await page
      .getByRole("button", { name: "Send message", exact: true })
      .click();
    await expect(
      body.locator('[data-placement="inline"] [data-status="sent"]'),
    ).toContainText("Human inline feedback");
    page.off("request", capture);
    expect(submissions).toHaveLength(1);
    expect(submissions[0].text).toBeNull();
    expect(submissions[0].comments).toHaveLength(1);
    expect(submissions[0].comments[0].source.target.type).toBe("human_message");
    expect(submissions[0].comments[0].source.quote).toBe("Human");
    await page.reload();
  }
  await expect(
    body.locator('[data-placement="inline"] [data-status="sent"]'),
  ).toContainText("Human inline feedback");
  await expect(
    body.getByRole("textbox", { name: "Message", exact: true }),
  ).toHaveCount(1);
  await body
    .getByRole("button", { name: "View message", exact: true })
    .first()
    .click();
  await body
    .getByRole("button", { name: /Go to source:/ })
    .first()
    .click();
  await expect(
    body.getByRole("textbox", { name: "Message", exact: true }),
  ).toBeFocused();
  expect(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Thread conversation"]')
        .analyze()
    ).violations,
  ).toEqual([]);
  console.log(
    JSON.stringify({ profile, mode, ordinaryThread: true, comments: 1 }),
  );
}

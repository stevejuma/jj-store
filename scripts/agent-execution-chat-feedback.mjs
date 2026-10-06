import { accessible } from "./agent-execution-browser-ui.mjs";
import { expect } from "@playwright/test";

export async function verifyChatFeedback(
  page,
  workspace,
  origin,
  admission,
  sessionId,
) {
  const pageOfEvents = await (
    await page.request.get(
      `${origin}/v1/local/agents/events?session=${sessionId}`,
      { headers: admission },
    )
  ).json();
  const events = pageOfEvents.events;
  const prompt = events.find((event) => event.payload.type === "prompt");
  const assistant = events.find(
    (event) =>
      event.payload.type === "activity" &&
      event.payload.update?.sessionUpdate === "agent_message_chunk",
  );
  for (const [name, event] of [
    ["You", prompt],
    ["Browser qualification", assistant],
  ]) {
    expect(event?.observed_at_millis).toBeGreaterThan(0);
    const message = workspace
      .getByRole("article", { name: `Message from ${name}` })
      .first();
    await expect(
      message.locator('[data-ui-component="ai-chat-message-metadata"] time'),
    ).toHaveAttribute(
      "datetime",
      new Date(event.observed_at_millis).toISOString(),
    );
  }
  await expect(
    workspace.locator('[data-ui-component="ai-chat-system-message"]'),
  ).toContainText("Turn completed");
  await expect(
    workspace.locator(
      '[data-ui-component="community-agent-composer-activity"]',
    ),
  ).toHaveCount(0);
  const before = events.map((event) => [
    event.sequence,
    event.observed_at_millis,
  ]);
  await page.reload();
  await expect(
    workspace
      .getByRole("article", { name: "Message from You" })
      .first()
      .locator("time"),
  ).toHaveAttribute(
    "datetime",
    new Date(prompt.observed_at_millis).toISOString(),
  );
  const refreshed = await (
    await page.request.get(
      `${origin}/v1/local/agents/events?session=${sessionId}`,
      { headers: admission },
    )
  ).json();
  expect(
    refreshed.events.map((event) => [event.sequence, event.observed_at_millis]),
  ).toEqual(before);
  await accessible(page, "conversation", '[aria-label="Agent sessions inbox"]');
  await page.emulateMedia({ colorScheme: "dark" });
  await expect(page.locator("html")).toHaveClass(/dark/);
  await accessible(
    page,
    "dark conversation",
    '[aria-label="Agent sessions inbox"]',
  );
  await page.emulateMedia({ colorScheme: "light" });
  await page.screenshot({
    path: ".local/agent-chat-feedback-final.png",
    animations: "disabled",
  });
}

export async function verifyChatWorking(workspace) {
  await expect(
    workspace.locator(
      '[role="log"] [data-ui-component="ai-chat-reasoning"][data-streaming="true"]',
    ),
  ).toBeVisible();
  await expect(
    workspace.locator(
      '[data-ui-component="community-agent-composer-activity"]',
    ),
  ).toContainText("Working");
  await workspace.page().screenshot({
    path: ".local/agent-chat-working-final.png",
    animations: "disabled",
  });
}

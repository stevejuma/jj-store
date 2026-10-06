import {
  discussionApi,
  qualifyNativeFeedback,
} from "./agent-execution-discussion-api.mjs";
import { expect } from "@playwright/test";
import { accessible, sendPrompt } from "./agent-execution-browser-ui.mjs";

export async function verifyAgentDiscussion(
  page,
  workspace,
  origin,
  admission,
  sessionId,
  repository,
) {
  const read = async (path) => {
    const response = await page.request.get(`${origin}/v1${path}`, {
      headers: admission,
    });
    expect(response.ok()).toBeTruthy();
    return response.json();
  };
  const feedback = () => read(`/local/agents/feedback?session=${sessionId}`);
  const events = async () =>
    (await read(`/local/agents/events?session=${sessionId}`)).events;
  await expect
    .poll(async () => (await feedback()).messages.length, { timeout: 30000 })
    .toBe(2);
  const original = await feedback();
  const profile = original.profile ?? "native";
  const api = discussionApi(page, origin, admission, repository, profile);
  if (profile === "native")
    await qualifyNativeFeedback(
      page,
      workspace,
      origin,
      admission,
      repository,
      original,
      feedback,
    );
  else expect(original.write_unavailable).toBeUndefined();
  const title = (await read("/local/agents")).sessions.find(
    (s) => s.id === sessionId,
  ).title;
  const before = (await events()).filter(
    (e) => e.payload.type === "prompt",
  ).length;
  for (const role of ["user", "assistant"]) {
    const row = original.messages.find((m) => m.row_id.endsWith(`/${role}`));
    expect(row).toBeTruthy();
    const message = workspace.locator(`[data-message-id="${row.row_id}"]`);
    await message.hover();
    await message.getByRole("button", { name: "Reply", exact: true }).click();
    const panel = workspace.getByRole("region", {
      name: role === "user" ? "Replies to You" : `Replies to ${title}`,
      exact: true,
    });
    await panel
      .getByRole("textbox", { name: "Markdown editor" })
      .fill(
        `Saved ${role} feedback: include followup context accepted in the next reply.`,
      );
    const saved = page.waitForResponse((response) => {
      if (
        !response.url().endsWith(api.commandPath("threads")) ||
        response.request().method() !== "POST"
      )
        return false;
      const data = response.request().postDataJSON().command;
      const command = data.intent ?? data;
      return (
        command.type === "message" &&
        command.body ===
          `Saved ${role} feedback: include followup context accepted in the next reply.`
      );
    });
    await panel
      .getByRole("button", { name: "Save reply", exact: true })
      .click();
    let receipt;
    try {
      receipt = await saved;
    } catch (cause) {
      console.error("Reply save diagnostic", await panel.innerText());
      throw cause;
    }
    expect(receipt.ok()).toBeTruthy();
    const exactRequest = receipt.request().postDataJSON().request_id;
    const committed = await api.receipt("threads", exactRequest);
    expect(committed.status).toBe("committed");
    expect(committed.receipt.request_id).toBe(exactRequest);
    await expect(
      panel
        .getByRole("article", { name: "Message from You", exact: true })
        .getByText(
          `Saved ${role} feedback: include followup context accepted in the next reply.`,
          { exact: true },
        ),
    ).toBeVisible();
    await expect(panel.getByRole("alert")).toHaveCount(0);
    const current = (await feedback()).messages.find(
      (m) => m.row_id === row.row_id,
    );
    expect(current.target).toEqual(row.target);
    expect(current.replies).toHaveLength(1);
    // These are ordinary repository ThreadStore messages, readable elsewhere.
    const discussion = await api.thread(current.threads[0]);
    expect(discussion.thread.value.agent_message).toEqual(row.target);
    // Use the shared reaction bar and exact original target.
    const reacted = await api.command("reactions", {
      type: "set",
      target: row.target,
      value: { type: "like" },
      observed: [],
    });
    expect(reacted.ok()).toBeTruthy();
    expect((await reacted.json()).status).toBe("committed");
    await expect(
      message.getByRole("button", { name: /1 reaction with (?:like|👍)/ }),
    ).toBeVisible({ timeout: 30000 });
    await message
      .getByRole("button", { name: /1 reaction with (?:like|👍)/ })
      .click();
    await expect(
      message.getByRole("button", { name: /1 reaction with (?:like|👍)/ }),
    ).toHaveCount(0);
  }
  expect(
    (await events()).filter((e) => e.payload.type === "prompt"),
  ).toHaveLength(before);
  await accessible(
    page,
    "agent message replies",
    '[aria-label="Agent sessions inbox"]',
  );
  await page.reload();
  const retained = await feedback();
  expect(retained.messages.map((m) => m.replies.length)).toEqual([1, 1]);
  const composer = workspace.getByRole("textbox", { name: "Markdown editor" });
  await composer.fill(
    "Reply briefly using the saved follow-up feedback. Do not run tools or change files.",
  );
  let submitted;
  const capture = (request) => {
    if (
      request.url().endsWith("/local/agents/command") &&
      request.method() === "POST" &&
      request.postDataJSON().action?.action === "prompt"
    )
      submitted = request.postDataJSON();
  };
  page.on("request", capture);
  await sendPrompt(page, workspace, sessionId, origin, admission);
  page.off("request", capture);
  expect(submitted).toBeTruthy();
  await expect
    .poll(
      async () =>
        (await read("/local/agents")).sessions.find((s) => s.id === sessionId)
          ?.state,
      { timeout: 180000 },
    )
    .toBe("idle");
  const after = (await events()).filter((e) => e.payload.type === "prompt");
  expect(after).toHaveLength(before + 1);
  const prepared = after.at(-1).payload;
  expect(prepared.text).toContain("Saved user feedback");
  expect(prepared.text).toContain("Saved assistant feedback");
  for (const message of retained.messages)
    expect(prepared.text).toContain(message.replies[0].revision.id);
  // Changing committed feedback cannot alter a retained prompt on exact retry.
  const originalThread = retained.messages[0].threads[0];
  const current = await api.thread(originalThread);
  const revised = await api.command("threads", {
    type: "message",
    thread_id: originalThread,
    expected_thread: current.thread.reference,
    message_id: crypto.randomUUID().replaceAll("-", ""),
    expected_revision: null,
    reply_to: null,
    body: "New feedback after dispatch must not replay the prompt.",
  });
  expect(revised.ok()).toBeTruthy();
  expect((await revised.json()).status).toBe("committed");
  const duplicate = await page.request.post(
    `${origin}/v1/local/agents/command`,
    { headers: admission, data: submitted },
  );
  expect(duplicate.ok()).toBeTruthy();
  expect((await events()).filter((e) => e.payload.type === "prompt")).toEqual(
    after,
  );
  console.log(
    `${profile === "native" ? "Native WorkStore" : "Registered-folder"} inbox: exact replies/reactions; no implicit prompt; next Send context and reload persistence.`,
  );
}

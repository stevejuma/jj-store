import { expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
export async function accessible(page, label, selector) {
  await page.evaluate(async () => {
    // Theme changes can install transitions after the first layout frame.
    // Observe two quiet frames, waiting for every finite animation created meanwhile.
    let quietFrames = 0;
    while (quietFrames < 2) {
      await new Promise((resolve) => requestAnimationFrame(resolve));
      const animations = document
        .getAnimations({ subtree: true })
        .filter(
          (animation) =>
            animation.effect?.getComputedTiming().iterations !== Infinity &&
            ["running", "pending"].includes(animation.playState),
        );
      if (!animations.length) quietFrames++;
      else {
        quietFrames = 0;
        await Promise.all(
          animations.map((animation) => animation.finished.catch(() => {})),
        );
      }
    }
  });
  const result = await new AxeBuilder({ page }).include(selector).analyze();
  if (result.violations.length)
    throw Error(
      `${label}: ${JSON.stringify(result.violations.map((v) => ({ rule: v.id, nodes: v.nodes.map((n) => ({ target: n.target, html: n.html, reason: n.failureSummary })) })))}`,
    );
}

export async function prepareAgent(
  page,
  origin,
  admission,
  agentName = "Browser qualification",
) {
  await page.getByRole("button", { name: "Add agent", exact: true }).click();
  const editor = page.getByRole("dialog", { name: "Add agent", exact: true });
  await editor.getByLabel("Agent name", { exact: true }).fill(agentName);
  await editor
    .getByRole("group", { name: "Agent instructions", exact: true })
    .getByRole("textbox", { name: "Markdown editor", exact: true })
    .fill("Reply briefly and preserve the source folder.");
  await editor
    .getByRole("button", { name: "Choose agent image", exact: true })
    .click();
  await page.getByRole("tab", { name: "Emoji", exact: true }).click();
  await page
    .getByRole("button", { name: "Use 🐝 as avatar", exact: true })
    .click();
  await page
    .getByRole("button", { name: "Use #30a46c background", exact: true })
    .click();
  await page.keyboard.press("Escape");
  await editor.getByRole("tab", { name: "Cursor", exact: true }).click();
  await editor.getByRole("option", { name: /Harness default/ }).click();
  await editor.getByRole("tab", { name: "Codex", exact: true }).click();
  await editor.getByRole("option", { name: /Harness default/ }).click();
  await accessible(page, "agent editor", '[role="dialog"]');
  await page.emulateMedia({ colorScheme: "dark" });
  await expect(page.locator("html")).toHaveClass(/dark/);
  await accessible(page, "dark agent editor", '[role="dialog"]');
  await page.emulateMedia({ colorScheme: "light" });
  await expect(page.locator("html")).not.toHaveClass(/dark/);
  const savedResponse = page.waitForResponse(
    (r) =>
      r.url() === `${origin}/v1/local/agents/setups` &&
      r.request().method() === "POST",
  );
  await editor.getByRole("button", { name: "Save agent", exact: true }).click();
  const saved = await savedResponse;
  if (!saved.ok()) throw Error(`Agent save refused: ${saved.status()}`);
  await expect(editor).toHaveCount(0);
  const savedStatus = await page.request.get(`${origin}/v1/local/agents`, {
    headers: admission,
  });
  const setup = (await savedStatus.json()).setups.find(
    (s) => s.name === agentName,
  );
  if (!setup) throw Error("Saved identity missing");
  if (setup.runner !== "codex")
    throw Error(`Unexpected saved harness: ${setup.runner}`);
  if (!decodeURIComponent(setup.avatar).includes('fill="#30a46c"'))
    throw Error("Colored emoji missing");
  const card = page.locator(`[data-agent-id="${setup.id}"]`);
  await expect(card.locator(".agents__avatar")).toHaveAttribute(
    "title",
    "Offline",
  );
  await expect
    .poll(() => card.locator("img").evaluate((image) => image.naturalWidth))
    .toBeGreaterThan(0);
  await card
    .getByRole("button", {
      name: `View ${agentName}`,
      exact: true,
    })
    .press("Enter");
  const details = page.getByRole("complementary", {
    name: "Agent details sidebar",
    exact: true,
  });
  await expect(details).toBeVisible();
  await accessible(
    page,
    "agent details",
    '[aria-label="Agent details sidebar"]',
  );
  await details
    .getByRole("button", { name: "View sessions", exact: true })
    .click();
  await expect(page).toHaveURL(new RegExp("/agents/sessions\\?agent="));
  await expect(
    page.getByText("No sessions yet", { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("form", { name: "Start agent session" }),
  ).toHaveCount(0);
  await accessible(page, "empty inbox", '[aria-label="Agent sessions inbox"]');
  await page.goBack();
  await details
    .getByRole("button", { name: "Activity log", exact: true })
    .click();
  await expect(page).toHaveURL(new RegExp("/agents/sessions\\?agent="));
  await expect(
    page.getByText("No sessions yet", { exact: true }),
  ).toBeVisible();
  await accessible(
    page,
    "activity fallback inbox",
    '[aria-label="Agent sessions inbox"]',
  );
  await page.goBack();

  await details
    .getByRole("button", { name: `Edit ${agentName}`, exact: true })
    .click();
  const edit = page.getByRole("dialog", { name: "Edit agent", exact: true });
  await edit
    .getByRole("button", { name: "Choose agent image", exact: true })
    .click();
  await page.getByRole("tab", { name: "Image", exact: true }).click();
  await page.getByLabel("Upload avatar image", { exact: true }).setInputFiles({
    name: "avatar.png",
    mimeType: "image/png",
    buffer: Buffer.from(
      "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jX1kAAAAASUVORK5CYII=",
      "base64",
    ),
  });
  await page.keyboard.press("Escape");
  await expect(
    edit.getByRole("img", { name: "Selected avatar" }),
  ).toHaveJSProperty("naturalWidth", 1);
  await edit.getByRole("button", { name: "Save agent", exact: true }).click();
  await expect(edit).toHaveCount(0);
  await page.reload();
  await expect(card.locator("img")).toHaveJSProperty("naturalWidth", 1);
  return { card, setup };
}

export async function sessionOption(page, name) {
  await page
    .getByRole("button", { name: "Session options", exact: true })
    .click();
  await page.getByRole("menuitem", { name, exact: true }).click();
}
export async function openRuntime(page) {
  await sessionOption(page, "Workspace and runtime");
  return page.getByRole("complementary", {
    name: "Workspace and runtime",
    exact: true,
  });
}

export async function configureRuntime(page, runtime) {
  await runtime
    .getByRole("button", { name: "Edit execution mode", exact: true })
    .click();
  const runtimeEditor = page.getByRole("dialog", {
    name: "Edit agent",
    exact: true,
  });
  await accessible(page, "runtime editor", '[role="dialog"]');
  await runtimeEditor.getByLabel("Execution mode", { exact: true }).click();
  await page
    .getByRole("option", { name: "Ask for approval", exact: true })
    .click();
  await runtimeEditor
    .getByRole("button", { name: "Save agent", exact: true })
    .click();
}

export const runtimeHint =
  "Choose an execution mode in Session options → Workspace and runtime → Edit execution mode before sending.";

export async function waitForConnection(page, origin, admission, sessionId) {
  await expect
    .poll(
      async () =>
        (
          await (
            await page.request.get(`${origin}/v1/local/agents`, {
              headers: admission,
            })
          ).json()
        ).sessions.find((s) => s.id === sessionId)?.connected,
      { timeout: 90000 },
    )
    .toBe(true);
}

export async function sendPrompt(
  page,
  workspace,
  sessionId,
  origin,
  admission,
) {
  const dispatched = [];
  const record = (request) => {
    if (
      request.url().endsWith("/v1/local/agents/command") &&
      request.method() === "POST"
    )
      dispatched.push(request.postDataJSON().action?.action);
  };
  page.on("request", record);
  const response = page.waitForResponse((reply) => {
    if (
      !reply.url().endsWith("/v1/local/agents/command") ||
      reply.request().method() !== "POST"
    )
      return false;
    const command = reply.request().postDataJSON();
    return (
      command.session_id === sessionId && command.action?.action === "prompt"
    );
  });
  await workspace.getByRole("button", { name: "Send", exact: true }).click();
  let reply;
  try {
    reply = await response;
  } catch (cause) {
    const status = await (
      await page.request.get(`${origin}/v1/local/agents`, {
        headers: admission,
      })
    ).json();
    const session = status.sessions.find((s) => s.id === sessionId);
    console.error(
      "Prompt readiness diagnostic",
      JSON.stringify({
        dispatched,
        session: session && {
          connected: session.connected,
          state: session.state,
          preferences: session.preferences,
          modeReady: session.capabilities?.modeReady,
        },
        agent: status.setups.find((s) => s.id === session?.setup_id)
          ?.preferences,
        visible: await workspace.innerText(),
      }),
    );
    throw cause;
  } finally {
    page.off("request", record);
  }
  expect(reply.ok()).toBe(true);
  // Chromium may discard response bytes during the concurrent activity stream.
  // Read the durable outcome by the exact submitted identity instead.
  const command = reply.request().postDataJSON();
  expect(command.session_id).toBe(sessionId);
  expect(command.action.action).toBe("prompt");
  const saved = await page.request.get(
    `${origin}/v1/local/agents/outcome?request=${encodeURIComponent(command.request_id)}`,
    { headers: admission },
  );
  expect(saved.ok()).toBe(true);
  const outcome = await saved.json();
  expect(outcome.request_id).toBe(command.request_id);
  expect(outcome.session_id).toBe(sessionId);
  expect(outcome.error).toBeNull();
}

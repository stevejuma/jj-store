import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo } from "./native-browser-fixture.js";
import {
  fixtureSession,
  fixtureRunners,
} from "../../../catalog/fixtures/agent-execution-session.js";
import type { AgentStatus } from "../../src/lib/controller/agent-execution-types.js";
test.use({ screenshot: "off" });
async function fleet(page: Page) {
  await api(page, { access: "local_trusted" });
  const capabilities = {
    ...sample("capabilities"),
    local_agent_execution: true,
  };
  await page.route("**/v1/bootstrap", (r) =>
    r.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities,
        storage: { ready: true },
        repositories: [
          { id: repo, name: "Local acceptance", permission: "admin" },
        ],
        backups: [],
      },
    }),
  );
  await page.route("**/v1/local/repositories", (r) =>
    r.fulfill({ json: { version: 1, revision: 0, bindings: [] } }),
  );
  let status: AgentStatus = {
    version: 1,
    runtime: "ready",
    revision: 0,
    setups: [
      {
        id: "Exact-Helper",
        name: "Fleet helper",
        avatar: "🤖",
        description: "Reusable helper",
        instructions: "",
        runner: "codex",
        preferences: { model: null, effort: null, mode: "ask" },
      },
    ],
    sessions: [],
    conversations: [],
    runners: fixtureRunners,
    live_agents: [],
  };
  const sends: any[] = [],
    controls: any[] = [];
  let events: any[] = [];
  await page.route("**/v1/local/agents**", async (r) => {
    const url = new URL(r.request().url());
    expect(r.request().headers()["authorization"]).toBeUndefined();
    if (url.pathname.endsWith("/targets")) return r.fulfill({ json: [] });
    if (url.pathname.endsWith("/feedback"))
      return r.fulfill({
        json: {
          version: 1,
          profile: "local_jj_git",
          messages: [],
          write_unavailable: null,
        },
      });
    if (url.pathname.endsWith("/events"))
      return r.fulfill({
        json: {
          version: 1,
          events: events.filter(
            (e) => e.sequence > Number(url.searchParams.get("after")),
          ),
          next: null,
        },
      });
    if (url.pathname.endsWith("/control")) {
      const body = r.request().postDataJSON();
      controls.push(body);
      status = {
        ...status,
        live_agents: [
          {
            agent_id: "Exact-Helper",
            generation: 1,
            state: body.start ? "idle" : "offline",
            enabled: body.start,
            error: null,
          },
        ],
      };
      return r.fulfill({
        json: {
          version: 1,
          request_id: body.request_id,
          session_id: null,
          state: "starting",
          error: null,
        },
      });
    }
    if (url.pathname.endsWith("/chat")) {
      const body = r.request().postDataJSON();
      sends.push(body);
      const session = fixtureSession({
        repository: repo,
        operation: "11".repeat(64),
        commit: "22".repeat(20),
        status: "Online",
        permission: false,
        workspacePath: "/device/chat",
        lastActivityAt: Date.now(),
        preferences: status.setups[0]!.preferences,
        modeReady: true,
        advertisedModels: false,
      });
      session.setup_id = "Exact-Helper";
      session.title = "Fleet helper";
      session.conversation_id = body.conversation_id;
      status = {
        ...status,
        sessions: [session],
        conversations: [
          {
            id: body.conversation_id,
            origin: body.origin,
            home: repo,
            discussion_thread: null,
            participants: [session.id],
          },
        ],
        live_agents: [
          {
            agent_id: "Exact-Helper",
            generation: 1,
            state: "idle",
            enabled: true,
            error: null,
          },
        ],
      };
      events = [
        {
          sequence: 0,
          observed_at_millis: 1791323999000,
          committed: true,
          payload: { type: "opened", requestId: "open-first" },
        },
        {
          sequence: 1,
          observed_at_millis: 1791324000000,
          committed: true,
          payload: {
            type: "prompt",
            requestId: body.request_id,
            text: body.text,
            context:
              "<agent_context>\n<repository_context>\n```text\nPinned repository facts\n```\n</repository_context>\n</agent_context>",
          },
        },
        {
          sequence: 2,
          committed: true,
          payload: {
            type: "activity",
            requestId: body.request_id,
            update: {
              sessionUpdate: "agent_message_chunk",
              content: { text: "Fleet reply in contextual chat." },
            },
          },
        },
      ];
      events.push(
        ...[
          {
            title: "Read captured.ts",
            kind: "read",
            locations: [{ path: "src/captured.ts" }],
            rawOutput: { formatted_output: "export const captured = 42;\n" },
          },
          {
            title: "Edit captured.ts",
            kind: "edit",
            content: [
              {
                type: "diff",
                path: "src/captured.ts",
                oldText: "export const captured = 41;\n",
                newText: "export const captured = 42;\n",
              },
            ],
          },
        ].map((toolSnapshot, index) => ({
          sequence: index + 3,
          committed: true,
          payload: {
            type: "activity",
            requestId: body.request_id,
            toolSnapshot: {
              toolCallId: `tool-${index}`,
              status: "completed",
              ...toolSnapshot,
            },
          },
        })),
      );
      events.push({
        sequence: 5,
        observed_at_millis: 1791324003000,
        committed: true,
        payload: {
          type: "record",
          requestId: body.request_id,
          record: { status: "completed" },
        },
      });
      events.forEach((event, index) => (event.sequence = index + 1));
      return r.fulfill({
        json: {
          request_id: body.request_id,
          conversation_id: body.conversation_id,
          agents: [
            {
              version: 1,
              request_id: body.request_id,
              session_id: session.id,
              state: "queued",
              error: null,
            },
          ],
        },
      });
    }
    return r.fulfill({ json: status });
  });
  return { sends, controls };
}
test("cold ACP tools show highlighted captured files and independent raw JSON views", async ({
  page,
}) => {
  const f = await fleet(page);
  await page.goto(`/#/repos/${repo}/files`);
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  const composer = page.getByRole("group", { name: "Message to agents" });
  const editor = composer.getByRole("textbox", { name: "Markdown editor" });
  await editor.fill("@Fleet");
  await page.getByRole("option", { name: /Fleet helper/ }).click();
  await editor.pressSequentially("Explain the captured tool output");
  await page
    .getByRole("button", {
      name: "Dismiss Commit metadata unavailable",
      exact: true,
    })
    .click();
  await composer.getByRole("button", { name: "Send message" }).click();
  await expect(
    page.getByRole("group", { name: "Session started", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("group", { name: "Turn completed", exact: true }),
  ).toBeVisible();
  await expect(
    page.locator('time[datetime="2026-10-06T21:59:59.000Z"]'),
  ).toBeVisible();
  await expect(
    page.locator('[data-ui-component="agent-sent-context"]'),
  ).toHaveCount(1);
  await page.getByText("Prepared context", { exact: true }).focus();
  await page.keyboard.press("Enter");
  await expect(
    page.locator('[data-ui-component="agent-sent-context"]'),
  ).toHaveAttribute("open", "");
  await page
    .getByRole("button", { name: "<agent_context>", exact: true })
    .click();
  await page
    .getByRole("button", { name: "<repository_context>", exact: true })
    .click();
  await expect(
    page.getByText("Pinned repository facts", { exact: true }),
  ).toBeVisible();
  for (const label of ["Session started", "Turn completed"]) {
    const marker = page.getByRole("group", { name: label, exact: true });
    await expect(marker).toHaveAttribute("title", label);
    await expect(marker.locator("svg")).toHaveCount(1);
    await expect(marker).not.toContainText(label);
  }
  const group = page.getByRole("button", {
    name: "Expand 2 tool calls",
    exact: true,
  });
  const groupWidth = await group.evaluate((element) => {
    const root = element.closest('[data-ui-component="ai-chat-tool-calls"]')!;
    const parent = root.parentElement!;
    const style = getComputedStyle(parent);
    return {
      actual: root.getBoundingClientRect().width,
      available:
        parent.clientWidth -
        parseFloat(style.paddingInlineStart) -
        parseFloat(style.paddingInlineEnd),
    };
  });
  expect(groupWidth.actual).toBeCloseTo(groupWidth.available, 0);
  await group.click();
  const toolMessage = page.getByRole("article", {
    name: "Tool call: Read captured.ts",
    exact: true,
  });
  await toolMessage.hover();
  await expect(toolMessage).toHaveAttribute(
    "data-ui-component",
    "agent-tool-message",
  );
  await expect(toolMessage.getByRole("img")).toHaveCount(0);
  const trigger = toolMessage.locator('[data-ai-chat-part="call-trigger"]');
  await trigger.hover();
  const inset = await trigger.evaluate((element) => {
    const icon = element.querySelector('[data-ui-part="call-status"]')!;
    return {
      leading:
        icon.getBoundingClientRect().left -
        element.getBoundingClientRect().left,
      hover: getComputedStyle(element).backgroundColor,
    };
  });
  const disclosure = await trigger
    .locator('[data-ui-part="detail-chevron"]')
    .boundingBox();
  expect(disclosure?.width).toBe(14);
  expect(disclosure?.height).toBe(14);
  expect(inset.leading).toBe(8);
  expect(inset.hover).not.toBe("rgba(0, 0, 0, 0)");
  await expect(
    toolMessage.getByRole("button", { name: "Reply", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", {
      name: "Show details for Read captured.ts",
      exact: true,
    })
    .click();
  const read = toolMessage.locator('[data-ui-component="agent-tool-detail"]');
  await expect(read).toContainText("src/captured.ts");
  await expect(
    read.getByRole("group", { name: "typescript", exact: true }),
  ).toBeVisible();
  await expect(read).toContainText("export const captured = 42;");
  await read.getByRole("button", { name: "Show raw output" }).click();
  await expect(read).toContainText("formatted_output");
  await read.getByRole("button", { name: "Show rich output" }).click();
  await page
    .getByRole("button", {
      name: "Show details for Edit captured.ts",
      exact: true,
    })
    .click();
  await expect(
    page.getByRole("region", { name: "Diff for src/captured.ts" }),
  ).toBeVisible();
  const findings = await new AxeBuilder({ page })
    .include('[data-ui-component="agent-chat"]')
    .analyze();
  expect(findings.violations).toEqual([]);
  await page.setViewportSize({ width: 1800, height: 1000 });
  await page.goto(`/#/agents/sessions/${"44".repeat(16)}`);
  const inboxGroup = page.getByRole("button", {
    name: "Expand 2 tool calls",
    exact: true,
  });
  await expect(inboxGroup).toBeVisible();
  const inboxWidth = await inboxGroup.evaluate((element) => {
    const root = element.closest('[data-ui-component="ai-chat-tool-calls"]')!;
    const parent = root.parentElement!;
    const style = getComputedStyle(parent);
    return {
      actual: root.getBoundingClientRect().width,
      available:
        parent.clientWidth -
        parseFloat(style.paddingInlineStart) -
        parseFloat(style.paddingInlineEnd),
    };
  });
  expect(inboxWidth.available).toBeGreaterThan(672);
  expect(inboxWidth.actual).toBeCloseTo(inboxWidth.available, 0);
  await inboxGroup.click();
  await expect(
    page.getByRole("button", {
      name: "Show details for Read captured.ts",
      exact: true,
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("group", { name: "Session started", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("group", { name: "Turn completed", exact: true }),
  ).toBeVisible();
  expect(f.sends).toHaveLength(1);
  expect(f.controls).toHaveLength(0);
});

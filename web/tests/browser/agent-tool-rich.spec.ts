import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import {
  appearanceStorageKey,
  defaultAppearance,
} from "../../src/lib/appearance/preferences.js";
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
    if (/\/agents\/(?:settings|chatgpt)$/.test(url.pathname))
      return r.fallback();
    expect(r.request().headers()["authorization"]).toBeUndefined();
    if (url.pathname.endsWith("/targets")) return r.fulfill({ json: [] });
    if (url.pathname.endsWith("/feedback"))
      return r.fulfill({
        json: {
          version: 1,
          profile: "native",
          messages: events
            .filter((e) => e.payload.toolSnapshot)
            .map((e, i) => {
              const tool = e.payload.toolSnapshot;
              return {
                row_id: `${e.payload.requestId}/tool/${tool.toolCallId}`,
                target: {
                  type: "tool_output",
                  session_id: "44".repeat(16),
                  tool_id: (i ? "78" : "77").repeat(16),
                  snapshot: { kind: "tool_snapshot", id: "88".repeat(32) },
                  content: { kind: "content", id: "66".repeat(32) },
                },
                text_parts: i
                  ? [
                      {
                        part: "/content/0/oldText",
                        text: tool.content[0].oldText,
                      },
                      {
                        part: "/content/0/newText",
                        text: tool.content[0].newText,
                      },
                    ]
                  : [
                      {
                        part: "/rawOutput/formatted_output",
                        text: tool.rawOutput.formatted_output,
                      },
                    ],
                answers: [],
                threads: [],
                replies: [],
              };
            }),
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
            sentContext: {
              delivery: "sent",
              wireContext:
                "<agent_context>\n<repository_context>\n```text\nPinned repository facts\n```\n</repository_context>\n</agent_context>",
            },
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
            locations: [{ path: "src/captured.ts" }],
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
  await page.addInitScript(
    ({ key, value }) => localStorage.setItem(key, JSON.stringify(value)),
    {
      key: appearanceStorageKey,
      value: { ...defaultAppearance, conversationView: "classic" },
    },
  );
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
  await page.getByRole("button", { name: "Send message", exact: true }).click();
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
  await page.getByRole("button", { name: "Sent context", exact: true }).focus();
  await page.keyboard.press("Enter");
  await expect(
    page.getByRole("button", { name: "Sent context", exact: true }),
  ).toHaveAttribute("aria-expanded", "true");
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
  const readTrigger = page.getByRole("button", {
    name: "Show details for Read captured.ts",
    exact: true,
  });
  const editTrigger = page.getByRole("button", {
    name: "Show details for Edited file",
    exact: true,
  });
  await expect(readTrigger).toBeVisible();
  await expect(editTrigger).toBeVisible();
  const width = await readTrigger.evaluate((element) => {
    const root = element.closest(".chat-tool-item")!;
    const parent = root.parentElement!;
    return [root.getBoundingClientRect().width, parent.clientWidth];
  });
  expect(width[0]).toBeCloseTo(width[1], 0);
  await readTrigger.click();
  const read = page.locator(".chat-tool-panel-content");
  await expect(read).toContainText("src/captured.ts");
  await expect(read.locator(".cm-content .cm-line span").first()).toBeVisible();
  await expect(read).toContainText("export const captured = 42;");
  await page
    .getByRole("button", {
      name: "Show raw output in read sidebar for Read captured.ts",
      exact: true,
    })
    .click();
  await expect(read).toContainText("formatted_output");
  await page
    .getByRole("button", {
      name: "Show rich output in read sidebar for Read captured.ts",
      exact: true,
    })
    .click();
  await page
    .getByRole("button", { name: "Close read tool details", exact: true })
    .click();
  await editTrigger.click();
  const edit = page.locator('.chat-tool-detail[aria-label="Edited file"]');
  await expect(edit).toContainText("export const captured = 41;");
  await edit
    .getByRole("button", {
      name: "Show captured file src/captured.ts",
      exact: true,
    })
    .click();
  await expect(edit).toContainText("export const captured = 42;");
  await page
    .getByRole("button", {
      name: "Show raw output for Edited file",
      exact: true,
    })
    .click();
  await expect(edit).toContainText("oldText");
  await page
    .getByRole("button", {
      name: "Show rich output for Edited file",
      exact: true,
    })
    .click();
  const findings = await new AxeBuilder({ page })
    .include('[data-ui-component="agent-chat"]')
    .analyze();
  expect(findings.violations).toEqual([]);
  await page.setViewportSize({ width: 1800, height: 1000 });
  await page.goto(`/#/agents/sessions/${"44".repeat(16)}`);
  const inboxRead = page.getByRole("button", {
    name: "Show details for Read captured.ts",
    exact: true,
  });
  await expect(inboxRead).toBeVisible();
  const inboxWidth = await inboxRead.evaluate((element) => {
    const root = element.closest(".chat-tool-item")!;
    return [
      root.getBoundingClientRect().width,
      root.parentElement!.clientWidth,
    ];
  });
  expect(inboxWidth[1]).toBeGreaterThan(672);
  expect(inboxWidth[0]).toBeCloseTo(inboxWidth[1], 0);
  await inboxRead.click();
  await expect(page.locator(".chat-tool-panel-content")).toContainText(
    "export const captured = 42;",
  );
  await expect(
    page.getByRole("button", {
      name: "Show raw output in read sidebar for Read captured.ts",
      exact: true,
    }),
  ).toHaveAttribute("aria-pressed", "false");
  await expect(
    page.getByRole("group", { name: "Session started", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("group", { name: "Turn completed", exact: true }),
  ).toBeVisible();
  expect(f.sends).toHaveLength(1);
  expect(f.controls).toHaveLength(0);
});

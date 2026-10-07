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
          sequence: 1,
          committed: true,
          payload: {
            type: "prompt",
            requestId: body.request_id,
            text: body.text,
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
test("cold contextual chat uses exact tags, retains pins and reopens independently of details", async ({
  page,
}) => {
  const f = await fleet(page);
  await page.goto(`/#/repos/${repo}/files`);
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  const header = page.locator(".agent-chat-heading");
  await expect(header.getByRole("button")).toHaveCount(1);
  await expect(
    header.getByRole("button", { name: "Close", exact: true }),
  ).toBeVisible();
  const composer = page.getByRole("group", { name: "Message to agents" });
  await expect(composer).toBeVisible();
  expect(f.sends).toHaveLength(0);
  const editor = composer.getByRole("textbox", { name: "Markdown editor" });
  await editor.fill("@Fleet");
  await page
    .getByRole("switch", { name: "Automatically mention agents" })
    .check();
  await page.getByRole("option", { name: /Fleet helper/ }).click();
  await expect(editor).toContainText("Fleet helper");
  await expect(
    composer.getByRole("button", {
      name: "Don't automatically mention Fleet helper in this thread",
    }),
  ).toBeVisible();
  await editor.pressSequentially("Explain this repository");
  await expect(editor).toContainText("Fleet helper");
  await page
    .getByRole("button", {
      name: "Dismiss Commit metadata unavailable",
      exact: true,
    })
    .click();
  await composer.getByRole("button", { name: "Send message" }).click();
  await expect(page.getByText("Fleet reply in contextual chat.")).toBeVisible();
  expect(f.sends[0].agent_ids).toEqual(["Exact-Helper"]);
  expect(f.sends[0].attachments[0].repository).toBe(repo);
  await page.getByRole("button", { name: "Close", exact: true }).click();
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  await composer.getByRole("button", { name: "Mention someone" }).click();
  await expect(
    page.getByRole("switch", {
      name: "Automatically mention agents",
    }),
  ).toBeChecked();
  await editor.press("Escape");
  await expect(editor).toContainText("Fleet helper");
  expect(
    (
      await new AxeBuilder({ page })
        .include('[data-ui-component="agent-chat"]')
        .analyze()
    ).violations,
  ).toEqual([]);
});
test("cold agent Start warms in place without entering a chat", async ({
  page,
}) => {
  const f = await fleet(page);
  await page.goto("/#/agents");
  await page
    .getByRole("button", { name: "Start Fleet helper", exact: true })
    .click();
  await expect.poll(() => f.controls.length).toBe(1);
  expect(f.controls[0]).toMatchObject({
    agent_id: "Exact-Helper",
    start: true,
  });
  expect(f.sends).toHaveLength(0);
  await expect(page).toHaveURL(/#\/agents$/);
  await expect(
    page.getByRole("img", { name: "Fleet helper, Idle", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Actions for Fleet helper", exact: true })
    .click();
  await page.getByRole("menuitem", { name: "New chat", exact: true }).click();
  await expect(page.getByText("General chat", { exact: true })).toBeVisible();
  expect(f.sends).toHaveLength(0);
});

import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { api, sample, repo } from "./native-browser-fixture.js";
import {
  fixtureSession,
  fixtureRunners,
} from "../../../catalog/fixtures/agent-execution-session.js";
import type {
  AgentStatus,
  AgentEvent,
} from "../../src/lib/controller/agent-execution-types.js";

test("cold chats show live avatars, composer reconnect warnings and exact saved changes", async ({
  page,
}) => {
  await api(page, { access: "local_trusted" });
  await page.route("**/v1/bootstrap", (r) =>
    r.fulfill({
      json: {
        version: 1,
        access_mode: "local_trusted",
        capabilities: {
          ...sample("capabilities"),
          local_agent_execution: true,
        },
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
  const operation = "11".repeat(64),
    base = "22".repeat(64),
    commit = "33".repeat(64);
  const setup = {
    id: "Exact-Helper",
    name: "Fleet helper",
    avatar: "🤖",
    description: "",
    instructions: "",
    runner: "codex",
    preferences: { model: null, effort: null, mode: "ask" },
  };
  let status: AgentStatus = {
    version: 1,
    runtime: "ready",
    revision: 0,
    setups: [setup],
    sessions: [],
    conversations: [],
    runners: fixtureRunners,
    live_agents: [],
  };
  let events: AgentEvent[] = [],
    sends = 0,
    controls = 0,
    unavailable = false,
    pathReads = 0;
  await page.route("**/v1/local/agents**", async (r) => {
    const url = new URL(r.request().url());
    if (/\/agents\/(?:settings|chatgpt)$/.test(url.pathname))
      return r.fallback();
    if (url.pathname.endsWith("/targets")) return r.fulfill({ json: [] });
    if (url.pathname.endsWith("/feedback"))
      return r.fulfill({
        json: {
          version: 1,
          profile: "native",
          messages: [],
          write_unavailable: null,
        },
      });
    if (url.pathname.endsWith("/events"))
      return unavailable
        ? r.fulfill({ status: 503, json: { code: "unavailable" } })
        : r.fulfill({
            json: {
              version: 1,
              events: events.filter(
                (e) =>
                  Number(e.sequence) > Number(url.searchParams.get("after")),
              ),
              next: null,
            },
          });
    if (url.pathname.endsWith("/stream"))
      return r.fulfill({
        contentType: "text/event-stream",
        body: `data: ${JSON.stringify({ events: [], next: null })}\n\n`,
      });
    if (url.pathname.endsWith("/control")) {
      controls++;
      return r.fulfill({ status: 503, json: { code: "unexpected_control" } });
    }
    if (url.pathname.endsWith("/chat")) {
      sends++;
      const body = r.request().postDataJSON();
      const session = fixtureSession({
        repository: repo,
        operation,
        commit,
        status: "Online",
        permission: false,
        workspacePath: "/device/chat",
        lastActivityAt: 1791324003000,
        preferences: setup.preferences,
        modeReady: true,
        advertisedModels: false,
      });
      session.setup_id = setup.id;
      session.title = setup.name;
      session.avatar = setup.avatar;
      session.conversation_id = body.conversation_id;
      session.last_sequence = 4;
      session.repository_bindings = [
        {
          repository: repo,
          target_id: "fixture",
          profile: "native",
          workspace: "agent-workspace",
          selected_operation: operation,
          base_commit: base,
          operation,
          commit,
        },
      ];
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
            agent_id: setup.id,
            state: "idle",
            enabled: true,
            generation: 1,
            error: null,
          },
        ],
      };
      events = [
        {
          sequence: 1,
          observed_at_millis: 1791323999900,
          committed: true,
          payload: {
            type: "repository_turn_start",
            requestId: body.request_id,
            repository: repo,
            state: "completed",
            operation,
            commit: base,
          },
        },
        {
          sequence: 2,
          observed_at_millis: 1791324000000,
          committed: true,
          payload: {
            type: "prompt",
            requestId: body.request_id,
            text: body.text,
          },
        },
        {
          sequence: 3,
          observed_at_millis: 1791324003000,
          committed: true,
          payload: {
            type: "record",
            requestId: body.request_id,
            record: { status: "completed" },
          },
        },
        {
          sequence: 4,
          observed_at_millis: 1791324003100,
          committed: true,
          payload: {
            type: "repository_snapshot",
            requestId: body.request_id,
            repository: repo,
            state: "completed",
            operation,
            commit,
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
              state: "completed",
              error: null,
            },
          ],
        },
      });
    }
    return r.fulfill({ json: status });
  });
  const change = (path: string) => ({
    path,
    before: [],
    after: [],
    sources: [],
    content: {
      status: "text",
      hunks: [
        {
          kind: "different",
          before_start: 0,
          after_start: 0,
          before: "old\n",
          after: "new\nmore\n",
        },
      ],
    },
  });
  await page.route(`**/v1/repos/${repo}/diff**`, (r) => {
    const url = new URL(r.request().url());
    if (
      url.searchParams.get("from") !== base ||
      url.searchParams.get("to") !== commit
    )
      return r.fallback();
    expect(url.searchParams.get("at_op")).toBe(operation);
    expect(r.request().method()).toBe("GET");
    const paths = url.pathname.endsWith("/paths");
    if (paths) pathReads++;
    return r.fulfill({
      json: {
        snapshot: {
          repo_id: repo,
          operation_id: operation,
          heads: [operation],
          epoch: 1,
          published_seq: 1,
          manifest_revision: 1,
        },
        data: {
          version: 1,
          from: base,
          to: commit,
          path: url.searchParams.get("path") ?? "",
          changes: paths
            ? Array.from({ length: 20 }, (_, n) =>
                change(`src/example-${n + 1}.ts`),
              )
            : [change(url.searchParams.get("path")!)],
        },
        next_cursor: null,
      },
    });
  });
  await page.goto(`/#/repos/${repo}/files`);
  await page.getByRole("button", { name: "Open chat", exact: true }).click();
  const editor = page.getByRole("textbox", { name: "Markdown editor" });
  await editor.pressSequentially("@Fleet");
  await page.getByRole("option", { name: /Fleet helper/ }).click();
  await editor.pressSequentially("Make the requested changes");
  await page
    .getByRole("button", {
      name: "Dismiss Commit metadata unavailable",
      exact: true,
    })
    .click();
  await page.getByRole("button", { name: "Send message", exact: true }).click();
  const header = page.locator(".agent-chat-heading");
  await expect(
    header.getByRole("img", { name: "Fleet helper, Idle" }),
  ).toBeVisible();
  const panel = page.getByRole("region", {
    name: /^Changes in Local acceptance · turn [0-9a-f]{8}$/,
  });
  await expect(
    panel.getByText("Edited 20 files", { exact: true }),
  ).toBeVisible();
  await expect(panel.getByText("+40", { exact: true })).toBeVisible();
  await expect(panel.getByText("-20", { exact: true })).toBeVisible();
  await expect(
    panel.getByRole("button", { name: /^src\/example-\d+\.ts(?:\s|$)/ }),
  ).toHaveCount(3);
  await panel
    .getByRole("button", { name: "Show 17 more files", exact: true })
    .click();
  await expect(
    panel.getByRole("button", { name: /^src\/example-\d+\.ts(?:\s|$)/ }),
  ).toHaveCount(20);
  await panel
    .getByRole("button", { name: "Show fewer files", exact: true })
    .click();
  await expect(
    panel.getByRole("button", { name: "Undo unavailable", exact: true }),
  ).toBeDisabled();
  expect((await new AxeBuilder({ page }).analyze()).violations).toEqual([]);
  unavailable = true;
  status.sessions[0]!.last_sequence = 5;
  await page.getByRole("button", { name: "Agents", exact: true }).click();
  const refreshed = page.waitForResponse(
    (response) => response.url().endsWith("/v1/local/agents") && response.ok(),
  );
  await page
    .getByRole("button", { name: "Refresh agents", exact: true })
    .click();
  await refreshed;
  await page.goBack();
  await page
    .getByRole("complementary", { name: "Chat", exact: true })
    .getByRole("button", { name: "Close", exact: true })
    .click();
  await page.getByRole("button", { name: "Open chat", exact: true }).focus();
  await page
    .getByRole("button", { name: "Open chat", exact: true })
    .press("Enter");
  const warning = page.getByText(
    "Chat updates are unavailable. Reconnecting…",
    { exact: true },
  );
  await expect(warning).toBeVisible({ timeout: 6000 });
  await expect(
    page
      .getByRole("group", { name: "Message to agents", exact: true })
      .getByText("Chat updates are unavailable. Reconnecting…", {
        exact: true,
      }),
  ).toBeVisible();
  await editor.pressSequentially("Keep my draft");
  events.push({
    ...events[2]!,
    sequence: 5,
    observed_at_millis: 1791324003200,
  });
  unavailable = false;
  // The failed history read retains dirtiness for bounded automatic recovery.
  await expect(warning).toHaveCount(0, { timeout: 12000 });
  await expect(editor).toContainText("Keep my draft");
  expect(pathReads).toBe(1);
  await page.screenshot({
    path: "/tmp/jj-store-agent-changes-cold-sidebar.png",
  });
  const savedFile = panel.locator('button[data-file-path="src/example-2.ts"]');
  await expect(savedFile).toHaveAccessibleName(/^src\/example-2\.ts(?:\s|$)/);
  await savedFile.click();
  const inspector = page.getByRole("complementary", {
    name: "Workspace file",
    exact: true,
  });
  await expect(
    inspector.getByText("src/example-2.ts", { exact: true }),
  ).toBeVisible();
  await expect(inspector.getByText("more", { exact: true })).toBeVisible();
  expect(
    (
      await new AxeBuilder({ page })
        .include('[aria-label="Workspace file"]')
        .analyze()
    ).violations,
  ).toEqual([]);
  await inspector
    .getByRole("button", { name: "Close workspace file", exact: true })
    .click();
  await expect(inspector).toHaveCount(0);
  expect(sends).toBe(1);
  expect(controls).toBe(0);
  await page.goto(`/#/agents/sessions/${status.sessions[0]!.id}`);
  await expect(
    page
      .getByRole("region", {
        name: /^Changes in Local acceptance · turn [0-9a-f]{8}$/,
      })
      .getByText("Edited 20 files", { exact: true }),
  ).toBeVisible();
  expect(sends).toBe(1);
  expect(controls).toBe(0);
});

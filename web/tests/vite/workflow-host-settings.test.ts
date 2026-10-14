import { expect, test, vi } from "vitest";
import {
  parseWorkflowHost,
  validateWorkflowConfiguration,
} from "../../src/lib/controller/workflow-host-configuration.js";
import { createServerSettings } from "../../src/lib/controller/server-settings.svelte.js";
import {
  copy,
  type SaveServerSettings,
} from "../../src/lib/controller/server-configuration.js";
import { serverSettingsValue } from "../../../catalog/fixtures/server-settings.js";
export const host = {
  version: 1,
  host: "builder",
  principal: "01".repeat(16),
  home: "/private/home",
  state_dir: "/private/host",
  jj_executable: "/usr/local/jj",
  allowed_owners: ["02".repeat(16)],
  sources: [{ repository: "03".repeat(16), workspace: "/private/code" }],
  commands: { build: { program: "/bin/sh", secrets: { TOKEN: "HOST_TOKEN" } } },
  agent_endpoint: null,
};
test("host parsing preserves the standalone format and normalizes defaults", () => {
  expect(parseWorkflowHost(JSON.stringify(host))).toMatchObject({
    host: "builder",
    labels: [],
    agents: [],
    poll_millis: 1000,
    sources: [{ target_id: null }],
    commands: { build: { args: [], environment: {}, artifacts: [] } },
  });
  for (const patch of [
    { sources: [] },
    { state_dir: "relative" },
    { principal: "short" },
    { poll_millis: 60001 },
    { labels: ["bad label"] },
    {
      commands: {
        build: { program: "/bin/sh", secrets: { TOKEN: "secret value" } },
      },
    },
    { agents: ["agent"] },
    { unsupported: true },
  ])
    expect(() =>
      parseWorkflowHost(JSON.stringify({ ...host, ...patch })),
    ).toThrow();
  for (const value of ["null", "[]", "42", "{"])
    expect(() => parseWorkflowHost(value)).toThrow();
});
test("agent endpoint configuration requires transport and credential references", () => {
  parseWorkflowHost(
    JSON.stringify({
      ...host,
      agents: ["agent"],
      agent_endpoint: {
        url: "http://127.0.0.1:9999/",
        local: true,
        token_environment: null,
      },
    }),
  );
  for (const endpoint of [
    {
      url: "http://remote.example/",
      local: false,
      token_environment: "HOST_TOKEN",
    },
    {
      url: "https://user:secret@example.com/",
      local: false,
      token_environment: "HOST_TOKEN",
    },
    {
      url: "http://127.0.0.1:9999/",
      local: true,
      token_environment: "HOST_TOKEN",
    },
  ])
    expect(() =>
      parseWorkflowHost(JSON.stringify({ ...host, agent_endpoint: endpoint })),
    ).toThrow();
  expect(() =>
    validateWorkflowConfiguration({ admitted: ["short"], local: null }),
  ).toThrow();
});
test("workflow save is conditional and inactive, preserving other drafts", async () => {
  let value = serverSettingsValue();
  const request = vi.fn(async (_path: string, body?: SaveServerSettings) => {
    if (body)
      value = {
        ...copy(value),
        revision: "cd".repeat(32),
        desired: {
          ...copy(value.desired),
          [body.update.section]: copy(body.update.configuration),
        },
      };
    return copy(value);
  });
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  manager.configure("protection", { bookmarks: ["release"], commits: [] });
  const configuration = {
    admitted: [host.principal],
    local: {
      enabled: false,
      configuration: parseWorkflowHost(JSON.stringify(host)),
    },
  };
  manager.configure("workflow", configuration);
  await manager.save("workflow");
  expect(request.mock.calls[1][1]).toMatchObject({
    expected_revision: "ab".repeat(32),
    update: { section: "workflow", configuration },
  });
  expect(manager.state.value?.effective.workflow.local).toBeNull();
  expect(manager.state.drafts.protection.dirty).toBe(true);
  expect(manager.state.drafts.workflow.dirty).toBe(false);
});
test("raw host JSON remains a draft across refresh and blocks accidental saves", async () => {
  const value = serverSettingsValue();
  const request = vi.fn(async () => copy(value));
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  manager.configure("workflow", {
    admitted: [host.principal],
    local: {
      enabled: false,
      configuration: parseWorkflowHost(JSON.stringify(host)),
    },
  });
  const edit = manager.state.drafts.workflow;
  edit.advanced = "{unfinished";
  edit.advancedPending = true;
  await manager.observe();
  await manager.save("workflow");
  expect(edit.advanced).toBe("{unfinished");
  expect(edit.validation).toContain("Apply the host configuration JSON");
  expect(request).toHaveBeenCalledTimes(2);
  manager.cancel("workflow");
  expect(edit.advancedPending).toBe(false);
});

test("unapplied JSON alone cannot silently save or rebase after external changes", async () => {
  let value = serverSettingsValue();
  const request = vi.fn(async () => copy(value));
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  const edit = manager.state.drafts.workflow;
  edit.advanced = "{draft";
  edit.advancedPending = true;
  await manager.save("workflow");
  expect(edit.validation).toContain("Apply the host configuration JSON");
  value = { ...value, revision: "cd".repeat(32) };
  await manager.observe();
  expect(edit.conflict).toBe(true);
  expect(edit.baseRevision).toBe("ab".repeat(32));
  expect(request).toHaveBeenCalledTimes(2);
});

test("JSON editor saves the edited host with normalized defaults", async () => {
  let value = serverSettingsValue();
  const request = vi.fn(async (_path: string, body?: SaveServerSettings) => {
    if (body)
      value = {
        ...copy(value),
        revision: "cd".repeat(32),
        desired: {
          ...copy(value.desired),
          [body.update.section]: copy(body.update.configuration),
        },
      };
    return copy(value);
  });
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  manager.configure("workflow", {
    admitted: [],
    local: {
      enabled: false,
      configuration: parseWorkflowHost(JSON.stringify(host)),
    },
  });
  const edit = manager.state.drafts.workflow;
  edit.advancedMode = "json";
  edit.advanced = JSON.stringify({ ...host, host: "updated" });
  edit.advancedPending = true;
  await manager.save("workflow");
  expect(request.mock.calls[1][1]).toMatchObject({
    update: {
      section: "workflow",
      configuration: {
        local: {
          enabled: false,
          configuration: {
            host: "updated",
            poll_millis: 1000,
            sources: [{ target_id: null }],
            commands: { build: { args: [], artifacts: [], environment: {} } },
          },
        },
      },
    },
  });
  expect(edit.unknown).toBeNull();
  expect(edit.dirty).toBe(false);
  expect(edit.advancedPending).toBe(false);
  expect(manager.state.value?.effective.workflow.local).toBeNull();
});

test("invalid JSON cannot replace form values or dispatch a save", async () => {
  const request = vi.fn(async () => copy(serverSettingsValue()));
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  manager.configure("workflow", {
    admitted: [],
    local: {
      enabled: false,
      configuration: parseWorkflowHost(JSON.stringify(host)),
    },
  });
  const edit = manager.state.drafts.workflow;
  edit.advancedMode = "json";
  edit.advanced = "null";
  edit.advancedPending = true;
  expect(manager.applyWorkflowJson()).toBe(false);
  await manager.save("workflow");
  expect(edit.validation).toBe(
    "Host configuration must be an object. Your text is retained.",
  );
  expect(edit.advanced).toBe("null");
  expect(edit.draft).toMatchObject({
    local: { configuration: { host: "builder" } },
  });
  expect(request).toHaveBeenCalledTimes(1);
});

test("workflow set ordering matches the server's canonical acknowledgement", async () => {
  let value = serverSettingsValue();
  const request = vi.fn(async (_path: string, body?: SaveServerSettings) => {
    if (body && body.update.section === "workflow") {
      const configuration = copy(body.update.configuration);
      configuration.admitted.sort();
      configuration.local!.configuration.allowed_owners.sort();
      configuration.local!.configuration.labels!.sort();
      value = {
        ...copy(value),
        revision: "cd".repeat(32),
        desired: { ...copy(value.desired), workflow: configuration },
      };
    }
    return copy(value);
  });
  const manager = createServerSettings({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  await manager.observe();
  manager.configure("workflow", {
    admitted: ["ff".repeat(16), "01".repeat(16)],
    local: {
      enabled: false,
      configuration: parseWorkflowHost(
        JSON.stringify({
          ...host,
          allowed_owners: ["ff".repeat(16), "02".repeat(16)],
          labels: ["z", "a"],
        }),
      ),
    },
  });
  await manager.save("workflow");
  expect(manager.state.drafts.workflow.unknown).toBeNull();
  expect(manager.state.drafts.workflow.dirty).toBe(false);
});

test("managed environment has no repository wiring and retains optional overrides", () => {
  const environment = parseWorkflowHost(
    JSON.stringify({
      version: 2,
      host: "builder",
      labels: ["z", "a"],
      commands: { build: { program: "/bin/sh" } },
      agents: ["agent"],
    }),
  );
  expect(environment).toMatchObject({
    version: 2,
    labels: ["a", "z"],
    agents: ["agent"],
    agent_endpoint: null,
    allowed_owners: [],
    state_dir: null,
    jj_executable: null,
    poll_millis: 1000,
  });
  for (const key of ["principal", "home", "sources"])
    expect(environment).not.toHaveProperty(key);
  for (const patch of [
    { version: 3 },
    { home: "/tmp/home" },
    { state_dir: "relative" },
    { poll_millis: 0 },
    { principal: "01".repeat(16) },
    { commands: { bad: { program: "relative" } } },
  ])
    expect(() =>
      parseWorkflowHost(JSON.stringify({ ...environment, ...patch })),
    ).toThrow();
});

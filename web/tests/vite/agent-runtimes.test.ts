import { expect, test, vi } from "vitest";
import {
  AgentRuntimes,
  type RuntimeStatus,
  type RuntimeOutcome,
  type RuntimeCommand,
} from "../../src/lib/controller/agent-runtimes.svelte.js";
import { ApiError } from "../../src/lib/api/client.js";
function fixture() {
  const value: RuntimeStatus = {
    version: 1,
    node_version: "24.18.0",
    npm_available: true,
    install_supported: true,
    install_unavailable: null,
    pending: null,
    runtimes: [
      {
        id: "codex",
        label: "Codex",
        state: "adapter_needed",
        adapter_version: null,
        cli_version: "0.156.1",
        managed: false,
        action: "install",
        detail: "Codex",
        guide_url: "https://developers.openai.com/codex/cli",
        target_version: "1.13.1",
        target_cli_version: "0.156.1",
      },
    ],
  };
  let outcome: RuntimeOutcome | null = null;
  const request = vi.fn(async (path: string, body?: RuntimeCommand) => {
    if (path === "agents/runtimes") return structuredClone(value);
    if (body)
      outcome = {
        version: 1,
        request_id: body.request_id,
        runtime_id: "codex",
        state: "installing",
        error: null,
      };
    return structuredClone(outcome);
  });
  const manager = new AgentRuntimes(() => true, undefined, {
    request: request as never,
    close: vi.fn(),
  });
  return {
    manager,
    request,
    value,
    get outcome() {
      return outcome;
    },
    set outcome(v) {
      outcome = v;
    },
  };
}
test("explicit install and original receipt polling never dispatch execution", async () => {
  const f = fixture();
  await f.manager.refresh();
  expect(f.request.mock.calls.map((c) => c[0])).toEqual(["agents/runtimes"]);
  await f.manager.install(f.value.runtimes[0]);
  const id = f.manager.pending;
  expect(id).toMatch(/^[a-f0-9]{32}$/);
  expect(f.request.mock.calls.find((c) => c[1])?.[1]).toEqual({
    version: 1,
    request_id: id,
    runtime_id: "codex",
    action: "install",
  });
  await f.manager.install(f.value.runtimes[0]);
  expect(f.request.mock.calls.filter((c) => c[1])).toHaveLength(1);
  f.outcome = { ...f.outcome!, state: "completed" };
  await f.manager.refresh();
  expect(f.manager.pending).toBeNull();
  expect(f.manager.outcome?.state).toBe("completed");
  expect(
    f.request.mock.calls.every((c) => c[0].startsWith("agents/runtimes")),
  ).toBe(true);
  f.manager.destroy();
});
test("failed refresh retains rows, marks status uncertain and blocks new installs until recovery", async () => {
  const f = fixture();
  await f.manager.refresh();
  f.request.mockRejectedValueOnce(new TypeError("offline"));
  await f.manager.refresh();
  expect(f.manager.value?.runtimes[0].label).toBe("Codex");
  expect(f.manager.error).toBe("offline");
  await f.manager.install(f.value.runtimes[0]);
  expect(f.request.mock.calls.filter((c) => c[1])).toHaveLength(0);
  await f.manager.refresh();
  expect(f.manager.error).toBe("");
  f.manager.destroy();
});
test("Ready never installs and unknown durable receipts remain reserved", async () => {
  const f = fixture();
  await f.manager.refresh();
  await f.manager.install({ ...f.value.runtimes[0], state: "ready" });
  expect(f.request.mock.calls.filter((c) => c[1])).toHaveLength(0);
  f.request.mockImplementationOnce(async (_path, body) => {
    f.outcome = {
      version: 1,
      request_id: body!.request_id,
      runtime_id: "codex",
      state: "outcome_unknown",
      error: "runtime_receipt_unavailable_restart",
    };
    return f.outcome;
  });
  await f.manager.install(f.value.runtimes[0]);
  expect(f.manager.pending).not.toBeNull();
  expect(f.manager.commandError).toContain("Restart the local service");
  await f.manager.install(f.value.runtimes[0]);
  expect(f.request.mock.calls.filter((c) => c[1])).toHaveLength(1);
  f.manager.destroy();
});
test("lost install response retains the exact request and recovery never resends", async () => {
  const f = fixture();
  await f.manager.refresh();
  f.request.mockImplementationOnce(async (_path, body) => {
    f.outcome = {
      version: 1,
      request_id: body!.request_id,
      runtime_id: "codex",
      state: "installing",
      error: null,
    };
    throw new TypeError("response lost");
  });
  await f.manager.install(f.value.runtimes[0]);
  const id = f.manager.pending;
  f.manager.stop();
  await f.manager.checkOutcome();
  expect(f.manager.pending).toBe(id);
  expect(f.request.mock.calls.filter((c) => c[1])).toHaveLength(1);
  f.outcome = {
    ...f.outcome!,
    state: "interrupted",
    error: "installation_interrupted_check_again",
  };
  await f.manager.refresh();
  expect(f.manager.pending).toBeNull();
  expect(f.manager.commandError).toContain(
    "explicitly start a new installation",
  );
  f.manager.destroy();
});
test("service refusals and foreign receipts preserve separate request state", async () => {
  const f = fixture();
  await f.manager.refresh();
  f.request.mockRejectedValueOnce(new ApiError("agent_execution_busy"));
  await f.manager.install(f.value.runtimes[0]);
  expect(f.manager.pending).toBeNull();
  expect(f.manager.commandError).toContain("End the connected agent");
  f.request.mockImplementationOnce(async () => ({
    version: 1,
    request_id: "ff".repeat(16),
    runtime_id: "codex",
    state: "completed",
    error: null,
  }));
  await f.manager.install(f.value.runtimes[0]);
  expect(f.manager.pending).not.toBeNull();
  expect(f.manager.outcome).toBeNull();
  f.manager.destroy();
});
test("read adopts another client's durable installation and teardown discards late responses", async () => {
  const f = fixture();
  f.value.pending = {
    version: 1,
    request_id: "ab".repeat(16),
    runtime_id: "codex",
    state: "installing",
    error: null,
  };
  await f.manager.refresh();
  expect(f.manager.pending).toBe("ab".repeat(16));
  let resolve!: (v: RuntimeStatus) => void;
  f.manager.pending = null;
  f.request.mockImplementationOnce(
    () =>
      new Promise<RuntimeStatus>((r) => {
        resolve = r;
      }),
  );
  const reading = f.manager.refresh();
  f.manager.destroy();
  resolve({ ...f.value, node_version: "99.0.0" });
  await reading;
  expect(f.manager.value?.node_version).toBe("24.18.0");
});

import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";
import { waitForRuntime } from "../../src/lib/ui/store-agent-execution/runtime-preparation.js";
import { prepareStandaloneRuntime } from "../../src/lib/conversations/prepare-standalone-runtime.js";

const wanted = { model: "saved-model", effort: "medium", mode: "ask" };
function fixture(state = "dispatching", request = "exact-request") {
  const manager = new AgentExecution(() => true, vi.fn());
  manager.selected = "session";
  manager.value = {
    version: 1,
    runtime: "ready",
    revision: 0,
    setups: [],
    runners: [],
    sessions: [
      {
        id: "session",
        connected: true,
        preferences: wanted,
        capabilities: { modeReady: true },
      } as AgentSession,
    ],
  };
  manager.pending = {
    path: "agents/command",
    body: {
      request_id: "exact-request",
      session_id: "session",
      action: { action: "configure", preferences: wanted },
    },
  };
  manager.outcome = {
    version: 1,
    request_id: request,
    session_id: "session",
    state,
    error: null,
  };
  return manager;
}

test("old ready capabilities cannot bypass a pending exact configure receipt", async () => {
  vi.useFakeTimers();
  const manager = fixture();
  const command = vi.spyOn(manager, "commandSession");
  const refresh = vi.spyOn(manager, "refresh");
  try {
    const waiting = waitForRuntime(
      manager,
      "session",
      "exact-request",
      wanted,
      () => true,
    );
    await vi.advanceTimersByTimeAsync(250);
    expect(manager.pending?.body.request_id).toBe("exact-request");
    manager.pending = null;
    manager.outcome!.state = "accepted";
    manager.runtimeObservation.notify();
    await vi.advanceTimersByTimeAsync(250);
    expect(await waiting).toBe(true);
    expect(refresh).not.toHaveBeenCalled();
    expect(command).not.toHaveBeenCalled();
  } finally {
    manager.destroy();
    vi.useRealTimers();
  }
});

test("Send prepares a disconnected session once and observes readiness without dispatching a prompt", async () => {
  vi.useFakeTimers();
  const manager = fixture("opening");
  manager.pending = null;
  manager.session!.connected = false;
  manager.session!.state = "interrupted";
  const command = vi
    .spyOn(manager, "commandSession")
    .mockImplementation(async () => {
      manager.outcome!.request_id = "open-request";
      manager.session!.state = "opening";
      return "open-request";
    });
  const refresh = vi.spyOn(manager, "refresh");
  try {
    const prepared = prepareStandaloneRuntime(manager, "session", () => true);
    await vi.advanceTimersByTimeAsync(500);
    expect(command).toHaveBeenCalledExactlyOnceWith("session", {
      action: "resume",
    });
    manager.session!.connected = true;
    manager.session!.state = "idle";
    manager.runtimeObservation.notify();
    await vi.advanceTimersByTimeAsync(1);
    expect(await prepared).toBe(true);
    expect(refresh).not.toHaveBeenCalled();
    expect(await prepareStandaloneRuntime(manager, "session", () => true)).toBe(
      true,
    );
    expect(command).toHaveBeenCalledTimes(1);
  } finally {
    manager.destroy();
    vi.useRealTimers();
  }
});

test.each(["failed", "outcome_unknown"])(
  "%s automatic preparation never sends or falls back",
  async (state) => {
    const manager = fixture(state);
    manager.pending = null;
    manager.session!.connected = false;
    manager.session!.state = "interrupted";
    const command = vi
      .spyOn(manager, "commandSession")
      .mockResolvedValue("exact-request");
    try {
      expect(
        await prepareStandaloneRuntime(manager, "session", () => true),
      ).toBe(false);
      expect(command).toHaveBeenCalledExactlyOnceWith("session", {
        action: "resume",
      });
    } finally {
      manager.destroy();
    }
  },
);

test("changing selection during opening does not retarget the captured submission", async () => {
  vi.useFakeTimers();
  const manager = fixture("opening");
  manager.pending = null;
  manager.session!.connected = false;
  const command = vi
    .spyOn(manager, "commandSession")
    .mockResolvedValue("exact-request");
  try {
    const preparing = prepareStandaloneRuntime(manager, "session", () => true);
    await vi.advanceTimersByTimeAsync(1);
    manager.selected = "other-session";
    manager.runtimeObservation.notify();
    await vi.advanceTimersByTimeAsync(1);
    expect(await preparing).toBe(false);
    expect(command).toHaveBeenCalledTimes(1);
  } finally {
    manager.destroy();
    vi.useRealTimers();
  }
});

test.each(["failed", "outcome_unknown", "not_known"])(
  "%s configuration never grants prompt readiness or changes the pending identity",
  async (state) => {
    const manager = fixture(state);
    const refresh = vi.spyOn(manager, "refresh");
    try {
      expect(
        await waitForRuntime(
          manager,
          "session",
          "exact-request",
          wanted,
          () => true,
        ),
      ).toBe(false);
      expect(manager.pending?.body.request_id).toBe("exact-request");
      expect(refresh).not.toHaveBeenCalled();
    } finally {
      manager.destroy();
    }
  },
);

test("accepted unrelated receipt cannot authorize the captured prompt", async () => {
  const manager = fixture("accepted", "other-request");
  try {
    expect(
      await waitForRuntime(
        manager,
        "session",
        "exact-request",
        wanted,
        () => true,
      ),
    ).toBe(false);
  } finally {
    manager.destroy();
  }
});

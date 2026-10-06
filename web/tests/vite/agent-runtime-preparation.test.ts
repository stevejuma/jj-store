import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";
import { waitForRuntime } from "../../src/lib/ui/store-agent-execution/runtime-preparation.js";

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
  let checks = 0;
  const refresh = vi.spyOn(manager, "refresh").mockImplementation(async () => {
    if (++checks === 2) {
      manager.pending = null;
      manager.outcome!.state = "accepted";
    }
  });
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
    await vi.advanceTimersByTimeAsync(250);
    expect(await waiting).toBe(true);
    expect(refresh).toHaveBeenCalledTimes(2);
    expect(command).not.toHaveBeenCalled();
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

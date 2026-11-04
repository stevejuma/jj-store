import { describe, expect, it, vi } from "vitest";
const { invoke } = vi.hoisted(() => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));
vi.mock("@tauri-apps/api/core", () => ({
  invoke,
  Channel: class {
    constructor(readonly onmessage: unknown) {}
  },
}));
import { createNativeHost } from "../../src/lib/host/tauri.js";

describe("native host adapter", () => {
  it("retains the named commands, exact terminal identity and channel payloads", async () => {
    const host = createNativeHost({
      version: 1,
      platform: "macos",
      terminal: 1,
    });
    expect(host.windowDragging).toBe(true);
    await host.setGlassBackground!(true);
    expect(invoke).toHaveBeenLastCalledWith("set_glass_background", {
      enabled: true,
    });
    await host.agentPower!(true, 1234);
    expect(invoke).toHaveBeenLastCalledWith("agent_power", {
      active: true,
      activity: 1234,
    });
    await host.terminal!.open("same-request", "repo", "native", "folder");
    expect(invoke).toHaveBeenLastCalledWith("terminal_open", {
      requestId: "same-request",
      repository: "repo",
      profile: "native",
      targetId: "folder",
    });
    const receive = vi.fn();
    await host.terminal!.attach("session", receive);
    expect(invoke).toHaveBeenLastCalledWith("terminal_attach", {
      sessionId: "session",
      onFrame: expect.objectContaining({ onmessage: receive }),
    });
    await host.terminal!.ack("session", "subscription", 2);
    expect(invoke).toHaveBeenLastCalledWith("terminal_ack", {
      sessionId: "session",
      subscriptionId: "subscription",
      sequence: 2,
    });
    await host.terminal!.detach("session", "subscription");
    expect(invoke).toHaveBeenLastCalledWith("terminal_detach", {
      sessionId: "session",
      subscriptionId: "subscription",
    });
  });

  it("omits terminals unless the supported terminal extension is advertised", () => {
    expect(
      createNativeHost({ version: 1, platform: "macos" }).terminal,
    ).toBeUndefined();
    expect(
      createNativeHost({ version: 1, platform: "macos", terminal: 2 }).terminal,
    ).toBeUndefined();
  });
});

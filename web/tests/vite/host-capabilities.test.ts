import { describe, expect, it, vi } from "vitest";
import type { AgentStatus } from "../../src/lib/controller/agent-execution-types.js";
import { browserHost } from "../../src/lib/host/contracts.js";
import { syncAgentPower } from "../../src/lib/controller/agent-power.js";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import { attachNativeGlass } from "../../src/lib/appearance/desktop-glass.js";

describe("injected host capabilities", () => {
  it("never touches native glass or power for either browser presentation", async () => {
    for (const host of [browserHost, { presentation: "macos" as const }]) {
      // No DOM is needed when the native capability is absent.
      expect(attachNativeGlass(undefined!, vi.fn(), host)).toBeUndefined();
      await syncAgentPower(null, true, host);
    }
  });

  it("uses an explicitly supplied power capability and releases it on owner disposal", async () => {
    const agentPower = vi.fn().mockResolvedValue(undefined);
    const host = { presentation: "macos" as const, agentPower };
    const manager = new AgentExecution(
      () => false,
      undefined,
      undefined,
      undefined,
      undefined,
      host,
    );
    expect(manager.host).toBe(host);
    const status = {
      live_agents: [{ state: "idle" }],
      sessions: [
        { last_activity_at_millis: 1234 },
        { last_activity_at_millis: 2345 },
      ],
    } as AgentStatus;
    await syncAgentPower(status, true, host);
    expect(agentPower).toHaveBeenLastCalledWith(true, 2345);
    await syncAgentPower(status, false, host);
    expect(agentPower).toHaveBeenLastCalledWith(false, 2345);
    await syncAgentPower(null, true, host);
    expect(agentPower).toHaveBeenLastCalledWith(false, 0);
    manager.destroy();
    expect(agentPower).toHaveBeenLastCalledWith(false, 0);
    agentPower.mockRejectedValueOnce(Error("unsupported host"));
    await expect(syncAgentPower(null, false, host)).resolves.toBeUndefined();
  });
});

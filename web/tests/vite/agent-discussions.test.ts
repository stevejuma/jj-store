import { expect, test, vi } from "vitest";
import { watchAgentDiscussion } from "../../src/lib/controller/agent-discussion-refresh.js";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import { agentDiscussionFixture } from "../../../catalog/fixtures/agent-discussion-fixture.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";

test("an ended agent accepts ordinary replies after feedback refresh fails", async () => {
  const fixture = agentDiscussionFixture(true);
  let fail = false;
  const paths: string[] = [];
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      const path = String(input);
      paths.push(path);
      if (fail && path.includes("/feedback"))
        throw new Error("Feedback offline");
      return (await fixture(path, init)) ?? new Response("{}", { status: 503 });
    },
  );
  const id = "44".repeat(16),
    row = `${"55".repeat(16)}/assistant`;
  manager.value = {
    version: 1,
    revision: 0,
    runtime: "offline",
    runners: [],
    setups: [],
    sessions: [
      {
        id,
        repository: "11".repeat(16),
        title: "Stopped",
        connected: false,
        state: "ended",
      } as AgentSession,
    ],
  };
  try {
    await manager.discussions.load(id);
    fail = true;
    await manager.discussions.load(id);
    expect(manager.discussions.errors[id]).toBe("Feedback offline");
    manager.discussions.drafts[`${id}:${row}`] = "Keep the normal reply flow";
    await manager.discussions.save(id, row, "Keep the normal reply flow");
    expect(manager.discussions.drafts[`${id}:${row}`]).toBe("");
    fail = false;
    await manager.discussions.load(id);
    expect(manager.discussions.message(id, row)?.replies[0]?.message.body).toBe(
      "Keep the normal reply flow",
    );
    expect(paths.some((p) => /agents\/(command|launch)/.test(p))).toBe(false);
  } finally {
    manager.destroy();
  }
});

test("discussion refresh runs independently and stops when the view closes", async () => {
  vi.useFakeTimers();
  const load = vi.fn(async () => {});
  const stop = watchAgentDiscussion(load, "repository");
  try {
    await vi.advanceTimersByTimeAsync(160);
    expect(load).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(2000);
    expect(load).toHaveBeenCalledTimes(2);
    stop();
    await vi.advanceTimersByTimeAsync(10000);
    expect(load).toHaveBeenCalledTimes(2);
  } finally {
    stop();
    vi.useRealTimers();
  }
});

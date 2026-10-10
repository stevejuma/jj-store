import { expect, test } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";

test("Stop bypasses a pending Send without overwriting it, its draft, or another selected session", async () => {
  const posted: Record<string, unknown>[] = [];
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      if (init?.method === "POST") {
        const body = JSON.parse(String(init.body));
        posted.push(body);
        return Response.json({
          version: 1,
          request_id: body.request_id,
          session_id: body.session_id,
          state: "accepted",
          error: null,
        });
      }
      return Response.json({
        version: 1,
        runtime: "ready",
        revision: 1,
        setups: [],
        sessions: [],
        runners: [],
      });
    },
  );
  try {
    manager.selected = "other";
    manager.busy = true;
    const pending = {
      path: "agents/command",
      body: { request_id: "original", session_id: "target" },
      draft: "target",
    };
    manager.pending = pending;
    manager.drafts.target = "Keep this draft";
    manager.checkOutcome = async () => {};
    await manager.stops.stop("target", {
      action: "cancel",
      target: "11".repeat(16),
    });
    expect(posted).toHaveLength(1);
    expect(posted[0]).toMatchObject({
      session_id: "target",
      action: { action: "cancel", target: "11".repeat(16) },
    });
    expect(manager.pending?.body.request_id).toBe("original");
    expect(manager.busy).toBe(true);
    expect(manager.selected).toBe("other");
    expect(manager.drafts.target).toBe("Keep this draft");
    expect(manager.stops.requests.target).toBeUndefined();
  } finally {
    manager.destroy();
  }
});

test("an uncertain Stop retains one identity; a second click only checks its outcome", async () => {
  const posts: string[] = [],
    reads: string[] = [];
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      if (init?.method === "POST") {
        posts.push(String(init.body));
        throw new TypeError("connection lost");
      }
      const url = String(input);
      if (url.includes("/outcome")) {
        reads.push(url);
        return Response.json({
          version: 1,
          request_id: new URL(url, "http://localhost").searchParams.get(
            "request",
          ),
          session_id: "target",
          state: "outcome_unknown",
          error: "connection_lost",
        });
      }
      return Response.json({
        version: 1,
        runtime: "ready",
        revision: 1,
        setups: [],
        sessions: [],
        runners: [],
      });
    },
  );
  try {
    await manager.stops.stop("target", { action: "end" });
    const id = manager.stops.requests.target.body.request_id;
    await manager.stops.stop("target", { action: "end" });
    expect(posts).toHaveLength(1);
    expect(manager.stops.requests.target.body.request_id).toBe(id);
    expect(manager.stops.requests.target.busy).toBe(false);
    expect(reads.every((url) => url.endsWith(String(id)))).toBe(true);
    expect(manager.stops.requests.target.error).toMatch(/unknown/);
  } finally {
    manager.destroy();
  }
});

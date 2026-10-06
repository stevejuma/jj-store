import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";

test("Activity log opens the associated active chat, retains exact historical links and never launches", () => {
  const fetcher = vi.fn(),
    navigate = vi.fn();
  const manager = new AgentExecution(() => true, fetcher);
  const session = (
    id: string,
    state: string,
    connected: boolean,
    setup_id = "helper",
  ) => ({ id, state, connected, setup_id }) as AgentSession;
  try {
    manager.onNavigate = navigate;
    manager.value = {
      version: 1,
      runtime: "ready",
      revision: 0,
      setups: [],
      runners: [],
      sessions: [
        session("foreign", "running", true, "other"),
        session("ended", "ended", false),
        session("idle", "idle", true),
        session("running", "running", true),
      ],
    };
    manager.drafts.running = "Keep this chat";
    manager.viewActivity("helper");
    expect(navigate).toHaveBeenLastCalledWith("session", "running", "helper");
    manager.viewActivity("helper", "ended");
    expect(navigate).toHaveBeenLastCalledWith("session", "ended", "helper");
    manager.value.sessions = [session("ended", "ended", false)];
    manager.viewActivity("helper");
    expect(navigate).toHaveBeenLastCalledWith("sessions", "helper");
    expect(manager.drafts.running).toBe("Keep this chat");
    expect(fetcher).not.toHaveBeenCalled();
  } finally {
    manager.destroy();
  }
});

test("stopping an agent targets its session without changing another chat or replaying a prompt", async () => {
  const commands: Record<string, unknown>[] = [];
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      if (String(input).endsWith("/command")) {
        const body = JSON.parse(String(init?.body));
        commands.push(body);
        return new Response(
          JSON.stringify({
            version: 1,
            request_id: body.request_id,
            session_id: body.session_id,
            state: "completed",
            error: null,
          }),
        );
      }
      return new Response(
        JSON.stringify(
          String(input).includes("/targets")
            ? []
            : {
                version: 1,
                runtime: "ready",
                revision: 1,
                setups: [],
                sessions: [],
                runners: [],
              },
        ),
      );
    },
  );
  try {
    manager.selected = "11".repeat(16);
    manager.drafts[manager.selected] = "Retained chat draft";
    await manager.commandSession("22".repeat(16), { action: "end" });
    expect(commands).toHaveLength(1);
    expect(commands[0]).toMatchObject({
      session_id: "22".repeat(16),
      action: { action: "end" },
    });
    expect(manager.selected).toBe("11".repeat(16));
    expect(manager.drafts[manager.selected]).toBe("Retained chat draft");
  } finally {
    manager.destroy();
  }
});

test("activity reconnect preserves explicit command errors and does not dispatch", async () => {
  vi.useFakeTimers();
  const paths: string[] = [];
  let streams = 0;
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      const path = String(input);
      paths.push(path);
      if (path.includes("/stream")) {
        if (++streams === 1) throw new TypeError("Stream connection lost");
        return new Response(
          new ReadableStream({
            start(controller) {
              controller.enqueue(
                new TextEncoder().encode(
                  `data: ${JSON.stringify({
                    version: 1,
                    next: null,
                    events: [
                      {
                        sequence: 1,
                        committed: false,
                        payload: { type: "ready" },
                      },
                    ],
                  })}\n\n`,
                ),
              );
              init?.signal?.addEventListener(
                "abort",
                () => controller.close(),
                { once: true },
              );
            },
          }),
          { headers: { "content-type": "text/event-stream" } },
        );
      }
      return new Response(
        JSON.stringify({ version: 1, events: [], next: null }),
        {
          headers: { "content-type": "application/json" },
        },
      );
    },
  );
  try {
    manager.error = "Retained command refusal";
    await manager.select("11".repeat(16));
    await vi.advanceTimersByTimeAsync(0);
    expect(streams).toBe(1);
    expect(manager.error).toBe("Retained command refusal");
    expect(manager.activityError).toBe("Stream connection lost");
    await vi.advanceTimersByTimeAsync(1000);
    expect(streams).toBe(2);
    expect(manager.events.map((e) => e.payload)).toEqual([{ type: "ready" }]);
    expect(manager.error).toBe("Retained command refusal");
    expect(manager.activityError).toBe("");
    expect(
      paths.every(
        (path) =>
          path.includes("/events?") ||
          path.includes("/stream?") ||
          path.includes("/feedback?"),
      ),
    ).toBe(true);
  } finally {
    manager.destroy();
    vi.useRealTimers();
  }
});

test("conditional setup save returns its acknowledged revision even if the following refresh advances", async () => {
  const status = (revision: number) => ({
    version: 1,
    runtime: "ready",
    revision,
    setups: [],
    sessions: [],
    runners: [],
  });
  const manager = new AgentExecution(
    () => true,
    async (url) =>
      new Response(
        JSON.stringify(
          String(url).endsWith("/setups")
            ? status(5)
            : String(url).includes("/targets")
              ? []
              : status(6),
        ),
        { headers: { "content-type": "application/json" } },
      ),
  );
  try {
    const acknowledged = await manager.saveSetup(
      {
        id: "helper",
        name: "Helper",
        avatar: "",
        description: "",
        instructions: "",
        runner: "codex",
        preferences: { model: null, effort: null, mode: "ask" },
      },
      undefined,
      4,
    );
    expect(acknowledged).toBe(5);
    expect(manager.value?.revision).toBe(6);
  } finally {
    manager.destroy();
  }
});

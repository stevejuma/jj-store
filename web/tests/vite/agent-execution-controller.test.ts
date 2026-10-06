import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";

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
        (path) => path.includes("/events?") || path.includes("/stream?"),
      ),
    ).toBe(true);
  } finally {
    manager.destroy();
    vi.useRealTimers();
  }
});

import { ApplicationFreshness } from "../../web/src/lib/controller/application-freshness.js";

/** Exercise production feed delivery instead of making receipts progress on reads. */
export function agentFreshnessFixture() {
  let stream: ReadableStreamDefaultController<Uint8Array> | undefined;
  let sequence = 0;
  const emit = (type: string, body: unknown) =>
    stream?.enqueue(
      new TextEncoder().encode(
        `id: jse1:${"1".repeat(32)}:${"2".repeat(32)}:${sequence++}\nevent: ${type}\ndata: ${JSON.stringify(body)}\n\n`,
      ),
    );
  const freshness = new ApplicationFreshness(
    async (_input, init) =>
      new Response(
        new ReadableStream({
          start(controller) {
            stream = controller;
            emit("ready", { version: 1, type: "ready", witnesses: [] });
            init?.signal?.addEventListener(
              "abort",
              () => {
                stream = undefined;
                controller.close();
              },
              { once: true },
            );
          },
        }),
        { headers: { "content-type": "text/event-stream" } },
      ),
  );
  void freshness.configure(true, null);
  return {
    freshness,
    changed: () =>
      emit("changed", {
        version: 1,
        type: "changed",
        change: {
          repository: null,
          domain: "agent_execution",
          witness: String(sequence),
        },
      }),
  };
}

import { decode, type Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
import type { AgentEvent } from "./agent-execution-types.js";
/** Reconnect reads only retained activity. It never dispatches or clears command refusals. */
export async function consumeAgentEvents(
  client: LocalClient,
  id: string,
  after: number,
  signal: AbortSignal,
  owner: {
    alive(): boolean;
    receive(events: AgentEvent[]): void;
    failed(message: string): void;
  },
) {
  while (owner.alive() && !signal.aborted) {
    try {
      const response = await client.fetch(
        `agents/stream?session=${encodeURIComponent(id)}&after=${after}`,
        { signal },
      );
      if (
        !response.ok ||
        !response.headers
          .get("content-type")
          ?.startsWith("text/event-stream") ||
        !response.body
      )
        throw new Error("Agent activity unavailable");
      const reader = response.body.getReader(),
        decoder = new TextDecoder();
      let buffer = "";
      try {
        while (!signal.aborted) {
          const next = await reader.read();
          if (next.done) break;
          buffer += decoder.decode(next.value, { stream: true });
          if (buffer.length > 16 * 1024 * 1024)
            throw new Error("Agent activity limit");
          let end: number;
          while ((end = buffer.indexOf("\n\n")) >= 0) {
            const frame = buffer.slice(0, end);
            buffer = buffer.slice(end + 2);
            const data = frame
              .split("\n")
              .filter((line) => line.startsWith("data:"))
              .map((line) => line.slice(5).trimStart())
              .join("\n");
            if (!data) continue;
            const page = decode(data) as Model<"AgentExecutionEventPage">;
            const added = page.events.filter(
              (event) => Number(event.sequence) > after,
            );
            if (added.length) after = Number(added.at(-1)!.sequence);
            if (!signal.aborted && owner.alive()) owner.receive(added);
          }
        }
      } finally {
        await reader.cancel().catch(() => {});
        reader.releaseLock();
      }
    } catch (cause) {
      if (!signal.aborted && owner.alive())
        owner.failed(
          cause instanceof Error
            ? cause.message
            : "Agent activity reconnecting",
        );
    }
    if (!signal.aborted)
      await new Promise<void>((resolve) => {
        const done = () => {
          clearTimeout(timer);
          signal.removeEventListener("abort", done);
          resolve();
        };
        const timer = setTimeout(done, 1000);
        signal.addEventListener("abort", done, { once: true });
      });
  }
}

export async function readAgentEventHistory(
  client: LocalClient,
  id: string,
  signal: AbortSignal,
  receive: (events: AgentEvent[]) => void,
) {
  let after = 0;
  do {
    const page = await client.request<Model<"AgentExecutionEventPage">>(
      `agents/events?session=${id}&after=${after}`,
      undefined,
      signal,
    );
    if (signal.aborted) break;
    receive(page.events);
    after = Number(page.events.at(-1)?.sequence ?? after);
    if (page.next === null || page.next === undefined) break;
  } while (!signal.aborted);
  return after;
}

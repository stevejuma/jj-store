import type { AgentChat } from "./agent-chat.svelte.js";
import { object, type AgentOutcome } from "./agent-execution-types.js";

export const acknowledged = (state: string) =>
  ["queued", "started", "running", "completed"].includes(state);

/** Live events trigger receipt reads, never another dispatch. */
export async function refreshChatReceipt(chat: AgentChat) {
  const pending = chat.pending;
  const receipt = pending && chat.receipts[pending.request_id];
  if (!pending || !receipt) return;
  const agents = await Promise.all(
    receipt.agents.map(async (entry) => {
      if (entry.state !== "dispatching" || !entry.session_id) return entry;
      const events = chat.events[entry.session_id] ?? [];
      const session = chat.manager.value?.sessions.find(
        (s) => s.id === entry.session_id,
      );
      const observed = events.some(
        (e) =>
          object(e.payload).requestId === entry.request_id &&
          ["queued", "started", "record", "error"].includes(
            String(object(e.payload).type),
          ),
      );
      if (!observed && session?.state !== "interrupted") return entry;
      const outcome = await chat.manager.client.request<AgentOutcome>(
        `agents/outcome?request=${entry.request_id}`,
      );
      return outcome.request_id === entry.request_id &&
        outcome.session_id === entry.session_id
        ? outcome
        : entry;
    }),
  );
  if (chat.pending?.request_id !== pending.request_id) return;
  chat.receipts[pending.request_id] = { ...receipt, agents };
  chat.outcomes = agents;
  if (agents.every((entry) => acknowledged(entry.state))) {
    chat.pending = null;
    chat.error = "";
  }
}

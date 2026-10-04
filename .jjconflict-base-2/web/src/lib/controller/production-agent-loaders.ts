import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export function createProductionAgentLoaders(context: {
  data: ProductionStoreState;
  source(): StoreDataSource;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
}) {
  const { data } = context;
  async function loadAgents(more = false) {
    const source = context.source(),
      ticket = context.version();
    data.status.agents = more ? "refreshing" : "loading";
    try {
      const page = await source.agentSessions(
        more ? (data.agentCursor ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.agentSessions = more
        ? [...data.agentSessions, ...page.sessions]
        : [...page.sessions];
      data.agentCursor = page.next_cursor;
      data.status.agents = data.agentSessions.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.agents = "failed";
      context.report(cause);
    }
  }

  async function loadTranscript(more = false) {
    const descriptor = data.transcriptDescriptor;
    const session = data.agentSession?.session.session_id;
    if (!descriptor || !session) return;
    const source = context.source(),
      ticket = context.version();
    data.status.transcript = more ? "refreshing" : "loading";
    try {
      const page = await source.agentTranscript(
        session,
        descriptor,
        more ? (data.transcriptCursor ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.agentTranscript = more
        ? [...data.agentTranscript, ...page.chunks]
        : [...page.chunks];
      data.transcriptCursor = page.next_cursor;
      data.status.transcript = data.agentTranscript.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.transcript = "failed";
      context.report(cause);
    }
  }

  async function loadAgent(id: string) {
    const source = context.source(),
      ticket = context.version();
    data.status.transcript = "loading";
    try {
      const session = await source.agentSession(id);
      if (!context.current(ticket, source)) return;
      data.agentSession = session;
      data.transcriptDescriptor = session.transcripts[0];
      data.agentTranscript = [];
      data.transcriptCursor = null;
      if (data.transcriptDescriptor) await loadTranscript();
      else data.status.transcript = "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.transcript = "failed";
      context.report(cause);
    }
  }

  return { loadAgents, loadTranscript, loadAgent };
}

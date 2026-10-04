import { readQuery } from "./read-query.js";
import type { QueryClient } from "@tanstack/svelte-query";
import { Client, fullId, pretty, type Model } from "../api/client.js";
import type { StoreReviewCommandRequest } from "./native-data-source.js";
import { storeQueryKeys, type StoreQueryScope } from "./store-query-keys.js";

export class HttpCollaborationDataSource {
  constructor(
    private client: Client,
    private queryClient: QueryClient,
    private scope: StoreQueryScope,
    private active: () => boolean,
  ) {}

  private query<T>(
    kind: string,
    query: Record<string, string | number | bigint | undefined>,
    read: (signal: AbortSignal) => Promise<T>,
  ): Promise<T> {
    return readQuery(this.queryClient, {
      queryKey: storeQueryKeys.resource(this.scope, kind, null, query),
      queryFn: async ({ signal }) => {
        const value = await read(signal);
        if (!this.active()) throw new DOMException("Closed", "AbortError");
        return value;
      },
    });
  }

  agentSessions(cursor?: string) {
    const query = { cursor, limit: 50 };
    return this.query<Model<"AgentSessionPage">>(
      "agent-sessions",
      query,
      (signal) => this.client.request("agent/sessions", query, { signal }),
    );
  }

  agentSession(id: string) {
    return this.query<Model<"AgentSessionDetail">>(
      "agent-session",
      { id },
      (signal) =>
        this.client.request(`agent/sessions/${fullId(id, 32)}`, {}, { signal }),
    );
  }

  agentRelations(
    subject: { commit: string } | { operation: string },
    cursor?: string,
  ) {
    const query = {
      ...("commit" in subject ? { commit: fullId(subject.commit) } : {}),
      ...("operation" in subject
        ? { operation: fullId(subject.operation) }
        : {}),
      cursor,
      limit: 256,
    };
    return this.query<Model<"AgentRelationPage">>(
      "agent-relations",
      query,
      (signal) => this.client.request("agent/associations", query, { signal }),
    );
  }

  agentTranscript(
    session: string,
    transcript: Model<"AgentTranscriptDescriptor">,
    cursor?: string,
  ) {
    const query = {
      source: transcript.source_instance_id,
      generation: transcript.source_generation,
      cursor,
      limit: 50,
      max_bytes: 262_144,
    };
    return this.query<Model<"AgentTranscriptPage">>(
      "agent-transcript",
      { session, transcript: transcript.transcript_id, ...query },
      (signal) =>
        this.client.request(
          `agent/sessions/${fullId(session, 32)}/transcripts/${fullId(transcript.transcript_id, 64)}`,
          query,
          { signal },
        ),
    );
  }

  reviewThreads(cursor?: string) {
    const query = { cursor, limit: 50 };
    return this.query<Model<"ReviewThreadPage">>(
      "review-threads",
      query,
      (signal) => this.client.request("reviews/threads", query, { signal }),
    );
  }

  reviewThread(id: string) {
    return this.query<Model<"ReviewThreadDetail">>(
      "review-thread",
      { id },
      (signal) =>
        this.client.request(
          `reviews/threads/${fullId(id, 32)}`,
          {},
          { signal },
        ),
    );
  }

  async applyReviewCommand(request: StoreReviewCommandRequest) {
    const result = await this.client.request<Model<"ReviewCommandOutcome">>(
      "reviews/commands",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
    if (result.status === "committed")
      await this.queryClient.invalidateQueries({
        predicate: (q) =>
          storeQueryKeys
            .root(this.scope)
            .every((v, i) => q.queryKey[i] === v) &&
          String(q.queryKey[4]).startsWith("review-"),
        refetchType: "none",
      });
    return result;
  }

  reviewCommandStatus(id: string) {
    return this.client.request<Model<"ReviewCommandStatus">>(
      `reviews/commands/${fullId(id, 32)}`,
    );
  }
}

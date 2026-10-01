import { Client, fullId, pretty, type Model } from "../api/client.js";
import type {
  ThreadTransport,
  ThreadRequest,
  ReactionRequest,
  ReactionTarget,
} from "./thread-types.js";
export class HttpThreadsDataSource implements ThreadTransport {
  constructor(private client: Client) {}
  list(cursor?: string) {
    return this.client.request<Model<"CollabPage">>("threads", {
      limit: 50,
      cursor,
    });
  }
  thread(id: string) {
    return this.client.request<Model<"ThreadDetailV1">>(
      `threads/${fullId(id, 32)}`,
    );
  }
  events(id: string, cursor?: string) {
    return this.client.request<Model<"CollabPage">>(
      `threads/${fullId(id, 32)}/timeline`,
      { limit: 50, cursor },
    );
  }
  emojis(cursor?: string) {
    return this.client.request<Model<"CollabPage">>("reaction-emojis", {
      limit: 50,
      cursor,
    });
  }
  reactions(target: ReactionTarget, cursor?: string) {
    return this.client.request<Model<"CollabPage">>(
      "reactions/query",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty({ target, limit: 50, ...(cursor ? { cursor } : {}) }),
      },
    );
  }
  asset(id: string) {
    return this.client.asset(id);
  }
  command(
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ) {
    return this.client.request<Model<"CollabCommandOutcome">>(
      `${kind}/commands`,
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(request),
      },
    );
  }
  status(kind: "threads" | "reactions", id: string) {
    return this.client.request<Model<"CollabCommandStatus">>(
      `${kind}/commands/${fullId(id, 32)}`,
    );
  }
}

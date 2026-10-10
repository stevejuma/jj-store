import { applicationEventTransport } from "./application-events-websocket.js";
import { ApiError } from "./api-error.js";
import {
  assertApplicationContinuity,
  type ApplicationEvent,
  type FreshnessChange,
} from "./application-events.js";

type Subscriber = {
  deliver(event: ApplicationEvent, id: string): void;
  end(error?: string): void;
};
type Group = {
  abort: AbortController;
  subscribers: Set<Subscriber>;
  witnesses: Map<string, FreshnessChange>;
  cursor?: string;
  resetAt?: string;
};

/** Memory-only, credential-scoped fanout. It owns hints, never repository data. */
export class ApplicationEventHub {
  private groups = new Map<string | null, Group>();
  constructor(private fetcher?: typeof fetch) {}

  subscribe(
    token: string | null,
    after: string | undefined,
    subscriber: Subscriber,
  ) {
    let group = this.groups.get(token);
    if (!group) {
      group = {
        abort: new AbortController(),
        subscribers: new Set(),
        witnesses: new Map(),
      };
      this.groups.set(token, group);
      group.subscribers.add(subscriber);
      void this.follow(token, after, group);
    } else {
      group.subscribers.add(subscriber);
      if (group.cursor) {
        const reset =
          after !== undefined &&
          (after.split(":").slice(0, 3).join(":") !==
            group.cursor.split(":").slice(0, 3).join(":") ||
            BigInt(after.split(":")[3]!) >
              BigInt(group.cursor.split(":")[3]!) ||
            (group.resetAt !== undefined &&
              BigInt(after.split(":")[3]!) <
                BigInt(group.resetAt.split(":")[3]!)));
        subscriber.deliver(
          {
            version: 1,
            type: reset ? "reset" : "ready",
            ...(reset ? { reason: "shared_event_continuity_reset" } : {}),
            // A domain witness can cover several identities since this reader left.
            witnesses: [...group.witnesses.values()].map(
              ({ identity, ...change }) => change,
            ),
          } as ApplicationEvent,
          group.cursor,
        );
      }
    }
    const retained = group;
    return () => {
      retained.subscribers.delete(subscriber);
      if (!retained.subscribers.size) {
        if (this.groups.get(token) === retained) this.groups.delete(token);
        retained.abort.abort();
        retained.witnesses.clear();
      }
    };
  }

  private async follow(
    token: string | null,
    after: string | undefined,
    group: Group,
  ) {
    let error: string | undefined;
    try {
      await applicationEventTransport(
        token,
        after,
        group.abort.signal,
        (event, id) => {
          if (group.abort.signal.aborted) return;
          assertApplicationContinuity(group.cursor, event, id);
          const reset =
            event.type === "reset" ||
            (event.type === "changed" &&
              event.change.repository === null &&
              event.change.domain === "repository");
          if (event.type !== "changed" || reset) group.witnesses.clear();
          if (reset) group.resetAt = id;
          for (const change of event.type === "changed"
            ? [event.change]
            : event.witnesses) {
            group.witnesses.set(
              `${change.repository ?? "installation"}:${change.domain}`,
              change,
            );
          }
          if (group.witnesses.size > 4096)
            throw new ApiError("event_frame_limit");
          group.cursor = id;
          for (const subscriber of group.subscribers)
            subscriber.deliver(event, id);
        },
        this.fetcher,
      );
    } catch (cause) {
      error = cause instanceof ApiError ? cause.code : "event_unavailable";
    } finally {
      if (this.groups.get(token) === group) this.groups.delete(token);
      group.witnesses.clear();
      for (const subscriber of group.subscribers) subscriber.end(error);
      group.subscribers.clear();
    }
  }
}

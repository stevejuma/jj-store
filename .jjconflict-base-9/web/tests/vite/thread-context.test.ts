import { expect, test } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import { activityLinks } from "../../src/lib/ui/store-threads/thread-context.js";

test("capture gaps open the agent directory without a fictional session or review", () => {
  const thread = "11".repeat(16),
    actor = "22".repeat(16);
  const observed = { kind: "thread_version" as const, id: "33".repeat(32) };
  const event: Model<"ThreadEventItemV1"> = {
    reference: { kind: "thread_event", id: "44".repeat(32) },
    message: null,
    value: {
      schema_version: 1,
      id: "55".repeat(16),
      thread_id: thread,
      actor,
      accepted_at_millis: 1,
      previous: null,
      event: {
        type: "link",
        version: { kind: "thread_link", id: "66".repeat(32) },
      },
    },
    link: {
      schema_version: 1,
      id: "77".repeat(16),
      thread_id: thread,
      actor,
      accepted_at_millis: 1,
      previous: null,
      active: true,
      target: {
        type: "capture_gap",
        source_state: { kind: "source_state", id: "88".repeat(32) },
        event_batch: { kind: "event_batch", id: "99".repeat(32) },
        binding: {
          thread_id: thread,
          observed_thread: observed,
          observed_assignment: null,
        },
      },
    },
  };
  const route = {
    ...initialStoreRoute(),
    repository: "aa".repeat(16),
    page: "threads" as const,
    threadId: thread,
  };
  const links = activityLinks([event], route);
  expect(links).toHaveLength(1);
  expect(links[0].label).toBe("Capture coverage gap");
  expect(links[0].link.route).toMatchObject({
    page: "agents",
    agentSessionId: null,
    reviewThreadId: null,
  });
  expect(links[0].link.route.threadId ?? null).toBeNull();
  expect(
    activityLinks(
      [{ ...event, link: { ...event.link!, active: false } }],
      route,
    ),
  ).toEqual([]);
});

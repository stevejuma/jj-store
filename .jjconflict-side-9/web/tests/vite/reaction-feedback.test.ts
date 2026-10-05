import { expect, test } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
import { createThreadsController } from "../../src/lib/controller/production-threads.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import { ApiError } from "../../src/lib/api/client.js";

function fixture(unknownFirst = false) {
  const f = createThreadTransport({ unknownFirst });
  const controller = createThreadsController({
    source: () => f.transport,
    repository: () => THREAD_IDS.repo,
    principal: () => THREAD_IDS.actor,
    route: () => ({
      ...initialStoreRoute(),
      repository: THREAD_IDS.repo,
      page: "threads",
      threadId: THREAD_IDS.thread,
    }),
    select: () => {},
  });
  return { ...f, controller };
}

test("reaction uncertainty stays targeted and retries its original request", async () => {
  const f = fixture(true);
  await f.controller.restore();
  const target = f.controller.projection.targets[THREAD_IDS.message]!;
  await expect(f.controller.toggle(target, "👍")).rejects.toThrow("unknown");
  expect(f.controller.error).toBe("");
  expect(f.controller.reactionError(target)).toMatch(/unknown/);
  const request = f.controller.pending!.request;
  await f.controller.refresh();
  expect(f.controller.pending!.request).toEqual(request);
  f.controller.clear(true);
  await f.controller.restore();
  const originalStatus = f.transport.status;
  f.transport.status = async () => ({
    status: "not_known",
    request_id: request.request_id,
  });
  await f.controller.checkPending();
  expect(f.controller.pending!.request).toEqual(request);
  expect(f.controller.error).toBe("");
  f.transport.status = originalStatus;
  await f.controller.retryPending();
  expect(f.requests[1]).toEqual(request);
  expect(f.controller.pending).toBeNull();
  expect(f.controller.reactionError(target)).toBe("");
  await f.controller.loadReactions(target);
  expect(f.controller.reactionModels(target)[0]!.count).toBe(1);
  f.controller.clear();
});

test("definitive reaction rejection stays targeted while Thread failures retain page feedback", async () => {
  const f = fixture();
  await f.controller.restore();
  const target = f.controller.projection.targets[THREAD_IDS.message]!;
  f.transport.command = async () => {
    throw new ApiError("access_denied");
  };
  await expect(f.controller.toggle(target, "👍")).rejects.toThrow(
    "access denied",
  );
  expect(f.controller.pending).toBeNull();
  expect(f.controller.error).toBe("");
  expect(f.controller.reactionError(target)).toBe("access denied");
  expect(
    f.controller.reactionError({
      ...target,
      revision: { kind: "human_message_revision", id: "88".repeat(32) },
    } as typeof target),
  ).toBe("");
  await expect(f.controller.send("Rejected message")).rejects.toThrow(
    "access denied",
  );
  expect(f.controller.error).toBe("access denied");
  f.controller.clear();
});

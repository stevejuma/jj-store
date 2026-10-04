import { expect, test } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
import { createWorkReactions } from "../../src/lib/controller/work-reactions.svelte.js";
import type { ReactionTarget } from "../../src/lib/controller/thread-types.js";
const target: ReactionTarget = {
  type: "work_comment",
  task_id: "77".repeat(16),
  comment_id: "88".repeat(16),
  version: 1,
};
function home(unknownFirst = false) {
  const f = createThreadTransport({ unknownFirst });
  let changes = 0;
  const controller = createWorkReactions({
    source: () => f.transport,
    repository: () => THREAD_IDS.repo,
    principal: () => THREAD_IDS.actor,
    changed: () => {
      changes++;
    },
  });
  return {
    ...f,
    controller,
    get changes() {
      return changes;
    },
  };
}
test("Work comment reactions use each home's source, exact versions and observed slots", async () => {
  const a = home(),
    b = home();
  const invalidated: ReactionTarget[] = [];
  a.transport.invalidate = async (scope, selected) => {
    expect(scope).toBe("reactions");
    invalidated.push(selected!);
  };
  // Reaction actions must never read a Thread directory or mutate task data.
  a.transport.list = async () => {
    throw new Error("unexpected Thread read");
  };
  await a.controller.toggle(target, "👍");
  expect(a.changes).toBe(1);
  expect(a.controller.reactionModels(target)).toMatchObject([
    { count: 1, selected: true },
  ]);
  await b.controller.loadReactions(target);
  expect(b.controller.reactionModels(target)).toEqual([]);
  await a.controller.loadReactions({ ...target, version: 2 });
  expect(a.controller.reactionModels({ ...target, version: 2 })).toEqual([]);
  const unrelated = a.controller.getReaction({ ...target, version: 2 });
  await a.controller.toggle(target, "👍", undefined, true);
  expect(a.requests[1]!.command).toMatchObject({
    type: "clear",
    target,
    observed: [{ kind: "reaction" }],
  });
  expect(a.controller.reactionModels(target)).toEqual([]);
  expect(a.controller.getReaction({ ...target, version: 2 })).toBe(unrelated);
  expect(invalidated).toEqual([target, target]);
  a.controller.clear();
  b.controller.clear();
});
test("automatic invalidation preserves the original unknown reaction and confirmed retry", async () => {
  const f = home(true);
  await expect(f.controller.toggle(target, "🎉")).rejects.toThrow("unknown");
  const original = f.controller.pending!.request;
  f.controller.invalidate();
  expect(f.controller.pending!.request).toEqual(original);
  await f.controller.retryPending();
  expect(f.requests[1]).toEqual(original);
  expect(f.controller.pending).toBeNull();
  await f.controller.loadReactions(target);
  expect(f.controller.reactionModels(target)[0]!.count).toBe(1);
  expect(f.changes).toBe(1);
  f.controller.clear();
});
test("late reaction reads cannot repopulate a cleared home session", async () => {
  const f = home();
  const response = await f.transport.reactions(target);
  let resolve!: (v: typeof response) => void;
  f.transport.reactions = () =>
    new Promise((done) => {
      resolve = done;
    });
  const reading = f.controller.loadReactions(target);
  f.controller.clear();
  resolve(response);
  await reading;
  expect(f.controller.getReaction(target)).toBeUndefined();
});

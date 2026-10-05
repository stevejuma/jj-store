import { expect, test } from "vitest";
import { createFileReadQueue } from "../../src/lib/controller/file-read-queue.js";

test("optional metadata has one worker, a bounded queue, and coalesces pending requests", async () => {
  const queue = createFileReadQueue();
  const active = Promise.withResolvers<void>();
  let running = 0;
  let canceled = 0;
  const run = async () => {
    running++;
    await active.promise;
  };
  queue.add("active", run, () => canceled++);
  for (let i = 0; i < 40; i++) queue.add(`page:${i}`, run, () => canceled++);
  queue.add("page:0", run, () => canceled++);
  expect(running).toBe(1);
  expect(canceled).toBe(8);
  queue.clear();
  expect(canceled).toBe(41);
  active.resolve();
  await expect.poll(() => running).toBe(1);
  queue.add(
    "page:0",
    async () => {
      running++;
    },
    () => canceled++,
  );
  await expect.poll(() => running).toBe(2);
});

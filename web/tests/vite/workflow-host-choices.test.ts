import { expect, test, vi } from "vitest";
import { createWorkflowHostChoices } from "../../src/lib/controller/workflow-host-choices.svelte.js";
const binding = {
  repository: "03".repeat(16),
  path: "/private/code",
  name: "Demo",
  availability: "ready",
  generation: 1,
};
test("attachment discovery coalesces reads and retains last choices after failure", async () => {
  let resolve!: (value: unknown) => void;
  const request = vi.fn(
    () =>
      new Promise<unknown>((r) => {
        resolve = r;
      }),
  );
  const choices = createWorkflowHostChoices({
    available: () => true,
    client: { request: request as never, close() {} },
  });
  const first = choices.refresh();
  expect(choices.refresh()).toBe(first);
  expect(request).toHaveBeenCalledWith("repositories");
  resolve({ bindings: [binding] });
  await first;
  expect(choices.state.bindings).toEqual([binding]);
  const next = choices.refresh();
  resolve({ bindings: [{ ...binding, path: "relative" }] });
  await next;
  expect(choices.state.error).toContain("could not be loaded");
  expect(choices.state.bindings).toEqual([binding]);
  const retry = choices.refresh();
  resolve({ bindings: [binding] });
  await retry;
  expect(choices.state.error).toBe("");
});
test("unavailable and destroyed readers never republish late attachment results", async () => {
  let allowed = false,
    resolve!: (value: unknown) => void;
  const request = vi.fn(
      () =>
        new Promise<unknown>((r) => {
          resolve = r;
        }),
    ),
    close = vi.fn();
  const choices = createWorkflowHostChoices({
    available: () => allowed,
    client: { request: request as never, close },
  });
  await choices.refresh();
  expect(request).not.toHaveBeenCalled();
  allowed = true;
  const read = choices.refresh();
  choices.destroy();
  resolve({ bindings: [binding] });
  await read;
  await choices.refresh();
  expect(choices.state.bindings).toEqual([]);
  expect(request).toHaveBeenCalledTimes(1);
  expect(close).toHaveBeenCalledTimes(1);
});

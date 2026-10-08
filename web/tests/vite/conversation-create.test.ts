import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createThreadConversations } from "../../src/lib/controller/thread-conversations.svelte.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
for (const code of [
  "not_found",
  "collaboration_not_found",
  "local_agent_not_found",
]) {
  test(`conversation creation handles ${code} without inventing a source`, async () => {
    const { transport } = createThreadTransport();
    const detail = await transport.thread(THREAD_IDS.thread);
    const read = vi
      .spyOn(transport, "thread")
      .mockRejectedValueOnce(new ApiError(code))
      .mockResolvedValue(detail);
    const execute = vi.fn().mockResolvedValue({ status: "committed" });
    const controller = createThreadConversations(
      {
        source: () => transport,
        repository: () => THREAD_IDS.repo,
        principal: () => THREAD_IDS.actor,
      },
      execute,
    );
    expect(
      await controller.ensureConversation(THREAD_IDS.thread, "create-request"),
    ).toEqual(detail);
    expect(execute).toHaveBeenCalledExactlyOnceWith({
      version: 1,
      request_id: "create-request",
      command: {
        type: "create",
        thread_id: THREAD_IDS.thread,
        title: "Agent conversation",
        baseline: null,
      },
    });
    expect(read).toHaveBeenCalledTimes(2);
  });
}
test("authorization failure never starts creation", async () => {
  const { transport } = createThreadTransport();
  vi.spyOn(transport, "thread").mockRejectedValue(new ApiError("forbidden"));
  const execute = vi.fn();
  const controller = createThreadConversations(
    {
      source: () => transport,
      repository: () => THREAD_IDS.repo,
      principal: () => THREAD_IDS.actor,
    },
    execute,
  );
  await expect(
    controller.ensureConversation(THREAD_IDS.thread, "request"),
  ).rejects.toThrow();
  expect(execute).not.toHaveBeenCalled();
});

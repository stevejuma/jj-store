import { expect, test, vi } from "vitest";
import {
  AgentCompute,
  type ComputeValue,
} from "../../src/lib/controller/agent-compute.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";

test("closing compute aborts a single file dialog and discards its late selection", async () => {
  let finish!: (response: Response) => void;
  let signal: AbortSignal | undefined;
  const fetcher = vi.fn(async (url: unknown, init?: RequestInit) => {
    if (String(url).endsWith("/pick")) {
      signal = init?.signal ?? undefined;
      return await new Promise<Response>((resolve) => {
        finish = resolve;
      });
    }
    return Response.json({ model_file_picker: true, state: "off" });
  });
  const client = new LocalClient("", fetcher);
  const compute = new AgentCompute(client);
  try {
    compute.start();
    await compute.refresh();
    compute.value = { model_file_picker: true, state: "off" } as ComputeValue;
    const selected = compute.pickModelFile();
    expect(compute.modelPicking).toBe(true);
    expect(await compute.pickModelFile()).toBeNull();
    expect(
      fetcher.mock.calls.filter(([url]) => String(url).endsWith("/pick")),
    ).toHaveLength(1);
    compute.stop();
    expect(signal?.aborted).toBe(true);
    finish(Response.json({ version: 1, path: "/models/late.gguf" }));
    expect(await selected).toBeNull();
    expect(compute.modelPickerError).toBe("");
    expect(compute.modelPicking).toBe(false);
  } finally {
    compute.stop();
    client.close();
  }
});

import { expect, test, vi } from "vitest";
import { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";
import { createCaptureManagement } from "../../src/lib/controller/capture-management.svelte.js";
import {
  captureSettings,
  captureSources,
  captureStatus,
} from "../../../catalog/fixtures/capture-management.js";
import type { CaptureTransport } from "../../src/lib/controller/capture-types.js";

test("healthy capture reads only changed status, sources and configuration interests", async () => {
  vi.useFakeTimers();
  let stream!: ReadableStreamDefaultController<Uint8Array>,
    revision = 0;
  const encoder = new TextEncoder();
  const send = (type: string, value: object) =>
    stream.enqueue(
      encoder.encode(
        `id: jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:${revision++}\nevent: ${type}\ndata: ${JSON.stringify({ version: 1, type, ...value })}\n\n`,
      ),
    );
  const freshness = new ApplicationFreshness(
    async () =>
      new Response(
        new ReadableStream({
          start(controller) {
            stream = controller;
            send("ready", { witnesses: [] });
          },
        }),
        { headers: { "content-type": "text/event-stream" } },
      ),
  );
  const transport = {
    settings: vi.fn(async () => captureSettings()),
    status: vi.fn(async () => captureStatus()),
    sources: vi.fn(async () => captureSources()),
    close: vi.fn(),
  } as unknown as CaptureTransport;
  const management = createCaptureManagement({
    transport,
    freshness,
    available: () => true,
  });
  const changed = async (domain: string, identity: string | null) => {
    send("changed", {
      change: { repository: null, domain, identity, witness: String(revision) },
    });
    await vi.advanceTimersByTimeAsync(151);
  };
  try {
    await freshness.configure(true, null);
    management.setActive(true);
    await vi.advanceTimersByTimeAsync(151);
    expect(transport.settings).toHaveBeenCalledOnce();
    expect(transport.status).toHaveBeenCalledOnce();
    expect(transport.sources).toHaveBeenCalledOnce();
    for (let i = 0; i < 4; i++) {
      stream.enqueue(encoder.encode(":keep-alive\n\n"));
      await vi.advanceTimersByTimeAsync(15000);
    }
    expect(transport.status).toHaveBeenCalledOnce();
    await changed("capture", "status");
    expect(transport.status).toHaveBeenCalledTimes(2);
    expect(transport.sources).toHaveBeenCalledOnce();
    expect(transport.settings).toHaveBeenCalledOnce();
    await changed("capture", "sources");
    expect(transport.sources).toHaveBeenCalledTimes(2);
    expect(transport.settings).toHaveBeenCalledOnce();
    const statusReads = vi.mocked(transport.status).mock.calls.length;
    await changed("conversion", "foreign-request");
    expect(transport.status).toHaveBeenCalledTimes(statusReads);
    await changed("configuration", null);
    expect(transport.settings).toHaveBeenCalledTimes(2);
    expect(transport.sources).toHaveBeenCalledTimes(3);
    const beforeReopen = vi.mocked(transport.status).mock.calls.length;
    management.setActive(false);
    await changed("capture", "status");
    expect(transport.status).toHaveBeenCalledTimes(beforeReopen);
    management.setActive(true);
    await vi.advanceTimersByTimeAsync(151);
    expect(transport.status).toHaveBeenCalledTimes(beforeReopen + 1);
  } finally {
    management.destroy();
    freshness.destroy();
    vi.useRealTimers();
  }
});

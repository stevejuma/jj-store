import { expect, test, vi } from "vitest";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { HttpCaptureManagement } from "../../src/lib/controller/capture-http.js";
import { captureStatus } from "../../../catalog/fixtures/capture-management.js";

const response = (code = "capture_control_busy") =>
  new Response(JSON.stringify({ version: 1, code }), {
    status: 409,
    headers: { "content-type": "application/json" },
  });
test("capture observations retry busy reads within a bounded window", async () => {
  vi.useFakeTimers();
  const fetcher = vi
    .fn<typeof fetch>()
    .mockImplementationOnce(async () => response())
    .mockImplementationOnce(async () => response())
    .mockImplementationOnce(
      async () =>
        new Response(JSON.stringify(captureStatus()), {
          headers: { "content-type": "application/json" },
        }),
    );
  const transport = new HttpCaptureManagement(new LocalClient("", fetcher));
  try {
    const read = transport.status();
    await vi.advanceTimersByTimeAsync(2000);
    expect(await read).toEqual(captureStatus());
    expect(fetcher).toHaveBeenCalledTimes(3);
    expect(
      fetcher.mock.calls.every(
        ([path, options]) => path === "/v1/local/capture" && !options?.method,
      ),
    ).toBe(true);
  } finally {
    transport.close();
    vi.useRealTimers();
  }
});
test("busy capture observation retries stop on cancellation", async () => {
  vi.useFakeTimers();
  const fetcher = vi
    .fn<typeof fetch>()
    .mockImplementation(async () => response());
  const transport = new HttpCaptureManagement(new LocalClient("", fetcher));
  const controller = new AbortController();
  try {
    const rejected = expect(
      transport.status(controller.signal),
    ).rejects.toMatchObject({ name: "AbortError" });
    await vi.advanceTimersByTimeAsync(0);
    controller.abort();
    await rejected;
    await vi.advanceTimersByTimeAsync(2000);
    expect(fetcher).toHaveBeenCalledOnce();
  } finally {
    transport.close();
    vi.useRealTimers();
  }
});
test("capture retries are bounded and commands and real refusals are never retried", async () => {
  vi.useFakeTimers();
  const fetcher = vi
    .fn<typeof fetch>()
    .mockImplementation(async () => response());
  const transport = new HttpCaptureManagement(new LocalClient("", fetcher));
  try {
    const rejected = expect(transport.sources()).rejects.toMatchObject({
      code: "capture_control_busy",
    });
    await vi.advanceTimersByTimeAsync(2000);
    await rejected;
    expect(fetcher).toHaveBeenCalledTimes(5);
    fetcher.mockClear();
    await expect(
      transport.rescan({
        version: 1,
        generation: "ab".repeat(16),
        expected_revision: 1,
      }),
    ).rejects.toMatchObject({ code: "capture_control_busy" });
    expect(fetcher).toHaveBeenCalledOnce();
    fetcher
      .mockClear()
      .mockImplementation(async () => response("capture_management_denied"));
    await expect(transport.status()).rejects.toMatchObject({
      code: "capture_management_denied",
    });
    expect(fetcher).toHaveBeenCalledOnce();
  } finally {
    transport.close();
    vi.useRealTimers();
  }
});

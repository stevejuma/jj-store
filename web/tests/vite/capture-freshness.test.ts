import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createCaptureManagement } from "../../src/lib/controller/capture-management.svelte.js";
import {
  captureStatus,
  captureSources,
  captureReceipt,
} from "../../../catalog/fixtures/capture-management.js";
import type { CaptureTransport } from "../../src/lib/controller/capture-types.js";
function fixture() {
  const transport: CaptureTransport = {
    status: vi.fn(async () => captureStatus()),
    sources: vi.fn(async () => captureSources()),
    control: vi.fn(async () => {
      throw new TypeError("lost response");
    }),
    rescan: vi.fn(async () => ({
      version: 1,
      accepted: true,
      status: captureStatus(),
    })),
    import: vi.fn(async (command) => captureReceipt(command)),
    inspect: vi.fn(async (id) => ({
      version: 1,
      request_id: id,
      receipt: null,
      progress: {
        ...captureReceipt().progress,
        phase: "pending",
        boundary_available: false,
      },
    })),
    close: vi.fn(),
  };
  return {
    transport,
    management: createCaptureManagement({
      transport,
      available: () => true,
      requestId: () => "cd".repeat(16),
    }),
  };
}
test("capture conditions recover only after complete successful observations", async () => {
  const f = fixture();
  try {
    vi.mocked(f.transport.status).mockRejectedValueOnce(
      new ApiError("capture_control_busy"),
    );
    await f.management.refresh();
    const error = f.management.state.error;
    expect(error).not.toBe("");
    vi.mocked(f.transport.sources).mockRejectedValueOnce(
      new ApiError("capture_control_busy"),
    );
    await f.management.refresh();
    expect(f.management.state.error).not.toBe("");
    await f.management.refresh();
    expect(f.management.state.error).toBe("");
  } finally {
    f.management.destroy();
  }
});
test("visible capture loads polls coalesces and stops on closure", async () => {
  vi.useFakeTimers();
  const f = fixture();
  try {
    f.management.setActive(true);
    await vi.advanceTimersByTimeAsync(151);
    expect(f.transport.status).toHaveBeenCalledOnce();
    await vi.advanceTimersByTimeAsync(5000);
    expect(f.transport.status).toHaveBeenCalledTimes(2);
    f.management.setActive(false);
    await vi.advanceTimersByTimeAsync(10000);
    expect(f.transport.status).toHaveBeenCalledTimes(2);
    f.management.setActive(true);
    await vi.advanceTimersByTimeAsync(151);
    expect(f.transport.status).toHaveBeenCalledTimes(3);
  } finally {
    f.management.destroy();
    vi.useRealTimers();
  }
});
test("background observations retain exact uncertain control intent", async () => {
  vi.useFakeTimers();
  const f = fixture();
  try {
    await f.management.refresh();
    await f.management.control("pause");
    const original = JSON.stringify(f.management.state.pending);
    f.management.setActive(true);
    await vi.advanceTimersByTimeAsync(10151);
    expect(JSON.stringify(f.management.state.pending)).toBe(original);
    expect(f.transport.control).toHaveBeenCalledOnce();
    expect(f.management.state.error).toContain("Outcome unknown");
  } finally {
    f.management.destroy();
    vi.useRealTimers();
  }
});
test("closure cancels an obsolete read without discarding the previous snapshot", async () => {
  vi.useFakeTimers();
  const f = fixture();
  try {
    await f.management.refresh();
    let resolve!: (value: ReturnType<typeof captureStatus>) => void;
    vi.mocked(f.transport.status).mockImplementationOnce(
      () =>
        new Promise((r) => {
          resolve = r;
        }),
    );
    f.management.setActive(true);
    await vi.advanceTimersByTimeAsync(151);
    f.management.setActive(false);
    resolve(captureStatus(true));
    await vi.advanceTimersByTimeAsync(0);
    expect(f.management.state.value!.policy!.paused).toBe(false);
  } finally {
    f.management.destroy();
    vi.useRealTimers();
  }
});
test("unsupported harness history cannot issue an import", async () => {
  const f = fixture();
  await f.management.refresh();
  f.management.state.sources!.sources[0].profile = "opencode-read-only-api-v1";
  await f.management.importHistory(
    f.management.state.sources!.sources[0].selector,
  );
  expect(f.transport.import).not.toHaveBeenCalled();
  f.management.destroy();
});

test("background refresh retains every loaded source page", async () => {
  const f = fixture();
  const first = captureSources(),
    second = captureSources();
  first.total = second.total = 2;
  first.next_cursor = first.sources[0].selector;
  second.sources[0].selector = second.runtime_generation! + ":0000000000000002";
  vi.mocked(f.transport.sources).mockImplementation(async (cursor) =>
    structuredClone(cursor ? second : first),
  );
  await f.management.refresh();
  await f.management.moreSources();
  expect(f.management.state.sources!.sources).toHaveLength(2);
  await f.management.refresh();
  expect(f.management.state.sources!.sources).toHaveLength(2);
  expect(f.management.state.sources!.next_cursor).toBeNull();
  f.management.destroy();
});
test("automatic import inspection uses the acknowledged original request", async () => {
  vi.useFakeTimers();
  const f = fixture();
  try {
    await f.management.refresh();
    await f.management.importHistory(
      f.management.state.sources!.sources[0].selector,
    );
    const original = f.management.state.importStatus!.receipt;
    f.management.setActive(true);
    await vi.advanceTimersByTimeAsync(5151);
    expect(f.transport.import).toHaveBeenCalledOnce();
    expect(f.transport.inspect).toHaveBeenCalledWith(
      original!.command.request_id,
      expect.any(AbortSignal),
    );
    expect(f.management.state.importStatus!.receipt).toEqual(original);
  } finally {
    f.management.destroy();
    vi.useRealTimers();
  }
});

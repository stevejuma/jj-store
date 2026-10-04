import { expect, test, vi } from "vitest";
import { ApiError, pretty } from "../../src/lib/api/client.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { HttpCaptureManagement } from "../../src/lib/controller/capture-http.js";
import { createCaptureManagement } from "../../src/lib/controller/capture-management.svelte.js";
import {
  validateImportStatus,
  validateCaptureSources,
  validateCaptureStatus,
} from "../../src/lib/controller/capture-validation.js";
import type {
  CaptureControl,
  CaptureImport,
} from "../../src/lib/controller/capture-types.js";
import {
  captureStatus,
  captureSources,
  captureReceipt,
} from "../../../catalog/fixtures/capture-management.js";

function fixture() {
  let supported = true;
  const transport = {
    status: vi.fn(async () => captureStatus()),
    sources: vi.fn(async (_cursor?: string) => captureSources()),
    control: vi.fn(async (command: CaptureControl) => ({
      version: 1,
      receipt: {
        ...command,
        revision: BigInt(command.expected_revision) + 1n,
        paused: command.action === "pause",
      },
      replayed: false,
      runtime_requested: true,
      status: captureStatus(
        command.action === "pause",
        BigInt(command.expected_revision) + 1n,
      ),
    })),
    rescan: vi.fn(async () => ({
      version: 1,
      accepted: true,
      status: captureStatus(),
    })),
    import: vi.fn(async (command: CaptureImport) => captureReceipt(command)),
    inspect: vi.fn(async (id: string) => ({
      version: 1,
      request_id: id,
      receipt: null,
      progress: {
        ...captureReceipt().progress,
        phase: "pending" as const,
        boundary_available: false,
      },
    })),
    close: vi.fn(),
  };
  const management = createCaptureManagement({
    transport,
    available: () => supported,
    requestId: () => "cd".repeat(16),
  });
  return {
    management,
    transport,
    unsupported: () => {
      supported = false;
    },
  };
}

test("cumulative unavailable observations do not disable capture controls", async () => {
  const f = fixture();
  const status = captureStatus();
  status.runtime!.unavailable = 14000n;
  f.transport.status.mockResolvedValue(status);
  await f.management.refresh();
  expect(f.management.state.status).toBe("ready");
  await f.management.control("pause");
  expect(f.transport.control).toHaveBeenCalledOnce();
  expect(() =>
    validateCaptureStatus({
      ...status,
      runtime: { ...status.runtime, unavailable: -1 },
    }),
  ).toThrow();
  expect(() =>
    validateCaptureStatus({
      ...status,
      runtime: { ...status.runtime, tracked: 4097 },
    }),
  ).toThrow();
  f.management.destroy();
});

test("capture transport preserves exact local policy", async () => {
  const f = fixture();
  await f.management.control("pause");
  expect(f.transport.control).not.toHaveBeenCalled();
  await f.management.refresh();
  f.management.state.value!.policy!.revision = 9007199254740993n;
  await f.management.control("pause");
  expect(f.transport.control.mock.calls[0][0]).toMatchObject({
    expected_revision: 9007199254740993n,
    action: "pause",
    request_id: "cd".repeat(16),
  });
  f.unsupported();
  await f.management.refresh();
  expect(f.transport.status).toHaveBeenCalledOnce();
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response(pretty(captureStatus(false, 9007199254740993n)), {
        headers: { "content-type": "application/json" },
      }),
  );
  const http = new HttpCaptureManagement(new LocalClient("", fetcher));
  expect((await http.status()).policy!.revision).toBe(9007199254740993n);
  await http.sources("ab".repeat(16) + ":0000000000000001").catch(() => {});
  expect(fetcher.mock.calls[0][0]).toBe("/v1/local/capture");
  expect(fetcher.mock.calls[0][1]).toMatchObject({
    credentials: "omit",
    redirect: "error",
    headers: { "x-jj-store-protocol": "1" },
  });
  expect(fetcher.mock.calls[1][0]).toContain(
    "/v1/local/capture/sources?limit=100&cursor=",
  );
});

test("unknown control retains exact original intent", async () => {
  const f = fixture();
  await f.management.refresh();
  f.transport.control.mockRejectedValueOnce(new TypeError("connection lost"));
  await f.management.control("pause");
  const original = JSON.parse(pretty(f.management.state.pending));
  expect(original!.uncertain).toBe(true);
  await f.management.refresh();
  f.transport.control.mockRejectedValueOnce(
    new ApiError("capture_policy_changed"),
  );
  await f.management.retry();
  expect(JSON.parse(pretty(f.management.state.pending))).toEqual(original);
  await f.management.control("resume");
  expect(f.transport.control).toHaveBeenCalledTimes(2);
  await f.management.retry();
  expect(f.transport.control.mock.calls[2][0]).toEqual(original!.command);
  expect(f.management.state.pending).toBeNull();
  expect(f.management.state.controlOutcome!.runtime_requested).toBe(true);
});

test("capture source pages reject stale continuations", async () => {
  const f = fixture();
  const page = captureSources();
  page.next_cursor = page.sources[0].selector;
  f.transport.sources.mockResolvedValueOnce(page);
  await f.management.refresh();
  f.transport.sources.mockResolvedValueOnce({
    ...captureSources(),
    runtime_generation: "ef".repeat(16),
  });
  await f.management.moreSources();
  expect(f.management.state.sources!.sources).toHaveLength(1);
  expect(f.management.state.sourceState).toBe("failed");
  await f.management.importHistory(page.sources[0].selector);
  expect(f.transport.import).not.toHaveBeenCalled();
  f.transport.sources.mockResolvedValueOnce(page);
  await f.management.refresh();
  f.transport.sources.mockResolvedValueOnce(page);
  await f.management.moreSources();
  expect(f.management.state.sourceState).toBe("failed");
});

test("capture import requires its matching original receipt", async () => {
  const f = fixture();
  await f.management.refresh();
  f.transport.import.mockRejectedValueOnce(
    new ApiError("capture_control_outcome_unknown"),
  );
  await f.management.importHistory(
    f.management.state.sources!.sources[0].selector,
  );
  const command = f.transport.import.mock.calls[0][0];
  await f.management.inspectImport(command.request_id);
  expect(f.management.state.pending).not.toBeNull();
  const outcome = captureReceipt(command);
  f.transport.inspect.mockResolvedValueOnce({
    version: 1,
    request_id: command.request_id,
    receipt: outcome.receipt,
    progress: outcome.progress,
  } as any);
  await f.management.inspectImport(command.request_id);
  expect(f.management.state.pending).toBeNull();
  expect(f.management.state.importStatus!.progress.phase).toBe("completed");
  expect(f.management.state.importStatus!.progress.tail_phase).toBe("running");
});

test("capture refuses malformed or mismatched acceptance", async () => {
  const f = fixture();
  await f.management.refresh();
  f.transport.control.mockResolvedValueOnce({} as any);
  await f.management.control("pause");
  expect(f.management.state.pending!.uncertain).toBe(true);
  const command = f.transport.control.mock.calls[0][0];
  f.transport.control.mockResolvedValueOnce({
    version: 1,
    receipt: {
      ...command,
      request_id: "ef".repeat(16),
      revision: 2,
      paused: true,
    },
    replayed: true,
    runtime_requested: false,
    status: captureStatus(true, 2),
  });
  await f.management.retry();
  expect(f.management.state.pending!.command).toEqual(command);
  const g = fixture();
  await g.management.refresh();
  g.transport.control.mockRejectedValueOnce(new ApiError("capture_paused"));
  await g.management.control("pause");
  expect(g.management.state.pending).toBeNull();
  await g.management.control("resume");
  expect(g.transport.control).toHaveBeenCalledOnce();
});

test("capture single flight and teardown ignore late replies", async () => {
  const f = fixture();
  let deliver!: (value: ReturnType<typeof captureStatus>) => void;
  f.transport.status.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const pending = f.management.refresh();
  const coalesced = f.management.refresh();
  expect(f.transport.status).toHaveBeenCalledOnce();
  f.management.destroy();
  deliver(captureStatus());
  await Promise.all([pending, coalesced]);
  expect(f.management.state.value).toBeNull();
  expect(f.transport.close).toHaveBeenCalledOnce();
});

test("capture responses cannot invent history completion", () => {
  const outcome = captureReceipt();
  const id = outcome.receipt.command.request_id;
  expect(() =>
    validateImportStatus(
      { version: 1, request_id: id, receipt: null, progress: outcome.progress },
      id,
    ),
  ).toThrow();
  const progress = { ...outcome.progress, committed_byte_offset: 512 };
  expect(() =>
    validateImportStatus(
      { version: 1, request_id: id, receipt: outcome.receipt, progress },
      id,
    ),
  ).toThrow();
});

test("API snapshot source coverage preserves exact positions and unavailable publication", async () => {
  const page = captureSources();
  page.sources[0].profile = "opencode-read-only-api-v1";
  page.sources[0].product_version = "1.18.30";
  page.sources[0].snapshot_publication = {
    history_available: true,
    captured_sequence: 9007199254740993n,
    captured_logical_records: 0,
    committed_sequence: 9007199254740993n,
    committed_logical_records: 0,
    outstanding_ranges: 0,
    receipts: 1,
  };
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response(pretty(page), {
        headers: { "content-type": "application/json" },
      }),
  );
  const transport = new HttpCaptureManagement(new LocalClient("", fetcher));
  const selected = await transport.sources();
  expect(selected.sources[0].snapshot_publication).toEqual(
    page.sources[0].snapshot_publication,
  );
  expect(selected.sources[0].snapshot_publication!.committed_sequence).toBe(
    9007199254740993n,
  );
  const f = fixture();
  f.transport.sources.mockResolvedValueOnce(selected);
  await f.management.refresh();
  expect(
    f.management.state.sources!.sources[0].snapshot_publication!
      .committed_logical_records,
  ).toBe(0);
  delete page.sources[0].snapshot_publication;
  f.transport.sources.mockResolvedValueOnce(page);
  await f.management.refresh();
  expect(
    f.management.state.sources!.sources[0].snapshot_publication,
  ).toBeUndefined();
  expect(f.transport.import).not.toHaveBeenCalled();
});

test("API snapshot coverage rejects malformed counts and unproved publication", () => {
  const page = captureSources();
  page.sources[0].profile = "opencode-read-only-api-v1";
  const coverage = {
    history_available: true,
    captured_sequence: 10,
    captured_logical_records: 2,
    committed_sequence: 8,
    committed_logical_records: 12,
    outstanding_ranges: 2,
    receipts: 1,
  };
  // Replacement generations may have fewer messages than the published frontier.
  page.sources[0].snapshot_publication = coverage;
  expect(() => validateCaptureSources(page)).not.toThrow();
  for (const patch of [
    { captured_sequence: 9007199254740992 },
    { captured_sequence: 18446744073709551616n },
    { captured_logical_records: -1 },
    { committed_sequence: 11 },
    { committed_sequence: 0 },
    { committed_logical_records: null },
    { committed_logical_records: -1 },
    { history_available: false },
    { receipts: 0 },
    { receipts: 1.5 },
    { outstanding_ranges: 4097 },
    { reader_source_id: "private" },
  ]) {
    page.sources[0].snapshot_publication = { ...coverage, ...patch };
    expect(() => validateCaptureSources(page)).toThrow(ApiError);
  }
  page.sources[0].snapshot_publication = {
    ...coverage,
    committed_sequence: 0,
    committed_logical_records: null,
    receipts: 0,
  };
  expect(() => validateCaptureSources(page)).not.toThrow();
  page.sources[0].profile = "codex-cli-jsonl-v1";
  expect(() => validateCaptureSources(page)).toThrow(ApiError);
});

test("configured harness roots are bounded absolute paths with honest counts", () => {
  const value = captureStatus();
  expect(() => validateCaptureStatus(value)).not.toThrow();
  for (const roots of [
    ["relative/sessions"],
    ["/bad\npath"],
    ["/" + "x".repeat(4096)],
    ["/same", "/same"],
    [],
  ]) {
    value.runtime!.harnesses[0].configured_roots = roots;
    expect(() => validateCaptureStatus(value)).toThrow(ApiError);
  }
  delete value.runtime!.harnesses[0].configured_roots;
  expect(() => validateCaptureStatus(value)).not.toThrow();
});

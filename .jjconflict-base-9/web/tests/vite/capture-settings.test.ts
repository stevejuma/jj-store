import { expect, test, vi } from "vitest";
import {
  captureSettings,
  captureStatus,
  captureSources,
} from "../../../catalog/fixtures/capture-management.js";
import { createCaptureManagement } from "../../src/lib/controller/capture-management.svelte.js";
import { ApiError } from "../../src/lib/api/client.js";
import type { CaptureTransport } from "../../src/lib/controller/capture-types.js";
import {
  captureConfigValue,
  captureConfigDraft,
  validateCaptureSettings,
} from "../../src/lib/controller/capture-configuration.js";
function fixture() {
  let value = captureSettings(false);
  const save = vi.fn<NonNullable<CaptureTransport["saveSettings"]>>(
    async (c) => {
      value = {
        ...value,
        revision: "cd".repeat(32),
        desired: c.configuration,
        restart_required: true,
      };
      return value;
    },
  );
  const transport: CaptureTransport = {
    settings: async () => structuredClone(value),
    saveSettings: save,
    status: vi.fn(async () => ({
      ...captureStatus(),
      enabled: false,
      policy: null,
      runtime: null,
    })),
    sources: async () => ({
      ...captureSources(),
      enabled: false,
      sources: [],
      total: 0,
      phase: null,
      runtime_generation: null,
    }),
    control: vi.fn(),
    rescan: vi.fn(),
    import: vi.fn(),
    inspect: vi.fn(),
    close: vi.fn(),
  };
  const management = createCaptureManagement({
    transport,
    available: () => true,
  });
  return {
    management,
    save,
    transport,
    external() {
      value = { ...value, revision: "ef".repeat(32) };
    },
  };
}
test("disabled configuration drafts survive refresh, save only typed settings and withhold runtime changes", async () => {
  const f = fixture();
  await f.management.refresh();
  f.management.configure({
    enabled: true,
    cliRoots: "/tmp/selected/sessions",
    readers: "2",
  });
  await f.management.refresh();
  expect(f.management.state.settings.draft?.cliRoots).toBe(
    "/tmp/selected/sessions",
  );
  await f.management.saveConfiguration();
  expect(f.save).toHaveBeenCalledWith(
    expect.objectContaining({
      expected_revision: "ab".repeat(32),
      configuration: expect.objectContaining({
        enabled: true,
        max_readers: 2,
        harnesses: [
          expect.objectContaining({ roots: ["/tmp/selected/sessions"] }),
        ],
      }),
    }),
  );
  expect(f.management.state.settings.value?.restart_required).toBe(true);
  await f.management.refresh();
  await f.management.control("resume");
  expect(f.transport.control).not.toHaveBeenCalled();
  f.management.destroy();
});
test("external revisions cannot silently overwrite a retained draft", async () => {
  const f = fixture();
  await f.management.refresh();
  f.management.configure({ enabled: true });
  f.external();
  await f.management.refresh();
  await f.management.saveConfiguration();
  expect(f.save).not.toHaveBeenCalled();
  expect(f.management.state.settings.conflict).toBe(true);
  expect(f.management.state.settings.draft?.enabled).toBe(true);
  f.management.cancelConfiguration();
  expect(f.management.state.settings.draft?.enabled).toBe(false);
  f.management.destroy();
});
test("unknown saves retain intent until fresh observation resolves the saved configuration", async () => {
  const f = fixture();
  await f.management.refresh();
  f.management.configure({ enabled: true });
  const implementation = f.save.getMockImplementation()!;
  f.save.mockImplementation(async (c) => {
    await implementation(c);
    throw new TypeError("lost response");
  });
  await f.management.saveConfiguration();
  expect(f.management.state.settings.unknown?.expected_revision).toBe(
    "ab".repeat(32),
  );
  await f.management.saveConfiguration();
  expect(f.save).toHaveBeenCalledOnce();
  f.management.cancelConfiguration();
  expect(f.management.state.settings.unknown).not.toBeNull();
  await f.management.refresh();
  expect(f.management.state.settings.unknown).toBeNull();
  expect(f.management.state.settings.dirty).toBe(false);
  f.management.destroy();
});
test("stale save keeps draft and forbids a second write until saved configuration is reviewed", async () => {
  const f = fixture();
  await f.management.refresh();
  f.management.configure({ enabled: true });
  f.save.mockRejectedValue(new ApiError("capture_settings_changed"));
  await f.management.saveConfiguration();
  await f.management.saveConfiguration();
  expect(f.save).toHaveBeenCalledOnce();
  expect(f.management.state.settings.conflict).toBe(true);
  expect(f.management.state.settings.unknown).toBeNull();
  f.management.destroy();
});
test("configuration validates bounds, selected harnesses, versions and absolute directory overrides", () => {
  const s = captureSettings(false);
  validateCaptureSettings(s);
  const d = captureConfigDraft(s.desired);
  for (const patch of [
    { enabled: true, cli: false },
    { readers: "5" },
    { interval: "0" },
    { sources: "4097" },
    { bytes: "67108865" },
    { cliRoots: "relative" },
    { cliRoots: "/tmp/../source" },
    { cliRoots: "/tmp/a\n/tmp/a" },
  ])
    expect(() => captureConfigValue({ ...d, ...patch }, s.defaults)).toThrow();
  expect(() =>
    validateCaptureSettings({
      ...s,
      desired: {
        ...s.desired,
        harnesses: [
          { ...s.desired.harnesses[0], product_version: "unqualified" },
        ],
      },
    }),
  ).toThrow();
});

test("configuration can save while an ordinary runtime observation is waiting", async () => {
  const f = fixture();
  await f.management.refresh();
  let deliver!: (v: Awaited<ReturnType<CaptureTransport["status"]>>) => void;
  vi.mocked(f.transport.status).mockImplementation(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const observation = f.management.refresh();
  await vi.waitFor(() => expect(deliver).toBeTypeOf("function"));
  expect(f.management.state.busy).toBe(true);
  f.management.configure({ enabled: true });
  await f.management.saveConfiguration();
  expect(f.save).toHaveBeenCalledOnce();
  expect(f.management.state.settings.value?.desired.enabled).toBe(true);
  expect(f.management.state.settings.saving).toBe(false);
  deliver({ ...captureStatus(), enabled: false, policy: null, runtime: null });
  await observation;
  f.management.destroy();
});

test("Cursor Desktop, CLI and OpenCode settings preserve roots, endpoint and versions", () => {
  const s = captureSettings(false);
  const additions = [
    {
      profile: "cursor-desktop-transcript-v1",
      product_version: "3.22.7",
      roots: ["/tmp/cursor/projects"],
    },
    {
      profile: "cursor-cli-transcript-v1",
      product_version: "2026.09.26-dd393fe",
      roots: ["/tmp/cursor/projects"],
    },
    {
      profile: "opencode-read-only-api-v1",
      product_version: "1.18.30",
      roots: [],
      endpoint: "http://127.0.0.1:4096/",
    },
  ];
  const defaults = s.defaults;
  const d = {
    ...captureConfigDraft(s.desired),
    enabled: true,
    cursorDesktop: true,
    cursorCli: true,
    openCode: true,
    cursorDesktopRoots: "/tmp/desktop/projects",
    cursorCliRoots: "/tmp/cli/projects",
    openCodeEndpoint: "http://127.0.0.1:4096/",
  };
  const c = captureConfigValue(d, defaults);
  expect(c.harnesses).toHaveLength(4);
  expect(c.harnesses[1]).toMatchObject({
    profile: additions[0].profile,
    roots: ["/tmp/desktop/projects"],
  });
  expect(c.harnesses[2]).toMatchObject({
    profile: additions[1].profile,
    product_version: additions[1].product_version,
    roots: ["/tmp/cli/projects"],
  });
  expect(c.harnesses[3]).toMatchObject({
    endpoint: "http://127.0.0.1:4096/",
    roots: [],
  });
  expect(captureConfigDraft(c)).toMatchObject(d);
  for (const endpoint of [
    "https://example.com/",
    "http://localhost:4096/",
    "http://user:secret@127.0.0.1:4096/",
    "http://127.0.0.1:4096/session",
  ])
    expect(() =>
      captureConfigValue({ ...d, openCodeEndpoint: endpoint }, defaults),
    ).toThrow();
});

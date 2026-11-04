import { describe, expect, it, vi } from "vitest";
import { resolveHostEnvironment } from "../../src/lib/host/environment.js";
import { browserHost } from "../../src/lib/host/contracts.js";

describe("renderer host selection", () => {
  it("renders desktop presentation in development without granting native capabilities", async () => {
    const loadNative = vi.fn();
    const host = await resolveHostEnvironment({
      development: true,
      desktopPreview: true,
      target: {},
      loadNative,
    });
    expect(host).toEqual({ presentation: "macos" });
    expect(Object.isFrozen(host)).toBe(true);
    expect(loadNative).not.toHaveBeenCalled();
  });

  it("ignores the desktop development override in production", async () => {
    expect(
      await resolveHostEnvironment({
        development: false,
        desktopPreview: true,
        target: {},
      }),
    ).toBe(browserHost);
  });

  it.each([
    {},
    { __JJ_STORE_DESKTOP__: { version: 1, platform: "macos", terminal: 1 } },
    { __TAURI_INTERNALS__: { invoke: vi.fn() } },
    {
      __JJ_STORE_DESKTOP__: { version: 2, platform: "macos" },
      __TAURI_INTERNALS__: { invoke: vi.fn() },
    },
    {
      __JJ_STORE_DESKTOP__: { version: 1, platform: "linux" },
      __TAURI_INTERNALS__: { invoke: vi.fn() },
    },
  ])(
    "never loads native IPC for an unsupported or incomplete host: %j",
    async (target) => {
      const loadNative = vi.fn();
      expect(await resolveHostEnvironment({ target, loadNative })).toBe(
        browserHost,
      );
      expect(loadNative).not.toHaveBeenCalled();
    },
  );

  it("loads the native adapter only for supported metadata and IPC, before applying a preview", async () => {
    const native = {
      presentation: "macos" as const,
      windowDragging: true as const,
    };
    const loadNative = vi.fn().mockResolvedValue(native);
    const target = {
      __JJ_STORE_DESKTOP__: { version: 1, platform: "macos", terminal: 1 },
      __TAURI_INTERNALS__: { invoke: vi.fn() },
    };
    expect(
      await resolveHostEnvironment({
        target,
        development: true,
        desktopPreview: true,
        loadNative,
      }),
    ).toBe(native);
    expect(loadNative).toHaveBeenCalledWith(target.__JJ_STORE_DESKTOP__);
  });
});

import { describe, expect, it, vi } from "vitest";
import { createGlassController } from "../../src/lib/appearance/desktop-glass.js";

function pending() {
  let resolve!: () => void;
  const promise = new Promise<void>((done) => {
    resolve = done;
  });
  return { promise, resolve };
}

describe("native glass handshake", () => {
  it("exposes glass only after installation and avoids rebuilding on theme or opacity changes", async () => {
    const ready = pending();
    const apply = vi.fn(() => ready.promise);
    const expose = vi.fn();
    const glass = createGlassController(apply, expose);
    const complete = glass.set(true);
    expect(expose).not.toHaveBeenCalledWith(true);
    ready.resolve();
    await complete;
    expect(expose).toHaveBeenLastCalledWith(true);
    await glass.set(true);
    expect(apply).toHaveBeenCalledTimes(1);
  });
  it("serializes rapid toggles and hides CSS before removing the native material", async () => {
    const ready = pending();
    const apply = vi.fn(async (enabled: boolean) => {
      if (enabled) await ready.promise;
    });
    const expose = vi.fn();
    const glass = createGlassController(apply, expose);
    const enabling = glass.set(true);
    const disabling = glass.set(false);
    expect(expose).toHaveBeenLastCalledWith(false);
    expect(apply.mock.calls).toEqual([[true]]);
    ready.resolve();
    await Promise.all([enabling, disabling]);
    expect(apply.mock.calls).toEqual([[true], [false]]);
    expect(expose).not.toHaveBeenCalledWith(true);
    await glass.set(true);
    expect(expose).toHaveBeenLastCalledWith(true);
  });
  it("keeps the window opaque on failure and retries an explicit subsequent request", async () => {
    const apply = vi
      .fn()
      .mockRejectedValueOnce(Error("unavailable"))
      .mockResolvedValue(undefined);
    const expose = vi.fn();
    const failed = vi.fn();
    const glass = createGlassController(apply, expose, failed);
    await glass.set(true);
    expect(expose).toHaveBeenLastCalledWith(false);
    expect(failed).toHaveBeenCalledTimes(1);
    await glass.set(true);
    expect(expose).toHaveBeenLastCalledWith(true);
  });
  it("clears a late enable after disposal without exposing a destroyed renderer", async () => {
    const ready = pending();
    const apply = vi.fn(async (enabled: boolean) => {
      if (enabled) await ready.promise;
    });
    const expose = vi.fn();
    const glass = createGlassController(apply, expose);
    const enabling = glass.set(true);
    const disposing = glass.dispose();
    ready.resolve();
    await Promise.all([enabling, disposing]);
    expect(apply.mock.calls).toEqual([[true], [false]]);
    expect(expose).not.toHaveBeenCalledWith(true);
  });
});

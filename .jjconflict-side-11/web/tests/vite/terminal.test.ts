import { describe, it, expect } from "vitest";
import { TerminalController } from "../../src/lib/terminal/controller.svelte.js";
import {
  encodePaste,
  encodeTerminalKey,
  accumulateScrollLines,
} from "../../src/lib/terminal/keyboard.js";
import { FakeTerminalBridge } from "../../../catalog/fixtures/terminal-bridge.js";

describe("desktop terminal controller", () => {
  it("retains an uncertain open identity and reattaches native sessions after renderer reload", async () => {
    const bridge = new FakeTerminalBridge();
    const errors: unknown[] = [];
    const first = new TerminalController(bridge, (error) => errors.push(error));
    await first.context("repo-a", "native");
    bridge.failOpen = true;
    await first.create();
    expect(errors).toHaveLength(1);
    await first.create();
    expect(bridge.opens[0]).toBe(bridge.opens[1]);
    expect(bridge.sessions).toHaveLength(1);
    expect(first.sessions[0].grid?.selectionRows()[0].text).toBe("Ready");
    first.dispose();
    expect(bridge.closeCalls).toHaveLength(0);
    const next = new TerminalController(bridge, (error) => errors.push(error));
    await next.context("repo-a", "native");
    await next.initialize();
    expect(next.sessions).toHaveLength(1);
    expect(next.activeId).toBe(bridge.sessions[0].id);
    await next.close(next.activeId);
    expect(bridge.closeCalls).toHaveLength(1);
    next.dispose();
  });
  it("fences stale folder discovery and never opens a shell for the previous repository", async () => {
    const bridge = new FakeTerminalBridge();
    const original = bridge.targets.bind(bridge);
    let release!: () => void;
    bridge.targets = async (repo) => {
      if (repo === "repo-a")
        await new Promise<void>((resolve) => (release = resolve));
      return original(repo);
    };
    const model = new TerminalController(bridge, (error) => {
      throw error;
    });
    const old = model.context("repo-a", "native");
    await model.context("repo-b", "native");
    release();
    await old;
    expect(model.targets[0].repository).toBe("repo-b");
    await model.show();
    expect(model.sessions[0].target.repository).toBe("repo-b");
    model.dispose();
  });
  it("keeps tab selection by repository", async () => {
    const bridge = new FakeTerminalBridge();
    const model = new TerminalController(bridge, (error) => {
      throw error;
    });
    await model.context("repo-a", "native");
    await model.create();
    const id = model.activeId;
    await model.context("repo-b", "native");
    expect(model.activeId).toBe("");
    await model.create();
    await model.context("repo-a", "native");
    expect(model.activeId).toBe(id);
    expect(model.sessions).toHaveLength(2);
    model.dispose();
  });
  it("ignores callbacks racing a successful close but reports active session failures", async () => {
    const bridge = new FakeTerminalBridge();
    const errors: unknown[] = [];
    const model = new TerminalController(bridge, (error) => errors.push(error));
    await model.context("repo-a", "native");
    await model.create();
    const id = model.activeId;
    let rejectFocus!: (error: Error) => void;
    bridge.focus = () =>
      new Promise<void>((_, reject) => (rejectFocus = reject));
    model.focus(id, false);
    await model.close(id);
    rejectFocus(Error("Terminal session has ended"));
    await Promise.resolve();
    expect(errors).toEqual([]);
    await model.create();
    model.focus(model.activeId, true);
    rejectFocus(Error("Active session failed"));
    await Promise.resolve();
    expect(errors).toEqual([Error("Active session failed")]);
    model.dispose();
  });
});
it("encodes shell input without consuming Command shortcuts and preserves wheel direction", () => {
  expect(
    encodeTerminalKey({
      key: "c",
      ctrlKey: true,
      altKey: false,
      metaKey: false,
    }),
  ).toBe("\u0003");
  expect(
    encodeTerminalKey({
      key: "j",
      ctrlKey: false,
      altKey: false,
      metaKey: true,
    }),
  ).toBe(null);
  expect(encodePaste("one\ntwo", true)).toBe("\u001b[200~one\ntwo\u001b[201~");
  expect(accumulateScrollLines({ remainderPx: 0 }, -34, 17).lines).toBe(-2);
});

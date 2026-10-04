import { test, expect } from "vitest";
import { isSvelteRuntimeDiagnostic } from "../.storybook/svelte-runtime-diagnostics.js";
import { preparePortableApplicationCanvas } from "../.storybook/portable-canvas.js";
test("recognizes_svelte_diagnostics", () => {
  expect(isSvelteRuntimeDiagnostic(["[svelte] hydration_mismatch"])).toBe(true);
  expect(isSvelteRuntimeDiagnostic(["ordinary message"])).toBe(false);
});
test("portable_canvas_is_scoped", () => {
  const toggles: unknown[] = [];
  const body = {
    classList: { toggle: (...args: unknown[]) => toggles.push(args) },
  };
  const doc = { body, getElementById: () => null } as unknown as Document;
  const canvas = { parentElement: body, id: "" } as unknown as HTMLElement;
  preparePortableApplicationCanvas(doc, canvas, true);
  expect(canvas.id).toBe("storybook-root");
  expect(toggles).toEqual([["sb-main-fullscreen", true]]);
});

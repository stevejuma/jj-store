import { test, expect } from "vitest";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { isSvelteRuntimeDiagnostic } from "../.storybook/svelte-runtime-diagnostics.js";
import { preparePortableApplicationCanvas } from "../.storybook/portable-canvas.js";
import { storeStorySections } from "../.storybook/story-sections.js";
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
test("store stories are grouped under ordered domain sections", () => {
  const walk = (directory: string): string[] =>
    readdirSync(directory).flatMap((name) => {
      const path = join(directory, name);
      return statSync(path).isDirectory() ? walk(path) : [path];
    });
  const sections = new Set(storeStorySections);
  for (const path of walk("catalog/components").filter((path) =>
    path.endsWith(".stories.svelte"),
  )) {
    const title = readFileSync(path, "utf8").match(
      /title: "(JJ Store\/[^"]+)"/,
    )?.[1];
    expect(title, path).toBeDefined();
    const parts = title!.split("/");
    expect(parts.length, title).toBeGreaterThanOrEqual(3);
    expect(
      sections.has(parts[1] as (typeof storeStorySections)[number]),
      title,
    ).toBe(true);
  }
});

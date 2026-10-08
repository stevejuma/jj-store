import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { expect, test } from "vitest";

test("Settings and agent option descendants never render native dropdowns", () => {
  const seen = new Set<string>();
  const pending = [
    "web/src/lib/ui/store-settings/StoreSettings.svelte",
    "web/src/lib/ui/store-agent-execution/StoreAgentAdvanced.svelte",
  ].map((path) => resolve(path));
  const offenders: string[] = [];
  while (pending.length) {
    const path = pending.pop()!;
    if (seen.has(path)) continue;
    seen.add(path);
    const source = readFileSync(path, "utf8");
    if (/<select(?:\s|>)/.test(source)) offenders.push(path);
    for (const match of source.matchAll(/from\s+["'](\.[^"']+\.svelte)["']/g))
      pending.push(resolve(dirname(path), match[1]));
  }
  expect(seen.size).toBeGreaterThan(20);
  expect(offenders).toEqual([]);
});

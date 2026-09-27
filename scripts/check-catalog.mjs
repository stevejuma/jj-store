import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
export function catalogErrors(source) {
  return /(?:from\s*|import\s*\()?["'][^"']*(?:nostr|turso|community-controller|(?:web\/src\/lib\/)?api\/client|web\/src\/(?:App|main))[^"']*["']|\b(?:fetch|WebSocket|EventSource)\s*\(/u.test(
    source,
  )
    ? [
        "catalog must use production presentation inputs and local fixtures, without transports",
      ]
    : [];
}
const files = readdirSync("catalog", { recursive: true }).filter((p) =>
  /\.(?:ts|svelte)$/.test(p),
);
const errors = files.flatMap((p) =>
  catalogErrors(readFileSync(join("catalog", p), "utf8")).map(
    (e) => `${p}: ${e}`,
  ),
);
if (errors.length) {
  console.error(errors.join("\n"));
  process.exitCode = 1;
} else console.log(`catalog boundaries ok (${files.length} modules)`);

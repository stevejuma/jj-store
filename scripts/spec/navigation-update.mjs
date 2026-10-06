import { readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import {
  indexBody,
  writeNavigation,
  writeIndexPages,
} from "./navigation-pages.mjs";
const read = (p) => readFileSync(p, "utf8");
export function updateNavigation(root = "spec/src") {
  function walk(dir) {
    const items = readdirSync(dir, { withFileTypes: true }).filter(
      (e) => !e.name.startsWith("SUMMARY"),
    );
    const links = [];
    for (const e of items
      .filter(
        (e) =>
          e.isFile() &&
          e.name.endsWith(".md") &&
          e.name !== "index.md" &&
          !(dir === root && e.name === "previous-locations.md"),
      )
      .sort((a, b) => a.name.localeCompare(b.name))) {
      const title = read(join(dir, e.name)).match(/^# (.+)$/m)?.[1] ?? e.name;
      links.push(`- [${title}](${e.name})`);
    }
    const fileLinkCount = links.length;
    for (const e of items
      .filter((e) => e.isDirectory())
      .sort((a, b) =>
        a.name.localeCompare(b.name, undefined, { numeric: true }),
      )) {
      const child = join(dir, e.name);
      const index = join(child, "index.md");
      if (!readdirSync(child).includes("index.md"))
        writeFileSync(index, `# ${e.name}\n\nBrowse the child pages below.\n`);
      walk(child);
      const title = read(index).match(/^# (.+)$/m)?.[1] ?? e.name;
      links.push(
        `- [${title}](${e.name}/index.md)`,
        `  {{#include ${e.name}/SUMMARY.nav.md}}`,
      );
    }
    if (dir === root && fileLinkCount) {
      const previous = links.splice(0, fileLinkCount);
      const extra = writeIndexPages(
        join(root, "previous-locations.md"),
        "# Previous chapter links\n\nThese compatibility pages retain the former chapter paths and anchors. Start with the current specification for implementation.\n",
        previous,
      );
      links.push(
        "- [Previous chapter links](previous-locations.md)",
        ...previous.map((link) => "  " + link),
        ...extra.map((link) => "  " + link),
      );
    }
    if (dir !== root) {
      const index = join(dir, "index.md");
      const source = indexBody(read(index));
      links.push(
        ...writeIndexPages(
          index,
          source,
          links.filter((line) => !line.includes("{{#include")),
        ),
      );
    }
    writeNavigation(dir, links);
  }
  walk(root);
  writeFileSync(
    join(root, "SUMMARY.md"),
    "# Summary\n\n- [Specification](index.md)\n{{#include SUMMARY.nav.md}}\n",
  );
}

import {
  existsSync,
  mkdirSync,
  readFileSync,
  writeFileSync,
  rmSync,
} from "node:fs";
import { dirname, join, relative } from "node:path";
export const navigationMarker = "\n<!-- generated-navigation -->";
export function indexBody(source) {
  const fieldEnd = source.lastIndexOf("<!-- /evidence-field -->");
  const from = fieldEnd < 0 ? 0 : fieldEnd + "<!-- /evidence-field -->".length;
  const marked = source.indexOf(navigationMarker, from);
  const legacy = source.indexOf("\n## Pages\n", from);
  const offset = marked >= 0 ? marked : legacy;
  return offset >= 0 ? source.slice(0, offset) : source;
}
const fits = (source) =>
  Buffer.byteLength(source) <= 32768 &&
  source.split("\n").length - (source.endsWith("\n") ? 1 : 0) <= 400;
function rebase(line, from, to) {
  const target = (path) => relative(to, join(from, path)).replaceAll("\\", "/");
  return line
    .replace(/\]\(([^)]+)\)/g, (_, path) => `](${target(path)})`)
    .replace(
      /\{\{#include ([^}]+)\}\}/g,
      (_, path) => `{{#include ${target(path)}}}`,
    );
}
export function writeNavigation(dir, lines) {
  const parts = join(dir, "SUMMARY.nav.parts");
  if (existsSync(parts)) rmSync(parts, { recursive: true, force: true });
  const source = lines.join("\n") + "\n";
  if (fits(source)) {
    writeFileSync(join(dir, "SUMMARY.nav.md"), source);
    return;
  }
  const batches = [];
  let batch = [];
  const candidate = (rows) =>
    rows.map((line) => rebase(line, dir, join(parts, "001"))).join("\n") + "\n";
  for (const line of lines) {
    if (batch.length && !fits(candidate([...batch, line]))) {
      batches.push(batch);
      batch = [];
    }
    batch.push(line);
    if (!fits(candidate(batch)))
      throw new Error(`navigation entry exceeds the authored budget in ${dir}`);
  }
  if (batch.length) batches.push(batch);
  const includes = [];
  for (let i = 0; i < batches.length; i++) {
    const name = String(i + 1).padStart(3, "0");
    const folder = join(parts, name);
    mkdirSync(folder, { recursive: true });
    writeFileSync(
      join(folder, "SUMMARY.nav.md"),
      batches[i].map((line) => rebase(line, dir, folder)).join("\n") + "\n",
    );
    includes.push(`{{#include SUMMARY.nav.parts/${name}/SUMMARY.nav.md}}`);
  }
  const output = includes.join("\n") + "\n";
  if (!fits(output))
    throw new Error(`navigation needs another folder boundary in ${dir}`);
  writeFileSync(join(dir, "SUMMARY.nav.md"), output);
}
export function writeIndexPages(file, body, links) {
  const dir = dirname(file);
  const group = `SUMMARY.index-${file.split(/[\\/]/).at(-1).slice(0, -3)}`;
  const folder = join(dir, group);
  if (existsSync(folder)) rmSync(folder, { recursive: true, force: true });
  const title = body.match(/^# (.+)$/m)?.[1] ?? "Pages";
  const batches = [];
  let offset = 0;
  const location = (part) =>
    part ? join(folder, String(part + 1).padStart(3, "0"), "index.md") : file;
  const page = (part, rows, more) => {
    const output = location(part);
    const heading = part
      ? `# ${title}: pages ${part + 1}\n\n[Back to the index](${relative(dirname(output), file).replaceAll("\\", "/")})\n`
      : body.trimEnd();
    const next = more
      ? `\n- [Next page](${relative(dirname(output), location(part + 1)).replaceAll("\\", "/")})\n`
      : "";
    return (
      heading +
      navigationMarker +
      "\n## Pages\n\n" +
      rows.map((line) => rebase(line, dir, dirname(output))).join("\n") +
      "\n" +
      next
    );
  };
  do {
    const rows = [];
    const part = batches.length;
    while (
      offset < links.length &&
      rows.length < 180 &&
      fits(page(part, [...rows, links[offset]], offset + 1 < links.length))
    ) {
      rows.push(links[offset++]);
    }
    if (!rows.length && offset < links.length)
      throw new Error(`index content leaves no navigation room: ${file}`);
    batches.push(rows);
  } while (offset < links.length);
  const added = [];
  for (let part = 0; part < batches.length; part++) {
    const output = location(part);
    const source = page(part, batches[part], part + 1 < batches.length);
    if (!fits(source)) throw new Error(`oversized generated index: ${output}`);
    mkdirSync(dirname(output), { recursive: true });
    writeFileSync(output, source);
    if (part)
      added.push(
        `- [${title}: pages ${part + 1}](${relative(dir, output).replaceAll("\\", "/")})`,
      );
  }
  return added;
}

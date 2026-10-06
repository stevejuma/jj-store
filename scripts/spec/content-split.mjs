import { relative, dirname, posix } from "node:path";
export const slug = (text) =>
  text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "")
    .slice(0, 90) || "overview";
export function compactTables(source) {
  return source
    .split("\n")
    .map((line) =>
      line.startsWith("|")
        ? "| " +
          line
            .trim()
            .slice(1, -1)
            .split(/(?<!\\)\|/)
            .map((c) => c.trim())
            .join(" | ") +
          " |"
        : line,
    )
    .join("\n");
}
export function splitContent(source, title) {
  const blocks = [];
  let active = [];
  let fenced = false;
  let marker = "";
  for (const line of source.split("\n")) {
    const fence = /^\s*(`{3,}|~{3,})/.exec(line);
    if (fence) {
      if (!fenced) {
        fenced = true;
        marker = fence[1][0];
      } else if (fence[1][0] === marker) fenced = false;
    }
    if (!fenced && /^#{1,4} /.test(line) && active.length) {
      blocks.push(active.join("\n"));
      active = [];
    }
    active.push(line);
  }
  if (active.length) blocks.push(active.join("\n"));
  const output = [];
  for (const block of blocks) {
    const name = /^#{1,4} (.+)$/m.exec(block)?.[1] ?? title;
    const units = block.split(/\n\n/);
    let body = "";
    let count = 1;
    const emit = () => {
      if (body.trim()) {
        const heading = count === 1 ? name : `${name} (continued ${count})`;
        const lines = body.split("\n");
        if (
          /^#{1,4} /.test(lines[0]) &&
          lines[0].replace(/^#{1,4} /, "") === name
        )
          lines.shift();
        output.push({
          title: heading,
          source: `# ${heading}\n\n${lines.join("\n").trimStart()}\n`,
        });
      }
      body = "";
      count++;
    };
    for (const unit of units) {
      const candidate = body + (body ? "\n\n" : "") + unit;
      if (
        Buffer.byteLength(candidate) > 24 * 1024 ||
        candidate.split("\n").length > 310
      )
        emit();
      if (Buffer.byteLength(unit) > 28 * 1024 || unit.split("\n").length > 360)
        throw new Error(`oversized atomic Markdown block in ${name}`);
      body += (body ? "\n\n" : "") + unit;
    }
    emit();
  }
  return output;
}
export function rewriteLinks(source, destination, original, relocation) {
  return source.replace(
    /\]\(([^\s)]+\.md)(#[^)]*)?\)/g,
    (whole, target, fragment = "") => {
      if (target.startsWith("?") || /^[a-z]+:/i.test(target)) return whole;
      const before = posix.normalize(
        posix.join(posix.dirname(original), target),
      );
      const mapped = relocation[before];
      const after =
        fragment && mapped?.anchors?.[fragment.slice(1)]
          ? mapped.anchors[fragment.slice(1)]
          : (mapped?.target ?? before);
      return `](${relative(dirname(destination), after).replaceAll("\\", "/")}${fragment})`;
    },
  );
}

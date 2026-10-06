import { parse, stringify } from "yaml";
const fieldStart =
  /^<!-- evidence-field:([A-Za-z_][A-Za-z_0-9]*) (text|markdown|yaml|list|runs) -->$/gm;
const end = "\n<!-- /evidence-field -->";
const fence = (value) =>
  "`".repeat(
    Math.max(3, ...[...value.matchAll(/`+/g)].map((m) => m[0].length + 1)),
  );
function textBlock(value, language = "") {
  const f = fence(value);
  return `${f}${language}\n${value}\n${f}`;
}
function mapProse(value, transform) {
  let offset = 0,
    output = "";
  for (const match of value.matchAll(
    /(`+)[\s\S]*?\1|(~{3,})[^\n]*\n[\s\S]*?\n\2/g,
  )) {
    output += transform(value.slice(offset, match.index)) + match[0];
    offset = match.index + match[0].length;
  }
  return output + transform(value.slice(offset));
}
const encodeProse = (value) =>
  mapProse(value, (part) =>
    part.replace(
      /[&<>]/g,
      (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[c],
    ),
  );
const decodeProse = (value) =>
  mapProse(value, (part) =>
    part.replace(
      /&(amp|lt|gt);/g,
      (_, name) => ({ amp: "&", lt: "<", gt: ">" })[name],
    ),
  );
function runBlock(run, index) {
  const { command, note, ...metadata } = run;
  const meta = {
    ...metadata,
    hasNote: Object.hasOwn(run, "note"),
    noteEncoding: "markdown",
  };
  return `<!-- run -->\n### Run ${index + 1}\n\n${textBlock(stringify(meta), "yaml")}\n\n${textBlock(command, "shell")}\n${note === undefined ? "" : `\n**Notes**\n\n${encodeProse(note)}\n`}<!-- /run -->`;
}
export function renderField(key, value) {
  const type =
    typeof value === "string" && !value.includes("<!-- /evidence-field -->")
      ? "markdown"
      : key === "runs"
        ? "runs"
        : Array.isArray(value) && value.every((v) => typeof v === "string")
          ? "list"
          : "yaml";
  const title =
    key === "runs"
      ? "Validation"
      : key.replaceAll("_", " ").replace(/^./, (c) => c.toUpperCase());
  const body =
    type === "markdown"
      ? encodeProse(value)
      : type === "list"
        ? stringify(value).trimEnd()
        : type === "runs"
          ? value.map(runBlock).join("\n\n")
          : textBlock(stringify(value), "yaml");
  return `<!-- evidence-field:${key} ${type} -->\n## ${title}\n\n${body}${end}\n`;
}
const fencedBlocks = (source) => [
  ...source.matchAll(/^(`{3,}|~{3,})[^\n]*\n([\s\S]*?)\n\1[ \t]*$/gm),
];
const blocks = (source) => fencedBlocks(source).map((match) => match[2]);
function markers(source, pattern) {
  const fences = fencedBlocks(source);
  return [...source.matchAll(pattern)].filter(
    (match) =>
      !fences.some(
        (fence) =>
          match.index >= fence.index &&
          match.index < fence.index + fence[0].length,
      ),
  );
}
export function parseEvidencePage(source) {
  const fields = {};
  const endings = markers(source, /^<!-- \/evidence-field -->$/gm);
  for (const match of markers(source, fieldStart)) {
    const start = match.index + match[0].length + 1;
    const close =
      (endings.find((ending) => ending.index > start)?.index ?? 0) - 1;
    if (close < 0) throw new Error(`unterminated evidence field ${match[1]}`);
    const section = source.slice(start, close);
    const body = section.slice(section.indexOf("\n\n") + 2);
    if (Object.hasOwn(fields, match[1]))
      throw new Error(`duplicate evidence field ${match[1]}`);
    if (match[2] === "text") fields[match[1]] = body;
    else if (match[2] === "markdown") fields[match[1]] = decodeProse(body);
    else if (match[2] === "list") fields[match[1]] = parse(body);
    else if (match[2] === "yaml") fields[match[1]] = parse(blocks(body)[0]);
    else
      fields[match[1]] = markers(body, /^<!-- run -->$/gm).map((runStart) => {
        const ending = markers(body, /^<!-- \/run -->$/gm).find(
          (match) => match.index > runStart.index,
        );
        if (!ending) throw new Error("unterminated evidence run");
        const content = body.slice(
          runStart.index + runStart[0].length + 1,
          ending.index,
        );
        const [metadata, command] = blocks(content);
        const { hasNote, noteEncoding, ...run } = parse(metadata);
        run.command = command;
        if (hasNote) {
          const marker = "\n**Notes**\n\n";
          const offset = content.indexOf(marker);
          if (offset < 0) throw new Error("missing run note");
          const note = content.slice(offset + marker.length, -1);
          run.note = noteEncoding === "markdown" ? decodeProse(note) : note;
        }
        return run;
      });
  }
  return fields;
}
export function mergeEvidencePages(pages) {
  const result = {};
  for (const source of pages)
    for (const [key, value] of Object.entries(parseEvidencePage(source))) {
      if (!Object.hasOwn(result, key)) result[key] = value;
      else if (Array.isArray(value) && Array.isArray(result[key]))
        result[key].push(...value);
      else if (typeof value === "string" && typeof result[key] === "string")
        result[key] += value;
      else throw new Error(`conflicting evidence fragments for ${key}`);
    }
  return result;
}
export function evidencePages(entry) {
  const header = `---\nschema: 1\nid: ${entry.id}\n---\n\n# ${entry.id}\n\n`;
  const fits = (text) =>
    Buffer.byteLength(text) <= 30 * 1024 && text.split("\n").length <= 380;
  const fragments = [];
  for (const [key, value] of Object.entries(entry)) {
    const whole = renderField(key, value);
    if (fits(header + whole)) {
      fragments.push(whole);
      continue;
    }
    if (Array.isArray(value)) {
      let batch = [];
      for (const item of value) {
        if (
          batch.length &&
          !fits(header + renderField(key, [...batch, item]))
        ) {
          fragments.push(renderField(key, batch));
          batch = [];
        }
        batch.push(item);
        if (!fits(header + renderField(key, batch)))
          throw new Error(
            `single ${key} item exceeds page budget in ${entry.id}`,
          );
      }
      if (batch.length) fragments.push(renderField(key, batch));
    } else if (typeof value === "string") {
      const characters = [...value];
      let offset = 0;
      while (offset < characters.length) {
        let low = 1,
          high = characters.length - offset,
          best = 0;
        while (low <= high) {
          const length = Math.floor((low + high) / 2);
          if (
            fits(
              header +
                renderField(
                  key,
                  characters.slice(offset, offset + length).join(""),
                ),
            )
          ) {
            best = length;
            low = length + 1;
          } else high = length - 1;
        }
        if (!best)
          throw new Error(`cannot split ${key} within budget in ${entry.id}`);
        let part = characters.slice(offset, offset + best).join("");
        const newline = part.lastIndexOf("\n");
        if (offset + best < characters.length && newline > part.length / 2)
          part = part.slice(0, newline + 1);
        fragments.push(renderField(key, part));
        offset += [...part].length;
      }
    } else throw new Error(`oversized ${key} field in ${entry.id}`);
  }
  const pages = [];
  let current = header;
  for (const fragment of fragments) {
    if (!fits(current + fragment)) {
      pages.push(current);
      current = header;
    }
    current += fragment + "\n";
  }
  if (current !== header) pages.push(current);
  return pages.map((page, index) =>
    index
      ? page.replace(`# ${entry.id}\n`, `# ${entry.id} details ${index + 1}\n`)
      : page,
  );
}

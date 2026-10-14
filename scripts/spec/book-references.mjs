import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  writeFileSync,
  statSync,
} from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const posix = (value) => value.split(sep).join("/");
const inside = (root, path) => path === root || path.startsWith(root + sep);
function rewriteLinks(source, replace) {
  let fenced = false,
    marker = "";
  return source
    .split("\n")
    .map((line) => {
      const fence = /^\s*(`{3,}|~{3,})/.exec(line);
      if (fence) {
        if (!fenced) {
          fenced = true;
          marker = fence[1][0];
        } else if (fence[1][0] === marker) fenced = false;
        return line;
      }
      if (fenced) return line;
      return line.replace(
        /\]\(([^\s)]+)(?:\s+"([^"]*)")?\)/g,
        (whole, target, title) => {
          const changed = replace(target);
          return changed === target
            ? whole
            : `](${changed}${title ? ` "${title}"` : ""})`;
        },
      );
    })
    .join("\n");
}

export function bundleReferences(book, { repoRoot, stageRoot, tracked }) {
  const canonicalRoot = join(repoRoot, "spec/src");
  const stagedSource = join(stageRoot, "src");
  const references = new Map(),
    assets = new Set();
  function reference(target) {
    const path = posix(relative(repoRoot, target));
    const directory = statSync(target).isDirectory();
    const members = directory
      ? [...tracked].filter((file) => file.startsWith(path + "/"))
      : [];
    if (
      !inside(repoRoot, target) ||
      !(directory ? members.length : tracked.has(path))
    )
      throw new Error(
        `book reference must be a tracked repository file: ${path}`,
      );
    const destination = `_repository/${path}${directory ? "/index.md" : ""}`;
    if (directory || target.endsWith(".md")) {
      if (!references.has(target))
        references.set(target, {
          destination,
          content: directory
            ? `# ${path}\n\nTracked repository files:\n\n${members
                .sort()
                .map(
                  (file) =>
                    `- [${file.slice(path.length + 1)}](${file.slice(path.length + 1)})`,
                )
                .join("\n")}\n`
            : readFileSync(target, "utf8"),
          original: directory ? join(target, "index.md") : target,
        });
    } else if (!assets.has(target)) {
      assets.add(target);
      mkdirSync(dirname(join(stagedSource, destination)), { recursive: true });
      const output = join(stagedSource, destination);
      if (
        !existsSync(output) ||
        !readFileSync(output).equals(readFileSync(target))
      )
        copyFileSync(target, output);
    }
    return destination;
  }
  function rewrite(content, originalPath, bookPath) {
    return rewriteLinks(content, (href) => {
      if (
        !href ||
        href.startsWith("#") ||
        href.startsWith("?") ||
        /^(?:[a-z]+:|\/\/)/i.test(href)
      )
        return href;
      const split = href.search(/[?#]/);
      const filePart = split < 0 ? href : href.slice(0, split);
      const suffix = split < 0 ? "" : href.slice(split);
      let decoded;
      try {
        decoded = decodeURIComponent(filePart);
      } catch {
        return href;
      }
      let target = resolve(dirname(originalPath), decoded);
      if (
        inside(canonicalRoot, target) &&
        existsSync(target) &&
        statSync(target).isDirectory()
      )
        target = join(target, "index.md");
      if (inside(canonicalRoot, target))
        return (
          posix(
            relative(dirname(bookPath), posix(relative(canonicalRoot, target))),
          ) + suffix
        );
      if (!existsSync(target))
        throw new Error(`missing book reference: ${target}`);
      return posix(relative(dirname(bookPath), reference(target))) + suffix;
    });
  }
  function chapters(sections) {
    for (const section of sections) {
      if (!section.Chapter) continue;
      const chapter = section.Chapter;
      if (chapter.source_path)
        chapter.content = rewrite(
          chapter.content,
          join(canonicalRoot, chapter.source_path),
          chapter.path ?? chapter.source_path,
        );
      chapters(chapter.sub_items ?? []);
    }
  }
  const items = book.items ?? book.sections;
  if (!Array.isArray(items)) throw new Error("unsupported mdBook item schema");
  chapters(items);
  const supplements = [];
  for (const [source, record] of references) {
    const content = rewrite(
      record.content,
      record.original,
      record.destination,
    );
    const provenance = `**Repository reference:** \`${posix(relative(repoRoot, source))}\`. Current requirements and milestone status remain in the current specification.\n\n`;
    const output = join(stagedSource, record.destination);
    mkdirSync(dirname(output), { recursive: true });
    if (
      !existsSync(output) ||
      readFileSync(output, "utf8") !== provenance + content
    )
      writeFileSync(output, provenance + content);
    supplements.push({
      Chapter: {
        name:
          record.content.match(/^# (.+)$/m)?.[1] ??
          posix(relative(repoRoot, source)),
        content: provenance + content,
        number: null,
        sub_items: [],
        path: record.destination,
        source_path: record.destination,
        parent_names: ["Repository references"],
      },
    });
  }
  if (supplements.length)
    items.push({ PartTitle: "Repository references" }, ...supplements);
  return book;
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  if (process.argv[2] === "supports")
    process.exit(process.argv[3] === "html" ? 0 : 1);
  let input = "";
  for await (const chunk of process.stdin) input += chunk;
  const [context, book] = JSON.parse(input);
  const stageRoot = resolve(context.root);
  const repoRoot = resolve(stageRoot, "../..");
  const tracked = new Set(
    execFileSync(
      existsSync(join(repoRoot, ".jj")) ? "jj" : "git",
      existsSync(join(repoRoot, ".jj"))
        ? ["--no-pager", "file", "list", "-r", "@"]
        : ["ls-files"],
      {
        cwd: repoRoot,
        encoding: "utf8",
        maxBuffer: 64 * 1024 * 1024,
      },
    )
      .trim()
      .split("\n"),
  );
  process.stdout.write(
    JSON.stringify(bundleReferences(book, { repoRoot, stageRoot, tracked })),
  );
}

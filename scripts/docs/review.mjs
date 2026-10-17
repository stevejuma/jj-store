import { createHash } from "node:crypto";
import {
  existsSync,
  readFileSync,
  readdirSync,
  writeFileSync,
  statSync,
} from "node:fs";
export function sourceFingerprint(paths) {
  const hash = createHash("sha256");
  function add(path) {
    if (statSync(path).isDirectory())
      for (const name of readdirSync(path).sort()) add(`${path}/${name}`);
    else {
      hash.update(path);
      hash.update("\0");
      hash.update(readFileSync(path));
      hash.update("\0");
    }
  }
  for (const path of [...paths].sort()) add(path);
  return hash.digest("hex");
}
export function validateReview(topics) {
  if (!topics.length) throw Error("Documentation coverage is empty");
  const names = new Set();
  for (const topic of topics) {
    if (
      !topic.id ||
      names.has(topic.id) ||
      !topic.sources?.length ||
      !topic.pages?.length
    )
      throw Error("Invalid or duplicate documentation topic");
    names.add(topic.id);
    for (const path of [...topic.sources, ...topic.pages])
      if (path.startsWith("/") || path.split("/").includes(".."))
        throw Error("Unsafe documentation coverage path");
    for (const page of topic.pages)
      if (!readFileSync(page, "utf8").startsWith("#"))
        throw Error(`Missing documentation page: ${page}`);
    if (
      !topic.review?.reason?.trim() ||
      topic.review.reason.trim().length < 20 ||
      topic.review.fingerprint !== sourceFingerprint(topic.sources)
    )
      throw Error(
        `Documentation review required: ${topic.id}. Update its pages, then pnpm docs:review ${topic.id} "specific explanation"`,
      );
  }
}
export function validateCoverage(topics, roots) {
  const paths = topics.flatMap((topic) => topic.sources);
  for (const root of roots)
    if (!paths.some((path) => root === path || root.startsWith(path + "/")))
      throw Error(`Unmapped documentation source: ${root}`);
}
export function review(id, reason) {
  const path = "docs/maintenance/coverage.json";
  const topics = JSON.parse(readFileSync(path, "utf8"));
  validateCoverage(topics, [
    ...readdirSync("crates", { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => `crates/${entry.name}/src`),
    ...(existsSync("packages")
      ? readdirSync("packages", { withFileTypes: true })
          .filter((entry) => entry.isDirectory())
          .map((entry) => `packages/${entry.name}/src`)
      : []),
    "web/src",
    "desktop/src",
    "schemas/openapi.json",
  ]);
  if (id) {
    if (!reason || reason.trim().length < 20)
      throw Error("A topic-specific review explanation is required");
    const topic = topics.find((t) => t.id === id);
    if (!topic) throw Error("Unknown documentation topic");
    topic.review = { reason, fingerprint: sourceFingerprint(topic.sources) };
    writeFileSync(path, JSON.stringify(topics, null, 2) + "\n");
  } else validateReview(topics);
}

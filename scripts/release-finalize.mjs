import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { version, validateTag } from "./release-metadata.mjs";
const tag = process.env.RELEASE_TAG,
  source = process.env.RELEASE_SOURCE;
validateTag(tag, version());
if (!/^[a-f0-9]{40}$/.test(source || "")) throw Error("invalid release source");
const gh = (args) => execFileSync("gh", args, { encoding: "utf8" });
const repository = "stevejuma/jj-store";
let existing;
try {
  existing = JSON.parse(gh(["api", `repos/${repository}/git/ref/tags/${tag}`]));
} catch (error) {
  if (!String(error.stderr).includes("404")) throw error;
}
if (
  existing &&
  (existing.object.type !== "commit" || existing.object.sha !== source)
)
  throw Error("immutable tag differs from qualified source");
if (!existing)
  gh([
    "api",
    "--method",
    "POST",
    `repos/${repository}/git/refs`,
    "-f",
    `ref=refs/tags/${tag}`,
    "-f",
    `sha=${source}`,
  ]);
const checksumFiles = readdirSync("dist").filter((n) => n.endsWith(".sha256"));
if (checksumFiles.length !== 4)
  throw Error("expected four native archive checksums");
writeFileSync(
  "dist/SHA256SUMS",
  checksumFiles
    .map((n) => readFileSync(`dist/${n}`, "utf8"))
    .sort()
    .join(""),
);
execFileSync("shasum", ["-a", "256", "-c", "SHA256SUMS"], {
  cwd: "dist",
  stdio: "inherit",
});
const images = ["amd64", "arm64"].map((arch) => {
  const digest = readFileSync(`dist/image-${arch}.digest`, "utf8").trim();
  if (!/^sha256:[a-f0-9]{64}$/.test(digest))
    throw Error("invalid qualified image digest");
  return `ghcr.io/${repository}@${digest}`;
});
let old;
try {
  old = JSON.parse(
    execFileSync(
      "docker",
      [
        "buildx",
        "imagetools",
        "inspect",
        `ghcr.io/${repository}:${tag}`,
        "--format",
        "{{json .Manifest}}",
      ],
      { encoding: "utf8" },
    ),
  );
} catch (error) {
  if (!/not found|manifest unknown/i.test(String(error.stderr))) throw error;
}
if (old) {
  const digests = new Set(old.manifests.map((m) => m.digest));
  for (const image of images)
    if (!digests.has(image.split("@")[1]))
      throw Error(
        "versioned image already has different architecture contents",
      );
} else
  execFileSync(
    "docker",
    [
      "buildx",
      "imagetools",
      "create",
      "--tag",
      `ghcr.io/${repository}:${tag}`,
      ...images,
    ],
    { stdio: "inherit" },
  );
try {
  gh(["release", "view", tag]);
} catch {
  gh([
    "release",
    "create",
    tag,
    "--verify-tag",
    "--draft",
    "--title",
    `jj-store ${tag}`,
    "--notes-file",
    "CHANGELOG.md",
  ]);
}
execFileSync(process.execPath, ["scripts/release-assets.mjs"], {
  stdio: "inherit",
});
gh(["release", "edit", tag, "--draft=false"]);
execFileSync(
  "docker",
  [
    "buildx",
    "imagetools",
    "create",
    "--tag",
    `ghcr.io/${repository}:latest`,
    `ghcr.io/${repository}:${tag}`,
  ],
  { stdio: "inherit" },
);
console.log(
  "Qualified package, binary and image release finalized. Verify public anonymous image access before reporting completion.",
);

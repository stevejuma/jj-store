import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";

export const repository = "stevejuma/jj-store";
export const workflow = ".github/workflows/npm-publish-jj-conversations.yml";
export const packageName = "@lapismd/jj-conversations";
export const packageDirectory = "packages/jj-conversations";
export const artifactDirectory = ".release/conversations";
export const integrity = (bytes) =>
  "sha512-" + createHash("sha512").update(bytes).digest("base64");
export const run = (command, args, options = {}) =>
  execFileSync(command, args, { stdio: "inherit", ...options });
export const output = (command, args, options = {}) =>
  run(command, args, { stdio: "pipe", encoding: "utf8", ...options }).trim();
export const readJson = (path) => JSON.parse(readFileSync(path, "utf8"));
export const writeJson = (path, value) =>
  writeFileSync(path, JSON.stringify(value, null, 2) + "\n");

export function validatePackage(pkg) {
  if (
    pkg.name !== packageName ||
    pkg.private ||
    !/^\d+\.\d+\.\d+(?:-[\w.-]+)?$/.test(pkg.version) ||
    pkg.publishConfig?.access !== "public" ||
    pkg.repository?.url !== `https://github.com/${repository}.git` ||
    pkg.repository?.directory !== packageDirectory ||
    Object.keys(pkg.dependencies ?? {}).length ||
    Object.keys(pkg.optionalDependencies ?? {}).length
  )
    throw Error("Invalid independent public Conversations package identity");
  for (const range of Object.values(pkg.devDependencies ?? {}))
    if (
      !/^[~^\d*><=| ]/.test(range) ||
      /(?:workspace:|file:|link:|git\+|\/Users\/)/.test(range)
    )
      throw Error(
        "Conversations package contains non-registry dependency metadata",
      );
}

export function validateManifest(manifest, expected, bytes) {
  if (
    manifest.schema !== 1 ||
    manifest.repository !== repository ||
    manifest.workflow !== workflow ||
    manifest.name !== packageName ||
    manifest.source !== expected.source ||
    !/^[a-f0-9]{40}$/.test(manifest.source) ||
    manifest.run !== expected.run ||
    manifest.attempt !== expected.attempt ||
    !/^\d+$/.test(manifest.run) ||
    !/^\d+$/.test(manifest.attempt) ||
    typeof manifest.tarball !== "string" ||
    basename(manifest.tarball) !== manifest.tarball ||
    !manifest.tarball.endsWith(".tgz") ||
    manifest.integrity !== integrity(bytes) ||
    manifest.size !== bytes.length
  )
    throw Error(
      "Conversations artifact source, qualification or checksum mismatch",
    );
}

export function expectedArtifact(env = process.env) {
  return {
    source: env.JJ_CONVERSATIONS_SOURCE ?? env.GITHUB_SHA,
    run: env.JJ_CONVERSATIONS_ARTIFACT_RUN ?? env.GITHUB_RUN_ID,
    attempt: env.JJ_CONVERSATIONS_ARTIFACT_ATTEMPT ?? env.GITHUB_RUN_ATTEMPT,
  };
}

export function loadArtifact(
  root = process.cwd(),
  expected = expectedArtifact(),
) {
  const directory = join(root, artifactDirectory);
  const manifest = readJson(join(directory, "manifest.json"));
  if (
    typeof manifest.tarball !== "string" ||
    basename(manifest.tarball) !== manifest.tarball
  )
    throw Error("Unsafe Conversations archive path");
  const path = join(directory, manifest.tarball),
    bytes = readFileSync(path);
  validateManifest(manifest, expected, bytes);
  const pkg = readJson(join(root, packageDirectory, "package.json"));
  validatePackage(pkg);
  if (manifest.version !== pkg.version)
    throw Error("Conversations artifact version differs from qualified source");
  const packed = JSON.parse(
    output("tar", ["-xOf", path, "package/package.json"]),
  );
  validatePackage(packed);
  if (packed.version !== manifest.version)
    throw Error("Conversations archive version differs from its manifest");
  return { manifest, path, bytes, directory };
}

export function prepareArtifact(root = process.cwd(), env = process.env) {
  const expected = expectedArtifact(env);
  if (
    env.GITHUB_REPOSITORY !== repository ||
    env.JJ_CONVERSATIONS_HEAVY_QUALIFIED !== "1" ||
    env.GITHUB_SHA !== expected.source ||
    !/^[a-f0-9]{40}$/.test(expected.source ?? "") ||
    !/^\d+$/.test(expected.run ?? "") ||
    !/^\d+$/.test(expected.attempt ?? "")
  )
    throw Error(
      "Conversations packing requires complete exact-source qualification",
    );
  const pkg = readJson(join(root, packageDirectory, "package.json"));
  validatePackage(pkg);
  const directory = join(root, artifactDirectory);
  rmSync(directory, { recursive: true, force: true });
  mkdirSync(directory, { recursive: true });
  run("npm", ["pack", "--ignore-scripts", "--pack-destination", directory], {
    cwd: join(root, packageDirectory),
  });
  const tarball = `lapismd-jj-conversations-${pkg.version}.tgz`;
  const path = join(directory, tarball),
    bytes = readFileSync(path);
  const files = output("tar", ["-tf", path]).split("\n");
  for (const required of [
    "dist/index.js",
    "dist/index.d.ts",
    "README.md",
    "LICENSE.md",
    "NOTICE.md",
  ])
    if (!files.includes(`package/${required}`))
      throw Error(`Missing npm input: ${required}`);
  for (const file of files)
    if (
      !file.startsWith("package/") ||
      /(?:^|\/)(?:node_modules|\.git|\.local|\.env)(?:\/|$)/.test(file) ||
      /\.(?:test|spec)\./.test(file)
    )
      throw Error(`Forbidden npm archive entry: ${file}`);
  const manifest = {
    schema: 1,
    repository,
    workflow,
    name: packageName,
    version: pkg.version,
    ...expected,
    tarball,
    size: bytes.length,
    integrity: integrity(bytes),
    files,
  };
  writeJson(join(directory, "manifest.json"), manifest);
  loadArtifact(root, expected);
  console.log(
    `Prepared exact qualified ${packageName}@${pkg.version}: ${manifest.integrity}`,
  );
  return manifest;
}

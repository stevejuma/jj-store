import { mkdtempSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import {
  integrity,
  sha256,
  loadArtifact,
  output,
  packageName,
  prepareArtifact,
  repository,
  run,
  writeJson,
  readJson,
} from "./conversations-npm-artifact.mjs";
import { plan } from "./conversations-npm-plan.mjs";

export async function registry(manifest) {
  const response = await fetch(
    `https://registry.npmjs.org/${encodeURIComponent(packageName)}/${manifest.version}`,
    { headers: { "Cache-Control": "no-cache" } },
  );
  if (response.status === 404) return null;
  if (!response.ok)
    throw Error(`npm metadata request failed: ${response.status}`);
  return response.json();
}

export function validateRegistry(manifest, metadata, bytes) {
  if (
    metadata.name !== packageName ||
    metadata.version !== manifest.version ||
    metadata.dist?.integrity !== manifest.integrity ||
    integrity(bytes) !== manifest.integrity ||
    sha256(bytes) !== manifest.sha256 ||
    bytes.length !== manifest.size
  )
    throw Error(
      "Downloaded Conversations version differs from the qualified artifact",
    );
  const provenance = metadata.dist?.attestations?.provenance;
  if (provenance != null && typeof provenance !== "object")
    throw Error("Conversations registry provenance is malformed");
}

export async function publish(root = process.cwd(), env = process.env) {
  const { manifest, path } = loadArtifact(root);
  if (
    env.GITHUB_REPOSITORY !== repository ||
    env.GITHUB_SHA !== manifest.source ||
    ![
      "refs/heads/main",
      `refs/tags/jj-conversations-v${manifest.version}`,
    ].includes(env.GITHUB_REF)
  )
    throw Error(
      "Conversations publication requires the exact qualified main/tag workflow",
    );
  const existing = await registry(manifest);
  if (existing) {
    const response = await fetch(existing.dist?.tarball);
    if (!response.ok)
      throw Error(`npm tarball request failed: ${response.status}`);
    validateRegistry(
      manifest,
      existing,
      Buffer.from(await response.arrayBuffer()),
    );
    console.log(
      "Exact Conversations version already exists; preserving the immutable package",
    );
    return;
  }
  if (env.GITHUB_REF === "refs/heads/main" && !env.NODE_AUTH_TOKEN)
    throw Error(
      "Initial publication needs the protected bootstrap credential or the signed attest artifact staged by its owner",
    );
  run(
    "npm",
    [
      "publish",
      path,
      "--provenance",
      "--access=public",
      "--tag=latest",
      "--registry=https://registry.npmjs.org",
      "--ignore-scripts",
    ],
    { cwd: root },
  );
}

export async function verifyRegistry(root = process.cwd()) {
  const { manifest, directory } = loadArtifact(root);
  let metadata;
  for (let attempt = 0; attempt < 24; attempt++) {
    metadata = await registry(manifest);
    if (metadata) break;
    await new Promise((done) => setTimeout(done, 5000));
  }
  if (!metadata)
    throw Error("Published Conversations version is not downloadable");
  const response = await fetch(metadata.dist?.tarball);
  if (!response.ok)
    throw Error(`npm tarball request failed: ${response.status}`);
  validateRegistry(
    manifest,
    metadata,
    Buffer.from(await response.arrayBuffer()),
  );
  const consumer = mkdtempSync(join(tmpdir(), "jj-conversations-registry-"));
  try {
    writeJson(join(consumer, "package.json"), {
      name: "conversations-registry-acceptance",
      private: true,
      version: "0.0.0",
      type: "module",
      dependencies: { [packageName]: manifest.version },
    });
    run(
      "npm",
      [
        "install",
        "--ignore-scripts",
        "--strict-peer-deps",
        "--registry=https://registry.npmjs.org",
      ],
      { cwd: consumer },
    );
    run(
      "npm",
      [
        "ci",
        "--ignore-scripts",
        "--strict-peer-deps",
        "--registry=https://registry.npmjs.org",
      ],
      { cwd: consumer },
    );
    run(
      "node",
      [
        "--input-type=module",
        "-e",
        `import assert from 'node:assert/strict'; import { ConversationClient } from '${packageName}'; assert.equal(typeof ConversationClient, 'function'); assert.equal(typeof ConversationClient.prototype.restoreUnknown, 'function');`,
      ],
      { cwd: consumer },
    );
    if (metadata.dist?.attestations?.provenance)
      run("npm", ["audit", "signatures", "--include-attestations"], {
        cwd: consumer,
      });
    writeJson(
      join(directory, "consumer-package.json"),
      readJson(join(consumer, "package.json")),
    );
    writeJson(
      join(directory, "consumer-package-lock.json"),
      readJson(join(consumer, "package-lock.json")),
    );
    writeJson(join(directory, "registry-receipt.json"), {
      schema: 1,
      name: packageName,
      version: manifest.version,
      source: manifest.source,
      qualification: { run: manifest.run, attempt: manifest.attempt },
      integrity: manifest.integrity,
      tarball: metadata.dist.tarball,
      attestations: metadata.dist.attestations,
      signatures: metadata.dist.signatures,
      consumer:
        "strict npm install, frozen npm ci, public runtime import and signature/attestation verification",
      verified: new Date().toISOString(),
    });
  } finally {
    rmSync(consumer, { recursive: true, force: true });
  }
  console.log(
    `Verified actual registry publication ${packageName}@${manifest.version}`,
  );
}

export function notes(root = process.cwd()) {
  const { manifest, path, directory } = loadArtifact(root);
  const receipt = readJson(join(directory, "registry-receipt.json"));
  if (
    receipt.name !== packageName ||
    receipt.version !== manifest.version ||
    receipt.source !== manifest.source ||
    receipt.integrity !== manifest.integrity
  )
    throw Error("GitHub release requires the matching actual registry receipt");
  const tag = `jj-conversations-v${manifest.version}`;
  let existing;
  try {
    existing = JSON.parse(
      output("gh", [
        "release",
        "view",
        tag,
        "--repo",
        repository,
        "--json",
        "targetCommitish",
      ]),
    );
  } catch (error) {
    if (!String(error.stderr ?? "").includes("release not found")) throw error;
  }
  if (existing) {
    if (existing.targetCommitish !== manifest.source)
      throw Error("Existing Conversations release targets another source");
    return;
  }
  run(
    "gh",
    [
      "release",
      "create",
      tag,
      path,
      join(directory, "manifest.json"),
      join(directory, "registry-receipt.json"),
      "--repo",
      repository,
      "--target",
      manifest.source,
      "--title",
      `${packageName}@${manifest.version}`,
      "--generate-notes",
    ],
    { cwd: root },
  );
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  const commands = {
    plan,
    prepare: prepareArtifact,
    "verify-artifact": loadArtifact,
    attest: async () =>
      (await import("./conversations-provenance.mjs")).prepareProvenance(),
    publish,
    verify: verifyRegistry,
    notes,
  };
  const command = commands[process.argv[2]];
  if (!command || process.argv.length !== 3)
    throw Error(
      "Usage: conversations-npm.mjs plan|prepare|verify-artifact|attest|publish|verify|notes",
    );
  await command();
}

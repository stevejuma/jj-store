import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

export function dependencyErrors(packages) {
  const errors = [];
  for (const pkg of packages)
    for (const dependency of pkg.dependencies ?? []) {
      if (
        pkg.name === "jj-store-conversations" &&
        ["jj-store-core", "jj-store-model", "jj-agent", "jj-collab"].includes(
          dependency.name,
        ) &&
        dependency.kind !== "dev" &&
        dependency.optional !== true
      )
        errors.push(
          `${pkg.name}: store capture interfaces must remain optional`,
        );
      if (
        /^lapis-(?:ai-|acp|agent|compute)/.test(dependency.name) &&
        !(
          (pkg.name === "jj-store-server" &&
            ["lapis-ai-runtime", "lapis-ai-config"].includes(
              dependency.name,
            )) ||
          (pkg.name === "jj-store-conversations" &&
            dependency.name === "lapis-agent-redaction" &&
            dependency.optional === true) ||
          (pkg.name === "jj-store-capture" &&
            ["lapis-agent-redaction", "lapis-ai-capture"].includes(
              dependency.name,
            ))
        )
      )
        errors.push(`${pkg.name} cannot depend on ${dependency.name}`);
    }

  const native = new Set(["jj-store-jj", "jj-store"]);
  const layers = {
    "jj-forge": [],
    "jj-store-read-index": [],
    "jj-store-model": [],
    "jj-store-core": [
      "jj-store-model",
      "jj-agent",
      "jj-review",
      "jj-work",
      "jj-collab",
    ],
    "jj-store-native": [
      "jj-store-core",
      "jj-store-model",
      "jj-store-wal",
      "jj-agent",
      "jj-review",
      "jj-work",
      "jj-collab",
      "jj-store-object",
    ],
    "jj-store-git": [
      "jj-store-core",
      "jj-store-model",
      "jj-agent",
      "jj-review",
      "jj-work",
      "jj-collab",
    ],
    "jj-work": ["jj-store-model"],
    "jj-workflow": ["jj-store-model", "jj-collab"],
    "jj-agent": ["jj-store-model"],
    "jj-review": ["jj-agent", "jj-store-model"],
    "jj-collab": ["jj-agent", "jj-review", "jj-work", "jj-store-model"],
    "jj-store-object": ["jj-store-model"],
    "jj-store-wal": [
      "jj-store-core",
      "jj-workflow",
      "jj-collab",
      "jj-work",
      "jj-agent",
      "jj-review",
      "jj-store-model",
      "jj-store-object",
    ],
    "jj-store-replication": [
      "jj-agent",
      "jj-review",
      "jj-store-model",
      "jj-store-object",
      "jj-store-wal",
    ],
    "jj-store-jj": [
      "jj-store-git",
      "jj-forge",
      "jj-store-model",
      "jj-store-object",
      "jj-store-wal",
    ],
    "jj-store-service": [
      "jj-store-native",
      "jj-store-git",
      "jj-store-core",
      "jj-forge",
      "jj-store-read-index",
      "jj-workflow",
      "jj-collab",
      "jj-review",
      "jj-store-adapters",
      "jj-store-model",
      "jj-store-jj",
      "jj-store-object",
      "jj-store-wal",
    ],
    "jj-store-rpc": ["jj-store-model", "jj-store-object", "jj-store-service"],
    "jj-store-conversations": [
      "jj-store-core",
      "jj-store-model",
      "jj-agent",
      "jj-collab",
    ],
    "tauri-plugin-jj-store": ["jj-store-jj", "jj-store-model"],
    "jj-store-capture": [
      "jj-agent",
      "jj-collab",
      "jj-store-model",
      "jj-store-wal",
      "jj-store-service",
      "jj-store-rpc",
    ],
    "jj-store-server": [
      "jj-store-conversations",
      "jj-workflow",
      "jj-agent",
      "jj-collab",
      "jj-store-capture",
      "jj-store-adapters",
      "jj-store-tantivy",
      "jj-store-http",
      "jj-store-model",
      "jj-store-object",
      "jj-store-jj",
      "jj-store-replication",
      "jj-store-service",
      "jj-store-rpc",
    ],
    "jj-store-http": [
      "jj-store-model",
      "jj-store-service",
      "jj-store-adapters",
      "jj-store-tantivy",
    ],
    "jj-store-adapters": ["jj-store-model", "jj-store-object", "jj-store-wal"],
    "jj-store-tantivy": [
      "jj-store-model",
      "jj-store-adapters",
      "jj-store-service",
    ],
  };
  for (const pkg of packages) {
    for (const dep of pkg.dependencies) {
      if (["jj-lib", "jj-cli"].includes(dep.name) && !native.has(pkg.name))
        errors.push(`${pkg.name}: upstream ${dep.name} outside JJ boundary`);
      if (
        /^(?:turso|rusqlite|sqlx|libsql|nostr|redis|postgres)$/.test(dep.name)
      )
        errors.push(`${pkg.name}: excluded core dependency ${dep.name}`);
      if (
        (dep.name === "jj-store" ||
          dep.name.startsWith("jj-store-") ||
          [
            "jj-forge",
            "jj-agent",
            "jj-review",
            "jj-work",
            "jj-workflow",
            "jj-collab",
          ].includes(dep.name)) &&
        layers[pkg.name] &&
        !layers[pkg.name].includes(dep.name) &&
        dep.kind !== "dev"
      )
        errors.push(`${pkg.name}: invalid layer dependency ${dep.name}`);
    }
  }
  return errors;
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const root = fileURLToPath(new URL("..", import.meta.url));
  try {
    const metadata = JSON.parse(
      execFileSync(
        "cargo",
        ["metadata", "--no-deps", "--format-version", "1", "--locked"],
        { cwd: root, encoding: "utf8" },
      ),
    );
    const errors = dependencyErrors(metadata.packages);
    if (errors.length) throw new Error(errors.join("\n"));
    console.log(`package boundaries ok (${metadata.packages.length} packages)`);
  } catch (err) {
    console.error(err.message);
    process.exitCode = 1;
  }
}

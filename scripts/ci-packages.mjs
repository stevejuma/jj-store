import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";

export const heavyPackages = [
  "jj-store",
  "jj-store-server",
  "jj-store-capture",
];
const portable = (path) => path.split(sep).join("/");
export function packageGraph(data, root = process.cwd()) {
  if (!Array.isArray(data?.workspace_members))
    throw Error("Invalid Cargo workspace metadata");
  const members = new Set(data.workspace_members);
  if (!members.size || !Array.isArray(data.packages))
    throw Error("Invalid Cargo workspace metadata");
  const packages = data.packages
    .filter((p) => members.has(p.id))
    .map((p) => {
      const directory = dirname(p.manifest_path);
      const path = portable(relative(root, directory));
      if (
        !/^[A-Za-z0-9_-]+$/.test(p.name) ||
        !path ||
        path.startsWith("../") ||
        !Array.isArray(p.dependencies)
      )
        throw Error("Invalid workspace package");
      return {
        name: p.name,
        path,
        directory: resolve(directory),
        dependencies: p.dependencies,
      };
    });
  if (
    packages.length !== members.size ||
    new Set(packages.map((p) => p.name)).size !== members.size
  )
    throw Error("Incomplete or duplicate workspace packages");
  const byPath = new Map(packages.map((p) => [p.directory, p]));
  const reverse = new Map(packages.map((p) => [p.name, new Set()]));
  for (const p of packages)
    for (const dep of p.dependencies) {
      if (!dep.path) continue;
      const target = byPath.get(resolve(dep.path));
      if (!target) {
        const sharedAi =
          ["lapis-ai-runtime", "lapis-ai-config"].includes(dep.name) &&
          p.name === "jj-store-server" &&
          resolve(dep.path) === resolve(root, `../lapis-ai/crates/${dep.name}`);
        const sharedPrivacy =
          (p.name === "jj-store-capture" ||
            (p.name === "jj-store-conversations" && dep.optional === true)) &&
          dep.name === "lapis-agent-redaction" &&
          resolve(dep.path) ===
            resolve(root, "../agent-redaction/crates/lapis-agent-redaction");
        const sharedAcquisition =
          p.name === "jj-store-capture" &&
          dep.name === "lapis-ai-capture" &&
          resolve(dep.path) ===
            resolve(root, "../lapis-ai/crates/lapis-ai-capture");
        if (sharedAi || sharedPrivacy || sharedAcquisition) continue;
        throw Error(`Unmodelled path dependency: ${p.name} -> ${dep.path}`);
      }
      reverse.get(target.name).add(p.name);
    }
  return { packages, reverse };
}
export function workspaceGraph(root = process.cwd()) {
  const data = JSON.parse(
    execFileSync(
      "cargo",
      [
        "metadata",
        "--no-deps",
        "--locked",
        "--offline",
        "--format-version",
        "1",
      ],
      { cwd: root, encoding: "utf8", maxBuffer: 16 * 1024 * 1024 },
    ),
  );
  return packageGraph(data, root);
}
export function cohortPackages(graph, cohort) {
  if (!["ordinary", "heavy"].includes(cohort)) throw Error("Unknown cohort");
  return graph.packages
    .filter((p) => heavyPackages.includes(p.name) === (cohort === "heavy"))
    .map((p) => p.name)
    .sort();
}
export function selectedPackages(
  cohort,
  selection = process.env.CI_PACKAGES,
  graph = workspaceGraph(),
) {
  const allowed = cohortPackages(graph, cohort);
  const selected = selection === undefined ? allowed : JSON.parse(selection);
  if (
    !Array.isArray(selected) ||
    !selected.length ||
    new Set(selected).size !== selected.length ||
    selected.some((p) => !allowed.includes(p))
  )
    throw Error("Invalid CI package selection");
  if (cohort === "heavy" && selected.length !== allowed.length)
    throw Error("Heavy qualification requires every package");
  return [...selected].sort();
}
export function reverseClosure(graph, initial) {
  const selected = new Set(initial),
    pending = [...initial];
  while (pending.length)
    for (const dependent of graph.reverse.get(pending.pop()) ?? [])
      if (!selected.has(dependent)) {
        selected.add(dependent);
        pending.push(dependent);
      }
  return [...selected].sort();
}

// Cargo does not describe sibling test fixtures imported through #[path] or
// include macros. Audit these source edges without inferring per-test coverage.
export function sourceEdges(graph, root = process.cwd()) {
  const inputs = new Map();
  inputs.uncertain = [];
  const add = (file, owner) => {
    const owners = inputs.get(file) ?? new Set();
    owners.add(owner);
    inputs.set(file, owners);
  };
  for (const pkg of graph.packages) {
    const walk = (dir) => {
      for (const item of readdirSync(dir, { withFileTypes: true })) {
        if (
          item.name.startsWith(".") ||
          ["target", "node_modules"].includes(item.name)
        )
          continue;
        const path = resolve(dir, item.name);
        if (item.isSymbolicLink())
          throw Error(`Unmodelled source symlink: ${path}`);
        if (item.isDirectory()) {
          walk(path);
          continue;
        }
        if (!item.isFile() || !path.endsWith(".rs")) continue;
        const text = readFileSync(path, "utf8");
        if (/include(?:_str|_bytes)?!\s*\(\s*[^"\s]/.test(text))
          inputs.uncertain.push(portable(relative(root, path)));
        const refs = [
          ...text.matchAll(
            /(?:include(?:_str|_bytes)?!\s*\(\s*|#\s*\[\s*path\s*=\s*)"([^"\n]+)"/g,
          ),
        ];
        for (const match of refs) {
          // In-file module nesting makes relative attributes ambiguous. Broaden
          // to the referenced package below as well as retaining the exact file.
          const input = portable(
            relative(root, resolve(dirname(path), match[1])),
          );
          if (input.startsWith("../"))
            throw Error(`Source input outside repository: ${input}`);
          add(input, pkg.name);
          const target = graph.packages.find((p) =>
            input.startsWith(p.path + "/"),
          );
          if (target && target.name !== pkg.name)
            graph.reverse.get(target.name).add(pkg.name);
        }
      }
    };
    walk(pkg.directory);
  }
  return inputs;
}

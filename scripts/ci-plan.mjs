import { execFileSync, spawnSync } from "node:child_process";
import { appendFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import {
  cohortPackages,
  heavyPackages,
  reverseClosure,
  sourceEdges,
  workspaceGraph,
} from "./ci-packages.mjs";

export function impact(
  files,
  full = false,
  graph = workspaceGraph(),
  inputs = new Map(),
) {
  const selected = new Set(),
    reasons = [],
    standard = cohortPackages(graph, "ordinary");
  let all = full,
    web = full;
  if (files.length && inputs.uncertain?.length) {
    all = web = true;
    reasons.push(
      `Unmodelled dynamic source inputs: ${inputs.uncertain.join(", ")}`,
    );
  }
  if (full) reasons.push("All standard checks requested (main or all_checks).");
  for (const file of files) {
    for (const owner of inputs.get(file) ?? []) selected.add(owner);
    if (
      /^(?:Cargo\.|rust-toolchain|rustfmt\.|pnpm-|package\.json$|\.cargo\/|\.github\/|\.config\/|scripts\/|schemas\/|proto\/|fixtures\/)/.test(
        file,
      ) ||
      /(?:^|\/)Cargo\.(?:toml|lock)$/.test(file) ||
      file.endsWith("/build.rs")
    ) {
      all = web = true;
      reasons.push(`${file}: shared build, test or contract input`);
      continue;
    }
    const pkg = graph.packages.find((p) => file.startsWith(p.path + "/"));
    if (pkg) {
      selected.add(pkg.name);
      reasons.push(`${file}: package ${pkg.name}`);
      if (!file.endsWith(".rs")) {
        all = true;
        reasons.push(
          `${file}: fixture/generated input; all standard Rust checks`,
        );
      }
      if (/^crates\/jj-store-(?:model|http|rpc)\//.test(file)) web = true;
      continue;
    }
    if (/^(?:web\/|catalog\/|\.storybook\/|docs\/site\/assets\/)/.test(file)) {
      web = true;
      reasons.push(`${file}: web/catalog input`);
      continue;
    }
    if (
      /^(?:spec\/|\.changeset\/[^/]+\.md$|README\.md$|LICENSE\.md$|docs\/releases\.md$)/.test(
        file,
      )
    ) {
      reasons.push(`${file}: documentation/release note`);
      continue;
    }
    if (/^runtime\/embedded-rustfs\//.test(file)) {
      reasons.push(`${file}: manual embedded qualification`);
      continue;
    }
    all = web = true;
    reasons.push(`${file}: unknown ownership; all standard checks`);
  }
  const closure = reverseClosure(graph, selected);
  if (
    closure.some((name) =>
      ["jj-store-model", "jj-store-http", "jj-store-rpc"].includes(name),
    )
  )
    web = true;
  const packages = all
    ? standard
    : closure.filter((name) => standard.includes(name));
  const excluded = closure.filter((name) => heavyPackages.includes(name));
  for (const name of closure)
    if (!selected.has(name))
      reasons.push(`${name}: depends on changed package or shared source`);
  return { rust: packages.length > 0, web, packages, excluded, reasons };
}
export function changedFiles(base, head, root = process.cwd()) {
  // Treat renames as deletion + addition so both owners are considered.
  return execFileSync(
    "git",
    ["diff", "--no-renames", "--name-only", "-z", base, head],
    { cwd: root, encoding: "utf8" },
  )
    .split("\0")
    .filter(Boolean);
}
export function cacheWritable({ event, ref, repository, sourceOnMain }) {
  if (repository !== "stevejuma/jj-store") return false;
  if (event === "pull_request")
    return /^refs\/pull\/\d+\/merge$/.test(ref ?? "");
  return (
    ["push", "workflow_dispatch"].includes(event) &&
    ref === "refs/heads/main" &&
    sourceOnMain === true
  );
}
export function sourceOnMain(source) {
  return (
    spawnSync(
      "git",
      ["merge-base", "--is-ancestor", source, "refs/remotes/origin/main"],
      { stdio: "ignore" },
    ).status === 0
  );
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(process.argv[1]).href
) {
  const source = process.env.JJ_STORE_CHECK_REVISION;
  const base = process.env.JJ_STORE_CHECK_BASE;
  if (
    !/^[a-f0-9]{40,64}$/.test(source ?? "") ||
    !/^[a-f0-9]{40,64}$/.test(base ?? "")
  )
    throw Error("Missing validated CI source range");
  const graph = workspaceGraph(),
    inputs = sourceEdges(graph);
  const files = changedFiles(base, source);
  const result = impact(
    files,
    process.env.CI_ALL_CHECKS === "true" ||
      (process.env.CI_EVENT === "push" &&
        process.env.CI_REF === "refs/heads/main"),
    graph,
    inputs,
  );
  const cache_write = cacheWritable({
    event: process.env.CI_EVENT,
    ref: process.env.CI_REF,
    repository: process.env.GITHUB_REPOSITORY,
    sourceOnMain: sourceOnMain(source),
  });
  for (const [key, value] of Object.entries({
    source,
    rust: result.rust,
    web: result.web,
    packages: JSON.stringify(result.packages),
    cache_write,
  }))
    appendFileSync(process.env.GITHUB_OUTPUT, `${key}=${value}\n`);
  const plan = { source, base, files, ...result, cache_write };
  console.log(JSON.stringify(plan, null, 2));
  if (process.env.GITHUB_STEP_SUMMARY)
    appendFileSync(
      process.env.GITHUB_STEP_SUMMARY,
      `## Standard check selection\n\nSource: \`${source}\`\n\nSelected packages: ${result.packages.join(", ") || "none"}\n\nManual cohort: ${heavyPackages.join(", ")}\n\nAffected manual packages: ${result.excluded.join(", ") || "none"}\n\nWeb checks: ${result.web}. Cache writes: ${cache_write}.\n\n${result.reasons.map((r) => "- " + r.replace(/[\r\n]/g, " ")).join("\n")}\n`,
    );
}

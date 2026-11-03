import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { lstat, readdir } from "node:fs/promises";
import { join, relative, resolve } from "node:path";

export async function filesUnder(root, directory = root) {
  assert.ok(
    (await lstat(directory)).isDirectory(),
    "Artifact directory is not regular",
  );
  const files = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) files.push(...(await filesUnder(root, path)));
    else {
      assert.ok(entry.isFile(), `Unsafe artifact entry: ${path}`);
      files.push(relative(root, path).split("\\").join("/"));
    }
  }
  return files.sort();
}

export async function safeFile(root, file) {
  // Reject symlinks in every ancestor, not just the final executable.
  let path = process.cwd();
  for (const component of relative(path, resolve(root, file)).split(/[\\/]/)) {
    assert.notEqual(component, "..");
    path = join(path, component);
    assert.ok(
      !(await lstat(path)).isSymbolicLink(),
      `Artifact symlink: ${path}`,
    );
  }
  assert.ok((await lstat(path)).isFile(), `Artifact is not a file: ${path}`);
  return path;
}

export async function githubApi(path) {
  const response = await fetch(
    `https://api.github.com/repos/${process.env.GITHUB_REPOSITORY}/${path}`,
    {
      headers: {
        Authorization: `Bearer ${process.env.GH_TOKEN}`,
        Accept: "application/vnd.github+json",
      },
      signal: AbortSignal.timeout(30000),
    },
  );
  if (!response.ok) throw Error(`GitHub API ${path}: ${response.status}`);
  return response.json();
}

export function downloadArtifact(name, root, expected) {
  execFileSync(
    "gh",
    [
      "run",
      "download",
      expected.run,
      "--repo",
      expected.repository,
      "--name",
      `${name}-${expected.attempt}`,
      "--dir",
      root,
    ],
    { stdio: "inherit" },
  );
}

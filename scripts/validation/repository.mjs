import { execFileSync } from "node:child_process";
import { dirname, resolve } from "node:path";
export function repositoryGitRoot(root) {
  try {
    return execFileSync("jj", ["git", "root"], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch {
    return execFileSync("git", ["rev-parse", "--absolute-git-dir"], {
      cwd: root,
      encoding: "utf8",
    }).trim();
  }
}
export function repositoryParent(root) {
  const git = repositoryGitRoot(root);
  const main = git.endsWith("/.jj/repo/store/git")
    ? resolve(git, "../../../..")
    : dirname(git);
  return dirname(main);
}
export function snapshotFile(root, revision, path) {
  return execFileSync(
    "git",
    ["--git-dir", repositoryGitRoot(root), "show", `${revision}:${path}`],
    { cwd: root, maxBuffer: 64 * 1024 * 1024 },
  );
}

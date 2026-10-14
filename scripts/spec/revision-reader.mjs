import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { execFileSync } from "node:child_process";
export function revisionReader(root, requested = "@") {
  if (!existsSync(resolve(root, ".jj")))
    return gitRevisionReader(root, requested);
  const jj = (args) =>
    execFileSync("jj", ["--no-pager", "--color=never", ...args], {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 16 * 1024 * 1024,
      stdio: ["ignore", "pipe", "pipe"],
    });
  const revision = jj([
    "log",
    "-r",
    requested,
    "--no-graph",
    "-T",
    "commit_id",
  ]).trim();
  if (!/^[a-f0-9]{40}$/.test(revision))
    throw new Error("selected revision is ambiguous or unavailable");
  const working = jj([
    "log",
    "-r",
    "@",
    "--no-graph",
    "-T",
    "commit_id",
  ]).trim();
  const different = new Set(
    jj(["diff", "--name-only", "--from", revision, "--to", working])
      .trim()
      .split("\n")
      .filter(Boolean),
  );
  const files = jj(["file", "list", "-r", revision])
    .trim()
    .split("\n")
    .filter(Boolean);
  const present = new Set(files);
  const cache = new Map();
  const read = (file) => {
    if (!present.has(file))
      throw new Error(`missing file at ${revision}: ${file}`);
    if (!cache.has(file))
      cache.set(
        file,
        different.has(file)
          ? jj(["file", "show", "-r", revision, file])
          : readFileSync(resolve(root, file), "utf8"),
      );
    return cache.get(file);
  };
  const parents = jj([
    "log",
    "-r",
    `${revision}-`,
    "--no-graph",
    "-T",
    'commit_id ++ "\\n"',
  ])
    .trim()
    .split("\n")
    .filter(Boolean);
  const changed = [
    ...new Set(
      parents.flatMap((parent) =>
        jj(["diff", "--name-only", "--from", parent, "--to", revision])
          .trim()
          .split("\n")
          .filter(Boolean),
      ),
    ),
  ];
  const verify = () => {
    if (
      jj(["log", "-r", "@", "--no-graph", "-T", "commit_id"]).trim() !== working
    )
      throw new Error(
        "working copy changed during revision-pinned validation; rerun the check",
      );
  };
  return { read, files, revision, parents, changed, verify };
}

function gitRevisionReader(root, requested) {
  const git = (args) =>
    execFileSync("git", args, {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
      stdio: ["ignore", "pipe", "pipe"],
    });
  const selection = requested === "@" ? "HEAD" : requested;
  const revision = git(["rev-parse", "--verify", selection]).trim();
  if (!/^[a-f0-9]{40,64}$/.test(revision))
    throw Error("invalid selected Git revision");
  const kind = git(["cat-file", "-t", revision]).trim();
  const empty = kind === "tree";
  if (empty && git(["ls-tree", revision]).trim())
    throw Error("only the empty baseline tree is accepted");
  const head = git(["rev-parse", "HEAD"]).trim();
  const files = git(["ls-tree", "-r", "--name-only", revision])
    .trim()
    .split("\n")
    .filter(Boolean);
  const present = new Set(files),
    cache = new Map();
  const read = (file) => {
    if (!present.has(file)) throw Error(`missing file at ${revision}: ${file}`);
    if (!cache.has(file)) cache.set(file, git(["show", `${revision}:${file}`]));
    return cache.get(file);
  };
  const parents = empty
    ? []
    : git(["rev-list", "--parents", "-n", "1", revision])
        .trim()
        .split(/\s+/)
        .slice(1);
  const baseline = process.env.JJ_STORE_CHECK_BASE;
  const changed = empty
    ? []
    : baseline &&
        (requested === "@" || revision === process.env.JJ_STORE_CHECK_REVISION)
      ? git(["diff", "--name-only", baseline, revision])
          .trim()
          .split("\n")
          .filter(Boolean)
      : [
          ...new Set(
            parents.flatMap((p) =>
              git(["diff", "--name-only", p, revision])
                .trim()
                .split("\n")
                .filter(Boolean),
            ),
          ),
        ];
  const verify = () => {
    if (git(["rev-parse", "HEAD"]).trim() !== head)
      throw Error("Git selection changed during validation");
  };
  return { read, files, revision, parents, changed, verify };
}

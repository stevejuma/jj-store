import { mkdir, readFile, rename, rm, stat, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { workSeedId } from "./dev-demo-work-samples.mjs";
async function exists(path) {
  try {
    await stat(path);
    return true;
  } catch (error) {
    if (error.code === "ENOENT") return false;
    throw error;
  }
}

/** Additional owned fixtures. Native code is published by jj; Git code stays local. */
export async function seedRepositoryProfiles({
  repositoryRoot,
  runtimeRoot,
  repositoryId,
  unixSocket,
  run,
}) {
  const marker = join(runtimeRoot, "repository-profiles.json");
  let state = { version: 1, nativeLocal: repositoryId };
  try {
    state = JSON.parse(await readFile(marker, "utf8"));
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
  }
  if (
    state.version !== 1 ||
    state.nativeLocal !== repositoryId ||
    [state.nativeLocal, state.nativeServer, state.localGit]
      .filter(Boolean)
      .some((id) => !/^[0-9a-f]{32}$/u.test(id))
  )
    throw Error("invalid development repository profiles marker");
  const save = async () => {
    await writeFile(`${marker}.tmp`, `${JSON.stringify(state, null, 2)}\n`, {
      mode: 0o600,
    });
    await rename(`${marker}.tmp`, marker);
  };
  const binary = resolve(repositoryRoot, "target/debug/jj");
  const env = { JJ_CONFIG: join(runtimeRoot, "jj.toml") };
  const jj = (folder, args) =>
    run(binary, ["-R", folder, "--no-pager", "--color=never", ...args], {
      env,
    });
  const history = async (folder, title, description) => {
    const main = await jj(folder, ["bookmark", "list", "main", "-T", "name"]);
    if (main.stdout.trim()) return;
    await mkdir(join(folder, "src"), { recursive: true });
    await writeFile(
      join(folder, "README.md"),
      `# ${title}\n\n${description}\n`,
    );
    await writeFile(
      join(folder, "src/example.ts"),
      'export const greeting: string = "Hello from JJ";\n',
    );
    await jj(folder, ["describe", "-m", `Seed ${title}`]);
    await jj(folder, ["bookmark", "create", "main", "-r", "@"]);
  };
  const serverRoot = join(runtimeRoot, "server-only-seed");
  const serverFolder = join(serverRoot, "repository");
  if (!state.nativeServer) {
    await mkdir(serverRoot, { recursive: true, mode: 0o700 });
    const created = await run(
      binary,
      [
        "store",
        "init",
        serverFolder,
        "--name",
        "demo/server-only",
        "--unix-socket",
        unixSocket,
        "--cache",
        join(serverRoot, "cache"),
        "--state-dir",
        join(serverRoot, "init-state"),
        "--workspace",
        "seed",
        "--format=json",
      ],
      { env },
    );
    await history(
      serverFolder,
      "Server-only native repository",
      "This repository is stored in jj-store. Its temporary seed checkout is removed after publication; no local folder is linked.",
    );
    state.nativeServer = JSON.parse(created.stdout).repository;
    await save();
  }
  // Save identity before removal so restart can finish cleanup without recreating it.
  if (await exists(serverFolder)) {
    const workspaces = await jj(serverFolder, [
      "workspace",
      "list",
      "-T",
      'name ++ "\\n"',
    ]);
    if (workspaces.stdout.split("\n").includes("seed"))
      await jj(serverFolder, ["workspace", "forget", "seed"]);
  }
  await rm(serverRoot, { recursive: true, force: true });
  const localFolder = join(runtimeRoot, "git-repository");
  if (!state.localGit) {
    if (!(await exists(join(localFolder, ".jj"))))
      await run(binary, ["git", "init", "--colocate", localFolder], { env });
    await history(
      localFolder,
      "Local Git-backed JJ repository",
      "Ordinary JJ Git storage with local companion metadata. Browser serving of this profile is pending; use the local project CLI.",
    );
    const initialized = JSON.parse(
      (
        await jj(localFolder, [
          "agent",
          "initialize",
          "--request-id",
          workSeedId(repositoryId, "request", "initialize-local-git"),
          "--format=json",
        ])
      ).stdout,
    );
    if (initialized.status !== "committed")
      throw Error(
        "local fixture metadata initialization outcome unknown; rerun with the retained request ID",
      );
    const diagnostic = JSON.parse(
      (await jj(localFolder, ["store", "doctor", "--format=json"])).stdout,
    );
    if (
      diagnostic.profile !== "local_jj_git" ||
      !diagnostic.metadata_initialized
    )
      throw Error(
        "local fixture is not an initialized Git-backed JJ repository",
      );
    state.localGit = diagnostic.repository;
    await save();
  }
  if (
    (
      await readFile(join(localFolder, ".jj/repo/store/type"), "utf8")
    ).trim() !== "git"
  )
    throw Error("local fixture storage type changed");
  const selected = JSON.parse(
    (await jj(localFolder, ["store", "doctor", "--format=json"])).stdout,
  );
  if (
    selected.repository !== state.localGit ||
    selected.profile !== "local_jj_git"
  )
    throw Error("local fixture metadata identity changed");
  const repositoryLinks = [
    state.nativeLocal,
    state.nativeServer,
    state.localGit,
  ];
  if (
    new Set(repositoryLinks).size !== 3 ||
    repositoryLinks.some((id) => !/^[0-9a-f]{32}$/u.test(id))
  )
    throw Error(
      "development profiles require three distinct repository identities",
    );
  return { ...state, localFolder, repositoryLinks };
}
